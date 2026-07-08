# Design: Add turn lifecycle hook boundaries

## Context

The runtime loop already has a pure transition core and a thin shell for model/tool adapters. Lifecycle hooks are typed DTOs with safe payloads and host-owned execution. The missing turn-level seam should preserve those boundaries rather than creating an adapter-owned FSM.

## Decisions

### 1. Startup reuses `session_start`

**Choice:** Runtime startup is represented by the existing `session_start` lifecycle hook point. Animus does not add a second `startup` spelling.

**Rationale:** A duplicate startup event would make policy ambiguous. `session_start` already means the beginning of a runtime session and has the right safe payload model.

### 2. Add pre-turn and post-turn hook points

**Choice:** Add `pre_turn` before loop-fabric evaluation and `post_turn` after a turn result is recorded.

**Rationale:** Hosts need turn-scoped boundaries that are neither provider/tool-specific nor terminal-run-only.

### 3. Keep the FSM in pure runtime code

**Choice:** Model turn lifecycle progression as a deterministic pure core that maps typed state and events to next state plus lifecycle hook effects. Adapters execute only the effects they are handed.

**Rationale:** The runtime must decide what is allowed. Host adapters perform side effects but must not own state transitions or bypass policy.

### 4. Post-turn cannot deny completed work

**Choice:** `pre_turn` may deny before fabric evaluation. `post_turn` may observe, warn, annotate through safe warnings, or request continuation, but it cannot deny or rewrite already completed work.

**Rationale:** Denial belongs before a guarded action crosses its boundary. Post-turn is for observation and follow-up scheduling.

## Risks / Trade-offs

- Adding turn boundaries increases hook vocabulary surface area. Keep labels stable and tests explicit.
- Host hook execution can be nondeterministic. Reusable code emits typed requests and validates outputs; hosts own timeouts and service failures.
- Post-turn continuation requests must re-enter normal continuation policy instead of mutating runtime state directly.
