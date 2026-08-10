//! Fixture child for bounded selected-check host tests.

use std::io::Write as _;
use std::process::ExitCode;
use std::time::Duration;

const SUCCESS: u8 = 0;
const FAILURE: u8 = 7;
const MODE_SUCCESS: &str = "success";
const MODE_NONZERO: &str = "nonzero";
const MODE_TIMEOUT: &str = "timeout";
const MODE_OVERFLOW: &str = "overflow";
const MODE_SIGNAL: &str = "signal";
const PRIVATE_OUTPUT: &str = "private-fixture-output";
const TIMEOUT_SLEEP_MS: u64 = 2_000;
const OVERFLOW_BYTES: usize = 4_096;

fn main() -> ExitCode {
    match std::env::args().nth(1).as_deref() {
        Some(MODE_SUCCESS) => write_private_output(SUCCESS),
        Some(MODE_NONZERO) => write_private_output(FAILURE),
        Some(MODE_TIMEOUT) => {
            std::thread::sleep(Duration::from_millis(TIMEOUT_SLEEP_MS));
            ExitCode::from(SUCCESS)
        }
        Some(MODE_OVERFLOW) => {
            let bytes = vec![b'x'; OVERFLOW_BYTES];
            let _ignored = std::io::stdout().write_all(bytes.as_slice());
            ExitCode::from(SUCCESS)
        }
        Some(MODE_SIGNAL) => std::process::abort(),
        _unknown => ExitCode::from(FAILURE),
    }
}

fn write_private_output(code: u8) -> ExitCode {
    let _ignored = writeln!(std::io::stdout(), "{PRIVATE_OUTPUT}");
    let _ignored = writeln!(std::io::stderr(), "{PRIVATE_OUTPUT}");
    ExitCode::from(code)
}
