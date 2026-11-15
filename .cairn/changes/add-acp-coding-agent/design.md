## Context

Harbor (0.24) runs ACP agents from a registry entry; a `local` distribution names a command that Harbor starts inside the task environment and speaks to over stdio. The Onix `assay` benchmark uses that path for Animus, so the binary must work with no network except the model endpoint, no package manager, and a workspace at a path Harbor chooses.

Existing pieces:

- `animus-std::CodingSessionHost` drives a session through nine injected services. Real implementations exist for persistence (`durable-session-store`), policy, authority, cancellation, timer and event sinks. Only fakes exist for the provider and tool services.
- `animus-acp-stdio-host` owns ACP framing, bounds, correlation and receipts, and adapts any `CodingSessionHost` through `AcpCodingSessionFacade`.
- `animus-std-workspace-edit` plans and publishes exact span edits against an admitted snapshot, with freshness checks, from one caller-opened capability root.
- `animus-std-check-exec` runs only `AllowedCheck`s (exact check id, argv and budget) with bounded output.
- `animus-live-steel-agent` has a provider stream service over `animus-std-openai`, but its tool cohort is fixed to `steel_repl`.

## What Harbor's ACP client provides

Read from Harbor 0.24 `agents/installed/acp_runner.py`:

- The agent process starts with `cwd` set to the task workspace, and `session/new` carries the same path as `cwd`. The workspace root therefore comes from the session, not from a flag.
- The child inherits Harbor's process environment. Credentials reach a local agent only through environment variables (Harbor's ACP authenticate methods also key on env vars), so a credential file inside the sandbox is not a usable channel.
- The requested model arrives as `HARBOR_ACP_REQUESTED_MODEL` and is applied with `session/set_model` or a model config option, only if the session advertises models.
- The client advertises `fs.read_text_file`, `fs.write_text_file` and `terminal`, and by default (`HARBOR_ACP_PERMISSION_MODE=allow`) grants every `session/request_permission`. ACP permission prompts are therefore not a boundary here; this agent's own admission is. The agent does not use the client's fs or terminal methods.

## Decisions

### Authority: edits under one root and declared checks only

The owner chose this over a general shell and over Steel skills. Requests are admitted by a pure core before any effect:

- Paths are relative UTF-8, normalized without `.` or `..`, with no empty, absolute or NUL components, and at most `MAX_PATH_BYTES` long. The shell resolves them under a root opened once with `openat`-style capability handles and refuses symlinks on every component.
- Reads return at most `MAX_READ_BYTES`; listings at most `MAX_LIST_ENTRIES`.
- Edits are exact `old`/`new` replacements. `old` must occur exactly once in the current snapshot; creating a file uses an empty `old` on an absent path. The bounds equal or tighten workspace-edit's: 64 KiB per replacement, 1 MiB per file, and 16 edits per call against its 64 operations.
- `animus-std-workspace-edit` edits only existing files. Edits to existing files publish through it; creation is a separate shell path that opens the parent capability-relative, refuses links, and creates with exclusive-create semantics.
- `run_check` takes only a check id. The argv, working directory (the root) and budget come from the declared checks file.

### Declared checks file

JSON: `{"schema": "animus.acp-coding-agent.checks.v1", "checks": [{"id", "argv", "timeout_ms", "stdout_max_bytes", "stderr_max_bytes"}]}`. Ids match `^[a-z0-9][a-z0-9-]{0,63}$`, are unique, and argv is non-empty. Every budget must be present and at most the crate's ceilings. The file is read once at startup and its BLAKE3 identity enters the session receipt. A missing file means no checks, which is allowed; a malformed one refuses startup.

The benchmark writes this file per task, so the agent can run, for example, `cargo test` or `nickel export` without a shell.

### Provider

A provider service over `animus-std-openai`'s stream parser with the four-tool cohort. The endpoint URL and API flavour are arguments. The model is an argument, and the session advertises it (plus any `--allowed-model` values) so Harbor's requested model selects among them. The credential is read once at startup from the environment variable named by `--credential-env`, removed from the process environment before serving, and never passed to checks, which run with check-exec's explicit environment. stdin is not a credential channel because it carries ACP. A missing credential for a non-loopback endpoint refuses startup.

### Process shape

`animus-acp-agent --state <dir> --checks <file> --endpoint <url> --model <id> [--allowed-model <id>]... [--credential-env <NAME>] [--allowed-root <dir>]`. The workspace root is the `cwd` of `session/new`; with `--allowed-root`, a `cwd` outside it is refused. stdout carries only ACP frames; diagnostics go to stderr. The state directory holds the durable session store.

## Open questions

- Whether `animus-acp-stdio-host` advertises session models; if not, model selection stays argument-only and assay passes the model through the registry entry.
- Which ACP methods `animus-acp-stdio-host` answers beyond `initialize`, `session/new`, `session/prompt` and `session/cancel`; Harbor also calls `authenticate` when the agent advertises methods.

## Risks

- The tool cohort shape must survive `animus-std-openai`'s canonical request planning, which was written for one tool. Task 2.1 confirms this before the provider work.
- Harbor's ACP client may require capabilities `animus-acp-stdio-host` does not advertise. Task 3.2 tests the real client offline first.
