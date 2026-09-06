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
const BINARY_STDOUT: &[u8] = b"private-capture\0\xff\r\n";
const BINARY_STDERR: &[u8] = b"private-error\0\xfe\r\n";

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
        Some("binary") => {
            std::io::stdout()
                .write_all(BINARY_STDOUT)
                .expect("fixture stdout");
            std::io::stderr()
                .write_all(BINARY_STDERR)
                .expect("fixture stderr");
            ExitCode::from(SUCCESS)
        }
        Some("stderr-overflow") => {
            std::io::stderr()
                .write_all(&vec![b'e'; OVERFLOW_BYTES])
                .expect("fixture stderr");
            ExitCode::from(SUCCESS)
        }
        Some("both-overflow") => {
            std::io::stdout()
                .write_all(&vec![b'x'; OVERFLOW_BYTES])
                .expect("fixture stdout");
            std::io::stderr()
                .write_all(&vec![b'e'; OVERFLOW_BYTES])
                .expect("fixture stderr");
            ExitCode::from(SUCCESS)
        }
        Some(mode @ ("marker" | "marker-wait")) => {
            let path = std::env::args().nth(2).expect("explicit marker path");
            let mut file = std::fs::File::create_new(path).expect("exactly one child");
            file.write_all(b"one execution").expect("fixture marker");
            if mode == "marker-wait" {
                std::thread::sleep(Duration::from_millis(TIMEOUT_SLEEP_MS));
            }
            ExitCode::from(SUCCESS)
        }
        Some("context") => write_context(),
        _unknown => ExitCode::from(FAILURE),
    }
}

fn write_context() -> ExitCode {
    use std::io::Read as _;
    let mut input = [0_u8; 1];
    assert_eq!(std::io::stdin().read(&mut input).expect("null stdin"), 0);
    let environment = std::env::vars().collect::<std::collections::BTreeMap<_, _>>();
    assert_eq!(environment.len(), 4);
    assert_eq!(environment["NIX_CONFIG"], "builders =\nsecret-key-files =");
    assert_eq!(environment["HOME"], environment["TMPDIR"]);
    assert_eq!(
        std::env::current_dir().expect("cwd"),
        std::path::Path::new(&environment["HOME"])
    );
    std::io::stdout()
        .write_all(b"explicit-context")
        .expect("fixture stdout");
    ExitCode::from(SUCCESS)
}

fn write_private_output(code: u8) -> ExitCode {
    let _ignored = writeln!(std::io::stdout(), "{PRIVATE_OUTPUT}");
    let _ignored = writeln!(std::io::stderr(), "{PRIVATE_OUTPUT}");
    ExitCode::from(code)
}
