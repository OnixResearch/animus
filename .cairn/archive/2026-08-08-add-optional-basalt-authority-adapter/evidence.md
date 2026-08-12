# Evidence: Optional Basalt authority adapter

## Scope

This evidence covers the non-default `animus-std-basalt-authority` bridge and the implementation-neutral `HostAuthorityService` contracts.

The change started from Animus commit `917f2f6b08bd361775efe68dfa15d0797b40e23b`.

The reviewed authority cohort is:

- Basalt revision: `e9891e32dcdf655c6e12d0bea2a420f7cdca31c7`
- vendored UCAN revision: `34449ba1f1a9ef991225e4e6683b5d6aca8c07cf`
- Basalt policy identity: `b3:5475d5473ec98cbf052a83512836b0739f5dad3e7b9b0ed201eb3094b1cf39bf`
- UCAN catalog identity: `b3:3266e7b1f2ce66f2ab5c8cbd7028f8bcf3502c7e64fe8d058c7b7e1f762dedb6`
- Basalt license boundary: `AGPL-3.0-or-later`

## Baseline and ownership

The existing agent-definition, role, UCAN-aligned authority, coding-session, runtime-authority-seal, feature-isolation, no-default, Wasm, component, facade, and safe-receipt checks passed before reusable logic changed.

`animus-adapters` owns generic requests, agent bindings, decisions, issue classes, gate intersection, and receipt projection. It remains `no_std` plus `alloc` and exposes no Basalt or UCAN type.

The optional standard-library bridge owns exact translation and request-scoped Basalt invocation. The caller owns key, proof, revocation, caveat, replay, time, limit, commit, and effect services.

Agent display names are informational. Only the admitted agent-definition BLAKE3 identity and audience DID binding can enter authority checks.

The closed mapping supports document read and document write filesystem actions. Model, tool, process, network, Steel, and semantic-role influence paths remain unsupported by this adapter and keep their existing owners.

## Implemented evidence

The generic contract tests cover valid provider replacement, deterministic request and receipt construction, binding drift, stale or duplicate binding reuse, name-only authority, wildcard or malformed inputs, local denial, inconsistent decisions, and receipt leak rejection.

The live adapter tests cover:

- direct and delegated holder-signed invocation;
- malformed tokens and holder-signature binding failure;
- audience and definition substitution;
- missing proof, revoked token, denied caveat, and stale time;
- replay denial after the final consuming admission step;
- policy, resource, ability, operation-profile, and request-digest mismatch;
- unavailable key resolution;
- unsupported action mapping;
- Basalt allow intersected with every Animus gate;
- durable safe-decision commit before effect dispatch;
- commit failure and every tested denial causing zero guarded effects.

Receipts bind safe BLAKE3 identities and bounded status facts. Receipt validation rejects token, key, signature, proof, caveat, revocation, replay, credential, prompt, tool, provider, path, environment, and output material.

## Deterministic checks

The focused rail is `scripts/cairn/check-basalt-authority-adapter --root .`. Its local receipt is `target/cairn/focused-rails/basalt-authority-adapter.md`.

The following checks passed:

- `cargo test -p animus-adapters host_authority --locked`
- `cargo test -p animus-std-basalt-authority --locked`
- `cargo clippy -p animus-std-basalt-authority --all-targets -- -D warnings`
- `cargo fmt --all -- --check`
- `cargo check --workspace --all-targets --locked`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace --locked`
- default, no-default, root-feature, generic-crate, and `wasm32-unknown-unknown` isolation checks
- `basalt-reference` alone and with `basalt-authority-adapter`
- Nickel export, generated projection freshness, and positive and negative fixtures
- Clash UCAN authority, coding-session host, runtime-authority-seal, component catalog, runtime facade, and evidence-safety rails
- `nix build .#checks.x86_64-linux.basalt-authority-adapter --no-link -L`
- `nix build .#checks.x86_64-linux.cargo-basalt-reference --no-link -L`
- `nix build .#checks.x86_64-linux.cargo-trellis-reference --no-link -L`
- `nix flake check --no-build -L`

The Nix Cargo vendor tree is partitioned by source. This preserves the distinct reviewed Basalt-vendored and Trellis `verified-logic` package identities without a same-name vendor collision. Both optional reference builds passed from their separate source partitions.

The configured remote Nix builder was unavailable at `ssh-ng://root@10.10.10.1`. Nix reported the connection failure and completed the checks locally.

`cargo tigerstyle check` remains blocked by 43 existing findings in `crates/animus-adapters/src/messaging.rs`. The focused crate Clippy, workspace Clippy, tests, no-default checks, Wasm checks, and Nix checks are bounded substitute evidence. They do not claim a passing Tiger Style rail.

Repository-wide Tracey coverage was run with `nix run .#cairn -- tracey coverage --root . --json`. It reported 841 referenced requirements out of 2,935 accepted requirements and failed on the existing accepted-spec coverage backlog. Cairn structural validation and the change-specific proposal, design, task, focused, and negative-fixture rails passed. This evidence does not claim repository-wide Tracey closure.

## Lifecycle evidence before final archive

- Proposal gate receipt: `cf0c39d89ba7f51ec917976896c1ac503820b430bb5be2165dd84f3966556520`
- Design gate receipt: `3749c90662098f477e65a65fc7bd0238a53f6b1170beace3a60851aba62c90ec`
- Preliminary tasks gate receipt: `084f9d22f248f33bb5617fcc73ced1605a50feb9932ae9d23a5e8b0a5387ddd9`, with 36 of 38 tasks complete
- Sync dry-run receipt: `f731102cc380eee63059b58192f1e66d2b12c0be175865b3bbc22e37e7c02d3d`
- Sync plan hash: `61e755df73982f272a98ab3eb3b98e4ae47d60575f29206a562332a27f433d47`
- The sync plan adds all 12 Basalt adapter requirements and changes the accepted specification identity from `60ff89ae3f2664d2e2adef86b8752d94be707bbdc30b74e10e9df8e1bba8b3c1` to `8ab1a5b55bded4edfe1ff04bda52ea7117c90ce25e45589f333465168fa3f7af`.
- Initial archive dry-run receipt: `68dc2e1eb40029c80da83a0b1b2c106c8f8c05baf40e01cffc0c61bef96398e6`
- The initial archive review correctly blocked while the final two closeout tasks remained open.

Final pre-mutation lifecycle results:

- Final proposal gate receipt: `4ab3070adbe502d34aadd4e432ac205923408e3fde46136d8ecabbc77b792256`
- Final design gate receipt: `10f645ea7d45983ea41a64ca92f1f8dc158655d75b9102d3110ea6f9af7ec4df`
- Final tasks gate receipt: `e6857e84d1a8dce0a325cb27547228d474b0985c0f10c808197c2bf12e40bd40`, with 38 of 38 tasks complete
- Final sync dry-run receipt: `2b8e00c7df868c1c03d45379d9c10771f4c2b26b90770023729c6f3f3a926d59`
- Final sync plan hash: `553615dd6192ecc9afdc748e26a154a95fe9974468b10f61094ce38c7adb6996`
- Final archive dry-run receipt: `f6dfb1e6e230f6f263a3b7b57fa32597583007ea5dc211b091c41f4588fab0dc`
- Final archive plan hash: `657a43e4ed828617549737c11a5d516af6685dd1500c8408e95a16fa5434ba1c`

Accepted-spec sync results:

- Sync execution receipt: `c9d2d8eb29565ed4b680c57ed8758968fb774627e681adc150ab4805deba522d`
- Sync execution plan hash: `1b39b76b7d43e85ae4f7c344f2ecf248cf6919a5decb287c8ee084d31cd6cbf6`
- Sync mutation manifest: `6b81fe426c0e610e76c09fb1cb49f179c01a51ecde53c2a687ca4ea3d7ec37f7`
- The accepted specification contains all 12 new Basalt adapter requirement IDs.
- The accepted specification identity is `8ab1a5b55bded4edfe1ff04bda52ea7117c90ce25e45589f333465168fa3f7af`.

Archive results:

- Archive execution receipt: `08ed8150b08574b7538d6162bf6960e39217f56b81651901d33bb388eddbe1f1`
- Archive execution plan hash: `3f3c894d2a68a578daf5de54bfe1903866b369f505190d39fc52f6dcf546aeb6`
- Archive mutation manifest: `96923a4fc5c675519125622f0be0e256bf48adabcfeb96c857e3a48b0cfce1cd`
- Archive path: `cairn/archive/2026-08-08-add-optional-basalt-authority-adapter/`
- Archived evidence identity at mutation: `49e5567bfc9a0a278c0d2664394529723e2e8bae28c275942e13553e4d465085`

## Non-claims

This evidence does not establish global trust, key truth, global revocation freshness, replay durability, transport security, sandbox confinement, effect correctness, agent safety, semantic-role correctness, completion, production readiness, release eligibility, future Basalt compatibility, or license compatibility. Basalt is an additive deny-capable gate only. It grants no compatibility exception and cannot widen another Animus gate.
