//! Thin explicit-input CLI for guardrail description, side-effect-free preflight, and execution.

use std::ffi::OsString;
use std::path::PathBuf;
use std::process::ExitCode;

use animus_std_agent_guardrail::{
    ExecutionObservation, ExitClass, GuardrailError, GuardrailErrorKind, GuardrailProfile,
    PreflightRequest, PreflightStatus, build_receipt, launch, load_profile_file, load_request_file,
    observe_and_match_executable, preflight,
};

const EXIT_SUCCESS: u8 = 0;
const EXIT_FAILURE: u8 = 1;
const FIELD_CLI: &str = "cli";
const PROFILE_FLAG: &str = "--profile";
const REQUEST_FLAG: &str = "--request";
const BWRAP_FLAG: &str = "--bwrap";
const COMMAND_SEPARATOR: &str = "--";

#[derive(Debug)]
enum CliCommand {
    Describe {
        profile: PathBuf,
    },
    Validate {
        profile: PathBuf,
        request: PathBuf,
        bwrap: PathBuf,
    },
    Exec {
        profile: PathBuf,
        request: PathBuf,
        bwrap: PathBuf,
        command: Vec<OsString>,
    },
}

fn main() -> ExitCode {
    let arguments = std::env::args_os().skip(1).collect::<Vec<_>>();
    match run(arguments.as_slice()) {
        Ok(code) => ExitCode::from(code),
        Err(failure) => {
            print_error(&failure);
            ExitCode::from(EXIT_FAILURE)
        }
    }
}

fn run(arguments: &[OsString]) -> Result<u8, GuardrailError> {
    match parse_command(arguments)? {
        CliCommand::Describe { profile } => {
            let profile = load_profile_file(profile.as_path())?;
            print_description(&profile);
            Ok(EXIT_SUCCESS)
        }
        CliCommand::Validate {
            profile,
            request,
            bwrap,
        } => {
            let profile = load_profile_file(profile.as_path())?;
            let request = admitted_request(request.as_path(), bwrap.as_path())?;
            let result = preflight(&profile, &request);
            let status = preflight_status(result.status);
            let plan_identity = result.plan.as_ref().map(|plan| plan.identity.as_str());
            println!(
                "{}",
                serde_json::json!({
                    "schema": "animus.agent-guardrail.preflight.v1",
                    "status": status,
                    "issue": result.issue,
                    "profile_identity": profile.identity(),
                    "plan_identity": plan_identity,
                    "mutated": false,
                    "bubblewrap_launched": false,
                    "child_launched": false,
                })
            );
            if result.status == PreflightStatus::Valid {
                Ok(EXIT_SUCCESS)
            } else {
                Ok(EXIT_FAILURE)
            }
        }
        CliCommand::Exec {
            profile,
            request,
            bwrap,
            command,
        } => {
            let profile = load_profile_file(profile.as_path())?;
            let request = admitted_request(request.as_path(), bwrap.as_path())?;
            let result = preflight(&profile, &request);
            let plan = result
                .plan
                .ok_or_else(|| GuardrailError::new(GuardrailErrorKind::Plan, FIELD_CLI))?;
            let launch = launch(&request.executable, &plan, command.as_slice())?;
            let (status, issue) = match launch.exit {
                ExitClass::Success => ("passed", None),
                ExitClass::NonZero => ("child-failed", Some(String::from("child-non-zero"))),
                ExitClass::Signal => ("child-failed", Some(String::from("child-signal"))),
                ExitClass::NotLaunched => ("launch-failed", Some(String::from("not-launched"))),
            };
            let observation = ExecutionObservation {
                status: String::from(status),
                issue,
                exit: launch.exit,
                cleanup: String::from("completed"),
                option_count: launch.option_count,
            };
            let receipt = build_receipt(&profile, &request, &plan, &observation)?;
            println!(
                "{}",
                serde_json::to_string(&receipt).map_err(|_error| {
                    GuardrailError::new(GuardrailErrorKind::Receipt, FIELD_CLI)
                })?
            );
            if launch.exit == ExitClass::Success {
                Ok(EXIT_SUCCESS)
            } else {
                Ok(EXIT_FAILURE)
            }
        }
    }
}

fn admitted_request(
    request_path: &std::path::Path,
    executable_path: &std::path::Path,
) -> Result<PreflightRequest, GuardrailError> {
    let mut request = load_request_file(request_path)?;
    request.executable = observe_and_match_executable(&request.executable, executable_path)?;
    Ok(request)
}

fn parse_command(arguments: &[OsString]) -> Result<CliCommand, GuardrailError> {
    let Some(command) = arguments.first().and_then(|value| value.to_str()) else {
        return Err(cli_error());
    };
    match command {
        "describe" => {
            let profile = parse_one_path(arguments, PROFILE_FLAG)?;
            Ok(CliCommand::Describe { profile })
        }
        "validate" => Ok(CliCommand::Validate {
            profile: parse_one_path(arguments, PROFILE_FLAG)?,
            request: parse_one_path(arguments, REQUEST_FLAG)?,
            bwrap: parse_one_path(arguments, BWRAP_FLAG)?,
        }),
        "exec" => {
            let separator = arguments
                .iter()
                .position(|value| value == COMMAND_SEPARATOR)
                .ok_or_else(cli_error)?;
            let command = arguments
                .get(separator.saturating_add(1)..)
                .filter(|values| !values.is_empty())
                .ok_or_else(cli_error)?
                .to_vec();
            Ok(CliCommand::Exec {
                profile: parse_one_path(&arguments[..separator], PROFILE_FLAG)?,
                request: parse_one_path(&arguments[..separator], REQUEST_FLAG)?,
                bwrap: parse_one_path(&arguments[..separator], BWRAP_FLAG)?,
                command,
            })
        }
        _unknown => Err(cli_error()),
    }
}

fn parse_one_path(arguments: &[OsString], flag: &str) -> Result<PathBuf, GuardrailError> {
    let positions = arguments
        .iter()
        .enumerate()
        .filter_map(|(index, value)| (value == flag).then_some(index))
        .collect::<Vec<_>>();
    let [position] = positions.as_slice() else {
        return Err(cli_error());
    };
    let value = arguments
        .get(position.saturating_add(1))
        .filter(|value| !value.is_empty())
        .ok_or_else(cli_error)?;
    Ok(PathBuf::from(value))
}

fn print_description(profile: &GuardrailProfile) {
    let root_labels = profile
        .roots()
        .iter()
        .map(|root| root.label.as_str())
        .collect::<Vec<_>>();
    println!(
        "{}",
        serde_json::json!({
            "schema": "animus.agent-guardrail.description.v1",
            "profile": profile.label(),
            "profile_identity": profile.identity(),
            "cohort": profile.cohort(),
            "trust_source": profile.trust_source(),
            "network": match profile.network() {
                animus_std_agent_guardrail::NetworkPosture::Deny => "deny",
                animus_std_agent_guardrail::NetworkPosture::Inherit => "inherit",
            },
            "root_labels": root_labels,
            "environment_keys": profile.environment_keys(),
            "project_local_policy": false,
            "raw_escape_hatches": false,
        })
    );
}

fn print_error(error: &GuardrailError) {
    eprintln!(
        "{}",
        serde_json::json!({
            "schema": "animus.agent-guardrail.error.v1",
            "status": "failed",
            "kind": format!("{:?}", error.kind()).to_ascii_lowercase(),
            "field": error.field(),
        })
    );
}

const fn preflight_status(status: PreflightStatus) -> &'static str {
    match status {
        PreflightStatus::Valid => "valid",
        PreflightStatus::Denied => "denied",
        PreflightStatus::Unsupported => "unsupported",
        PreflightStatus::MissingSource => "missing-source",
        PreflightStatus::Drifted => "drifted",
        PreflightStatus::Invalid => "invalid",
    }
}

const fn cli_error() -> GuardrailError {
    GuardrailError::new(GuardrailErrorKind::Cli, FIELD_CLI)
}
