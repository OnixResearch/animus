# Coding-session host facade

Animus exposes one non-default coding-session composition seam without turning the reusable runtime into an application shell. The facade proves a bounded durable fake-backed session rung: deterministic fake provider and summarizer services can run through an explicit local durable store, ordered provider/UI-neutral events, restart recovery, and metadata-only receipts.

It does **not** prove live-provider behavior, concrete product tools, TUI dogfood, daemon attach, plugin execution, OAuth, global session discovery, or production readiness.

## Public boundary

The pure API is `animus_runtime::coding_session_host`. It owns validated session state, commands, effect requests and outcomes, deterministic message encoding, receipt projection, and reducers. It performs no filesystem, provider, tool, timer, terminal, credential, process, network, or persistence effect.

The shell API is `animus_std::coding_session_host`, enabled only by the `coding-session-host` feature. `CodingSessionHost` exposes:

- `prompt` and `continue_run`;
- `steer` and `follow_up`;
- compare-and-swap queued edit and removal with explicit entry, generation, position, and private text-identity preconditions;
- `cancel`, `resume`, and `compact`;
- state, committed-message, and ordered-event snapshots;
- an injected `CodingSessionEventSink` for event subscription;
- metadata-only terminal receipt projection.

Construction requires explicit provider, tool, persistence, policy, authority-enforcement, cancellation, timer, summarizer, and event-sink services. A missing service is a typed failure; the host does not discover a replacement from ambient state. Inputs must already be expanded and admitted by caller-owned command, resource, prompt-template, skill, and project policy.

## No-duplication ownership map

| Concern | Existing owner reused by the facade | Facade responsibility |
| --- | --- | --- |
| Agent turn and adapter DTOs | `animus-runtime` loop contracts and `animus-adapters` model/tool DTOs | Correlate session effects; do not redefine provider or tool payloads. |
| Steering and follow-up | `animus-runtime` message-queue reducer and [queue compare-and-swap](coding-session-queue-cas.md) | Hold bounded pending bodies, require exact mutation preconditions, and select injection; do not treat pending UI intent as transcript history. |
| Continuation and interrupted tools | continuation and `interrupted_tool_repair` contracts | Block or synthesize one durable repair before continuation; never re-execute a recovered tool call. |
| Durable framing and recovery | `animus-runtime::durable_session_store` and non-default `animus-std/durable-session-store` | Adapt committed messages to explicit store transactions and consume validated recovery snapshots. |
| Session authority continuity | [`session_authority_profile`](session-runtime-authority-profile.md) and injected host probes | Commit requested metadata before effects, block drift on resume, and re-probe current enforcement without treating identity equality as authority. |
| Session-tree and branch metadata | session-tree and persistence contracts | Preserve the configured branch and active leaf; do not discover sessions or own a global store. |
| Runtime observation | `RuntimeEvent`, headless event projection, observability projection, and optional Preserves envelopes | Emit one ordered provider/UI-neutral stream; session envelopes carry validated metadata only. |
| TUI state | `animus-tui::CodingSessionRenderModel` | Reduce the shared event stream; do not mutate authoritative transcript state. |
| Compaction | injected summarizer plus existing compaction events | Commit the summary before emitting the compaction boundary. |
| MCP tools | separate non-default `animus-std-mcp` host plus existing MCP mapping and tool authority contracts | Supply only an already-admitted tool effect. The coding-session host does not discover, start, trust, or restart MCP servers. |
| Product readiness | dogfood readiness and focused-rail documents | Claim only the bounded fake-backed durable rung and list omitted surfaces. |

Resource discovery, slash commands, provider login, credential lookup, model catalogs, keybindings, themes, rendering, and terminal ownership remain outside the facade.

## Deterministic ordering invariants

The reducer returns next state, ordered events, and at most one explicit effect request. The shell commits before every dependent effect:

1. metadata-only authority-profile binding before the first user/provider boundary;
2. admitted user message and active leaf before provider dispatch;
3. assistant tool-call message before tool dispatch;
4. exact staged tool result before post-commit observation and provider continuation;
5. final assistant message before terminal completion;
6. selected queued input before it becomes model-visible;
7. summary or synthetic repair before its completion event.

One session admits at most one active or cancelling run. A stale queue mutation returns a metadata-only conflict without state mutation. An overlapping ordinary prompt, stale run or cancellation identity, stale effect, malformed recovery snapshot, unavailable required service, failed commit, or ambiguous leaf fails closed. Cancellation is a correlated request and observation; it is not proof that an external provider, tool, process, filesystem, or network effect rolled back.

## External pause and resume

`CodingSessionAgentControlGate` composes an injected `AgentControlWait` port with `drive_one_pending_with_gate` or `drive_pending_with_gate`.

The gate lets prerequisite commit, recovery, and cancellation effects continue. It blocks provider, tool, and summarizer effects at their declared safe checkpoints.

`ResumeRun` means live control recovery. Existing `CodingSessionCommand::Resume` still means durable session recovery. See [external agent control](agent-control-pubsub.md).

## Recovery and restart

`resume` must run before prompt or continuation when the state declares recovery required. The host reopens only the explicitly supplied durable store, consumes its validated committed head and active leaf, decodes bounded messages, compares the committed and current requested authority profiles, and asks the pure reducer to validate lineage and tool bindings. A clean unmatched tool call produces a synthetic terminal repair that must commit before recovery becomes ready. Incomplete tails, committed corruption, cursor or leaf disagreement, malformed tool bindings, or unavailable persistence keep the session blocked.

The separate [active-turn recovery](active-turn-recovery.md) contract protects accepted work before provider dispatch. Host integration must bind its transaction evidence to this session authority.

The [pending-input vault](pending-input-vault.md) combines active and queued durable references for one explicit restart review. Restore does not bypass this host. Each restored reference must pass current queue, authority, policy, durable-commit, and provider gates before the host can load its body or submit work. A corrupt or cleanup-required vault blocks ordinary ingress.

The [transactional daemon handoff](transactional-daemon-handoff.md) binds this host's durable session head and composer-draft identity before lease release. A replacement worker must prove those exact values before the old worker can exit. Handoff does not gain transcript, queue, provider, tool, or global-store authority from this host.

The [revision-gated workspace edit host](revision-gated-workspace-edit-host.md) is a separate optional effect owner. This coding-session host does not open workspace roots or publish files. An embedder must pass an admitted edit through its existing tool, authority, approval, mutation, cancellation, deadline, and output gates before it calls the edit host. A successful edit receipt does not prove semantic correctness or authorize a later provider turn.

Child-process tests prove that a committed session reopens after process exit and that an incomplete trailing transaction does not become continuation authority. These tests do not prove filesystem, kernel, device-cache, power-loss, backup, replication, encryption, or production-recovery correctness.

## Safe evidence

`project_coding_session_receipt` exposes schema, session and branch identities, terminal class, bounded counters, durable cursor, committed head, authority-profile, and authority-binding hashes, a redaction marker, explicit non-claims, and a BLAKE3 receipt identity. It rejects raw prompts, provider payloads, tool output, credentials, capability paths, store paths, and raw errors.

The packaged `animus-coding-session-smoke` binary uses deterministic fake services and an explicit temporary store supplied by the caller. Its output is bounded receipt metadata, not a transcript or product-readiness claim.

## Optional ACP stdio caller

The [`animus-acp-stdio-host`](acp-stdio-host.md) crate is an optional protocol caller of this facade. `CodingSessionEffectGate` and the `drive_*_with_gate` methods allow a protocol-side permission decision to deny before an effect, but an allow never replaces the existing authority probe or injected service admission. ACP new/load/prompt/cancel requests delegate into this same host; ACP does not own a second transcript, queue, run state, active leaf, durable store, or tool executor.

## Tool-stage commit boundary

`CodingSessionCommitRequest::tool_response_after_commit` carries a final-validated staged tool result. The reducer checks its exact result and commit-owner bindings after the durable message commit.

The host then calls `CodingSessionToolService::observe_committed` with metadata-only commit facts. Observer failure blocks provider continuation. The observer cannot change the committed result.

See [tool execution stages](tool-execution-stages.md).

## Model-request snapshot boundary

`CodingSessionProviderRequest` carries the committed session, branch, active leaf, durable head, and request sequence. The reducer derives these values from state.

`SnapshotBoundCodingSessionProvider` wraps an existing provider service. It commits a complete private snapshot and rechecks current bindings before it calls that service. See [durable model-request snapshots](model-request-snapshots.md).

## Verification

Run from the repository root:

```sh
scripts/cairn/check-coding-session-host --root .
cargo test -p animus-std-control --locked
cargo test -p animus-runtime sessionhost::tests --locked
cargo test -p animus-std --features coding-session-host --locked
cargo test -p animus-tui frontend --locked
cargo test -p animus-preserves coding_session_events --locked
cargo run -p animus-std --features coding-session-host --bin animus-coding-session-smoke -- --store-root "$TMPDIR/animus-coding-session-smoke" --prompt bounded
```

The focused receipt is written to `target/cairn/focused-rails/coding-session-host.md`. The rail is bounded evidence for this composition seam; it is not a release or production certificate.
