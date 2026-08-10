//! Fixture child that emits one malformed protocol response.

use std::io::{BufRead, Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut request = String::new();
    let request_bytes = std::io::stdin().lock().read_line(&mut request)?;
    if request_bytes == 0 {
        return Ok(());
    }

    let mut stdout = std::io::stdout().lock();
    writeln!(stdout, "not-json")?;
    stdout.flush()?;
    Ok(())
}
