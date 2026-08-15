# Design: Adapt the Zeron steerable external-agent harness for Animus

## Context

Zeron's durable command plane is documented in its `ARCHITECTURE.md`:
durable command entries, executor gated on chat ownership,
mark-processed-before-execute, a processed-command ledger, steering that
dispatches as the next turn when no live run exists, and offline sends
that stay queued. Animus already has queue identity and interrupt
classification contracts. It lacks the harness abstraction that runs an
external agent CLI as an injected, bounded subprocess and the durable
owner-gated command plane around it.

This change adapts the semantics, not Zeron's code. No vendoring is
planned, so the MIT license requires no notice change.

## Decisions

### 1. The harness is an injected service, not a daemon

**Choice:** Reusable crates define typed harness DTOs and validation.
A separate non-default std shell host owns the concrete agent-CLI bridge
and delegates the subprocess to the pinned `bounded-exec` API.

**Rationale:** This preserves the Animus functional-core /
imperative-shell split. No reusable crate spawns processes, discovers
agent CLIs, reads credentials, or opens sockets. `bounded-exec` is
`AGPL-3.0-or-later` and already consumed by `animus-std-check-exec`; this
change follows that precedent.

### 2. The command plane is durable typed data, not a CRDT

**Choice:** Send, steer, interrupt, and respond-input are typed durable
command entries in core state. Persistence, ordering, and transport are
host-owned. Ownership appears as an explicit owner identity on each entry.

**Rationale:** The Zeron ownership gate is about authority, not about Loro
or Cloudflare. Modeling it as a durable entry keeps the authority
semantics without adopting the sync stack. Multi-device transport and the
registry are deferred to a separate change after this one.

### 3. Mark-processed-before-execute is adopted verbatim

**Choice:** The executor marks a command processed before host execution
and consults the processed-command ledger to reject re-execution.

**Rationale:** This is the proven idempotence property from Zeron's design
and is a strict safety win over mark-after-execute.

### 4. Steering and offline commands have explicit semantics

**Choice:** A steer or respond-input with no live active run dispatches as
the next turn. Commands for an unavailable harness stay queued and never
enter transcript content.

**Rationale:** Silent drops corrupt a session's intent. The explicit
next-turn and queued semantics make every command's fate deterministic and
testable.

### 5. Harness and command-plane receipts are metadata-only

**Choice:** Receipts record command kind, sequence, command hash, outcome
class, status class, payload hash, harness family label, and bounded
counts. Raw prompts, harness JSONL lines, provider payloads, tool bodies,
model outputs, credentials, and full paths are excluded.

**Rationale:** This matches the accepted Animus safe-receipt discipline and
keeps durable evidence small and redaction-safe.

## Risks / Trade-offs

- **Overlap with existing queue contracts.** `coding-session-queue-cas`,
  `adapt-goose-message-queue-interrupts`, and continuation policy already
  own queue identity and interrupt classification. This change owns the
  harness outcome layer and durable command DTOs; it references the
  accepted queue and continuation contracts as prerequisites instead of
  duplicating them.
- **Bounded-exec licensing.** The subprocess dependency is
  `AGPL-3.0-or-later`. The std shell host that depends on it must remain a
  separate opt-in crate, consistent with existing `animus-std-check-exec`.
- **Steering subtlety.** Steering behaves differently with and without a
  live run. Tests must cover both paths plus the offline queue, or a later
  consumer will misread the contract.
- **Multi-device ownership is modeled, not transported.** Owner identity
  exists in the entry, but no device sync ships here. A later change can
  add transport without changing the command-plane contract.
