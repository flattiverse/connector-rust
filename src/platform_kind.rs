
#[repr(u8)]
pub enum PlatformKind {
    Unknown = 0,
    DotNet = 16,
    Mono = 17,
    Java = 20,
    Android = 21,
    Ascii = 32,
    Python = 48,
    Php = 49,
    Perl = 50,
    Ruby = 51,
    D = 52,
    Swift = 53,
    R = 54,
    Matlab = 55,
    Groovy = 56,
    Tcl = 57,
    Haskell = 58
}

impl PlatformKind {
    pub fn from_id(id: u8) -> PlatformKind {
        match id {
            16 => PlatformKind::DotNet,
            17 => PlatformKind::Mono,
            20 => PlatformKind::Java,
            21 => PlatformKind::Android,
            32 => PlatformKind::Ascii,
            48 => PlatformKind::Python,
            49 => PlatformKind::Php,
            50 => PlatformKind::Perl,
            51 => PlatformKind::Ruby,
            52 => PlatformKind::D,
            53 => PlatformKind::Swift,
            54 => PlatformKind::R,
            55 => PlatformKind::Matlab,
            56 => PlatformKind::Groovy,
            57 => PlatformKind::Tcl,
            58 => PlatformKind::Haskell,
            _ => PlatformKind::Unknown,
        }
    }
}