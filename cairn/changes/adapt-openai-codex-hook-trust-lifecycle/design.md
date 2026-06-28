# Design: Adapt OpenAI Codex hook trust lifecycle for Animus

## Context

Codex's hook documentation identifies a broad lifecycle surface and a trust model where non-managed command hooks must be reviewed before execution. Hook inputs are JSON event objects and outputs have event-specific effects. Animus can adapt those ideas into typed Rust DTOs and Nickel-owned configuration while keeping actual script/process execution in host shells.

## Decisions

### 1. Hook events are typed runtime extension points

**Choice:** Define hook event variants for session start, user prompt submit, pre-tool use, permission request, post-tool use, pre-compact, post-compact, subagent start, subagent stop, and stop.

**Rationale:** These events cover the useful Codex lifecycle without coupling Animus to Codex's exact names or command runner.

### 2. Trust is content-addressed

**Choice:** Hook definitions carry stable identities and BLAKE3 definition hashes. Trust records bind a hash to a scope and decision.

**Rationale:** Changed hook definitions become untrusted by default while managed policy can still pre-authorize specific hooks.

### 3. Hook effects are event-scoped

**Choice:** Each event declares which effects are accepted. Unsupported fields fail closed for that hook result and are recorded as hook-output errors.

**Rationale:** A hook that can add context should not implicitly gain the ability to rewrite tool input or stop a run.

### 4. Hooks augment policy, not replace it

**Choice:** Hook deny or allow recommendations flow through the same runtime policy boundary as normal permission decisions. Hooks cannot directly call adapters or weaken sandbox policy.

**Rationale:** Hooks are extensibility points, not a second privileged execution path.

### 5. Shells own hook execution

**Choice:** Runtime core models hook requests and responses. Concrete shells decide whether hooks run as processes, WASM modules, Steel snippets, or hosted services.

**Rationale:** Animus remains embeddable and testable while supporting multiple host execution strategies.

## Risks / Trade-offs

- More hook events increase surface area. Event-scoped outputs and typed validation keep the blast radius bounded.
- Trust records can become operationally heavy. Content hashes and managed scopes make changes explicit and reviewable.
- Hook order and concurrency can affect behavior. Animus should make ordering or parallelism a shell policy choice and record enough evidence to replay outcomes deterministically.
