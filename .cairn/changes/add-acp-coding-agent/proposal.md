## Why

Animus has no headless coding agent that can change a workspace. The live Steel agent admits only `context_read` and `subagent`, its one tool is `steel_repl`, and its model and endpoint are compiled into its configuration. `animus-acp-stdio-host` can serve a coding session over ACP stdio, but no binary uses it, and the coding-session host has only a fake provider and a deny-all tool service. So Animus cannot be compared with other agent scaffolds on real coding tasks, for example through Harbor's ACP agent in the Onix `assay` benchmark.

The owner chose the authority for such a session: file edits confined to one workspace root, and command execution limited to checks the task declares. There is no general shell.

## What Changes

- Add the optional crate `animus-acp-coding-agent`, outside the default members, with the binary `animus-acp-agent`. r[agent_harness_runtime.acp_coding_agent.boundary]
- Add a workspace tool service for the coding-session host with four tools: `read_file`, `list_files`, `edit_file` (exact span replacement through `animus-std-workspace-edit`) and `run_check`. A pure core admits each request: the path is relative, normalized and inside the workspace root, and every size is bounded. The shell opens the root once as a capability and never follows a link out of it. r[agent_harness_runtime.acp_coding_agent.tools]
- `run_check` runs only a check named in a declared checks file, as an exact argv with a finite budget, through `animus-std-check-exec`. The file is read once at start; a missing, malformed, duplicate or unbounded entry refuses startup. r[agent_harness_runtime.acp_coding_agent.checks]
- Add a provider service for an OpenAI-compatible endpoint with the four-tool cohort, over the existing `animus-std-openai` stream parser. The endpoint, model and credential file come from explicit arguments, never ambient discovery. r[agent_harness_runtime.acp_coding_agent.provider]
- Serve the session over ACP stdio with `animus-acp-stdio-host`: `initialize`, `session/new` with the workspace as `cwd`, `session/prompt`, and `session/cancel`, with metadata-only receipts. r[agent_harness_runtime.acp_coding_agent.acp]
- Prove the ACP handshake and a full scripted session offline against Harbor's ACP client with a fixture provider. r[agent_harness_runtime.acp_coding_agent.harbor]
- Add positive and negative tests for every admission rule, and the focused Nix checks `cargo-acp-coding-agent` and `acp-coding-agent-octet`. r[agent_harness_runtime.acp_coding_agent.verification]

## Impact

- **Rust**: new workspace member `crates/animus-acp-coding-agent`, not in the default members. It depends on `animus-std` (coding-session host), `animus-acp-stdio-host`, `animus-std-workspace-edit`, `animus-std-check-exec` and `animus-std-openai`.
- **Nix**: the package `animus-acp-agent` and the two focused checks.
- **Non-claims**:
  - The tool service is not a sandbox. Harbor's environment (bubblewrap or a container) is the isolation boundary; the tool service narrows what the agent can ask for inside it.
  - Benchmark scores obtained through this binary measure this scaffold with a given model on given tasks. They do not establish Animus's general coding ability or superiority over other scaffolds.
  - No general shell, network tool, or package installation is provided, by design.
