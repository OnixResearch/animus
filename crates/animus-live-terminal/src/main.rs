//! Live terminal binary entry point.

use std::env;
use std::path::Path;
use std::sync::atomic::Ordering;

const CONFIG_PATH_ENV: &str = "ANIMUS_TERMINAL_CONFIG_PATH";
const ENDPOINT_ROOT_ENV: &str = "ANIMUS_TERMINAL_ENDPOINT_ROOT";
const WORKER_GENERATION_ENV: &str = "ANIMUS_TERMINAL_WORKER_GENERATION";
const MAX_FRAME_BYTES_ENV: &str = "ANIMUS_TERMINAL_MAX_FRAME_BYTES";
const MAXIMUM_ITERATIONS: u64 = 10_000_000;

fn main() {
    if let Err(error) = run() {
        eprintln!("live terminal failed: {error}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let config_path =
        env::var(CONFIG_PATH_ENV).map_err(|_error| String::from("config-path-missing"))?;
    let config_bytes = std::fs::read(config_path).map_err(|_error| String::from("config-read"))?;
    let config = animus_live_terminal::parse_terminal_config(config_bytes.as_slice())
        .map_err(|_error| String::from("config-invalid"))?;
    if config.connection_mode() != animus_live_terminal::ConnectionMode::Daemon {
        return Err(String::from("direct-mode-needs-injected-service"));
    }
    let endpoint_root =
        env::var(ENDPOINT_ROOT_ENV).map_err(|_error| String::from("endpoint-root-missing"))?;
    let generation = parse_positive_u64(WORKER_GENERATION_ENV)?;
    let max_frame_bytes = parse_positive_u32(MAX_FRAME_BYTES_ENV)?;
    let mut connection = animus_live_terminal::connect_explicit_daemon(
        &config,
        Path::new(endpoint_root.as_str()),
        generation,
        max_frame_bytes,
    )
    .map_err(|_error| String::from("daemon-unavailable"))?;

    let mut lifecycle = animus_live_terminal::CrosstermTerminalLifecycle::new(std::io::stdout());
    let signal_flag = lifecycle.signal_flag();
    let mut guard = animus_live_terminal::TerminalLifecycleGuard::enter(&mut lifecycle)
        .map_err(|_error| String::from("terminal-start"))?;
    let backend = ratatui::backend::CrosstermBackend::new(std::io::stdout());
    let host_result = animus_live_terminal::CrosstermTerminalHost::new(
        backend,
        std::sync::Arc::clone(&signal_flag),
    );
    let mut host = match host_result {
        Ok(host) => host,
        Err(_error) => {
            guard
                .restore()
                .map_err(|_cleanup| String::from("terminal-cleanup-after-backend-start"))?;
            return Err(String::from("backend-start"));
        }
    };
    let result =
        animus_live_terminal::run_attached_terminal(animus_live_terminal::LiveTerminalRequest {
            config: &config,
            connection: &mut connection,
            host: &mut host,
            maximum_iterations: MAXIMUM_ITERATIONS,
        });
    drop(host);
    let cleanup = guard
        .restore()
        .map_err(|_error| String::from("terminal-cleanup"))?;
    let (state, exit) = result.map_err(|_error| String::from("terminal-run"))?;
    let receipt = state
        .receipt(animus_live_terminal::TerminalReceiptFacts {
            config_identity: config.identity(),
            connection: "daemon",
            cleanup,
            exit,
        })
        .map_err(|_error| String::from("receipt-invalid"))?;
    if signal_flag.load(Ordering::SeqCst) && exit != animus_live_terminal::TerminalExit::Signal {
        return Err(String::from("signal-exit-drift"));
    }
    print!("{}", receipt.render());
    Ok(())
}

fn parse_positive_u64(name: &str) -> Result<u64, String> {
    let value = env::var(name).map_err(|_error| format!("{name}-missing"))?;
    let value = value
        .parse::<u64>()
        .map_err(|_error| format!("{name}-invalid"))?;
    if value == 0 {
        return Err(format!("{name}-zero"));
    }
    Ok(value)
}

fn parse_positive_u32(name: &str) -> Result<u32, String> {
    let value = env::var(name).map_err(|_error| format!("{name}-missing"))?;
    let value = value
        .parse::<u32>()
        .map_err(|_error| format!("{name}-invalid"))?;
    if value == 0 {
        return Err(format!("{name}-zero"));
    }
    Ok(value)
}
