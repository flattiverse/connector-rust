
#[repr(u8)]
#[derive(Copy, Clone, Debug)]
pub enum PlatformKind {
    Unknown = 0,
    /// Binary Rust connector
    Rust = 4,
    /// Binary .NET Framework Connector running
    /// on Microsoft .NET Framework.
    DotNet = 64,
    /// Binary .NET Framework Connector running
    /// on Mono.
    Mono = 65,
    /// Binary .Net Core 2.0 Connector running on Windows.
    DotNetCoreWindows = 68,
    /// Binary .Net Core 2.0 Connector running on Linux.
    DotNetCoreLinux = 69,
    /// Binary .Net Core 2.0 Connector running on Mac.
    DotNetCoreMac = 70,
    /// Binary Java Connector.
    Java = 80,
    /// Someone connected via ASCII-Protocoll without
    /// specifying a language.
    Ascii = 128,
    /// ASCII-Protocol Python program.
    Python = 144,
    /// ASCII-Protocol PHP program.
    Php = 145,
    /// ASCII-Protocol Perl program.
    Perl = 146,
    /// ASCII-Protocol Ruby program.
    Ruby = 147,
    /// ASCII-Protocol D program.
    D = 148,
    /// ASCII-Protocol Swift program.
    Swift = 149,
    /// ASCII-Protocol R program.
    R = 150,
    /// ASCII-Protocol Matlab program.
    Matlab = 151,
    /// ASCII-Protocol Groovy program.
    Groovy = 152,
    /// ASCII-Protocol Tcl(/Tk) program.
    Tcl = 153,
    /// ASCII-Protocol Haskell program.
    Haskell = 154,
}

impl PlatformKind {
    pub fn from_id(id: u8) -> PlatformKind {
        match id {
              4 => PlatformKind::Rust,
             64 => PlatformKind::DotNet,
             65 => PlatformKind::Mono,
             68 => PlatformKind::DotNetCoreWindows,
             69 => PlatformKind::DotNetCoreLinux,
             70 => PlatformKind::DotNetCoreMac,
             80 => PlatformKind::Java,
            128 => PlatformKind::Ascii,
            144 => PlatformKind::Python,
            145 => PlatformKind::Php,
            146 => PlatformKind::Perl,
            147 => PlatformKind::Ruby,
            148 => PlatformKind::D,
            149 => PlatformKind::Swift,
            150 => PlatformKind::R,
            151 => PlatformKind::Matlab,
            152 => PlatformKind::Groovy,
            153 => PlatformKind::Tcl,
            154 => PlatformKind::Haskell,
            _ => PlatformKind::Unknown,
        }
    }
}