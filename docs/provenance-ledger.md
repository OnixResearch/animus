# Provenance ledger boundary

Animus owns its internal provenance and session-ledger contracts. Clankers is a bounded implementation reference for neutral replay, safe typed facts, BLAKE3 artifact identity, adapter-safe tool/effect receipts, and redaction discipline. Toolpath is an optional interchange and review vocabulary, not the Animus runtime ledger or conversation IR.

## Internal model

The reusable `animus-runtime` provenance module defines DTOs and pure helpers for:

- neutral in-memory ledger entries for model-visible messages, summaries, usage observations, safe receipts, and unsupported adapter-owned entries;
- deterministic replay into model-visible context plus safe counts;
- append-only safe provenance facts for model activity, tool activity, runtime blocks, errors, artifact references, authorization decisions, observations, and opaque future records;
- versioned BLAKE3 artifact envelopes with artifact kind, redaction class, normalized payload, and sorted dependency hashes;
- pure parent/branch queries for ancestry, active-head reconstruction, branch uniqueness, dead-end attempts, duplicate ids, missing parents, and cycles;
- safe runtime/provenance receipts that carry labels, bounded counts, artifact hashes, effect/mutation kinds, backend/interchange labels, and redaction markers only;
- adapter-safe tool/effect outcomes that fail before completed-effect evidence is claimed when services are missing, unavailable, denied, cancelled, or incomplete;
- redacted Toolpath-compatible Graph/Path/Step projection types.

The provenance module is a functional core over already-loaded data. The adjacent `animus-runtime::session_persistence` seam adds host-injected `LedgerSessionService` descriptors, replay/restore planning over already-loaded ledger entries, mutation-record DTOs, and safe ledger/session receipts. Neither module reads files, inspects environment, spawns processes, calls providers, uses clocks, discovers sessions, persists ledgers, or requires a concrete storage backend.

## Clankers adoption boundary

Animus adapts these Clankers patterns:

- neutral session replay over typed entries;
- append-only safe fact ledgers;
- BLAKE3 artifact identity and receipt hashes;
- receipt redaction classes and leak rejection;
- tool-host outcomes and effect request/result metadata;
- dependency-isolated reusable runtime contracts.

Animus does not adopt Clankers desktop storage, Automerge sessions, daemon/attach control plane, TUI, plugin runtime, Matrix/P2P routing, global session directories, transcript compatibility records, provider router, product CLI, or self-evolution workflows. The ledger/session seam rejects ambient global-store fallback unless a host supplies an explicit descriptor and already-loaded records.

## Toolpath matrix

| Toolpath surface | Animus decision |
| --- | --- |
| Graph / Path / Step DAG vocabulary | Adapt as optional projection vocabulary. |
| Actor taxonomy (`human:`, `agent:`, `tool:`, `ci:`) | Adapt as safe labels derived from Animus actors. |
| Parent and path-head semantics | Adapt by projecting Animus parent ids and active head. |
| Dead-end queries | Adapt as Animus-owned branch/dead-end helpers, then project. |
| Typed refs and correlation links | Adapt as safe `refs` fields in projections. |
| Structural perspectives | Adapt only when redacted and hash-backed. |
| JSONL streaming shape | Future shell/review concern; not internal storage. |
| Markdown/DOT review renderers | Future optional review surfaces. |
| Toolpath internal ledger/storage | Reject as Animus internal source of truth. |
| `toolpath-convo` runtime IR | Reject as runtime conversation IR. |
| Pathbase/cache/share/resume CLI | Reject from default runtime contracts. |
| Provider session importers | Reject from default runtime contracts. |
| Git/GitHub derivation | Reject from default runtime contracts. |
| Raw prompts, provider payloads, full paths, credentials, raw diffs, unbounded before/after content | Reject or redact by default; use hashes and redaction markers. |

## Evidence and validation

Focused validation lives in `scripts/cairn/check-provenance-ledger --root .`. It runs the runtime provenance tests and checks that default Cargo manifests do not introduce Toolpath, `toolpath-convo`, Pathbase, provider importer, Git/GitHub derivation, Clankers session/message, Automerge, daemon, Matrix, plugin-runtime, or global session-directory dependencies. The focused ledger/session seam test is `cargo test -p animus-runtime session_persistence --locked`.

Closeout evidence should name the focused rail, runtime crate tests, clippy/workspace checks when run, Nickel/config drift if applicable, Cairn validation, and Cairn gates. Any skipped rail must name the concrete blocker and next-best validation.
