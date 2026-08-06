## Context

Pijul `1.0.0-beta` is a distributed version control system built on a theory of patches: changes commute or depend, application is associative, and conflicts are regular internal data resolved by one change. Repositories live in a `.pijul` directory over a Sanakirja database. Channels replace branches. Changes are identified by base32 BLAKE3 hashes and can be signed with locally managed identity keys. The CLI (`record`, `unrecord`, `apply`, `fork`, `channel`, `log`, `change`, `diff`, `reset`, `credit`) is the stable user surface; the underlying crates are GPL-2.0 family with at least one AGPL-3.0 component.

Animus already has overlapping but distinct owners: checkpoint/rollback DTOs own backend-neutral rollback targets, the snapshot ledger owns recovery snapshot plans, the provenance domain owns VCS-neutral attribution observations, isolated workspaces own task sandbox descriptors, process-job profiles describe host-dispatched work, and safe receipts bound evidence. The new adapter must implement those existing ports rather than establish a second ledger, provenance store, or workspace mechanism.

A bounded architecture search considered linking `libpijul` into a Rust adapter crate, direct Sanakirja database access, and a pinned CLI behind an injected bounded executor. The CLI-service route is the only initial mechanism that preserves license separation, keeps ambient filesystem/process authority out of reusable crates, and matches the existing Animus host-adapter pattern.

## Decisions

### Decision: pin one exact CLI cohort, keep linking a future option

**Choice:** The initial adapter supports a reviewed `pijul` CLI release in the `1.0.0-beta` line at a recorded upstream source revision, with its change-hash format, `--output-format` machine-output identity, and GPL-2.0-family license cohort recorded in policy and conformance evidence. Reusable, default, no-std, and Wasm crates never link `libpijul`, `sanakirja`, or any Pijul crate, and never open `.pijul` databases directly. Licensing is not the driver: Animus accepts AGPL-compatible relicensing if a later change selects an in-process `libpijul` backend, and the backend service port is shaped so a linked host adapter could implement the same port without touching reusable cores.

**Rationale:** Pijul is beta software whose internal formats and APIs evolve independently of the CLI surface. Linking would couple a host crate to unstable internals and the Sanakirja on-disk format, and would still have to sit behind the same injected-service boundary to keep reusable crates authority-free. The CLI process boundary makes the exact reviewed behavior set explicit and defers, rather than forecloses, an in-process backend.

### Decision: read-only by default, protected mutations behind policy and confirmation

**Choice:** Query operations (`log`, `change`, `diff`, channel listing, status) are the default admitted class. Mutating operations (`record`, `unrecord`, `apply`, `fork`, `channel switch`, `reset`, `move`, `remove`) are a protected class that requires an explicit policy grant, a validated plan, and host confirmation, mirroring the snapshot-ledger protected-action model. No automatic per-turn recording, auto-commit, or background snapshot policy is introduced.

**Rationale:** An agent that can silently rewrite repository history defeats the audit value of a VCS backend. The existing Animus protected-action and confirmation machinery already covers this shape; the adapter reuses it.

### Decision: exclude network, identity, and hosting surfaces

**Choice:** `clone`, `pull`, `push`, `remote`, Nest integration, `identity new/prove`, key storage, and signing ceremonies are excluded. Signature handling is limited to reporting signature-status classes from CLI output; Animus does not verify signatures, manage keys, or grant authority based on authorship.

**Rationale:** Network transport, credential custody, and identity authority are separate claim surfaces with their own trust models. Beta-stage identity semantics are not a stable interop contract.

### Decision: functional core with an injected bounded executor

**Choice:** `animus-adapters` defines request/result/observation DTOs and a Pijul backend service port. `animus-runtime` owns pure validation, classification, and projection over loaded values. A dedicated non-default std adapter builds argv vectors, parses bounded stdout/stderr, and invokes only an explicitly injected bounded executor with an exact supplied executable identity. It does not search `PATH`, inspect environment variables, scan for `.pijul` ancestors, or call `std::process::Command` as a fallback.

**Rationale:** Every Animus policy decision stays unit-testable without a repository, a CLI binary, or filesystem state, and host authority remains explicit and visible.

### Decision: make Nickel policy authoritative

**Choice:** A typed Nickel profile owns the exact cohort, operation-class allowlists, named log/diff/record/path/channel bounds, channel allowlists, identity and signature-status handling, timeout/resource labels, projection targets, receipt fields, and required non-claims. Generated JSON is checked in and Rust revalidates it before any invocation or projection.

**Rationale:** These are reviewable configuration choices with cross-field invariants. Runtime Rust must not evaluate Nickel or trust generated bytes blindly.

### Decision: revalidate machine outputs in pure Rust

**Choice:** The pure validator checks CLI protocol facts before any projection: base32 BLAKE3 change-hash syntax and length, unambiguous hash prefixes, channel-name character policy, state-identifier shape, output-envelope fields, conflict-status classes, exit-status mapping, record ordering, and configured collection/body bounds. Unknown fields, hash shapes, channels, statuses, or over-limit values fail closed.

**Rationale:** A successful exit status and parseable JSON do not establish that the selected CLI, repository, or output contract was honored.

### Decision: project through existing VCS-neutral owners

**Choice:** Validated observations map deterministically into existing Animus records: channel state identifiers feed snapshot identities, change hashes feed opaque checkpoint IDs, author/timestamp metadata feeds provenance observations as descriptive data only, and channel names feed workspace labels. Domain-separated BLAKE3 projection identities bind validated content, policy identity, and record position. Timestamps and authorship remain descriptive and never become freshness, causality, or authority evidence.

**Rationale:** Pijul's patch-set model is one backend behind VCS-neutral ports, not a replacement for Animus checkpoint, snapshot, provenance, or workspace semantics.

### Decision: default-deny sensitive content classes

**Choice:** Raw diffs, file bodies, change descriptions, author names and emails, key fingerprints, and absolute repository paths exist only in transient host data until policy admits them. Projection and receipts default to hashes, counts, status classes, and redaction markers.

**Rationale:** Diffs and change metadata routinely contain secrets, private paths, and personal data. Machine-readable does not mean safe for persistence, evidence, or model context.

### Decision: receipts prove bounded adapter facts only

**Choice:** A receipt may bind safe repository class, cohort/policy/executor identities, operation class, status, bounded change/conflict/file counts, confirmation state, redaction classes, and a domain-separated BLAKE3 receipt identity. It cannot claim repository integrity, change semantic correctness, merge safety of content, authorship authenticity, signature validity, or release eligibility. Pijul's commutation and merge-correctness guarantees remain Pijul's claims, not Animus receipts.

**Rationale:** The adapter proves only that a pinned CLI processed a bounded request and produced validated output under policy.

## Alternatives Considered

### Link libpijul directly

Deferred, not rejected on licensing grounds: Animus accepts AGPL-compatible relicensing, so a future change may add an in-process `libpijul` host adapter behind the existing backend service port. It is excluded from this change because it couples to beta-internal APIs and the Sanakirja format, and because reusable crates would still need the same effect-free port to avoid ambient database and filesystem authority.

### Open Sanakirja databases read-only from a reusable crate

Rejected because it still links the storage engine, bakes in an unstable on-disk format, bypasses the CLI's own consistency handling, and weakens the exact-cohort story.

### Support network operations in the first adapter

Rejected because clone/pull/push add credential, transport, remote-state, and partial-clone claim surfaces that are separable and deserve their own review.

### Add a Pijul-specific ledger beside the existing VCS-neutral surfaces

Rejected because checkpoint, snapshot, provenance, and workspace owners already exist; a parallel Pijul-shaped ledger would fork semantics and duplicate evidence rails.

## Risks / Trade-offs

- The `1.0.0-beta` CLI surface may shift between releases; exact pinning means deliberate cohort updates and a recorded output-format identity per update.
- A child-process boundary adds argv/stdout protocol work and loses streaming access to internal APIs; if that cost dominates, the recorded license posture permits a future linked backend instead of a workaround.
- Adopting AGPL-compatible relicensing for a future linked backend is a repo-wide licensing decision and must not happen silently inside an adapter change.
- Pijul's lack of a native staging-area model and its interactive defaults mean the host adapter must force non-interactive flags and fail closed on prompt-like output.
- Hash prefixes are user-facing conveniences; the validator must reject ambiguous prefixes rather than guess.
- Partial-clone and change-commutation features are powerful for agent fan-out, but enabling them would widen claim scope, so the initial adapter stays local and conservative.

## Non-Claims

This design does not establish Pijul CLI or format stability across releases, repository or database integrity, semantic or build correctness of recorded content, merge-safety guarantees for any specific content, authorship or signature authenticity, identity custody, network or Nest compatibility, partial-clone correctness, conflict-resolution quality, license advice beyond the recorded cohort boundary, a commitment to or against future AGPL-compatible relicensing, production readiness of a beta VCS, or release eligibility.
