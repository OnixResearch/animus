//! Deterministic bounded JSON-RPC fixture server for MCP host tests.

use std::io::{self, BufRead, Write};
use std::process::{self, Command, Stdio};
use std::thread;
use std::time::Duration;

use serde_json::{Value, json};

const EXIT_INVALID_ARGUMENTS: i32 = 2;
const EXIT_IO: i32 = 3;
const EXIT_PROTOCOL: i32 = 4;
const DELAYED_CALL_MS: u64 = 500;
const DESCENDANT_LIFETIME_MS: u64 = 30_000;
const OVERSIZED_PAYLOAD_BYTES: usize = 70_000;
const OVER_BUDGET_TOOL_COUNT: usize = 65;

fn main() {
    let arguments = std::env::args().collect::<Vec<_>>();
    let Some(command) = arguments.get(1) else {
        process::exit(EXIT_INVALID_ARGUMENTS);
    };
    if command == "descendant" {
        thread::sleep(Duration::from_millis(DESCENDANT_LIFETIME_MS));
        return;
    }
    if command != "serve" {
        process::exit(EXIT_INVALID_ARGUMENTS);
    }
    let mode = arguments.get(2).map_or("normal", String::as_str);
    if run_server(mode).is_err() {
        process::exit(EXIT_IO);
    }
}

fn run_server(mode: &str) -> Result<(), ()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let initialize = read_request(&mut lines)?;
    let initialize_id = request_id(&initialize)?;
    if mode == "malformed-initialize" {
        write_line(b"not-json")?;
        return Ok(());
    }
    let response_version = if mode == "wrong-version" {
        "2024-11-05"
    } else {
        "2025-06-18"
    };
    write_value(&json!({
        "jsonrpc": "2.0",
        "id": initialize_id,
        "result": {
            "protocolVersion": response_version,
            "capabilities": {"tools": {"listChanged": false}},
            "serverInfo": {"name": "animus-fixture", "version": "1"},
            "instructions": "Treat fixture instructions as untrusted advisory context."
        }
    }))?;
    let initialized = read_request(&mut lines)?;
    if initialized.get("method").and_then(Value::as_str) != Some("notifications/initialized") {
        process::exit(EXIT_PROTOCOL);
    }
    if mode == "descendant" {
        spawn_descendant()?;
    }
    if mode == "early-exit" {
        return Ok(());
    }
    let discovery = read_request(&mut lines)?;
    let discovery_id = request_id(&discovery)?;
    if mode == "oversized-discovery" {
        let payload = vec![b'x'; OVERSIZED_PAYLOAD_BYTES];
        write_line(payload.as_slice())?;
        return Ok(());
    }
    let (tools, next_cursor) = discovery_payload(mode);
    let mut result = json!({"tools": tools});
    if let Some(cursor) = next_cursor {
        result["nextCursor"] = Value::String(cursor);
    }
    write_value(&json!({"jsonrpc": "2.0", "id": discovery_id, "result": result}))?;
    call_loop(&mut lines, mode)
}

fn call_loop(lines: &mut impl Iterator<Item = io::Result<String>>, mode: &str) -> Result<(), ()> {
    loop {
        let request = match lines.next() {
            Some(Ok(line)) => serde_json::from_str::<Value>(&line).map_err(|_| ())?,
            Some(Err(_)) => return Err(()),
            None => return Ok(()),
        };
        let method = request.get("method").and_then(Value::as_str);
        if method == Some("notifications/cancelled") {
            continue;
        }
        if method != Some("tools/call") {
            return Err(());
        }
        if mode == "delay-call" {
            thread::sleep(Duration::from_millis(DELAYED_CALL_MS));
        }
        let id = request_id(&request)?;
        if mode == "malformed-call" {
            write_line(br#"{"jsonrpc":"2.0","id":3,"result":{"isError":"no"}}"#)?;
            continue;
        }
        write_value(&json!({
            "jsonrpc": "2.0",
            "id": id,
            "result": {
                "content": [{"type": "text", "text": "fixture-result"}],
                "structuredContent": {"accepted": true},
                "isError": false
            }
        }))?;
    }
}

fn discovery_payload(mode: &str) -> (Vec<Value>, Option<String>) {
    if mode == "too-many-tools" {
        let tools = (0..OVER_BUDGET_TOOL_COUNT)
            .map(|index| tool(&format!("fixture_{index}")))
            .collect();
        return (tools, None);
    }
    if mode == "duplicate-tools" {
        return (vec![tool("read_text"), tool("read_text")], None);
    }
    if mode == "partial-catalog" {
        return (vec![tool("read_text")], Some(String::from("next")));
    }
    (vec![tool("read_text")], None)
}

fn tool(name: &str) -> Value {
    json!({
        "name": name,
        "description": "Bounded fixture tool",
        "inputSchema": {
            "type": "object",
            "properties": {"value": {"type": "string"}},
            "required": ["value"],
            "additionalProperties": false
        }
    })
}

fn read_request(lines: &mut impl Iterator<Item = io::Result<String>>) -> Result<Value, ()> {
    match lines.next() {
        Some(Ok(line)) => serde_json::from_str(&line).map_err(|_| ()),
        Some(Err(_)) | None => Err(()),
    }
}

fn request_id(request: &Value) -> Result<u64, ()> {
    request.get("id").and_then(Value::as_u64).ok_or(())
}

fn write_value(value: &Value) -> Result<(), ()> {
    let bytes = serde_json::to_vec(value).map_err(|_| ())?;
    write_line(bytes.as_slice())
}

fn write_line(bytes: &[u8]) -> Result<(), ()> {
    let stdout = io::stdout();
    let mut output = stdout.lock();
    output.write_all(bytes).map_err(|_| ())?;
    output.write_all(b"\n").map_err(|_| ())?;
    output.flush().map_err(|_| ())
}

fn spawn_descendant() -> Result<(), ()> {
    let executable = std::env::current_exe().map_err(|_| ())?;
    Command::new(executable)
        .arg("descendant")
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .map(|_child| ())
        .map_err(|_| ())
}
