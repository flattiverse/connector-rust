
use std::fmt::Display;
use std::fmt::Debug;
use std::fmt::Result;
use std::fmt::Formatter;

const MODIFIER_MAJOR : u32 = 16777216;
const MODIFIER_MINOR : u32 = 65536;
const MODIFIER_BUILD : u32 = 256;


pub struct Version {
    raw_version: u32
}

impl Version {
    pub const fn new(major: u8, minor: u8, build: u8, revision: u8) -> Version {
        Version {
            raw_version: major as u32 * MODIFIER_MAJOR
                + minor as u32 * MODIFIER_MINOR
                + build as u32 * MODIFIER_BUILD
                + revision as u32
        }
    }

    /// The decoded major value of this [Version]
    pub fn major(&self) -> u8 {
        ((self.raw_version / MODIFIER_MAJOR) % 256) as u8
    }

    /// The decoded minor value of this [Version]
    pub fn minor(&self) -> u8 {
        ((self.raw_version / MODIFIER_MINOR) % 256) as u8
    }

    /// The decoded build value of this [Version]
    pub fn build(&self) -> u8 {
        ((self.raw_version / MODIFIER_BUILD) % 256) as u8
    }

    /// The decoded revision value of this [Version]
    pub fn revision(&self) -> u8 {
        (self.raw_version % 256) as u8
    }

    /// The raw version information
    pub fn raw(&self) -> u32 {
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