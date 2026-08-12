# Evidence

## Baseline

- Proposal gate: PASS. Receipt `9192658eaee3b1b6fd0eec1525b6421e9b4710a367fa97ac60e06c77f4eed38c`.
- Design gate: PASS. Receipt `ea130d883ba5c89328d065ea3eb8c9e976491d2fae846ed059a7ccf830f04e9c`.
- Tasks gate before implementation: PASS. Receipt `f6e7ecca5969e249728d1a396807122f0863c3bb601c2f24f4afa51caa9361b2`.
- `cargo test -p animus-adapters multimodal --locked`: PASS, 10 tests before edits.
- `cargo check -p animus-adapters --no-default-features --locked`: PASS before edits.
- `cargo check -p animus-adapters --target wasm32-unknown-unknown --locked`: PASS before edits.
- `cairn validate --root .`: PASS before edits with 13 active changes and 17 validated specs.

## Implementation evidence

- `animus-adapters` contains a distinct video part, bounded declared metadata, source trust, BLAKE3 identity, ordered outcomes, and named local and aggregate bounds.
- Video capability is separate from image capability. Required video fails closed. Optional omission needs an explicit policy and produces a visible dispatch result.
- `VideoPayloadService` is injected. Reusable code does not load media, resolve paths or URLs, inspect containers, use codecs, upload files, read credentials, or call providers.
- Provider projection uses one exact credential-free `google-gemini-v1beta-files-fixture` profile. Generic JSON conversion and unrepresentable fields fail closed.
- Video receipts contain bounded metadata, exact profiles, loss classes, non-claims, content identity, and deterministic BLAKE3 receipt identity. Leak fixtures cover bytes, base64, data URLs, remote URLs, private paths, credentials, provider payloads, frame bodies, audio bodies, and unbounded metadata.
- The adapter crate gained no provider client, codec, media framework, file loader, URL client, async runtime, or terminal dependency.

## Focused verification

- `scripts/cairn/check-provider-video-payloads --root .`: PASS. This includes static positive and negative tests, 19 multimodal Rust tests, Nickel export drift, three Nickel fixtures, Clippy with warnings denied, no-default compilation, and Wasm compilation.
- `scripts/cairn/check-component-catalog --root .`: PASS. Catalog BLAKE3 `f9db4b6a356b3d587771bcd5cf31156bc51b451a53c673d8173c6456e097f605`.
- `scripts/cairn/check-runtime-facade-inventory --root .`: PASS. Inventory BLAKE3 `3e17a7622de6b2f8a63a9fa67a2c50d148c1efd0c1647d71b6c3403262240ade`.
- `scripts/config/check-nickel-configs`: PASS after the generated video policy was added to the Nix source view. Nix remote SSH failed, then the derivation built locally.
- `scripts/cairn/check-adapter-conformance-index --root .`: PASS. Family BLAKE3 `bd7f47f343a8fc22db75e57b8230f6e46504a1d7140521bc26e7447ea86872ba`.
- `scripts/cairn/check-safe-receipts-artifacts --root .`: PASS.
- `cargo test -p animus-adapters --locked`: PASS with 618 tests before two additional negative video service tests were split out. The final focused rail passed all 19 multimodal tests after that split.
- `cargo test -p animus-adapters provider_conformance --locked`: PASS.
- `cargo check -p animus --locked`: PASS.
- `scripts/cairn/check-composition-conformance --root .`: PASS with six positive and negative tests.
- `scripts/cairn/check-wasm-surface --root .`: PASS for core, adapters, router, and root facade boundaries.
- `scripts/cairn/check-evidence-safety --root .`: PASS with all five negative fixtures.
- `scripts/cairn/check-roadmap --root .`: BLOCKED only by the pre-existing unclassified active changes `add-optional-basalt-authority-adapter` and `add-pijul-vcs-adapter`. It classified this change in batch 0 and reported no missing product safety path.
- `nix run .#cargo-tigerstyle -- check`: BLOCKED by 43 pre-existing findings in `crates/animus-adapters/src/messaging.rs`. The final report contains no finding in the new or changed multimodal files.
- Final proposal, design, and tasks gates: PASS. Receipts: `b96b7489b760442caa4c9f7f4a62af09671127845d37c197b82e8bb80f982983`, `4ba48d46c978950c8cd0a931e45276b9a6024dc95fc4db199a863ac17fc709bd`, and `b7ab9f63dff9626f0ca56053d1cf7eab57cf044b57804f1eba094a9edfd4ed5d`.
- `nix build .#checks.x86_64-linux.provider-video-payloads --no-link -L`: PASS. The remote builder failed to connect, then Nix built the complete focused rail locally.
- Accepted-spec sync dry-run and execution: PASS. Dry-run receipt `155a79c02c07beff6d99f01460509436fa871d5a6ecd55138770d358612cc1b9`; execute receipt `2efb820e44097641c60867dab7e769a099f3016abc5d723cf53588d51f788999`. Seven requirements were added to `cairn/specs/agent-harness-runtime/spec.md`.
- Archive dry-run and execution: PASS. Dry-run receipt `8eee16e54619b6447c2d3eb4dfe150eef9832ec198aa1004c255e4dc7b7c56d4`; execute receipt `e6f37e4c40b5f3d5faa64c9f2cc938b8a0171f991d7227b32fbbe88c62db91ce`.

## Explicit non-claims

No live provider, upload, file loading, URL resolution, media inspection, decoding, transcoding, playback, audio extraction, production, performance, or release check ran. This evidence does not prove media authenticity, codec safety, successful decoding, upload completion, provider acceptance, semantic understanding, playback, audio correctness, privacy, model correctness, production readiness, or release eligibility.
