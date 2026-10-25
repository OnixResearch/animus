## Why

Campaign now has an advisory seam (`campaign-core` at `796fe3844c91c3060da5ffc573a6f801a214aba7`, spec `advisory-seam`). An outside advisor reads a bounded brief, proposes candidate batches, guidance, revisions, retirements, a prune, or a stop under a declared mandate, and Campaign reviews the proposal into ordinary control events. Animus has model adapters but no role that uses this seam. A host that wants a model to steer an exploration campaign would have to write the brief rendering, the reply parsing, the mapping to `AdvisoryProposal`, and the publication loop itself.

Campaign stays the authority. The model's reply is only a proposal: Campaign's review admits it or refuses it, and only admitted events reach the control branch.

## What Changes

- Add the optional crate `animus-campaign-manager`. It holds pure manager logic over explicit values. The model is reached through `animus_adapters::ModelAdapter` and durability through a host-supplied `DurableControlStore`. It is licensed AGPL-3.0-or-later because it links Campaign, and it stays outside the workspace's default members. r[agent_harness_runtime.campaign_manager.boundary]
- Render Campaign's advisory brief as bounded model text. The text shows the objective, the reply schema, the rank domain, the granted and refused actions, campaign counters, the listed candidates with host labels, and the host's candidate offers. Every candidate and offer is a per-brief handle (`c<n>`, `o<n>`); the text never shows a Campaign identity. r[agent_harness_runtime.campaign_manager.brief]
- Parse exactly one JSON object in the declared reply schema and refuse anything else. r[agent_harness_runtime.campaign_manager.reply]
- Map a parsed reply onto one `AdvisoryProposal`:
  - First and raised scores become guidance, lowered scores become revisions, and repeated scores are dropped.
  - Added offers become a candidate batch with optional first guidance.
  - Retirements, a prune, and a stop carry reason identities scoped to the advisor.
  - The proposal's source facts are the BLAKE3 identity of the whole reply.

  r[agent_harness_runtime.campaign_manager.proposal]
- Run one manager turn: refuse a stopped campaign, brief, render, call the model exactly once, build the proposal, and submit it to Campaign's review. r[agent_harness_runtime.campaign_manager.turn]
- Publish admitted events in order through `ControlSession`: plan each event, make it durable through the host store, then commit it. A failure reports the committed prefix. r[agent_harness_runtime.campaign_manager.publication]
- Refuse every malformed, out-of-brief, out-of-domain, or ungranted reply, and every invalid setup, without publishing anything. Refuse an invalid setup or a stopped campaign before the model is called. r[agent_harness_runtime.campaign_manager.refusals]
- Add positive and negative integration tests over real Choregraph history with a scripted model, and the focused Nix checks `cargo-campaign-manager` and `campaign-manager-octet`. r[agent_harness_runtime.campaign_manager.verification]

## Impact

- **Rust**:
  - New workspace member `crates/animus-campaign-manager`. It is not in the default members.
  - It depends on `campaign-core` and `campaign-choregraph`, pinned by version `0.1.0` and git revision `796fe3844c91c3060da5ffc573a6f801a214aba7` from `https://seed.radicle.garden/z2scC9MCm3pxk9mX4FEidRKabQ5LN.git`, and transitively on `choregraph-history` at `b3e08e19750f53bdbcae970cdf58a47a791ed20b`.
- **Toolchain**: Campaign and Choregraph declare `rust-version = "1.96.0"`. `flake.lock` moves the dev-shell and check nixpkgs from `567a49d1913ce81ac6e9582e3553dd90a955875f` (rustc 1.95.0) to `0bb7ec54c8483066ec9d7720e780a5caa71f8612`, the first nixos-unstable revision with rustc 1.96.1. The workspace `rust-version` stays `1.85`; only the new crate declares `1.96`.
- **Nix**:
  - `cargoDependencyGroups` gains `campaign` and `choregraph` vendoring groups.
  - The new checks are `cargo-campaign-manager` (tests and Clippy) and `campaign-manager-octet` (Octet deny-all).
- **Authority**: The model proposes and Campaign reviews. The host keeps candidate meaning, execution, scheduling, persistence, and all effects.
- **Non-claims**:
  - The manager does not prove that advice is good, that a campaign finds anything, or that a model follows the schema.
  - It does not make a durable store durable.
  - It does not add Campaign authority to any Animus default surface.
