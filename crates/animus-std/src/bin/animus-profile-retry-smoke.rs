//! Packaged entrypoint for deterministic profile-bound provider retry.

use std::process::ExitCode;

const SUCCESS: u8 = 0;
const FAILURE: u8 = 1;

fn main() -> ExitCode {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    match animus_std::run_packaged_profile_retry_smoke(args.as_slice()) {
        Ok(receipt) => {
            print!("{receipt}");
            ExitCode::from(SUCCESS)
        }
        Err(message) => {
            eprintln!("{message}");
            ExitCode::from(FAILURE)
        }
    }
}
