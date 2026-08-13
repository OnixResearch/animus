//! Packaged opt-in live Steel agent entrypoint.

use std::io::Read as _;
use std::path::PathBuf;

const CONFIG_JSON: &[u8] =
    include_bytes!("../../../config/generated/live-steel-agent-shell.json");
const EXPECTED_ARGUMENTS: usize = 2;
const MAX_CREDENTIAL_BYTES: u64 = 16_384;

fn main() {
    if let Err(error) = run() {
        eprintln!("animus-live-steel-agent: {error}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let arguments = std::env::args().skip(1).collect::<Vec<_>>();
    if arguments.len() != EXPECTED_ARGUMENTS {
        return Err("usage: animus-live-steel-agent <store-root> <prompt> < credential".into());
    }
    let store_root = PathBuf::from(&arguments[0]);
    let prompt = arguments[1].as_str();
    let mut credential = String::new();
    std::io::stdin()
        .take(MAX_CREDENTIAL_BYTES)
        .read_to_string(&mut credential)?;
    let credential = credential.trim_end_matches(['\r', '\n']).to_owned();
    let output = animus_live_steel_agent::run_live_application(
        animus_live_steel_agent::LiveApplicationInput {
            config_json: CONFIG_JSON,
            store_root: store_root.as_path(),
            prompt,
            credential,
            context: animus_live_steel_agent::ExplicitContextMap::default(),
        },
    )?;
    println!("{}", output.final_text());
    println!("{}", output.receipt().render());
    Ok(())
}
