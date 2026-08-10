//! Packaged bounded selected-check completion smoke.

use std::process::ExitCode;

const SUCCESS: u8 = 0;
const FAILURE: u8 = 1;

fn main() -> ExitCode {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    match animus_std_check_exec::run_packaged_real_check_completion_smoke(args.as_slice()) {
        Ok(output) => {
            print!("{output}");
            ExitCode::from(SUCCESS)
        }
        Err(failure) => {
            eprintln!("{failure}");
            ExitCode::from(FAILURE)
        }
    }
}
