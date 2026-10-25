# Campaign manager

`crates/animus-campaign-manager` lets a model steer a [Campaign](https://github.com/OnixResearch/campaign) exploration campaign through Campaign's advisory seam. The model proposes, and Campaign decides. Each turn renders one advisory brief as model text, makes one model call, turns the reply into an `AdvisoryProposal`, and hands it to Campaign's `review_advisory_proposal`. Only the events Campaign admits are published, through `campaign_choregraph::ControlSession`.

The crate is the separate non-default `AGPL-3.0-or-later` member of the workspace because it links Campaign. It is `no_std` + `alloc` and has no effects of its own.

## Authority boundary

- **Campaign** owns briefs, mandates, grants, review, admission, control events, and receipts. A refused proposal changes nothing.
- **Choregraph**, through `ControlSession`, owns publication planning and commit validation.
- **The model** sees only what the brief text shows and can only propose.
- **The host** keeps the following:
  - Candidate meaning: labels and offered candidate records.
  - The mandate and its terms.
  - The model adapter and the durable store.
  - Scheduling, execution, retries, and recovery.

The rendered brief never contains a Campaign identity. Candidates appear as `c1…cn` in brief order and host offers as `o1…om` in menu order. A reply can therefore name only what it was shown. Handles must be canonical: `c01`, `c+1`, and `c0` name nothing.

## Host sequence

```rust
let setup = animus_campaign_manager::ManagerSetup {
    mandate,               // a derived campaign_core::AdvisoryMandate
    objective,             // at most MAX_OBJECTIVE_CHARS, one line
    rank_dimensions: 2,    // used until the campaign's rank shape is fixed
    admission_rule,        // for any candidates the model adds
    coalescing_policy_id,
};
let menu = animus_campaign_manager::CandidateMenu { labels: &labels, offers: &offers };
let turn = animus_campaign_manager::advise(&mut model, session.state(), &setup, menu)?;
let session = animus_campaign_manager::publish_events(session, &turn.admission.events, &mut store)?;
```

`advise` takes these steps in order:
1. Refuse a stopped campaign.
2. Brief the campaign under the mandate.
3. Render the brief, refusing invalid setup text, more than `MAX_OFFERS` offers, or an impossible rank dimension count.
4. Call the model exactly once.
5. Build the proposal.
6. Submit it to Campaign's review.

A stopped campaign or an invalid setup never reaches the model.

`publish_events` handles each admitted event in order:
1. Plan the publication.
2. Call the host's `DurableControlStore::make_durable` with the planned history event and branch update.
3. Commit with the returned `DurablePublicationEvidence`.

On a store or session failure it returns `PublicationFailure { session, committed, cause }`. The `session` field is the session after the last commit.

## Reply schema

The model answers with one JSON object. Every field is optional:

```json
{"scores":[{"candidate":"c1","ranks":[0]}],"add":[{"offer":"o1","ranks":[0]}],"retire":[{"candidate":"c2","reason":"label"}],"prune":{"candidates":["c3"],"reason":"label"},"stop":{"reason":"label"}}
```

The manager reads the span from the first `{` to the last `}`, so prose or a code fence around the object is ignored. Unknown fields, wrong types, and non-integer ranks give `MalformedReply`.

The mapping follows Campaign's guidance rules:
- A score of an unscored candidate becomes generation-1 guidance.
- A lexicographically higher score becomes next-generation guidance.
- A lower score becomes a revision with the advisor-scoped reason `advisor-rescore`.
- An equal score adds nothing.
- An added offer joins the candidate batch, with first guidance if the reply scores it.
- Reasons are one-line labels of at most `MAX_REASON_CHARS` characters. They become `Reason` identities scoped to the advisor.
- The proposal's source facts are the BLAKE3 identity of the whole reply.

## Refusals

`TurnFailure::Model` carries the model adapter's error. `TurnFailure::Refused` carries one of:
- `CampaignStopped`, or any other Campaign review refusal such as `UngrantedAdvice` or `AdviceLimitExceeded`, inside `ManagerError::Campaign`;
- `InvalidText { field }`, `TooManyOffers`, or `RankDimensions` for the setup and menu;
- `MalformedReply`, `UnknownHandle`, `RankLength`, or `RankMagnitude` for the reply;
- `Prompt` when the rendered brief exceeds the model request contract.

No refusal publishes anything.

## Pins and toolchain

- `campaign-core` and `campaign-choregraph` are pinned to version `0.1.0` at revision `796fe3844c91c3060da5ffc573a6f801a214aba7` of `https://seed.radicle.garden/z2scC9MCm3pxk9mX4FEidRKabQ5LN.git` (`rad://z2scC9MCm3pxk9mX4FEidRKabQ5LN`).
- `choregraph-history` comes transitively at `b3e08e19750f53bdbcae970cdf58a47a791ed20b` (`rad://zL2ncTUeASVYwcoGkEXv9JKgGbAF`).
- The version requirement matters. Campaign's repository also holds unversioned infra-checker fixture manifests with the same package names, and Cargo could otherwise pick one of them.
- Campaign and Choregraph require rustc 1.96. The flake's nixpkgs pin carries rustc 1.96.1, and the crate declares `rust-version = "1.96"`.
- Hosts build the mandate themselves. `campaign-profile` cannot join the Animus graph, because it requires `serde_json ^1.0.151` while the pinned `basalt`→`ucan` cohort requires `serde_json =1.0.149`.

## Non-claims

- The manager does not judge advice. Campaign's review bounds what advice can do, not whether it is wise.
- It does not prove that a model follows the schema, that a campaign finds anything, or that a store is durable. The evidence a store returns is the host's claim.
- It adds no Campaign authority to any default Animus surface.

## Verification

```sh
nix build .#checks.x86_64-linux.cargo-campaign-manager   # integration tests and Clippy -D warnings
nix build .#checks.x86_64-linux.campaign-manager-octet   # Octet deny-all over lib and tests
```

The integration tests open real Campaign campaigns on Choregraph history and drive turns with a scripted model and an in-memory store. `tests/turns.rs` covers admitted turns, and `tests/refusals.rs` covers refusals and publication failure.

To measure how well a manager steers campaigns, as opposed to whether its mechanics hold, use the [evaluation lane](campaign-manager-eval.md).
