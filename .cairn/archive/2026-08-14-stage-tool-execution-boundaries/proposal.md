# Proposal: Stage tool-execution boundaries

## Why

The Animus tool executor kit already performs exact catalog, schema, capability, approval, mutation, cancellation, deadline, executor-binding, output, and receipt checks. Coding sessions also commit tool calls and results before dependent provider work.

The current public contract does not define typed extension phases around that authoritative path. Hosts can add wrappers, but the ordering, denial behavior, output narrowing, commit barrier, and final observation rules are not one reusable contract.

DeepSeek Harness commit `47f943859bef60e4160492346772ded9b24f765a` provides a useful staged tool-interception pattern. Animus should adapt deterministic phase and monotonic-guard semantics. It should not adopt arbitrary plugin middleware, reorderable authority checks, or JavaScript interception.

## What Changes

- Add a typed stage plan for pre-admission, final admission, pre-execution, execution, output narrowing, final validation, durable commit, and post-commit observation. r[agent_harness_runtime.tool_execution_stages.plan]
- Require every stage candidate to re-enter Rust validation and bind the exact tool request, executor, authority seal, mutation target, output policy, and stage-plan identity. r[agent_harness_runtime.tool_execution_stages.binding]
- Make denials and authority narrowing monotonic. No later stage can add capability, reverse denial, widen mutation, increase bounds, or change failure into success. r[agent_harness_runtime.tool_execution_stages.monotonic]
- Permit model-visible output transformation only as bounded redaction or narrowing before final validation. r[agent_harness_runtime.tool_execution_stages.output]
- Require exact durable tool-result commit before provider continuation, then permit metadata-only observation with no mutation authority. r[agent_harness_runtime.tool_execution_stages.commit]
- Add positive and adversarial stage-order, substitution, widening, output, commit, and observation tests. r[agent_harness_runtime.tool_execution_stages.verification]

## Prerequisites

- `agent_harness_runtime.tool_executor_kit.admission`
- `agent_harness_runtime.tool_calling_breaking.bounded_result`
- `agent_harness_runtime.runtime_authority_seal.monotonic`
- `agent_harness_runtime.coding_session_host.commit_order`
- `agent_harness_runtime.turn_lifecycle_hooks.fsm`

## Impact

- **Pure core**: stage descriptors, plan identities, transitions, candidate validation, and typed denials in `animus-runtime`.
- **Shell**: ordered host-injected stage services around the existing executor kit, not instead of it.
- **Session**: the coding-session host owns the durable commit and provider-continuation barriers.
- **Existing active work**: `compose-rlm-host-effect-executor` and `add-live-steel-agent-shell` consume the staged boundary after acceptance.

## Out of Scope

- Built-in product tools, arbitrary plugins, dynamic handler discovery, provider calls from handlers, authority grants, effect rollback claims, or raw output in receipts.
- Moving final tool admission out of `tool_executor_kit` or moving durable ordering out of the coding-session owner.

## Affected Specs

- `agent-harness-runtime`: stage planning, exact binding, monotonic authority, output narrowing, commit barriers, and verification.
