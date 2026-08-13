# Design: Live Steel agent shell

## Context

Animus components currently prove isolated contracts and bounded host rungs. The fake-backed coding-session smoke does not use a live provider. The live OpenAI smoke does not support tools or coding-session continuation.

The application shell must compose accepted components. It must not implement another agent loop.

## Success Contract

A headless run can accept one prompt, call the exact configured provider, receive a `steel_repl` tool call, evaluate Steel, execute admitted host requests, return the tool result, and reach a final assistant response.

The shell durably commits each accepted message and tool boundary in the required order. Restart restores only committed session and kernel safe points.

A binary that bypasses the coding-session facade, uses fake-only adapters, prints a host-request marker, or stops after the first provider tool call is false completion.

## Decisions

### 1. Put all concrete selection in one composition root

The root constructs:

- validated runtime and authority policy
- durable session store
- coding-session host
- tool-capable OpenAI provider host
- Steel kernel and resumable bridge
- complete RLM host executor
- child-session service
- selected Steel skill modules
- event and safe receipt sinks
- control and cancellation services

No reusable core selects these adapters.

### 2. Reuse coding-session tool-loop semantics

The shell submits prompts through the coding-session facade. Provider tool calls become committed assistant tool-call messages before dispatch.

The `steel_repl` result commits as a tool result before provider continuation. The loop repeats until final response, denial, cancellation, budget stop, or terminal failure.

### 3. Use typed Nickel configuration

Human-authored configuration names exact component identities, model profile, provider endpoint cohort, credential reference, capability roots, command profiles, tool bindings, store descriptor, skill modules, limits, and non-claims.

The shell validates the deterministic export before it opens the store, resolves credentials, or contacts the provider.

### 4. Keep credentials and roots explicit

The shell can receive an explicit credential value or an approved credential service result. It does not discover provider accounts.

Workspace and store locations use explicit capability roots. Current-directory defaults are not authority.

### 5. Preserve ordered recovery facts

The shell uses queue compare-and-swap, active-turn recovery, interrupted-tool repair, and durable kernel restore.

A crash after an observed effect does not cause automatic effect replay. Recovery returns a typed blocked state when safe continuation is not possible.

### 6. Separate deterministic and live evidence

The required conformance run uses literal provider stream fixtures and controlled host adapters through production entrypoints.

An optional live run requires explicit credentials and records a separate safe receipt. It does not upgrade the whole shell to production-ready status.

## Composition Flow

```text
headless command
  -> typed configuration and authority admission
  -> durable coding session
  -> tool-capable provider stream
  -> committed steel_repl call
  -> resumable Steel bridge
  -> admitted RLM host executor or child service
  -> committed tool result
  -> provider continuation
  -> committed final assistant result
  -> safe events and receipt
```

## Alternatives Considered

### Extend the current fake dogfood binary

Rejected because its bounded claim and dependency graph intentionally exclude live provider and product effects.

### Build a new loop directly in the CLI

Rejected because this would duplicate coding-session ordering, recovery, cancellation, and event policy.

### Require daemon and TUI in the first shell

Rejected because headless composition is independently testable and is a prerequisite for those product shells.

## Risks and Controls

- A composition root can hide authority. Construction requires explicit typed values for every external capability.
- Tool loops can run without end. Named turn, call, token, time, and output bounds stop them.
- Restart can duplicate effects. Commit ordering, repair, captured results, and no-effect replay fail closed.
- Live provider behavior can drift. Fixture and optional live claims remain separate.

## Non-Claims

This design does not establish sandboxing, provider correctness, tool correctness, task success, daemon reliability, terminal compatibility, production readiness, or release eligibility.
