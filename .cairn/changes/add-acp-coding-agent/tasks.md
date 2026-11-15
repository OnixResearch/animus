## Phase 1: Admission core

- [x] [serial] Add the non-default crate `animus-acp-coding-agent` with a pure admission core: path normalization and confinement, read and list bounds, exact-once edit admission, and tool-call parsing for the four tools. r[agent_harness_runtime.acp_coding_agent.boundary] r[agent_harness_runtime.acp_coding_agent.tools]
- [x] [serial] Add the declared checks file parser with schema, id, uniqueness, argv and budget-ceiling refusals, and its BLAKE3 identity. r[agent_harness_runtime.acp_coding_agent.checks]
- [x] [parallel] Add positive and negative tests for every admission rule and checks-file refusal. r[agent_harness_runtime.acp_coding_agent.verification]

Phase 1 evidence (2026-11-15): `cargo test -p animus-acp-coding-agent` (8 passed) and `cargo clippy -p animus-acp-coding-agent --all-targets -- -D warnings` in the workspace; `cairn validate --root .` valid with no issues, run with the `cairn` CLI built from the flake's pinned revision `5e1fe08` because the session could not fetch flake inputs. The roadmap rail and the octet check have not run.

## Phase 2: Services

- [x] [serial] Confirm `animus-std-openai` canonical request planning accepts a four-tool cohort; record the result in design.md before provider work. Result: it does not (one tool, named `steel_repl` only); the agent uses one `workspace` function, whose schema the host admits. r[agent_harness_runtime.acp_coding_agent.provider]
- [ ] [serial] Let `OpenAiToolCapability` admit a caller-registered tool name and schema label, with positive and negative tests in `animus-std-openai`. r[agent_harness_runtime.acp_coding_agent.provider]
- [ ] [serial] Implement the tool service shell: capability-root file access with symlink refusal, `animus-std-workspace-edit` publication for edits to existing files, exclusive-create for new files, and `animus-std-check-exec` for declared checks with an explicit environment. r[agent_harness_runtime.acp_coding_agent.tools] r[agent_harness_runtime.acp_coding_agent.checks]
- [ ] [serial] Implement the provider service with explicit endpoint and model, the credential from the `--credential-env` variable read once and removed, and session model advertisement if `animus-acp-stdio-host` supports it (open question in design.md). r[agent_harness_runtime.acp_coding_agent.provider]

## Phase 3: ACP binary

- [ ] [serial] Compose `CodingSessionHost` with the durable store and the new services, and serve it through `animus-acp-stdio-host` in `animus-acp-agent`. r[agent_harness_runtime.acp_coding_agent.acp]
- [ ] [serial] Run Harbor's ACP client against the binary offline with a fixture provider: edit a file, run a declared check, complete. r[agent_harness_runtime.acp_coding_agent.harbor]

## Phase 4: Packaging and closeout

- [ ] [serial] Add the Nix package `animus-acp-agent` and the checks `cargo-acp-coding-agent` and `acp-coding-agent-octet`. r[agent_harness_runtime.acp_coding_agent.verification]
- [ ] [parallel] Document the binary, its arguments, the checks file and the non-claims in `docs/acp-coding-agent.md` and `README.md`. r[agent_harness_runtime.acp_coding_agent.boundary]
- [ ] [serial] Register the change in `.cairn/roadmap.ncl` and pass the roadmap and Cairn validation rails. r[agent_harness_runtime.acp_coding_agent.verification]
- [ ] [serial] After review, sync the requirements into `.cairn/specs/agent-harness-runtime/spec.md` and archive the change. r[agent_harness_runtime.acp_coding_agent.verification]
