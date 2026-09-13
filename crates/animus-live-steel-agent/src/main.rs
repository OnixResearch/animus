//! Packaged opt-in live Steel agent entrypoint.

use std::io::Read as _;

const CONFIG_JSON: &[u8] = include_bytes!("../../../config/generated/live-steel-agent-shell.json");
const MAX_CREDENTIAL_BYTES: u64 = 16_384;

mod launch;

fn main() {
    if let Err(error) = run() {
        eprintln!("animus-live-steel-agent: {error}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let _activation = animus_live_steel_agent::preflight_live_application_activation()
        .map_err(|error| format!("activation preflight failed: {error:?}"))?;
    let raw = clap_lex::RawArgs::from_args();
    let arguments = launch::parse(&raw)?;
    let mut credential = String::new();
    let read_length_bytes = std::io::stdin()
        .take(MAX_CREDENTIAL_BYTES)
        .read_to_string(&mut credential)?;
    assert_eq!(read_length_bytes, credential.len());
    assert!(u64::try_from(read_length_bytes)? <= MAX_CREDENTIAL_BYTES);
    let credential = credential.trim_end_matches(['\r', '\n']).to_owned();
    let output = animus_live_steel_agent::run_live_application(
        animus_live_steel_agent::LiveApplicationInput {
            config_json: CONFIG_JSON,
            store_root: arguments.store_root.as_path(),
            prompt: arguments.prompt.as_str(),
            credential,
            context: animus_live_steel_agent::ExplicitContextMap::new([])?,
        },
    )?;
    println!("{}", output.final_text());
    println!("{}", output.receipt().render());
    Ok(())
}
