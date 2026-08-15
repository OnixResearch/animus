## Why

Animus composes exact per-model prompt, skill, memory, and tool fragments, but the composition contract does not require fragments to be conditional on tools and skills actually available for a turn. A role that names a missing tool can still emit instructions for it, which wastes context and misleads the model.

An explicit composition invariant should render each fragment only when the named tool or skill is available for the turn and deny dangling instructions under composition. Tool discovery, model dispatch, and policy enforcement remain host-owned.

## What Changes

- Make availability an explicit input to prompt composition, resolved through extension defaults, model compatibility, policy rules, and the selected role. r[agent_harness_runtime.capability_conditional_prompt.availability]
- Render each fragment only when the named tool or skill is available for the turn. r[agent_harness_runtime.capability_conditional_prompt.conditional]
- Reject a composition in which a fragment names a missing tool or skill, with a mandatory negative fixture. r[agent_harness_runtime.capability_conditional_prompt.missing]
- Fail closed on unresolvable capability with a bounded diagnostic. r[agent_harness_runtime.capability_conditional_prompt.fail_closed]
- Emit metadata-only receipts. r[agent_harness_runtime.capability_conditional_prompt.receipts]
- Add positive and negative fixtures. r[agent_harness_runtime.capability_conditional_prompt.verification]

## Impact

- **Rust**: extends the capability-bound prompt composition surface in the runtime; the availability resolver stays host-owned.
- **Consumers**: composed prompts stay aligned with the tools and skills that can actually run.
- **Claims**: availability discovery and live tool admission remain outside this change.
