//! Small effect fixture used only by the opt-in real Bubblewrap Linux behavior test.

use std::io::{Read, Write};
use std::process::ExitCode;

const EXIT_SUCCESS: u8 = 0;
const EXIT_FAILURE: u8 = 1;

fn main() -> ExitCode {
    let arguments = std::env::args().skip(1).collect::<Vec<_>>();
    let success = match arguments.as_slice() {
        [operation, path] if operation == "read" => std::fs::read(path).is_ok(),
        [operation, path] if operation == "write" => std::fs::write(path, b"guarded").is_ok(),
        [operation, address] if operation == "connect" => std::net::TcpStream::connect(address)
            .and_then(|mut stream| {
                stream.write_all(b"probe")?;
                let mut response = [0_u8; 1];
                stream.read_exact(&mut response)
            })
            .is_ok(),
        _unknown => false,
    };
    if success {
        ExitCode::from(EXIT_SUCCESS)
    } else {
        ExitCode::from(EXIT_FAILURE)
    }
}
