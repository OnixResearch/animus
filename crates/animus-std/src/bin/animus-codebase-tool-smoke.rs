//! Explicit-root packaged read-only codebase tool smoke.

use std::process::ExitCode;

const ARG_ROOT: &str = "--root";
const ARG_PATH: &str = "--path";
const ARG_SYMBOL: &str = "--symbol";
const ARG_KIND: &str = "--kind";
const ARG_MAX_RESULTS: &str = "--max-results";
const STATUS_FAILED: &str = "CODEBASE_TOOL_SMOKE status=failed";
const SUCCESS: u8 = 0;
const FAILURE: u8 = 1;

struct SmokeArgs {
    root: String,
    paths: Vec<String>,
    symbol: String,
    kind: String,
    max_results: usize,
}

fn main() -> ExitCode {
    let arguments = std::env::args().skip(1).collect::<Vec<_>>();
    match parse_args(arguments.as_slice()).and_then(|parsed| run(&parsed)) {
        Ok(receipt) => {
            print!("{receipt}");
            ExitCode::from(SUCCESS)
        }
        Err(issue) => {
            eprintln!("{STATUS_FAILED} issue={issue}");
            ExitCode::from(FAILURE)
        }
    }
}

fn run(arguments: &SmokeArgs) -> Result<String, String> {
    animus_std::codebase_tool::run_packaged_codebase_tool_smoke(
        &animus_std::codebase_tool::CodebaseToolSmokeRequest {
            root_path: std::path::Path::new(arguments.root.as_str()),
            relative_paths: arguments.paths.as_slice(),
            symbol: arguments.symbol.as_str(),
            kind: arguments.kind.as_str(),
            max_results: arguments.max_results,
        },
    )
    .map_err(|error| format!("{error:?}"))
}

fn parse_args(args: &[String]) -> Result<SmokeArgs, String> {
    let mut root = None;
    let mut paths = Vec::new();
    let mut symbol = None;
    let mut kind = None;
    let mut max_results = None;
    let mut index = 0_usize;
    while index < args.len() {
        let argument = args
            .get(index)
            .ok_or_else(|| String::from("missing_argument"))?;
        index = index.saturating_add(1);
        let value = args
            .get(index)
            .ok_or_else(|| String::from("missing_argument_value"))?;
        index = index.saturating_add(1);
        match argument.as_str() {
            ARG_ROOT => set_once(&mut root, value, "duplicate_root")?,
            ARG_PATH => paths.push(value.clone()),
            ARG_SYMBOL => set_once(&mut symbol, value, "duplicate_symbol")?,
            ARG_KIND => set_once(&mut kind, value, "duplicate_kind")?,
            ARG_MAX_RESULTS => {
                let parsed = value
                    .parse::<usize>()
                    .map_err(|_error| String::from("invalid_max_results"))?;
                if max_results.replace(parsed).is_some() {
                    return Err(String::from("duplicate_max_results"));
                }
            }
            _unsupported => return Err(String::from("unsupported_argument")),
        }
    }
    if paths.is_empty() {
        return Err(String::from("missing_path"));
    }
    Ok(SmokeArgs {
        root: root.ok_or_else(|| String::from("missing_root"))?,
        paths,
        symbol: symbol.ok_or_else(|| String::from("missing_symbol"))?,
        kind: kind.ok_or_else(|| String::from("missing_kind"))?,
        max_results: max_results.ok_or_else(|| String::from("missing_max_results"))?,
    })
}

fn set_once(target: &mut Option<String>, value: &str, duplicate: &str) -> Result<(), String> {
    if target.replace(String::from(value)).is_some() {
        return Err(String::from(duplicate));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const MAX_RESULTS: usize = 2;

    #[test]
    fn positive_explicit_arguments_are_parsed() {
        let parsed = parse_args(
            &[
                ARG_ROOT,
                "fixture",
                ARG_PATH,
                "src/lib.rs",
                ARG_SYMBOL,
                "run",
                ARG_KIND,
                "definition",
                ARG_MAX_RESULTS,
                "2",
            ]
            .map(String::from),
        )
        .expect("valid arguments");
        assert_eq!(parsed.root, "fixture");
        assert_eq!(parsed.paths, ["src/lib.rs"]);
        assert_eq!(parsed.max_results, MAX_RESULTS);
    }

    #[test]
    fn negative_missing_duplicate_and_unknown_arguments_fail() {
        assert!(parse_args(&[]).is_err());
        assert!(parse_args(&[ARG_ROOT, "a", ARG_ROOT, "b"].map(String::from)).is_err());
        assert!(parse_args(&["--unknown", "value"].map(String::from)).is_err());
    }
}
