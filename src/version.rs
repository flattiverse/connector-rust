
use std::fmt::Display;
use std::fmt::Debug;
use std::fmt::Result;
use std::fmt::Formatter;

const MODIFIER_MAJOR : u64 = 16777216;
const MODIFIER_MINOR : u64 = 65536;
const MODIFIER_BUILD : u64 = 256;


pub struct Version {
    raw_version: u64
}

impl Version {
    pub const fn new(major: u16, minor: u16, build: u16, revision: u16) -> Version {
        Version {
            raw_version: major as u64 * MODIFIER_MAJOR
                + minor as u64 * MODIFIER_MINOR
                + build as u64 * MODIFIER_BUILD
                + revision as u64
        }
    }

    /// The decoded major value of this [Version]
    pub fn major(&self) -> u16 {
        (self.raw_version / MODIFIER_MAJOR) as u16 % 256u16
    }

    /// The decoded minor value of this [Version]
    pub fn minor(&self) -> u16 {
        (self.raw_version / MODIFIER_MINOR) as u16 % 256u16
    }

    /// The decoded build value of this [Version]
    pub fn build(&self) -> u16 {
        (self.raw_version / MODIFIER_BUILD) as u16 % 256u16
    }

    /// The decoded revision value of this [Version]
    pub fn revision(&self) -> u16 {
        self.raw_version as u16 % 256u16
    }

    /// The raw version information
    pub fn raw(&self) -> u64 {
        self.raw_version
    }
}

impl Debug for Version {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}.{}.{}-{}@{}", self.major(), self.minor(), self.build(), self.revision(), self.raw())
    }
}

impl Display for Version {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}.{}.{}-{}", self.major(), self.minor(), self.build(), self.revision())
    }
}