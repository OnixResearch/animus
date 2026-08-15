# Proposal: Adapt the Zeron steerable external-agent harness for Animus

## Why

Zeron (previously `comet`, `github.com/zeronsh/comet`, MIT) is a Rust
rewrite of a local-first multi-device controller for coding agents. Its
command plane treats send, steer, interrupt, and respond-input as durable
command entries. The host device that owns the session executes them.
Follower devices only enqueue. The executor marks a command processed
BEFORE executing it and keeps a processed-command ledger, so replay cannot
double-execute. A steer with no live run dispatches as the next turn.
Offline sends remain queued.

Animus already owns queue identity (`coding-session-queue-cas`),
queued-message and interrupt classification (`goose-message-queue-interrupts`),
active-turn recovery, and continuation policy. The missing boundary is a
typed contract for controlling an external agent CLI subprocess as an
injected, bounded service, plus the durable owner-gated command plane
around it. This change adapts that missing boundary from Zeron as a
bounded reference.

## What Changes

- Define a typed external-agent harness contract with send, steer,
  interrupt, and respond-input operations, neutral command DTOs, and
  explicit outcome classes.
- Require injected bounded external-process delegation for harness
  execution, with explicit limits, deadlines, cancellation, and owned
  teardown. Reusable core crates do not spawn processes. Dispatch fails
  closed when the bounded-process service is unavailable.
- Define durable command-plane entries with stable identity, kind,
  sequence, target session, ownership metadata, payload hash, and safe
  envelope metadata.
- Gate command execution on host ownership, mark commands processed
  before execution, and reject re-execution through a processed-command
  ledger.
- Define steering and offline semantics: a steer with no live active run
  dispatches as the next turn, and commands for an unavailable harness
  stay queued without entering transcript content.
- Emit safe metadata-only receipts for command-plane and harness
  decisions.
- Add positive and negative tests for every behavior above.

## Impact

- **Files**: harness and command-plane DTOs and pure validation in core
  or adapter crates; a non-default std shell host that delegates the
  subprocess to pinned `bounded-exec`; tests; docs.
- **Testing**: valid command admission, owner-gated execution,
  processed-ledger idempotence, steer-with-no-active-run, offline queue,
  missing or timed-out bounded-process service, invalid execution bounds,
  malformed harness outcomes, and receipt redaction.
- **Non-claims**: this change does not adopt the gpui UI, Loro CRDT sync,
  the TypeScript edge worker, Cloudflare Durable Objects, or token-usage
  display, and it does not ship multi-device transport (deferred to a
  future change). It does not claim harness, cloud-session, or agent-CLI
  equivalence.
