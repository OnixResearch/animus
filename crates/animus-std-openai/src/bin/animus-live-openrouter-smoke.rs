//! Explicit-input packaged live `OpenRouter` smoke.

use std::io::Read as _;

const ARG_MODEL: &str = "--model";
const ARG_CREDENTIAL_STDIN: &str = "--credential-stdin";
const STATUS_FAILED: &str = "LIVE_OPENROUTER_SMOKE status=failed";
const MAX_CREDENTIAL_STDIN_BYTES: u64 = 4_096;
const INPUT_SENTINEL_BYTES: u64 = 1;
const SUCCESS: u8 = 0;
const FAILURE: u8 = 1;

#[derive(Clone, Debug, Eq, PartialEq)]
struct SmokeArgs {
    model: String,
    has_credential_stdin: bool,
}

fn main() -> std::process::ExitCode {
    let arguments = std::env::args().skip(1).collect::<Vec<_>>();
    match parse_args(arguments.as_slice())
        .and_then(|parsed| read_credential(&parsed).map(|credential| (parsed, credential)))
        .and_then(|(parsed, credential)| run(&parsed, credential))
    {
        Ok(evidence) => {
            print!("{evidence}");
            std::process::ExitCode::from(SUCCESS)
        }
        Err(issue) => {
            eprintln!("{STATUS_FAILED} issue={issue}");
            std::process::ExitCode::from(FAILURE)
        }
    }
}

fn run(arguments: &SmokeArgs, credential: String) -> Result<String, String> {
    animus_std_openai::run_packaged_live_openrouter_smoke(arguments.model.as_str(), credential)
        .map_err(|error| error.to_string())
}

fn parse_args(args: &[String]) -> Result<SmokeArgs, String> {
    // r[impl agent_harness_runtime.packaged_live_openrouter_smoke.binary.failure]
    let mut model = None;
    let mut has_credential_stdin = false;
    let mut index = 0_usize;
    while index < args.len() {
        let argument = args
            .get(index)
            .ok_or_else(|| String::from("missing_argument"))?;
        index = index
            .checked_add(1)
            .ok_or_else(|| String::from("missing_argument"))?;
        match argument.as_str() {
            ARG_MODEL => {
                let value = args
                    .get(index)
                    .ok_or_else(|| String::from("missing_model_value"))?;
                index = index
                    .checked_add(1)
                    .ok_or_else(|| String::from("missing_argument"))?;
                if model.replace(value.clone()).is_some() {
                    return Err(String::from("duplicate_model"));
                }
            }
            ARG_CREDENTIAL_STDIN => {
                if has_credential_stdin {
                    return Err(String::from("duplicate_credential_stdin"));
                }
                has_credential_stdin = true;
            }
            _unsupported => return Err(String::from("unsupported_argument")),
        }
    }
    if !has_credential_stdin {
        return Err(String::from("missing_credential_stdin"));
    }
    let model = model.ok_or_else(|| String::from("missing_model"))?;
    // r[impl agent_harness_runtime.packaged_live_openrouter_smoke.reviewed_model.failure]
    if model != animus_std_openai::OPENROUTER_SMOKE_MODEL_ID {
        return Err(String::from("unsupported_model"));
    }
    assert!(has_credential_stdin);
    assert_eq!(index, args.len());
    Ok(SmokeArgs {
        model,
        has_credential_stdin,
    })
}

fn read_credential(arguments: &SmokeArgs) -> Result<String, String> {
    if !arguments.has_credential_stdin {
        return Err(String::from("missing_credential_stdin"));
    }
    let read_limit_bytes = MAX_CREDENTIAL_STDIN_BYTES
        .checked_add(INPUT_SENTINEL_BYTES)
        .ok_or_else(|| String::from("credential_limit_overflow"))?;
    let mut input = String::new();
    std::io::stdin()
        .take(read_limit_bytes)
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
        let new_len = input
            .len()
            .checked_sub("\r\n".len())
            .ok_or_else(|| String::from("credential_length_overflow"))?;
        input.truncate(new_len);
    } else if input.ends_with('\n') {
        let new_len = input
            .len()
            .checked_sub('\n'.len_utf8())
            .ok_or_else(|| String::from("credential_length_overflow"))?;
        input.truncate(new_len);
    }
    if input.is_empty() {
        return Err(String::from("credential_stdin_empty"));
    }
    if input.contains(['\r', '\n']) {
        return Err(String::from("credential_stdin_multiline"));
    }
    assert!(!input.is_empty());
    assert!(!input.contains(['\r', '\n']));
    Ok(input)
}

#[cfg(test)]
mod tests {

    const MODEL_ID: &str = animus_std_openai::OPENROUTER_SMOKE_MODEL_ID;
    const SUBSTITUTED_MODEL_ID: &str = "openai/gpt-5.5";

    #[test]
    fn positive_explicit_arguments_and_credential_are_admitted() {
        // r[verify agent_harness_runtime.packaged_live_openrouter_smoke.binary.pass]
        let parsed = super::parse_args(
            &[super::ARG_MODEL, MODEL_ID, super::ARG_CREDENTIAL_STDIN].map(String::from),
        )
        .expect("valid arguments");
        assert_eq!(parsed.model, MODEL_ID);
        assert!(parsed.has_credential_stdin);
        assert_eq!(
            super::decode_credential(String::from("secret-value\n")),
            Ok(String::from("secret-value"))
        );
    }

    #[test]
    fn negative_missing_duplicate_unknown_and_unsafe_input_fail() {
        // r[verify agent_harness_runtime.packaged_live_openrouter_smoke.binary.failure]
        assert!(super::parse_args(&[]).is_err());
        assert!(
            super::parse_args(
                &[
                    super::ARG_MODEL,
                    MODEL_ID,
                    super::ARG_MODEL,
                    MODEL_ID,
                    super::ARG_CREDENTIAL_STDIN,
                ]
                .map(String::from),
            )
            .is_err()
        );
        assert!(super::parse_args(&["--unknown"].map(String::from)).is_err());
        assert_eq!(
            super::parse_args(
                &[
                    super::ARG_MODEL,
                    SUBSTITUTED_MODEL_ID,
                    super::ARG_CREDENTIAL_STDIN
                ]
                .map(String::from)
            ),
            Err(String::from("unsupported_model"))
        );
        assert!(super::decode_credential(String::new()).is_err());
        assert!(super::decode_credential(String::from("first\nsecond")).is_err());
        let refused_length_bytes = usize::try_from(super::MAX_CREDENTIAL_STDIN_BYTES)
            .expect("test limit must fit usize")
            .checked_add(1)
            .expect("test sentinel fits usize");
        assert!(super::decode_credential("x".repeat(refused_length_bytes)).is_err());
    }
}
