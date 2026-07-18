//! Fixture child that emits one malformed protocol response.

use std::io::Write;

fn main() {
    let mut stdout = std::io::stdout().lock();
    let _ignored = writeln!(stdout, "not-json");
    let _ignored = stdout.flush();
}
