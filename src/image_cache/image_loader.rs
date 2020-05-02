use std;
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::mpsc::{channel, Receiver, Sender, TryRecvError};
use std::sync::{Arc, Mutex};
use std::thread;

use gelatin::glium;
use gelatin::image::{self, gif::GifDecoder, AnimationDecoder, ImageFormat};

use glium::texture::{MipmapsOption, RawImage2d, SrgbTexture2d};

pub mod errors {
	use gelatin::glium::texture;
	use gelatin::image;
	use std::io;

	error_chain! {
		foreign_links {
			Io(io::Error) #[doc = "Error during IO"];
			TextureCreationError(texture::TextureCreationError);
			ImageLoadError(image::ImageError);
		}
	}
}

use self::errors::*;

/// We want to prevent prefetch operations taking place when the target image is not yet loaded.
/// To implement this we define a variable that is read by the loader threads and
/// which will only carry out the request if the focused request id matches their request or
/// if the focused is set to `NO_FOCUSED_REQUEST`
pub static FOCUSED_REQUEST_ID: AtomicU32 = AtomicU32::new(0); // The first request usually
pub const NO_FOCUSED_REQUEST: u32 = std::u32::MAX;

pub fn load_image(image_path: &Path) -> Result<image::RgbaImage> {
	Ok(image::open(image_path)?.to_rgba())
}

pub fn texture_from_image(
	display: &glium::Display,
	image: image::RgbaImage,
) -> Result<SrgbTexture2d> {
	let dimensions = image.dimensions();
	let data = image.into_raw();
	let raw_image = RawImage2d::from_raw_rgba(data, dimensions);

	let x_pow = (31 as u32) - dimensions.0.leading_zeros();
	let y_pow = (31 as u32) - dimensions.1.leading_zeros();

	let max_mipmap_levels = x_pow.min(y_pow).min(4);

	let mipmaps = if max_mipmap_levels == 1 {
		MipmapsOption::NoMipmap
	} else {
		MipmapsOption::AutoGeneratedMipmapsMax(max_mipmap_levels)
	};
	Ok(SrgbTexture2d::with_mipmaps(display, raw_image, mipmaps)?)
}

pub fn is_file_supported(filename: &Path) -> bool {
	if let Some(ext) = filename.extension() {
		if let Some(ext) = ext.to_str() {
			let ext = ext.to_lowercase();
			match ext.as_str() {
				"jpg" | "jpeg" | "png" | "gif" | "webp" | "tif" | "tiff" | "tga" | "bmp"
				| "ico" | "hdr" | "pbm" | "pam" | "ppm" | "pgm" => {
					return true;
				}
				_ => (),
			}
		}
	}
	false
}

pub struct LoadRequest {
	pub req_id: u32,
	pub path: PathBuf,
}

pub enum LoadResult {
	Start { req_id: u32, metadata: fs::Metadata },
	Frame { req_id: u32, image: image::RgbaImage, delay_nano: u64 },
	Done { req_id: u32 },
	Failed { req_id: u32 },
}

impl LoadResult {
	pub fn req_id(&self) -> u32 {
		match self {
			LoadResult::Start { req_id, .. } => *req_id,
			LoadResult::Frame { req_id, .. } => *req_id,
			LoadResult::Done { req_id, .. } => *req_id,
			LoadResult::Failed { req_id, .. } => *req_id,
		}
	}
}

pub struct ImageLoader {
	running: Arc<AtomicBool>,
	join_handles: Option<Vec<thread::JoinHandle<()>>>,
	image_rx: Receiver<LoadResult>,
	path_tx: Sender<LoadRequest>,
}

impl ImageLoader {
	/// # Arguemnts
	/// * `capacity` - Number of bytes. The last image loaded will be the one at which the allocated memory reaches or exceeds capacity
	pub fn new(threads: u32) -> ImageLoader {
		let running = Arc::new(AtomicBool::from(true));
		let (load_request_tx, load_request_rx) = channel();
		let load_request_rx = Arc::new(Mutex::new(load_request_rx));

		let (loaded_img_tx, loaded_img_rx) = channel();

		let mut join_handles = Vec::new();
		for _ in 0..threads {
			let running = running.clone();
			let request_recv = load_request_rx.clone();
			let request_send = load_request_tx.clone();
			let img_sender = loaded_img_tx.clone();
			join_handles.push(thread::spawn(move || {
				Self::thread_loop(running, request_recv, request_send, img_sender);
			}));
		}

		ImageLoader {
			running,
			join_handles: Some(join_handles),

			image_rx: loaded_img_rx,
			path_tx: load_request_tx,
		}
	}

	fn thread_loop(
		running: Arc<AtomicBool>,
		request_recv: Arc<Mutex<Receiver<LoadRequest>>>,
		request_send: Sender<LoadRequest>,
		img_sender: Sender<LoadResult>,
	) {
		// The size was an arbitrary choice made with the argument that this should be
		// enough to fit enough image file info to determine the format.

		while running.load(Ordering::Acquire) {
			let request;
			{
				// It is very important that we release the mutex before starting to load the image
				let load_request = request_recv.lock().unwrap();
				let focused = FOCUSED_REQUEST_ID.load(Ordering::Relaxed);
				request = load_request.recv().unwrap();
				let focus_test_passed = focused == request.req_id || focused == NO_FOCUSED_REQUEST;
				if !focus_test_passed {
					// Just place the request neatly back to the request queue.
					request_send.send(request).unwrap();
					continue;
				}
			};
			Self::load_and_send(&img_sender, request);
		}
	}

	pub fn try_recv_prefetched(&mut self) -> std::result::Result<LoadResult, TryRecvError> {
		self.image_rx.try_recv()
	}

	pub fn send_load_request(&mut self, request: LoadRequest) {
		self.path_tx.send(request).unwrap();
	}

	fn load_and_send(img_sender: &Sender<LoadResult>, request: LoadRequest) {
		let mut file_start_bytes = [0; 512];
		let mut load_succeeded = false;
		if let Ok(metadata) = fs::metadata(&request.path) {
			let mut is_gif = false;
			if let Ok(mut file) = fs::File::open(&request.path) {
				if file.read_exact(&mut file_start_bytes).is_ok() {
					if let Ok(ImageFormat::Gif) = image::guess_format(&file_start_bytes) {
						is_gif = true;
					}
				}
			}
			img_sender.send(LoadResult::Start { req_id: request.req_id, metadata }).unwrap();
			if is_gif {
				if let Ok(file) = fs::File::open(&request.path) {
					if let Ok(decoder) = GifDecoder::new(file) {
						let frames = decoder.into_frames();
						load_succeeded = true;
						for frame in frames {
							if let Ok(frame) = frame {
								let (numerator_ms, denom_ms) = frame.delay().numer_denom_ms();
								let numerator_nano = numerator_ms as u64 * 1_000_000;
								let denom_nano = denom_ms as u64;
								let delay_nano = numerator_nano / denom_nano;
								let image = frame.into_buffer();
								img_sender
									.send(LoadResult::Frame {
										req_id: request.req_id,
										image,
										delay_nano,
									})
									.unwrap();
							} else {
								load_succeeded = false;
								break;
							}
						}
					}
				}
			} else {
				if let Ok(image) = load_image(request.path.as_path()) {
					img_sender
						.send(LoadResult::Frame { req_id: request.req_id, image, delay_nano: 0 })
						.unwrap();
					load_succeeded = true;
				}
			}
		}
		if load_succeeded {
			img_sender.send(LoadResult::Done { req_id: request.req_id }).unwrap();
		} else {
			img_sender.send(LoadResult::Failed { req_id: request.req_id }).unwrap();
		}
	}
}

impl Drop for ImageLoader {
	fn drop(&mut self) {
		self.running.store(false, Ordering::Release);
		if let Some(join_handles) = self.join_handles.take() {
			for _ in join_handles.iter() {
				self.path_tx.send(LoadRequest { req_id: 0, path: PathBuf::from("") }).unwrap();
			}

			for handle in join_handles.into_iter() {
				if let Err(err) = handle.join() {
					eprintln!("Error occured while joining handle {:?}", err);
				}
			}
		}
	}
}
