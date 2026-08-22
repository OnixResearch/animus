//! Explicit-input packaged live `local-dspark` smoke.

use std::io::Read as _;
use std::process::ExitCode;

const ARG_MODEL: &str = "--model";
const ARG_CREDENTIAL_STDIN: &str = "--credential-stdin";
const STATUS_FAILED: &str = "LIVE_LOCAL_DSPARK_SMOKE status=failed";
const KIBIBYTE_BYTES: u64 = 1_024;
const MAX_CREDENTIAL_STDIN_KIBIBYTES: u64 = 4;
const MAX_CREDENTIAL_STDIN_BYTES: u64 = MAX_CREDENTIAL_STDIN_KIBIBYTES * KIBIBYTE_BYTES;
const INPUT_SENTINEL_BYTES: u64 = 1;
const SUCCESS: u8 = 0;
const FAILURE: u8 = 1;

#[derive(Clone, Debug, Eq, PartialEq)]
struct SmokeArgs {
    model: String,
    credential_stdin: bool,
}

fn main() -> ExitCode {
    let arguments = std::env::args().skip(1).collect::<Vec<_>>();
    match parse_args(arguments.as_slice())
        .and_then(|parsed| read_credential(&parsed).map(|credential| (parsed, credential)))
        .and_then(|(parsed, credential)| run(&parsed, credential))
    {
        Ok(evidence) => {
            print!("{evidence}");
            ExitCode::from(SUCCESS)
        }
        Err(issue) => {
            eprintln!("{STATUS_FAILED} issue={issue}");
            ExitCode::from(FAILURE)
        }
    }
}

fn run(arguments: &SmokeArgs, credential: String) -> Result<String, String> {
    animus_std_openai::run_packaged_live_local_dspark_smoke(arguments.model.as_str(), credential)
        .map_err(|error| error.to_string())
}

fn parse_args(args: &[String]) -> Result<SmokeArgs, String> {
    // r[impl agent_harness_runtime.local_dspark_responses.verification]
    let mut model = None;
    let mut credential_stdin = false;
    let mut index = 0_usize;
    while index < args.len() {
        let argument = args
            .get(index)
            .ok_or_else(|| String::from("missing_argument"))?;
        index = index.saturating_add(1);
        match argument.as_str() {
            ARG_MODEL => {
                let value = args
                    .get(index)
                    .ok_or_else(|| String::from("missing_model_value"))?;
                index = index.saturating_add(1);
                if model.replace(value.clone()).is_some() {
                    return Err(String::from("duplicate_model"));
                }
            }
            ARG_CREDENTIAL_STDIN => {
                if credential_stdin {
                    return Err(String::from("duplicate_credential_stdin"));
                }
                credential_stdin = true;
            }
            _unsupported => return Err(String::from("unsupported_argument")),
        }
    }
    if !credential_stdin {
        return Err(String::from("missing_credential_stdin"));
    }
    let model = model.ok_or_else(|| String::from("missing_model"))?;
    if model != animus_std_openai::LOCAL_DSPARK_SMOKE_MODEL_ID {
        return Err(String::from("unsupported_model"));
    }
    Ok(SmokeArgs {
        model,
        credential_stdin,
    })
}

fn read_credential(arguments: &SmokeArgs) -> Result<String, String> {
    if !arguments.credential_stdin {
        return Err(String::from("missing_credential_stdin"));
    }
    let read_limit = MAX_CREDENTIAL_STDIN_BYTES
        .checked_add(INPUT_SENTINEL_BYTES)
        .ok_or_else(|| String::from("credential_limit_overflow"))?;
    let mut input = String::new();
    std::io::stdin()
        .take(read_limit)
        .read_to_string(&mut input)
        .map_err(|_error| String::from("credential_stdin_read_failed"))?;
    decode_credential(input)
}

fn decode_credential(mut input: String) -> Result<String, String> {
    let byte_count =
        u64::try_from(input.len()).map_err(|_error| String::from("credential_length_overflow"))?;
    if byte_count > MAX_CREDENTIAL_STDIN_BYTES {
        return Err(String::from("credential_stdin_oversized"));
    }
    if input.ends_with("\r\n") {
        let new_len = input.len().saturating_sub("\r\n".len());
        input.truncate(new_len);
    } else if input.ends_with('\n') {
        let new_len = input.len().saturating_sub('\n'.len_utf8());
        input.truncate(new_len);
    }
    if input.is_empty() {
        return Err(String::from("credential_stdin_empty"));
    }
    if input.contains(['\r', '\n']) {
        return Err(String::from("credential_stdin_multiline"));
    }
    Ok(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const MODEL_ID: &str = animus_std_openai::LOCAL_DSPARK_SMOKE_MODEL_ID;
    const SUBSTITUTED_MODEL_ID: &str = "deepseek/deepseek-v4-flash-0731";

    #[test]
    fn positive_exact_arguments_and_credential_are_admitted() {
        // r[verify agent_harness_runtime.local_dspark_responses.verification]
        let parsed = parse_args(&[ARG_MODEL, MODEL_ID, ARG_CREDENTIAL_STDIN].map(String::from))
            .expect("valid arguments");
        assert_eq!(parsed.model, MODEL_ID);
        assert!(parsed.credential_stdin);
        assert_eq!(
            decode_credential(String::from("local\n")),
            Ok(String::from("local"))
        );
    }

    #[test]
    fn negative_missing_substituted_and_unsafe_input_fail() {
        // r[verify agent_harness_runtime.local_dspark_responses.verification]
        assert!(parse_args(&[]).is_err());
        assert_eq!(
            parse_args(&[ARG_MODEL, SUBSTITUTED_MODEL_ID, ARG_CREDENTIAL_STDIN].map(String::from)),
            Err(String::from("unsupported_model"))
        );
        assert!(parse_args(&["--unknown"].map(String::from)).is_err());
        assert!(decode_credential(String::new()).is_err());
        assert!(decode_credential(String::from("first\nsecond")).is_err());
        let oversized_len = usize::try_from(MAX_CREDENTIAL_STDIN_BYTES)
            .expect("test limit must fit usize")
            .saturating_add(1);
        assert!(decode_credential("x".repeat(oversized_len)).is_err());
    }
}
