extern crate wild;

pub fn args() -> impl Iterator<Item=String> {
    wild::args()
}

#[cfg(feature = "libc")]
pub extern crate libc;
#[cfg(feature = "winapi")]
pub extern crate winapi;
#[cfg(feature = "failure")]
extern crate failure;
#[cfg(feature = "failure_derive")]
#[macro_use]
extern crate failure_derive;
#[cfg(feature = "nix")]
extern crate nix;
#[cfg(all(feature = "lazy_static", target_os = "linux"))]
#[macro_use]
extern crate lazy_static;
#[cfg(feature = "platform-info")]
extern crate platform_info;

#[macro_use]
#[path = "features/macros.rs"]
mod macros;

#[macro_use]
#[path = "mods/coreopts.rs"]
pub mod coreopts;

#[path = "mods/panic.rs"]
pub mod panic;

#[cfg(feature = "fs")]
#[path = "features/fs.rs"]
pub mod fs;
#[cfg(feature = "encoding")]
#[path = "features/encoding.rs"]
pub mod encoding;
#[cfg(feature = "parse_time")]
#[path = "features/parse_time.rs"]
pub mod parse_time;

#[cfg(all(not(windows), feature = "mode"))]
#[path = "features/mode.rs"]
pub mod mode;
#[cfg(all(unix, not(target_os = "fuchsia"), not(target_env="musl"), feature = "utmpx"))]
#[path = "features/utmpx.rs"]
pub mod utmpx;
#[cfg(all(unix, feature = "entries"))]
#[path = "features/entries.rs"]
pub mod entries;
#[cfg(all(unix, feature = "process"))]
#[path = "features/process.rs"]
pub mod process;
#[cfg(all(unix, not(target_os = "fuchsia"), feature = "signals"))]
#[path = "features/signals.rs"]
pub mod signals;

#[cfg(feature = "zero-copy")]
#[path = "features/zero_copy/mod.rs"]
pub mod zero_copy;

#[cfg(all(windows, feature = "wide"))]
#[path = "features/wide.rs"]
pub mod wide;
