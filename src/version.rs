//! HTTP version
//!
//! This module contains a definition of the `Version` type. The `Version`
//! type is intended to be accessed through the root of the crate
//! (`http::Version`) rather than this module.
//!
//! The `Version` type contains constants that represent the various versions
//! of the HTTP protocol.
//!
//! # Examples
//!
//! ```
//! use http::Version;
//!
//! let rtsp1 = Version::RTSP_10;
//! let rtsp2 = Version::RTSP_20;
//! assert!(rtsp1 != rtsp2);
//!
//! println!("{:?}", rtsp2);
//! ```

use std::fmt;

/// Represents a version of the HTTP spec.
#[derive(PartialEq, PartialOrd, Copy, Clone, Eq, Ord, Hash)]
pub struct Version(Rtsp);

impl Version {
    /// `RTSP/1.0`
    pub const RTSP_10: Version = Version(Rtsp::Rtsp10);

    /// `RTSP/2.0`
    pub const RTSP_20: Version = Version(Rtsp::Rtsp20);

}

#[derive(PartialEq, PartialOrd, Copy, Clone, Eq, Ord, Hash)]
enum Rtsp {
    Rtsp10,
    Rtsp20,
    __NonExhaustive,
}

impl Default for Version {
    #[inline]
    fn default() -> Version {
        Version::RTSP_10
    }
}

impl fmt::Debug for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use self::Rtsp::*;

        f.write_str(match self.0 {
            Rtsp10 => "RTSP/1.0",
            Rtsp20 => "RTSP/2.0",
            __NonExhaustive => unreachable!(),
        })
    }
}
