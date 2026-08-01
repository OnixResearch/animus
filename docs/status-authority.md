# Agent status authority

Animus represents agent status as typed observations plus a pure runtime selector. Host integrations may observe processes, terminals, panes, or native sessions, but reusable crates receive only validated `animus-adapters` DTOs.

## Contracts

- `animus-adapters::AgentStatusObservation` carries semantic state, source identity, target identity, agent label, optional native session reference, bounded message text, presentation metadata, sequence, TTL, and safe BLAKE3 evidence hashes.
- `animus-adapters::HostStatusAdapter` is the implementation-free port for hosts that can supply observations.
- `animus-runtime::select_agent_status` is a pure selector over a `HostStatusRequest`, logical sequence, and in-memory observations.
- `animus-runtime::StatusDecisionExplanation` records selected authority, rejected candidates, freshness, fallback reason, safe hashes, and bounded redacted summaries.

## Precedence

A current trusted lifecycle observation owns semantic state for its target and suppresses heuristic fallback. Expired or untrusted lifecycle observations are rejected and degrade deterministically to a current heuristic fallback when one exists, otherwise to `unknown`. Conflicting current lifecycle authorities fail closed to `unknown`.

Presentation fields such as titles, display agent names, activity text, and custom labels are visual-only. They never change semantic state, waits, notifications, rollups, or policy decisions.

The [agent-control state machine](agent-control-pubsub.md) owns live pause, resume, and cancellation. A host can project its current phase as a trusted lifecycle observation. Status selection cannot wake or resume a controlled run.

## Evidence safety

Durable status explanations include safe hashes and bounded summaries only. Raw prompts, terminal snapshots, provider payloads, credentials, and unbounded detector transcripts must stay outside reusable runtime evidence.
