use std::process::ExitCode;

fn main() -> ExitCode {
    ste::run(std::env::args_os())
}
