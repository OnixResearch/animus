//! Local fake-backed transactional daemon-handoff smoke.

fn main() {
    match animus_std::daemon_handoff_host::run_fake_handoff_smoke() {
        Ok(receipt) => {
            println!("state={}", receipt.state);
            println!("handoff={}", receipt.handoff);
            println!("transition={}", receipt.transition);
            println!("receipt_identity={}", receipt.receipt_identity);
            println!("host_call_count={}", receipt.host_call_count);
            println!("redaction={}", receipt.redaction);
            for non_claim in receipt.non_claims {
                println!("non_claim={non_claim}");
            }
        }
        Err(error) => {
            eprintln!("daemon handoff smoke failed: {error:?}");
            std::process::exit(1);
        }
    }
}
