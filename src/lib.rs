pub mod adopt;
pub mod build;
pub mod check;
pub mod cli;
pub mod disk;
pub mod embedded;
pub mod error;
pub mod install;
pub mod spec;

use std::ffi::OsString;
use std::process::ExitCode;

pub fn run<I: IntoIterator<Item = OsString>>(arguments: I) -> ExitCode {
    cli::run(arguments)
}
