// uucore ~ library

//## external crates

extern crate wild;

// * feature-gated external crates
#[cfg(feature = "failure")]
extern crate failure;
#[cfg(feature = "failure_derive")]
extern crate failure_derive;
#[cfg(all(feature = "lazy_static", target_os = "linux"))]
extern crate lazy_static;
#[cfg(feature = "libc")]
pub extern crate libc;
#[cfg(feature = "nix")]
extern crate nix;
#[cfg(feature = "platform-info")]
extern crate platform_info;
#[cfg(feature = "winapi")]
pub extern crate winapi;

//## internal modules

mod features; // modularized feature-gated code
mod mods; // core cross-platform modules

// * crate macros (macro_rules-type)
pub use mods::macros;

// * cross-platform modules
pub use mods::coreopts;
pub use mods::panic;

// * feature-gated modules
#[cfg(feature = "encoding")]
pub use features::encoding;
#[cfg(feature = "fs")]
pub use features::fs;
#[cfg(feature = "parse_time")]
pub use features::parse_time;
#[cfg(feature = "zero-copy")]
pub use features::zero_copy;

// * (platform-specific) feature-gated modules
// ** non-windows
#[cfg(all(not(windows), feature = "mode"))]
pub use features::mode;
// ** unix-only
#[cfg(all(unix, feature = "entries"))]
pub use features::entries;
#[cfg(all(unix, feature = "process"))]
pub use features::process;
#[cfg(all(unix, not(target_os = "fuchsia"), feature = "signals"))]
pub use features::signals;
#[cfg(all(
    unix,
    not(target_os = "fuchsia"),
    not(target_env = "musl"),
    feature = "utmpx"
))]
pub use features::utmpx;
// ** windows-only
#[cfg(all(windows, feature = "wide"))]
pub use features::wide;

// args() ...
pub fn args() -> impl Iterator<Item = String> {
    wild::args()
}
