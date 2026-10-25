## Context

Campaign's advisory seam has three parts:
- `brief_advisor(state, mandate)` builds an `AdvisoryBrief`: a bounded view of one frontier state under one `AdvisoryMandate`.
- An `AdvisoryProposal` names a candidate batch, guidance, revisions, retirements, one prune, and one stop reason.
- `review_advisory_proposal` either refuses the proposal or returns `AdvisoryAdmission`, which holds the ordinary control events and a receipt.

`campaign-choregraph`'s `ControlSession` publishes control events onto a Choregraph branch. It plans one publication at a time, takes `DurablePublicationEvidence` from the host, and commits.

Animus has a text-in, text-out model port, `animus_adapters::ModelAdapter`, with requests bounded by `MAX_ADAPTER_FIELD_CHARS`. Animus has no role that connects this port to the advisory seam.

## Decisions

### Decision: One optional AGPL crate outside the default members

**Choice:**
- `crates/animus-campaign-manager` is a `no_std` + `alloc` library, licensed `AGPL-3.0-or-later`.
- It is a workspace member and stays outside `default-members`.
- It depends only on `animus-adapters`, `campaign-core`, `campaign-choregraph`, `serde`, and `serde_json`.

**Rationale:**
- Campaign is AGPL. A separate crate keeps the default Animus build, its license, and its dependency footprint unchanged, as `animus-std-check-exec` already does for its sources.
- `no_std` and no ambient effects keep the manager a functional core. The host supplies the model, the store, and every effect.

### Decision: The model sees handles, labels, counters, and grants

**Choice:**
- The rendered brief names candidates `c1…cn` in brief order and offers `o1…om` in menu order.
- It shows host labels, the latest ranks, selection counts, running work, campaign counters, the remaining budgets, and which actions the mandate grants with their per-reply limits.
- It never shows a Campaign identity.
- Handles parse only in canonical form: `c01`, `c+1`, and `c0` name nothing.

**Rationale:**
- A reply can only name what the brief presented. Campaign's own scope check (`UnbriefedCandidate`) stays a second line of defence rather than the first.
- Candidate meaning stays with the host through labels and offers.

### Decision: Rank comparison decides guidance or revision

**Choice:** The mapping follows Campaign's guidance rules (Campaign spec requirements `campaign.guidance.update`, `campaign.guidance.coalesced`, and `campaign.guidance.non_improving`). A score of a presented candidate becomes:
- generation-1 guidance when the candidate has no guidance yet;
- guidance at the next generation when its ranks are lexicographically greater than the latest ranks;
- a revision with the advisor-scoped reason `advisor-rescore` when they are lower;
- nothing when they are equal.

**Rationale:**
- Campaign admits a successor only with a higher generation and strictly better ranks, coalesces equal ranks, and refuses non-improving successors.
- Lowering a score therefore needs an explicit revision, and repeating a score would be a no-op.

### Decision: The whole reply is the proposal's source facts

**Choice:**
- `source_facts_id` is a `SourceFacts` BLAKE3 identity of the complete reply text under the label `animus-campaign-manager-reply-v1`.
- Reason labels become `Reason` identities over the advisor identity and the label text, under the label `animus-campaign-manager-reason-v1`.

**Rationale:** The receipt then binds exactly the text the model produced. Two advisors that use the same reason label get distinct reason identities.

### Decision: Durability is a host port; publication stops at the first failure

**Choice:**
- `publish_events` plans each admitted event, asks the host's `DurableControlStore` to make the planned history event and branch update durable, and commits with the returned evidence.
- On a store or session failure it returns the session after the last commit, the committed count, and the cause.

**Rationale:**
- The crate performs no I/O.
- The host decides how to retry or recover from a committed prefix. Campaign's review already fixed the event order.

### Decision: Upgrade the pinned toolchain instead of lowering Campaign's floor

**Choice:**
- `flake.lock` moves `nixpkgs` from `567a49d1913ce81ac6e9582e3553dd90a955875f` to `0bb7ec54c8483066ec9d7720e780a5caa71f8612`, which carries rustc 1.96.1.
- Only the new crate declares `rust-version = "1.96"`.

**Rationale:**
- Campaign and Choregraph declare `rust-version = "1.96.0"`.
- The user chose the toolchain upgrade over lowering their MSRV.
- On the new toolchain, `cargo clippy --workspace --all-targets -- -D warnings` passes and `cargo test --workspace` matches the 1.95.0 baseline.

### Decision: Hosts supply the mandate; the crate does not load Campaign profiles

**Choice:**
- `ManagerSetup` takes a derived `AdvisoryMandate`.
- The crate does not depend on `campaign-profile`.
- Tests build full-manager terms from the default advisory mandate profile's values: window 64; batch, revision, and retirement 16; guidance and prune 64; stop 1.

**Rationale:**
- `campaign-profile` 0.1.0 requires `serde_json ^1.0.151`, while Animus's pinned `basalt`→`ucan` requires `serde_json =1.0.149`. The two cannot share one Cargo graph.
- Mandate terms are host policy in any case.

### Decision: Pin Campaign crates by version as well as revision

**Choice:** Every Campaign git dependency declares `version = "0.1.0"`.

**Rationale:**
- Campaign's repository also contains infra-checker fixture manifests named `campaign-core`, `campaign-choregraph`, and `campaign-profile` without a version, which Cargo reads as `0.0.0`.
- Without a version requirement, Cargo resolved a fixture rather than unlock `serde`.

## Risks / Trade-offs

- A model may ignore the schema. Every deviation is a typed refusal and publishes nothing; the host decides whether to retry.
- Advice quality is not checked. Campaign's review bounds what advice can do, not whether it is wise.
- The brief window bounds what the model sees. Candidates outside the window are counted, not listed, and cannot be named.
- `animus-std-agent-guardrail`'s `fake_bwrap` test fails 3 cases on both rustc 1.95.0 and 1.96.1. This change does not touch it.
