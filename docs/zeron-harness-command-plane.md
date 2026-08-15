# Zeron harness and durable command-plane contracts

## Context

Zeron (`github.com/zeronsh/comet`, MIT) controls coding-agent CLIs through
durable command entries. Its design marks a command processed before host
execution, keeps a processed-command ledger, and gates execution on session
ownership. Steering with no live run dispatches as the next turn. Offline
sends stay queued.

Animus adapts these boundaries as a bounded reference. The adaptation is
semantics, not code. No CRDT, edge worker, or UI layer is imported.

## Ownership

- `crates/animus-adapter-zeron-harness` is a `no_std` functional contract
  crate. It owns typed command DTOs, payload identities, the pure
  owner-gated command plane, the implementation-free bounded process
  service port, and metadata-only receipts.
- `crates/animus-std-zeron-harness` is a non-default `AGPL-3.0-or-later`
  host that implements the port over the pinned `bounded-exec` surface. It
  owns process spawn, bounded I/O, deadlines, cancellation shaping, owned
  teardown, working-directory resolution, and environment translation.

Reusable crates never spawn processes, discover executables, read
credentials, open transports, or apply implicit default limits. The host
owns those effects.

## Contract

The crate API is the reviewed contract. Key surfaces:

- `HarnessCommand`, `CommandEnvelope`, `CommandKind`, and `Outcome` model
  send, steer, interrupt, and respond-input with explicit outcome classes.
- `PayloadIdentity` and `command identity` use domain-separated BLAKE3.
  Identity material is hash-based and never embeds raw payload bodies.
- `PlaneState`, `evaluate_command`, `begin_execution`, and
  `complete_execution` implement the owner gate, the
  mark-processed-before-execute invariant, and the processed-command
  ledger. Steer with no active run dispatches as the next turn. Commands
  for an unavailable harness stay queued and never enter transcript content.
- `BoundedProcessRequest` requires every execution bound explicitly.
  `plan_harness_dispatch` fails closed when the injected service is
  unavailable.
- `BoundedAgentProcessService` is the implementation-free injected port.
- `PlaneReceipt`, `HarnessReceipt`, and `validate_metadata_only` keep all
  durable evidence metadata-only. Raw prompts, harness JSONL, provider
  payloads, tool bodies, model outputs, credentials, full paths, and secret
  summaries are excluded.

## Behavior evidence

Positive coverage:

- bounded payload and command identities
- owner-gated send and interrupt execution
- steer with an active run, and steer with no active run dispatching as the
  next turn
- mark-processed-before-execute and processed-command skip
- explicit bounded process requests
- real bounded subprocess success, exit visibility, and flood retention

Negative coverage:

- empty and oversized payloads
- raw body material leaking into identity
- payload hash mismatch
- non-owner enqueue without execution
- unavailable-harness queuing
- stale processed markers
- empty, missing, and oversized execution limits
- dispatch fail-closed without an injected service
- receipt candidates that carry raw material
- missing executable, expired deadline, and output flood through the real
  host

## Commands

Run the focused rail from the repository root:

```sh
nix develop -c scripts/cairn/check-zeron-harness --root .
```

Indirect checks:

```sh
cargo test -p animus-adapter-zeron-harness --locked
cargo test -p animus-std-zeron-harness --locked
cargo clippy -p animus-adapter-zeron-harness -p animus-std-zeron-harness --all-targets -- -D warnings
cargo check -p animus-adapter-zeron-harness --target wasm32-unknown-unknown --locked
```

## Non-claims

The contracts do not adopt the gpui UI, Loro CRDT sync, the TypeScript
edge worker, Cloudflare Durable Objects, token-usage display, or
multi-device transport. The adaptation does not prove product sandboxing,
executable authorization, harness-output meaning, credential freshness,
release eligibility, or multi-device convergence. The host's process
observations are bounded facts supplied by `bounded-exec`; they do not
prove child correctness or host equivalence. The root `zeron-harness-host`
feature is a composition facade; the adapter and host retain their own
authority boundaries.
