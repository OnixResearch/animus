# Evidence

## Selected cohort and review

- Canonical remote: `git@github.com:brittonr/rats.git`, default branch `main`.
- Prior Animus revision: `e9cb7865fe400de6220aea8bfc08ddf4f75da3d3`.
- Implementation-baseline head observed on 2026-08-08: `3b498c3d7c248c335d3e7faf12f133a18c87d632`.
- Cargo manifests, Cargo lock state, `flake.nix`, and Nix-generated `flake.lock` bind the selected immutable revision.
- The reviewed range adds the combined Rats workspace, `rat-image`, typed finite image limits, explicit protocol selection, caller-owned writer capability, bounded preparation, partial-write outcomes, and unrelated UI packages. Animus selects only optional TUI packages.
- `rat-image` uses the Rats MIT workspace license. Direct content dependencies are `base64 0.22.1` and `image 0.25.10`, both MIT OR Apache-2.0. The selected `image` release declares Rust 1.88.0, so the optional terminal-image feature requires Rust 1.88 or newer.
- The selected API already supplies explicit bytes, protocol, bounds, typed limits, typed failures, preparation, and writer inputs. It does not open `/dev/tty`, fall back to stdout, or inspect process environment. No upstream patch or local protocol copy was required.

## Baseline

Before dependency changes:

- Proposal gate receipt `38f99c696fa191ead1316ce2f5dc82f52f818acdf28216de61e8dd0d23046970`: PASS.
- Design gate receipt `f2527f2b9f82e16b0301416d467fddc3e5dcfab2929da6a4574d84ffa38b624f`: PASS.
- Tasks gate receipt `fb063fd6147200c39e121b7353dab3ebebf5d3f2cade7e4743870a153361b358`: PASS.
- `cargo test -p animus-tui --locked`: PASS, 154 tests.
- `cargo test -p animus-adapters multimodal --locked`: PASS, 19 focused tests.
- `cargo check -p animus --no-default-features --locked`: PASS.
- `cargo check -p animus --no-default-features --target wasm32-unknown-unknown --locked`: PASS.
- `nix run path:$PWD#check-nickel-configs`: PASS after the unavailable SSH builder fell back to a local build.
- `scripts/cairn/check-ratatui-tool-host-surface --root .`: PASS.
- `scripts/cairn/check-wasm-surface --root .`: PASS with positive and negative dependency fixtures.
- `nix run path:$PWD#cairn -- validate --root .`: PASS.

## Implemented boundary

- Added a non-default `terminal-image` feature under `animus-tui` and the root facade.
- Added pure planning over safe handles, BLAKE3 identities, source metadata, operation-level capabilities, terminal geometry, lifecycle intent, and named policy limits.
- Added bounded Kitty capability-response parsing without retaining terminal responses.
- Added exact Kitty query/transmit/place, replacement, deletion, and cleanup plans; exact iTerm2 and Sixel transmit rows; and visible policy-controlled text fallback.
- Added an injected byte resolver, injected lifecycle service, and explicit writer shell. Only transmit invokes upstream `rat-image` preparation and write APIs.
- Added count, identity, source, decoded, prepared, cell, chunk, operation, lifecycle, and alternative-text bounds.
- Added typed failure and cleanup outcomes. Partial transmission cannot become success, and failed cleanup remains `cleanup-incomplete`.
- Added deterministic metadata-only BLAKE3 receipts and leak rejection.
- Added typed Nickel policy, generated JSON, positive and negative fixtures, component and facade inventory rows, docs, a focused checker, and a Nix check.

## Operation-level conformance

- Kitty: bounded capability response parsing; host-supplied query, placement, replacement, deletion, and cleanup; upstream Rats transmit-and-display payload preparation.
- iTerm2: upstream Rats inline transmit only. Independent placement, replacement, deletion, and animation remain unsupported.
- Sixel: upstream Rats bounded preparation and transmit only. Independent lifecycle operations and animation remain unsupported.
- Text: bounded visible alternative text only when policy permits.

A protocol name or successful transmission never implies complete protocol support.

## Final verification

- `scripts/cairn/check-terminal-image-protocols --root .`: PASS after implementation, including 168 `animus-tui` tests, bounded Kitty multi-chunk transmission, strict focused Clippy, root feature compile, default dependency isolation, no-default checks, Wasm isolation, Nickel parity, and positive/negative static checker tests.
- `cargo check -p animus-tui --features rats-widgets,terminal-image --locked`: PASS. The latest keymap, widget, and image packages compile together in the host checkout.
- `cargo check --workspace --all-targets --locked`: PASS after the dependency update.
- `nix run .#cargo-tigerstyle -- check`: the broad rail remains limited to the 43 pre-existing findings in `crates/animus-adapters/src/messaging.rs`; it reported no finding in the changed terminal-image files.
- Final proposal gate receipt `259c896c0fe307adf3a8923400ee8660177852028f6b6f3a2d764760f7a95d3f`: PASS.
- Final design gate receipt `5b1c121c39e817040a650dad89ca29fc0529656407d0ac0f4dd5fb8ec37fc2ef`: PASS.
- Final tasks gate receipt `80a1c6cae7c4d609951d6b3389a723807e49dd9afae55c047149b670865ec439`: PASS with all 15 tasks complete.
- `scripts/cairn/check-runtime-facade-inventory --root .`: PASS with exact classification for every new public export and all positive/negative fixtures.
- `scripts/cairn/check-component-catalog --root .`: PASS after adding the terminal-image feature vocabulary and planner/shell rows.
- `nix run path:$PWD#check-nickel-configs`: PASS.
- `nix build .#checks.x86_64-linux.terminal-image-protocols --no-link -L`: PASS after the unavailable SSH builder fell back to a local build. An earlier over-broad version also compiled unrelated `ratcore`; Nix Cargo vendoring omitted that package's workspace-external theme files. The focused derivation now checks only the terminal-image feature, while the separate host command above checks combined widget compatibility.

## Lifecycle receipts

- Sync dry-run receipt `62a8cbbe41ba1b25d07b7cc18bcd707f761fb1b0c953219bbcc4fba8d422a9d5`: PASS with one unblocked delta-spec action.
- Sync execute receipt `588638158d68b5b816dad824012bbef8ae4e4eb016256bb7537d2b31f7b63ac4`: PASS. The accepted specification changed from `8f39de29bdd375e349aec3b924eddc30f89e83add4f1aa47c7aec16e90a0a768` to `5222c7dfd8cb3fad3a7c0ba7d24490bddf9bd2fac43db5bb1c6c4114ac3e7441` and gained all seven terminal-image requirements.
- Archive dry-run receipt `764377553be45c36cb173a5a123ec1059c026b666fdf8fd12aab889f21f01331`: PASS and unblocked.
- Archive execute receipt `01f26eeac63b94e6eef9f4cd70aa86c4138d449b5438e5b0398a3880fb7bddf8`: PASS. The change moved to `cairn/archive/2026-08-08-adopt-latest-rats-image-protocol-adapters/`.
- Post-archive focused and evidence-safety rails: PASS. The roadmap recognizes `add-bounded-terminal-video-playback` as unblocked; its broad verdict remains blocked only by the pre-existing unclassified active changes `add-optional-basalt-authority-adapter` and `add-pijul-vcs-adapter`.

## Non-claims

This evidence does not establish live-terminal behavior, terminal-emulator correctness, visual fidelity, image-decoder correctness, hostile-image safety, process isolation, remote-session transparency, provider availability, video playback, audio playback, production readiness, or release eligibility.
