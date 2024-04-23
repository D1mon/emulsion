use gelatin::winit::keyboard::{Key, NamedKey};
use log::warn;

/// Returns the textual name of the key as written in the config file.
///
/// (Right side, eg "Return" is the string that should be used in the config file)
pub fn virtual_keycode_to_string(key: &Key) -> String {
	match key {
		Key::Unidentified(_) => Default::default(),
		Key::Character(ch) => ch.to_string(),
		Key::Dead(ch) => ch.map_or(Default::default(), |ch| ch.into()),
		Key::Named(named_key) => match named_key {
			// ------------------------------------------------------
			// Cases where the variant name doesn't match the config string
			NamedKey::Enter => "Return".into(),
			NamedKey::ArrowDown => "Down".into(),
			NamedKey::ArrowLeft => "Left".into(),
			NamedKey::ArrowRight => "Right".into(),
			NamedKey::ArrowUp => "Up".into(),
			// ------------------------------------------------------
			NamedKey::Alt => "Alt".into(),
			NamedKey::AltGraph => "AltGraph".into(),
			NamedKey::CapsLock => "CapsLock".into(),
			NamedKey::Control => "Control".into(),
			NamedKey::Fn => "Fn".into(),
			NamedKey::FnLock => "FnLock".into(),
			NamedKey::NumLock => "NumLock".into(),
			NamedKey::ScrollLock => "ScrollLock".into(),
			NamedKey::Shift => "Shift".into(),
			NamedKey::Symbol => "Symbol".into(),
			NamedKey::SymbolLock => "SymbolLock".into(),
			NamedKey::Meta => "Meta".into(),
			NamedKey::Hyper => "Hyper".into(),
			NamedKey::Super => "Super".into(),
			NamedKey::Tab => "Tab".into(),
			NamedKey::Space => "Space".into(),
			NamedKey::End => "End".into(),
			NamedKey::Home => "Home".into(),
			NamedKey::PageDown => "PageDown".into(),
			NamedKey::PageUp => "PageUp".into(),
			NamedKey::Backspace => "Backspace".into(),
			NamedKey::Clear => "Clear".into(),
			NamedKey::Copy => "Copy".into(),
			NamedKey::CrSel => "CrSel".into(),
			NamedKey::Cut => "Cut".into(),
			NamedKey::Delete => "Delete".into(),
			NamedKey::EraseEof => "EraseEof".into(),
			NamedKey::ExSel => "ExSel".into(),
			NamedKey::Insert => "Insert".into(),
			NamedKey::Paste => "Paste".into(),
			NamedKey::Redo => "Redo".into(),
			NamedKey::Undo => "Undo".into(),
			NamedKey::Accept => "Accept".into(),
			NamedKey::Again => "Again".into(),
			NamedKey::Attn => "Attn".into(),
			NamedKey::Cancel => "Cancel".into(),
			NamedKey::ContextMenu => "ContextMenu".into(),
			NamedKey::Escape => "Escape".into(),
			NamedKey::Execute => "Execute".into(),
			NamedKey::Find => "Find".into(),
			NamedKey::Help => "Help".into(),
			NamedKey::Pause => "Pause".into(),
			NamedKey::Play => "Play".into(),
			NamedKey::Props => "Props".into(),
			NamedKey::Select => "Select".into(),
			NamedKey::ZoomIn => "ZoomIn".into(),
			NamedKey::ZoomOut => "ZoomOut".into(),
			NamedKey::BrightnessDown => "BrightnessDown".into(),
			NamedKey::BrightnessUp => "BrightnessUp".into(),
			NamedKey::Eject => "Eject".into(),
			NamedKey::LogOff => "LogOff".into(),
			NamedKey::Power => "Power".into(),
			NamedKey::PowerOff => "PowerOff".into(),
			NamedKey::PrintScreen => "PrintScreen".into(),
			NamedKey::Hibernate => "Hibernate".into(),
			NamedKey::Standby => "Standby".into(),
			NamedKey::WakeUp => "WakeUp".into(),
			NamedKey::AllCandidates => "AllCandidates".into(),
			NamedKey::Alphanumeric => "Alphanumeric".into(),
			NamedKey::CodeInput => "CodeInput".into(),
			NamedKey::Compose => "Compose".into(),
			NamedKey::Convert => "Convert".into(),
			NamedKey::FinalMode => "FinalMode".into(),
			NamedKey::GroupFirst => "GroupFirst".into(),
			NamedKey::GroupLast => "GroupLast".into(),
			NamedKey::GroupNext => "GroupNext".into(),
			NamedKey::GroupPrevious => "GroupPrevious".into(),
			NamedKey::ModeChange => "ModeChange".into(),
			NamedKey::NextCandidate => "NextCandidate".into(),
			NamedKey::NonConvert => "NonConvert".into(),
			NamedKey::PreviousCandidate => "PreviousCandidate".into(),
			NamedKey::Process => "Process".into(),
			NamedKey::SingleCandidate => "SingleCandidate".into(),
			NamedKey::HangulMode => "HangulMode".into(),
			NamedKey::HanjaMode => "HanjaMode".into(),
			NamedKey::JunjaMode => "JunjaMode".into(),
			NamedKey::Eisu => "Eisu".into(),
			NamedKey::Hankaku => "Hankaku".into(),
			NamedKey::Hiragana => "Hiragana".into(),
			NamedKey::HiraganaKatakana => "HiraganaKatakana".into(),
			NamedKey::KanaMode => "KanaMode".into(),
			NamedKey::KanjiMode => "KanjiMode".into(),
			NamedKey::Katakana => "Katakana".into(),
			NamedKey::Romaji => "Romaji".into(),
			NamedKey::Zenkaku => "Zenkaku".into(),
			NamedKey::ZenkakuHankaku => "ZenkakuHankaku".into(),
			NamedKey::Soft1 => "Soft1".into(),
			NamedKey::Soft2 => "Soft2".into(),
			NamedKey::Soft3 => "Soft3".into(),
			NamedKey::Soft4 => "Soft4".into(),
			NamedKey::ChannelDown => "ChannelDown".into(),
			NamedKey::ChannelUp => "ChannelUp".into(),
			NamedKey::Close => "Close".into(),
			NamedKey::MailForward => "MailForward".into(),
			NamedKey::MailReply => "MailReply".into(),
			NamedKey::MailSend => "MailSend".into(),
			NamedKey::MediaClose => "MediaClose".into(),
			NamedKey::MediaFastForward => "MediaFastForward".into(),
			NamedKey::MediaPause => "MediaPause".into(),
			NamedKey::MediaPlay => "MediaPlay".into(),
			NamedKey::MediaPlayPause => "MediaPlayPause".into(),
			NamedKey::MediaRecord => "MediaRecord".into(),
			NamedKey::MediaRewind => "MediaRewind".into(),
			NamedKey::MediaStop => "MediaStop".into(),
			NamedKey::MediaTrackNext => "MediaTrackNext".into(),
			NamedKey::MediaTrackPrevious => "MediaTrackPrevious".into(),
			NamedKey::New => "New".into(),
			NamedKey::Open => "Open".into(),
			NamedKey::Print => "Print".into(),
			NamedKey::Save => "Save".into(),
			NamedKey::SpellCheck => "SpellCheck".into(),
			NamedKey::Key11 => "Key11".into(),
			NamedKey::Key12 => "Key12".into(),
			NamedKey::AudioBalanceLeft => "AudioBalanceLeft".into(),
			NamedKey::AudioBalanceRight => "AudioBalanceRight".into(),
			NamedKey::AudioBassBoostDown => "AudioBassBoostDown".into(),
			NamedKey::AudioBassBoostToggle => "AudioBassBoostToggle".into(),
			NamedKey::AudioBassBoostUp => "AudioBassBoostUp".into(),
			NamedKey::AudioFaderFront => "AudioFaderFront".into(),
			NamedKey::AudioFaderRear => "AudioFaderRear".into(),
			NamedKey::AudioSurroundModeNext => "AudioSurroundModeNext".into(),
			NamedKey::AudioTrebleDown => "AudioTrebleDown".into(),
			NamedKey::AudioTrebleUp => "AudioTrebleUp".into(),
			NamedKey::AudioVolumeDown => "AudioVolumeDown".into(),
			NamedKey::AudioVolumeUp => "AudioVolumeUp".into(),
			NamedKey::AudioVolumeMute => "AudioVolumeMute".into(),
			NamedKey::MicrophoneToggle => "MicrophoneToggle".into(),
			NamedKey::MicrophoneVolumeDown => "MicrophoneVolumeDown".into(),
			NamedKey::MicrophoneVolumeUp => "MicrophoneVolumeUp".into(),
			NamedKey::MicrophoneVolumeMute => "MicrophoneVolumeMute".into(),
			NamedKey::SpeechCorrectionList => "SpeechCorrectionList".into(),
			NamedKey::SpeechInputToggle => "SpeechInputToggle".into(),
			NamedKey::LaunchApplication1 => "LaunchApplication1".into(),
			NamedKey::LaunchApplication2 => "LaunchApplication2".into(),
			NamedKey::LaunchCalendar => "LaunchCalendar".into(),
			NamedKey::LaunchContacts => "LaunchContacts".into(),
			NamedKey::LaunchMail => "LaunchMail".into(),
			NamedKey::LaunchMediaPlayer => "LaunchMediaPlayer".into(),
			NamedKey::LaunchMusicPlayer => "LaunchMusicPlayer".into(),
			NamedKey::LaunchPhone => "LaunchPhone".into(),
			NamedKey::LaunchScreenSaver => "LaunchScreenSaver".into(),
			NamedKey::LaunchSpreadsheet => "LaunchSpreadsheet".into(),
			NamedKey::LaunchWebBrowser => "LaunchWebBrowser".into(),
			NamedKey::LaunchWebCam => "LaunchWebCam".into(),
			NamedKey::LaunchWordProcessor => "LaunchWordProcessor".into(),
			NamedKey::BrowserBack => "BrowserBack".into(),
			NamedKey::BrowserFavorites => "BrowserFavorites".into(),
			NamedKey::BrowserForward => "BrowserForward".into(),
			NamedKey::BrowserHome => "BrowserHome".into(),
			NamedKey::BrowserRefresh => "BrowserRefresh".into(),
			NamedKey::BrowserSearch => "BrowserSearch".into(),
			NamedKey::BrowserStop => "BrowserStop".into(),
			NamedKey::AppSwitch => "AppSwitch".into(),
			NamedKey::Call => "Call".into(),
			NamedKey::Camera => "Camera".into(),
			NamedKey::CameraFocus => "CameraFocus".into(),
			NamedKey::EndCall => "EndCall".into(),
			NamedKey::GoBack => "GoBack".into(),
			NamedKey::GoHome => "GoHome".into(),
			NamedKey::HeadsetHook => "HeadsetHook".into(),
			NamedKey::LastNumberRedial => "LastNumberRedial".into(),
			NamedKey::Notification => "Notification".into(),
			NamedKey::MannerMode => "MannerMode".into(),
			NamedKey::VoiceDial => "VoiceDial".into(),
			NamedKey::TV => "TV".into(),
			NamedKey::TV3DMode => "TV3DMode".into(),
			NamedKey::TVAntennaCable => "TVAntennaCable".into(),
			NamedKey::TVAudioDescription => "TVAudioDescription".into(),
			NamedKey::TVAudioDescriptionMixDown => "TVAudioDescriptionMixDown".into(),
			NamedKey::TVAudioDescriptionMixUp => "TVAudioDescriptionMixUp".into(),
			NamedKey::TVContentsMenu => "TVContentsMenu".into(),
			NamedKey::TVDataService => "TVDataService".into(),
			NamedKey::TVInput => "TVInput".into(),
			NamedKey::TVInputComponent1 => "TVInputComponent1".into(),
			NamedKey::TVInputComponent2 => "TVInputComponent2".into(),
			NamedKey::TVInputComposite1 => "TVInputComposite1".into(),
			NamedKey::TVInputComposite2 => "TVInputComposite2".into(),
			NamedKey::TVInputHDMI1 => "TVInputHDMI1".into(),
			NamedKey::TVInputHDMI2 => "TVInputHDMI2".into(),
			NamedKey::TVInputHDMI3 => "TVInputHDMI3".into(),
			NamedKey::TVInputHDMI4 => "TVInputHDMI4".into(),
			NamedKey::TVInputVGA1 => "TVInputVGA1".into(),
			NamedKey::TVMediaContext => "TVMediaContext".into(),
			NamedKey::TVNetwork => "TVNetwork".into(),
			NamedKey::TVNumberEntry => "TVNumberEntry".into(),
			NamedKey::TVPower => "TVPower".into(),
			NamedKey::TVRadioService => "TVRadioService".into(),
			NamedKey::TVSatellite => "TVSatellite".into(),
			NamedKey::TVSatelliteBS => "TVSatelliteBS".into(),
			NamedKey::TVSatelliteCS => "TVSatelliteCS".into(),
			NamedKey::TVSatelliteToggle => "TVSatelliteToggle".into(),
			NamedKey::TVTerrestrialAnalog => "TVTerrestrialAnalog".into(),
			NamedKey::TVTerrestrialDigital => "TVTerrestrialDigital".into(),
			NamedKey::TVTimer => "TVTimer".into(),
			NamedKey::AVRInput => "AVRInput".into(),
			NamedKey::AVRPower => "AVRPower".into(),
			NamedKey::ColorF0Red => "ColorF0Red".into(),
			NamedKey::ColorF1Green => "ColorF1Green".into(),
			NamedKey::ColorF2Yellow => "ColorF2Yellow".into(),
			NamedKey::ColorF3Blue => "ColorF3Blue".into(),
			NamedKey::ColorF4Grey => "ColorF4Grey".into(),
			NamedKey::ColorF5Brown => "ColorF5Brown".into(),
			NamedKey::ClosedCaptionToggle => "ClosedCaptionToggle".into(),
			NamedKey::Dimmer => "Dimmer".into(),
			NamedKey::DisplaySwap => "DisplaySwap".into(),
			NamedKey::DVR => "DVR".into(),
			NamedKey::Exit => "Exit".into(),
			NamedKey::FavoriteClear0 => "FavoriteClear0".into(),
			NamedKey::FavoriteClear1 => "FavoriteClear1".into(),
			NamedKey::FavoriteClear2 => "FavoriteClear2".into(),
			NamedKey::FavoriteClear3 => "FavoriteClear3".into(),
			NamedKey::FavoriteRecall0 => "FavoriteRecall0".into(),
			NamedKey::FavoriteRecall1 => "FavoriteRecall1".into(),
			NamedKey::FavoriteRecall2 => "FavoriteRecall2".into(),
			NamedKey::FavoriteRecall3 => "FavoriteRecall3".into(),
			NamedKey::FavoriteStore0 => "FavoriteStore0".into(),
			NamedKey::FavoriteStore1 => "FavoriteStore1".into(),
			NamedKey::FavoriteStore2 => "FavoriteStore2".into(),
			NamedKey::FavoriteStore3 => "FavoriteStore3".into(),
			NamedKey::Guide => "Guide".into(),
			NamedKey::GuideNextDay => "GuideNextDay".into(),
			NamedKey::GuidePreviousDay => "GuidePreviousDay".into(),
			NamedKey::Info => "Info".into(),
			NamedKey::InstantReplay => "InstantReplay".into(),
			NamedKey::Link => "Link".into(),
			NamedKey::ListProgram => "ListProgram".into(),
			NamedKey::LiveContent => "LiveContent".into(),
			NamedKey::Lock => "Lock".into(),
			NamedKey::MediaApps => "MediaApps".into(),
			NamedKey::MediaAudioTrack => "MediaAudioTrack".into(),
			NamedKey::MediaLast => "MediaLast".into(),
			NamedKey::MediaSkipBackward => "MediaSkipBackward".into(),
			NamedKey::MediaSkipForward => "MediaSkipForward".into(),
			NamedKey::MediaStepBackward => "MediaStepBackward".into(),
			NamedKey::MediaStepForward => "MediaStepForward".into(),
			NamedKey::MediaTopMenu => "MediaTopMenu".into(),
			NamedKey::NavigateIn => "NavigateIn".into(),
			NamedKey::NavigateNext => "NavigateNext".into(),
			NamedKey::NavigateOut => "NavigateOut".into(),
			NamedKey::NavigatePrevious => "NavigatePrevious".into(),
			NamedKey::NextFavoriteChannel => "NextFavoriteChannel".into(),
			NamedKey::NextUserProfile => "NextUserProfile".into(),
			NamedKey::OnDemand => "OnDemand".into(),
			NamedKey::Pairing => "Pairing".into(),
			NamedKey::PinPDown => "PinPDown".into(),
			NamedKey::PinPMove => "PinPMove".into(),
			NamedKey::PinPToggle => "PinPToggle".into(),
			NamedKey::PinPUp => "PinPUp".into(),
			NamedKey::PlaySpeedDown => "PlaySpeedDown".into(),
			NamedKey::PlaySpeedReset => "PlaySpeedReset".into(),
			NamedKey::PlaySpeedUp => "PlaySpeedUp".into(),
			NamedKey::RandomToggle => "RandomToggle".into(),
			NamedKey::RcLowBattery => "RcLowBattery".into(),
			NamedKey::RecordSpeedNext => "RecordSpeedNext".into(),
			NamedKey::RfBypass => "RfBypass".into(),
			NamedKey::ScanChannelsToggle => "ScanChannelsToggle".into(),
			NamedKey::ScreenModeNext => "ScreenModeNext".into(),
			NamedKey::Settings => "Settings".into(),
			NamedKey::SplitScreenToggle => "SplitScreenToggle".into(),
			NamedKey::STBInput => "STBInput".into(),
			NamedKey::STBPower => "STBPower".into(),
			NamedKey::Subtitle => "Subtitle".into(),
			NamedKey::Teletext => "Teletext".into(),
			NamedKey::VideoModeNext => "VideoModeNext".into(),
			NamedKey::Wink => "Wink".into(),
			NamedKey::ZoomToggle => "ZoomToggle".into(),
			NamedKey::F1 => "F1".into(),
			NamedKey::F2 => "F2".into(),
			NamedKey::F3 => "F3".into(),
			NamedKey::F4 => "F4".into(),
			NamedKey::F5 => "F5".into(),
			NamedKey::F6 => "F6".into(),
			NamedKey::F7 => "F7".into(),
			NamedKey::F8 => "F8".into(),
			NamedKey::F9 => "F9".into(),
			NamedKey::F10 => "F10".into(),
			NamedKey::F11 => "F11".into(),
			NamedKey::F12 => "F12".into(),
			NamedKey::F13 => "F13".into(),
			NamedKey::F14 => "F14".into(),
			NamedKey::F15 => "F15".into(),
			NamedKey::F16 => "F16".into(),
			NamedKey::F17 => "F17".into(),
			NamedKey::F18 => "F18".into(),
			NamedKey::F19 => "F19".into(),
			NamedKey::F20 => "F20".into(),
			NamedKey::F21 => "F21".into(),
			NamedKey::F22 => "F22".into(),
			NamedKey::F23 => "F23".into(),
			NamedKey::F24 => "F24".into(),
			NamedKey::F25 => "F25".into(),
			NamedKey::F26 => "F26".into(),
			NamedKey::F27 => "F27".into(),
			NamedKey::F28 => "F28".into(),
			NamedKey::F29 => "F29".into(),
			NamedKey::F30 => "F30".into(),
			NamedKey::F31 => "F31".into(),
			NamedKey::F32 => "F32".into(),
			NamedKey::F33 => "F33".into(),
			NamedKey::F34 => "F34".into(),
			NamedKey::F35 => "F35".into(),
			key => {
				warn!("Unsupported key received {key:?}");
				Default::default()
			}
		},
	}
}
