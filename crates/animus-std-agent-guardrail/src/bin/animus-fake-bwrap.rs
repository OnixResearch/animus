//! Deterministic fake Bubblewrap executable for sealed-transport integration tests.

use std::io::Read;
use std::process::ExitCode;

const EXIT_SUCCESS: u8 = 0;
const EXIT_FAILURE: u8 = 7;
const VERSION: &str = "bubblewrap 0.11.2";

fn main() -> ExitCode {
    let arguments = std::env::args().skip(1).collect::<Vec<_>>();
    if arguments.as_slice() == ["--version"] {
        println!("{VERSION}");
        return ExitCode::from(EXIT_SUCCESS);
    }
    if arguments.len() < 4 || arguments[0] != "--args" || arguments[2] != "--" {
        return ExitCode::from(EXIT_FAILURE);
    }
    let descriptor = match arguments[1].parse::<i32>() {
        Ok(value) if value >= 0 => value,
        _invalid => return ExitCode::from(EXIT_FAILURE),
    };
    let mut payload = Vec::new();
    let path = format!("/proc/self/fd/{descriptor}");
    if std::fs::File::open(path)
        .and_then(|mut file| file.read_to_end(&mut payload))
        .is_err()
        || payload.last() != Some(&0)
    {
        return ExitCode::from(EXIT_FAILURE);
    }
    match arguments[3].as_str() {
        "fixture-success" => ExitCode::from(EXIT_SUCCESS),
        "fixture-signal" => std::process::abort(),
        _other => ExitCode::from(EXIT_FAILURE),
    }
}
