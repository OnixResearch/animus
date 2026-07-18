//! Fixture child that never answers, used to prove bounded deadline and cancellation handling.

const FIXTURE_SLEEP_SECONDS: u64 = 30;

fn main() {
    std::thread::sleep(std::time::Duration::from_secs(FIXTURE_SLEEP_SECONDS));
}
