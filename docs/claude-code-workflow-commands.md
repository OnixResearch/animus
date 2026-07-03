# Claude Code workflow command adaptation

Animus treats Claude Code slash commands as a bounded ergonomics reference for named workflow invocation. It does not adopt Claude command files as executable scripts, does not grant authority from prompt text, and does not open pull requests as a default command behavior.

## Contract

`animus-runtime` owns pure workflow command DTOs:

- `WorkflowCommandManifest` validates command name, summary, input schema, capabilities, side-effect posture, retrieval budget, validation gates, output contract, and safe provenance.
- `build_workflow_command_registry` builds a deterministic in-memory registry and emits source-id-only duplicate diagnostics.
- `bind_workflow_command_arguments` validates invocation arguments before prompt rendering, tool dispatch, or workflow mutation.
- `prepare_workflow_command_invocation` binds arguments and checks host-supplied policy allowlists before a workflow can be expanded.
- `validate_workflow_command_output` checks completion evidence against explanation, review, patch-summary, validation-report, commit-message-proposal, or structured-DTO contracts.

Hosts still own command discovery, prompt rendering, model/tool adapters, permissions, VCS operations, shell execution, network-capable services, persistence, and UI. Reusable runtime code only accepts already-loaded manifests and already-parsed arguments.

## Non-goals

- No shell-script execution from command content by default.
- No pull-request creation or PR management.
- No command prompt text as a substitute for tool, permission, capability, or side-effect policy.
- No raw prompt bodies, credentials, provider payloads, private paths, or unbounded tool output in registry diagnostics or output evidence.

## Verification

Focused rail:

```sh
cargo test -p animus-runtime workflow_command --locked
```

The focused tests include positive manifest, registry, binding, and output cases plus negative invalid manifest, duplicate, malformed argument, policy-denial, output mismatch, unsafe text, and invalid provenance cases.
