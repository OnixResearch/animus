## Context

Harbor (0.24) runs ACP agents from a registry entry; a `local` distribution names a command that Harbor starts inside the task environment and speaks to over stdio. The Onix `assay` benchmark uses that path for Animus, so the binary must work with no network except the model endpoint, no package manager, and a workspace at a path Harbor chooses.

Existing pieces:

- `animus-std::CodingSessionHost` drives a session through nine injected services. Real implementations exist for persistence (`durable-session-store`), policy, authority, cancellation, timer and event sinks. Only fakes exist for the provider and tool services.
- `animus-acp-stdio-host` owns ACP framing, bounds, correlation and receipts, and adapts any `CodingSessionHost` through `AcpCodingSessionFacade`.
- `animus-std-workspace-edit` plans and publishes exact span edits against an admitted snapshot, with freshness checks, from one caller-opened capability root.
- `animus-std-check-exec` runs only `AllowedCheck`s (exact check id, argv and budget) with bounded output.
- `animus-live-steel-agent` has a provider stream service over `animus-std-openai`, but its tool cohort is fixed to `steel_repl`.

## Decisions

### Authority: edits under one root and declared checks only

The owner chose this over a general shell and over Steel skills. Requests are admitted by a pure core before any effect:

- Paths are relative UTF-8, normalized without `.` or `..`, with no empty, absolute or NUL components, and at most `MAX_PATH_BYTES` long. The shell resolves them under a root opened once with `openat`-style capability handles and refuses symlinks on every component.
- Reads return at most `MAX_READ_BYTES`; listings at most `MAX_LIST_ENTRIES`.
- Edits are exact `old`/`new` replacements. `old` must occur exactly once in the current snapshot; creating a file uses an empty `old` on an absent path. Each edit is bounded by the workspace-edit limits.
- `run_check` takes only a check id. The argv, working directory (the root) and budget come from the declared checks file.

### Declared checks file

JSON: `{"schema": "animus.acp-coding-agent.checks.v1", "checks": [{"id", "argv", "timeout_ms", "stdout_max_bytes", "stderr_max_bytes"}]}`. Ids match `^[a-z0-9][a-z0-9-]{0,63}$`, are unique, and argv is non-empty. Every budget must be present and at most the crate's ceilings. The file is read once at startup and its BLAKE3 identity enters the session receipt. A missing file means no checks, which is allowed; a malformed one refuses startup.

The benchmark writes this file per task, so the agent can run, for example, `cargo test` or `nickel export` without a shell.

### Provider

A provider service over `animus-std-openai`'s stream parser with the four-tool cohort. Endpoint URL, model id and API flavour are arguments; the credential is read from a file whose path is an argument, because stdin carries ACP. A missing credential file with a non-local endpoint refuses startup.

### Process shape

`animus-acp-agent --workspace <dir> --state <dir> --checks <file> --endpoint <url> --model <id> [--credential-file <path>]`. stdout carries only ACP frames; diagnostics go to stderr. The state directory holds the durable session store.

## Risks

- The tool cohort shape must survive `animus-std-openai`'s canonical request planning, which was written for one tool. Task 2.1 confirms this before the provider work.
- Harbor's ACP client may require capabilities `animus-acp-stdio-host` does not advertise. Task 3.2 tests the real client offline first.
