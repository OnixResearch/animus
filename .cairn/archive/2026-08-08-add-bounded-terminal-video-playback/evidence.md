# Evidence

## Baseline and prerequisite

- Base revision: `d7164b6956e83f1d270774fc084c20d141ca0973` from current `origin/main`.
- The prerequisite is archived at `cairn/archive/2026-08-08-adopt-latest-rats-image-protocol-adapters/`.
- Its sync receipt is `588638158d68b5b816dad824012bbef8ae4e4eb016256bb7537d2b31f7b63ac4`; its archive receipt is `01f26eeac63b94e6eef9f4cd70aa86c4138d449b5438e5b0398a3880fb7bddf8`.
- The prerequisite pins Rats revision `3b498c3d7c248c335d3e7faf12f133a18c87d632` and records exact static Kitty, iTerm2, Sixel, text, writer, partial-write, cleanup, dependency-isolation, Nickel, no-default, Wasm, and Nix evidence.
- Proposal, design, tasks, and initial Cairn validation gates passed before playback implementation.

## Selected backend inventory

- The pinned `rat-image` Kitty implementation prepares bounded static `a=T` transfers through an explicit writer.
- The pinned boundary rejects animated GIF and WebP inputs. It provides no Kitty animation-control encoder.
- Kitty animation therefore requires four independent facts: static frame transfer through Rats, host-admitted animation control, host-admitted placement, and host-admitted deletion.
- Kitty static replacement requires frame transfer and an exact host lifecycle row. iTerm2 and Sixel remain static transmit-only and do not gain an independent replacement or animation claim.
- Thumbnail and text are separate policy-controlled fallbacks. Static image success never implies animation support.

## Implementation

- Added non-default `terminal-video` features to `animus-tui` and the root facade. The feature includes `terminal-image` and adds no dependency.
- Added a pure reducer over safe descriptors, operation-specific capability facts, named policy, explicit monotonic ticks, bounded frame metadata, lifecycle commands, and prior state.
- Added exact start, request, display, wait, pause, resume, drop, source-finish, cancellation, source-failure, terminal-loss, layout-eviction, shutdown, cleanup, and cleanup-failure transitions.
- Added backpressure before retention, exact frame order and schedule checks, bounded drops, action bounds, and exact cleanup obligations.
- Added an injected decoded-frame source with bounded requests, typed failures, byte-count recheck, and BLAKE3 identity recheck.
- Added a thin frame shell that consumes one transient frame, calls the terminal-image Rats writer, and then calls one exact injected playback backend operation.
- Added metadata-only deterministic receipts and leak rejection for media content, encoded payloads, terminal content, paths, URLs, decoder output, credentials, timing traces, and smoothness overclaims.
- Added typed Nickel policy, generated JSON, positive and negative fixtures, component and facade inventory rows, documentation, a focused rail, and a Nix check.

## Focused verification

- `cargo test -p animus-tui --features terminal-video --locked`: PASS with 184 tests, including 16 terminal-video positive and negative tests.
- `cargo clippy -p animus-tui --all-targets --features terminal-video --locked -- -D warnings`: PASS.
- `scripts/cairn/check-terminal-video-playback --root .`: PASS, including static checker fixtures, Nickel parity, scheduler and shell tests, strict Clippy, root feature compilation, default dependency isolation, no-default checks, and adapter/runtime/Steel Wasm isolation.
- `scripts/cairn/check-component-catalog --root .`: PASS; component catalog BLAKE3 was `c0fbeb55a5221ae8cc3bda2a4fcf6a5b15f8369bb64bb847dc5a4b239f849fb0`.
- `scripts/cairn/check-runtime-facade-inventory --root .`: PASS; runtime facade inventory BLAKE3 was `a31b0c96e728161890567d776dd0a2dfb0cc132a767d7adac15dd3f8e3127609`.
- `cargo check --workspace --all-targets --locked`: PASS.
- `nix run path:$PWD#check-nickel-configs`: PASS after local fallback from the unavailable SSH builder.
- Ratatui host, safe-receipt, evidence-safety, request-lifecycle, Wasm-surface, Cairn validation, proposal, and design rails: PASS.
- `nix run .#cargo-tigerstyle -- check`: the broad rail remains limited to the 43 pre-existing findings in `crates/animus-adapters/src/messaging.rs`; it reported no finding in the changed terminal-video files.
- `scripts/cairn/check-terminal-image-protocols --root .`: PASS after playback implementation with 168 static-image tests.
- `scripts/cairn/check-provider-video-payloads --root .`: PASS after playback implementation with 19 multimodal tests.
- Final proposal gate receipt `aea1bb973e9d316c9ae0a1904f697c7993bb9d84876db493c710b0742d96d9f5`: PASS.
- Final design gate receipt `3ce8c2a55750dbe2c5a2b6e70e9850ed5340b4e62cda3dad5773abe4bb833ba7`: PASS.
- Final tasks gate receipt `856076631a117471959934e3a5af78af70efb34102400d7ad81818526f6f300e`: PASS with all 15 tasks complete.
- `nix build .#checks.x86_64-linux.terminal-video-playback --no-link -L`: PASS with 184 tests after local fallback from the unavailable SSH builder.

## Lifecycle receipts

- Sync dry-run receipt `98b2a29b7f18fb569a0512dff817e869e9681737e1a197c47cceaca7c9dc4591`: PASS with one unblocked delta-spec action.
- Sync execute receipt `209afb968f0cecac52ecd627e3e97d92a48c38cb748a49a5899a0ce22372c7db`: PASS. The accepted specification changed from `5222c7dfd8cb3fad3a7c0ba7d24490bddf9bd2fac43db5bb1c6c4114ac3e7441` to `c393adc70177e727343f5b14bea27aefb0898fb8ad87cb4de719838f500c16eb` and gained all six playback requirements.
- Archive dry-run receipt `0ca471564e009e5cb6a8e2b7835dccecbe03ab90b06193a0dbbb3e6c06c4febc`: PASS and unblocked.
- Archive execute receipt `45316e1403f6d34f6d68b0264ce146060a299e9d206cb5b7c3ca24d7f8db1111`: PASS. The change moved to `cairn/archive/2026-08-08-add-bounded-terminal-video-playback/`.

## Non-claims

No decoder, live-terminal, smoothness, color, audio, audio-video synchronization, production, or release check was run. This evidence does not establish decoder safety, media authenticity, exact timing, smooth playback, frame completeness, color fidelity, audio support, terminal correctness, sandboxing, production readiness, or release eligibility.
