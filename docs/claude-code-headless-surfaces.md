# Claude Code headless automation surfaces

Animus treats Claude Code CLI, CI, and SDK headless automation as a bounded product reference for noninteractive run ergonomics. The reusable runtime owns typed DTO validation and event projection only; hosts own CLI flags, SDK transport, provider sessions, VCS side effects, and service wiring.

## Contract map

- `r[agent_harness_runtime.claude_code_headless.scope]`: this surface is a bounded reference. Animus does not copy Claude Code flags, require cloud sessions, create pull requests by default, or require networked providers.
- `r[agent_harness_runtime.claude_code_headless.scope.no_default_pr]`: completion receipts may include final responses, patch summaries, or commit-message proposals, but PR creation requires an explicit host adapter and policy grant.
- `r[agent_harness_runtime.claude_code_headless.request]`: `RunRequestDraft` and `RunRequest` validate objective, session/workspace identity, input references, policy profile, allowed capabilities, output contract, validation plan, noninteractive behavior, sandbox profile, and optional resume facts before execution.
- `r[agent_harness_runtime.claude_code_headless.request.invalid]`: request validation returns typed `AutomationError` diagnostics before any model, tool, shell, network, filesystem, or worker service is called.
- `r[agent_harness_runtime.claude_code_headless.events]`: `AutomationEventDraft`, `AutomationEnvelope`, and `AutomationEventKind` provide lifecycle, prompt, model, tool, permission, background work, validation, receipt, final response, completion, and failure event DTOs.
- `r[agent_harness_runtime.claude_code_headless.events.bounded]`: `summarize_event_payload` creates bounded visible windows with byte counts and BLAKE3 hashes for oversized payloads so raw full bodies are not embedded by default.
- `r[agent_harness_runtime.claude_code_headless.serialization]`: `project_event`, `AutomationEnvelope::jsonl_line`, and `AutomationEnvelope::preserves_record` preserve event identity, ordering, redaction markers, and BLAKE3 hashes without making JSONL or Preserves the core state model.
- `r[agent_harness_runtime.claude_code_headless.serialization.unsupported]`: unsupported projections return typed errors and do not fall back to lossy debug text.
- `r[agent_harness_runtime.claude_code_headless.completion]`: `CompletionReceipt`, `CompletionStatus`, and `select_final_output` define success, denied, prompt-required, input-required, validation-failed, service-unavailable, cancelled, and execution-failed outcomes with safe evidence references.
- `r[agent_harness_runtime.claude_code_headless.completion.prompt_required]`: `prompt_required_completion` produces prompt-required receipts for unresolved permission asks, and `CompletionStatus::allows_guarded_dispatch` remains false for that status.

## Non-claims

This contract does not add a daemon, cloud-session dependency, provider client, VCS hosting integration, default PR creation, shell-command surface, or transport-specific SDK. Those belong to explicit host adapters and separate policy grants.

## Focused verification

Use `cargo test -p animus-runtime headless --locked`. The focused tests include positive valid-request, event stream, JSONL/Preserves projection, completion, and final-output cases plus negative malformed request, duplicate identity/input/evidence, unsupported projection, oversized event, failed run, and prompt-required no-dispatch cases.
