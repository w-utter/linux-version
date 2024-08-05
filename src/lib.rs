#[derive(PartialEq, Eq)]
pub struct KernelVersion {
    major: u8,
    minor: u8,
    patch: u8,
}

use linux_version_sys as sys;

impl KernelVersion {
    pub const CURRENT_RAW: u32 = sys::LINUX_VERSION_CODE;
    pub const CURRENT: KernelVersion = Self::from_raw(Self::CURRENT_RAW);

    pub const fn as_raw(&self) -> u32 {
        (self.major << 16) + (self.minor << 8) + self.patch
    }

    pub const fn from_raw(val: u32) -> Self {
        let major = val >> 16 as u8;
        let minor = val >> 8 as u8;
        let patch = val as u8;
        Self {
            major,
            minor,
            patch
        }
    }
}

pub const LINUX_VERSION_CODE: u32 = sys::LINUX_VERSION_CODE;

macro_rules! kernel_version {
    ($major:expr, $minor:expr, $patch:expr) => {
        ($major << 16) + ($minor << 8) + $patch
    }
}

pub use kernel_version;

use core::cmp::Ordering;

macro_rules! compare_versions {
    ($lhs:expr, $rhs:expr) => {
        match $lhs.cmp($rhs) {
            Ordering::Equal => (),
            ord => return Some(ord),
        }
    }
}

impl core::cmp::PartialOrd for KernelVersion {
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        Some(self.cmp(rhs))
    }
}

impl core::cmp::Ord for KernelVersion {
    fn cmp(&self, rhs: &Self) -> Ordering {
        compare_versions!(self.major, rhs.major);
        compare_versions!(self.minor, rhs.minor);
        self.patch.cmp(rhs.patch)
    }
}
