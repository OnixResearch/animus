//! Non-default packaged deterministic TUI dogfood smoke executable.

const EXIT_FAILURE: i32 = 2;

fn main() {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    match animus_tui::run_packaged_tui_dogfood_smoke(&args) {
        Ok(output) => print!("{output}"),
        Err(error) => {
            eprintln!("error: {error}");
            std::process::exit(EXIT_FAILURE);
        }
    }
}
