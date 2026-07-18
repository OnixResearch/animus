//! Explicit-root packaged durable coding-session smoke.

use std::process::ExitCode;

const ARG_STORE_ROOT: &str = "--store-root";
const ARG_PROMPT: &str = "--prompt";
const STATUS_FAILED: &str = "CODING_SESSION_SMOKE status=failed";
const SUCCESS: u8 = 0;
const FAILURE: u8 = 1;

fn main() -> ExitCode {
    match parse_args(std::env::args().skip(1).collect::<Vec<_>>().as_slice()).and_then(
        |(root, prompt)| {
            animus_std::coding_session_host::run_packaged_durable_coding_session_smoke(
                std::path::Path::new(root.as_str()),
                prompt.as_str(),
            )
            .map_err(|error| format!("{STATUS_FAILED} issue={:?}", error.issue()))
        },
    ) {
        Ok(receipt) => {
            print!("{receipt}");
            ExitCode::from(SUCCESS)
        }
        Err(message) => {
            eprintln!("{message}");
            ExitCode::from(FAILURE)
        }
    }
}

fn parse_args(args: &[String]) -> Result<(String, String), String> {
    let mut root = None;
    let mut prompt = None;
    let mut index = 0_usize;
    while index < args.len() {
        let argument = args.get(index).ok_or_else(|| String::from(STATUS_FAILED))?;
        index = index.saturating_add(1);
        let value = args
            .get(index)
            .ok_or_else(|| format!("{STATUS_FAILED} issue=missing_argument_value"))?;
        index = index.saturating_add(1);
        match argument.as_str() {
            ARG_STORE_ROOT => root = Some(value.clone()),
            ARG_PROMPT => prompt = Some(value.clone()),
            _unsupported => return Err(format!("{STATUS_FAILED} issue=unsupported_argument")),
        }
    }
    let root = root.ok_or_else(|| format!("{STATUS_FAILED} issue=missing_store_root"))?;
    let prompt = prompt.ok_or_else(|| format!("{STATUS_FAILED} issue=missing_prompt"))?;
    Ok((root, prompt))
}
