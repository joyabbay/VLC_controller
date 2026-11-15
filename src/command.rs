use std::fmt;

#[derive(Debug, Clone)]
pub enum VlcCommand {
    Play,
    Pause,
    Next,
    Previous,

    SetVolume(u32),
    VolumeUp,
    VolumeDown,

    Seek(u32),

    FullScreen(bool),

    Faster,
    Slower,
    NormalSpeed,

    Quit,
}

impl fmt::Display for VlcCommand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            // Playback
            Self::Play => write!(f, "play"),
            Self::Pause => write!(f, "pause"),
            Self::Next => write!(f, "next"),
            Self::Previous => write!(f, "prev"),

            // Volume: rc.lua uses 0..=1024, where 256 is 100%.
            Self::SetVolume(volume) => write!(f, "volume {volume}"),
            Self::VolumeUp => write!(f, "volup"),
            Self::VolumeDown => write!(f, "voldown"),

            // rc.lua's `seek` takes an absolute position in seconds.
            Self::Seek(seconds) => write!(f, "seek {seconds}"),

            // `fullscreen` can either toggle or take on/off.
            Self::FullScreen(true) => write!(f, "fullscreen on"),
            Self::FullScreen(false) => write!(f, "fullscreen off"),

            // rc.lua only exposes faster/slower/normal, not an arbitrary rate.
            Self::Faster => write!(f, "faster"),
            Self::Slower => write!(f, "slower"),
            Self::NormalSpeed => write!(f, "normal"),

            Self::Quit => write!(f, "quit"),
        }
    }
}
