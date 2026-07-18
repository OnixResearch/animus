//! Deterministic fixture child for bounded Lean-loop process integration tests.

use std::io::{BufRead, Write};

const PROTOCOL: &str = "animus.lean-loop.protocol.v1";
const SCHEMA: &str = "animus.lean-loop.envelope.v1";
const EXIT_SUCCESS: std::process::ExitCode = std::process::ExitCode::SUCCESS;
const EXIT_FAILURE: std::process::ExitCode = std::process::ExitCode::FAILURE;

fn main() -> std::process::ExitCode {
    match run() {
        Ok(()) => EXIT_SUCCESS,
        Err(()) => EXIT_FAILURE,
    }
}

fn run() -> Result<(), ()> {
    let stdin = std::io::stdin();
    let mut stdout = std::io::stdout().lock();
    for line in stdin.lock().lines() {
        let line = line.map_err(|_error| ())?;
        let request =
            serde_json::from_str::<serde_json::Value>(line.as_str()).map_err(|_error| ())?;
        let response = response_for(&request)?;
        writeln!(stdout, "{response}").map_err(|_error| ())?;
        stdout.flush().map_err(|_error| ())?;
        if request["type"] == "shutdown" {
            return Ok(());
        }
    }
    Ok(())
}

fn response_for(request: &serde_json::Value) -> Result<serde_json::Value, ()> {
    if request["protocol"] != PROTOCOL || request["schema"] != SCHEMA {
        return Err(());
    }
    let response = match request["type"].as_str() {
        Some("handshake") => base_response(request, "handshake_ack", 0),
        Some("shutdown") => base_response(
            request,
            "shutdown_ack",
            request["sequence"].as_u64().ok_or(())?,
        ),
        Some("turn") => turn_response(request)?,
        _other => return Err(()),
    };
    Ok(response)
}

fn turn_response(request: &serde_json::Value) -> Result<serde_json::Value, ()> {
    let sequence = request["sequence"].as_u64().ok_or(())?;
    let mut response = base_response(request, "decision", sequence);
    let object = response.as_object_mut().ok_or(())?;
    if let Some(output) = request["last_tool_output"].as_str() {
        object.insert(String::from("decision"), serde_json::json!("respond"));
        object.insert(String::from("message"), serde_json::json!(output));
    } else if let Some(message) = request["last_model_message"].as_str() {
        object.insert(String::from("decision"), serde_json::json!("respond"));
        object.insert(String::from("message"), serde_json::json!(message));
    } else if request["summary"] == "fixture.stop" {
        object.insert(String::from("decision"), serde_json::json!("stop"));
    } else if request["summary"] == "fixture.tool" {
        object.insert(String::from("decision"), serde_json::json!("tool"));
        object.insert(String::from("tool_name"), serde_json::json!("search"));
        object.insert(
            String::from("schema_identity"),
            serde_json::json!("schema.search.v1"),
        );
        object.insert(
            String::from("arguments"),
            serde_json::json!([{"name": "query", "value": "adapter contracts"}]),
        );
        object.insert(
            String::from("correlation_identity"),
            request["correlation_identity"].clone(),
        );
        object.insert(String::from("visible_output_budget"), serde_json::json!(64));
        object.insert(
            String::from("mutation_targets"),
            serde_json::json!(["resource:file:README"]),
        );
        object.insert(
            String::from("required_capabilities"),
            serde_json::json!(["tool.read"]),
        );
        object.insert(
            String::from("approval_posture"),
            serde_json::json!("pre_approved"),
        );
    } else {
        object.insert(String::from("decision"), serde_json::json!("model"));
        object.insert(String::from("prompt"), request["summary"].clone());
    }
    Ok(response)
}

fn base_response(
    request: &serde_json::Value,
    response_type: &str,
    sequence: u64,
) -> serde_json::Value {
    serde_json::json!({
        "type": response_type,
        "protocol": PROTOCOL,
        "schema": SCHEMA,
        "profile_identity": request["profile_identity"],
        "artifact_identity": request["artifact_identity"],
        "sequence": sequence,
    })
}
