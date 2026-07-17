//! Test-only child process for opt-in Sandlock Linux behavior checks.

use std::env;
use std::net::{SocketAddr, TcpStream};
use std::path::Path;
use std::process::{Child, Command, ExitCode};
use std::time::Duration;

const EXIT_SUCCESS: u8 = 0;
const EXIT_FAILURE: u8 = 1;
const MODE_PASS: &str = "pass";
const MODE_FILESYSTEM_DENIED: &str = "filesystem-denied";
const MODE_NETWORK_DENIED: &str = "network-denied";
const MODE_ENVIRONMENT_CLEAN: &str = "environment-clean";
const MODE_COW_DISCARD: &str = "cow-discard";
const MODE_PROCESS_DENIED: &str = "process-denied";
const MODE_HOLD: &str = "hold";
const CREATED_FILE: &str = "sandlock-live-created.txt";
const DEFAULT_PROCESS_ATTEMPTS: usize = 1;
const EXTRA_PROCESS_ATTEMPTS: usize = 1;
const HOLD_MILLIS: u64 = 2_000;
const INHERITED_ENV_KEY: &str = "PATH";

fn main() -> ExitCode {
    let arguments = env::args().skip(1).collect::<Vec<_>>();
    let passed = match arguments.first().map(String::as_str) {
        Some(MODE_PASS) => true,
        Some(MODE_FILESYSTEM_DENIED) => arguments
            .get(1)
            .is_some_and(|path| std::fs::read(Path::new(path)).is_err()),
        Some(MODE_NETWORK_DENIED) => arguments
            .get(1)
            .and_then(|address| address.parse::<SocketAddr>().ok())
            .is_some_and(|address| TcpStream::connect(address).is_err()),
        Some(MODE_ENVIRONMENT_CLEAN) => env::var_os(INHERITED_ENV_KEY).is_none(),
        Some(MODE_COW_DISCARD) => std::fs::write(CREATED_FILE, b"discard-me").is_ok(),
        Some(MODE_PROCESS_DENIED) => arguments
            .get(1)
            .map(|value| value.parse::<usize>())
            .transpose()
            .ok()
            .flatten()
            .map_or_else(
                || process_creation_is_denied(DEFAULT_PROCESS_ATTEMPTS),
                process_creation_is_denied,
            ),
        Some(MODE_HOLD) => {
            std::thread::sleep(Duration::from_millis(HOLD_MILLIS));
            true
        }
        Some(_) | None => false,
    };
    if passed {
        ExitCode::from(EXIT_SUCCESS)
    } else {
        ExitCode::from(EXIT_FAILURE)
    }
}

fn process_creation_is_denied(admitted_processes: usize) -> bool {
    let Ok(executable) = env::current_exe() else {
        return false;
    };
    let attempts = admitted_processes.saturating_add(EXTRA_PROCESS_ATTEMPTS);
    let mut children = Vec::with_capacity(attempts);
    for _ in 0..attempts {
        let Ok(child) = Command::new(executable.as_path()).arg(MODE_HOLD).spawn() else {
            terminate_children(children.as_mut_slice());
            return true;
        };
        children.push(child);
    }
    terminate_children(children.as_mut_slice());
    false
}

fn terminate_children(children: &mut [Child]) {
    for child in children {
        let _ = child.kill();
        let _ = child.wait();
    }
}
