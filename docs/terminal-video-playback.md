# Bounded terminal video playback

Animus provides deterministic terminal-video scheduling under the non-default `terminal-video` feature. This feature includes `terminal-image`. Default, core, adapter, runtime, Steel, no-std, and Wasm profiles do not gain a decoder, video transport, terminal writer, clock, audio system, or media store.

## Authority boundary

`reduce_terminal_video` is the functional core. It receives an admitted descriptor, safe decoded-frame metadata, explicit monotonic ticks, exact terminal capabilities, typed policy, lifecycle commands, and prior bounded state. It returns fetch, display, wait, drop, pause, resume, stop, fallback, cleanup, or completion actions. It does not read a clock, terminal, environment, file, URL, process, network, credential, codec, audio device, or persistence state.

The host owns decoded frames and effects:

1. `TerminalVideoFrameSource` resolves one approved opaque source handle to one bounded transient static frame.
2. `execute_terminal_video_frame` rechecks frame size and BLAKE3 identity, then uses the pinned terminal-image shell and an explicit writer.
3. `TerminalVideoBackendService` performs only the exact metadata-level animation, replacement, fallback, or cleanup operation selected by the pure plan.

Durable playback state and receipts retain no frame body. The transient frame packet must be consumed and released by the host shell.

## Backend inventory

The terminal-image cohort remains `3b498c3d7c248c335d3e7faf12f133a18c87d632`. Its `rat-image` crate prepares and writes bounded static Kitty, iTerm2, and Sixel payloads. It rejects animated GIF and WebP inputs. It does not encode Kitty animation-control commands.

Playback support is operation-specific:

| Mode | Static frame transfer | Animation control | Placement | Deletion | Source |
| --- | --- | --- | --- | --- | --- |
| Kitty animation | pinned `rat-image` static-frame transfer | exact injected host row | exact injected host row | exact injected host row | injected decoded-frame source |
| Static replacement | pinned `rat-image` transfer plus exact host lifecycle row | no | exact host row | exact host row | injected decoded-frame source |
| Thumbnail fallback | no video frame stream | no | host UI projection | host UI projection | injected approved thumbnail source |
| Text fallback | no | no | normal bounded text | normal bounded text | admitted safe label |

Static image support does not imply animation support. Kitty animation is selected only when frame transfer, animation control, placement, and deletion are all explicit. Unsupported Kitty extensions stay disabled. iTerm2 and Sixel remain static transmit-only in the pinned cohort, so Animus does not claim independent replacement or animation for them.

## Scheduling and lifecycle

`config/terminal-video-policy.ncl` owns named frame-byte, aggregate-buffer, queue, geometry, frame-count, frame-rate, duration, drop, action, lag, frame-source deadline, and cleanup bounds. Its generated runtime projection is `config/generated/terminal-video-policy.json`.

The reducer admits frames in exact index order. Presentation ticks derive from the declared rate and the supplied playback start tick. Early frames produce a wait action without body retention. Late or over-capacity frames are dropped only when policy allows it and the drop count remains bounded. Otherwise, reduction fails closed.

Pause and resume stop and restart frame requests through explicit actions. Cancellation, source failure, terminal loss, layout eviction, finish, and shutdown stop new requests. Allocated Kitty animation or static image state creates exact cleanup obligations. Playback remains `cleanup-pending` until each obligation succeeds. A cleanup failure becomes `cleanup-incomplete`.

## Evidence

Receipts bind the safe source class, exact backend mode, pinned terminal-image cohort, policy and source BLAKE3 identities, displayed and dropped counts, status, cleanup count, and explicit non-claims. They omit source handles, media or frame bytes, base64, escape sequences, decoder output, paths, URLs, credentials, environment values, terminal responses, and raw timing traces.

## Verification

Run:

```sh
scripts/cairn/check-terminal-video-playback --root .
nix build .#checks.x86_64-linux.terminal-video-playback --no-link -L
```

The focused rail covers ordered scheduling, explicit ticks, pause and resume, queue backpressure, bounded dropping, cancellation, source failure, terminal loss, cleanup, injected sources, partial writes, backend failures, fallbacks, receipt leaks, dependency isolation, Nickel parity, no-default profiles, and Wasm isolation.

Passing these checks establishes deterministic behavior for supplied fixtures and injected fake services. It does not prove decoder safety, media authenticity, exact timing, smooth playback, frame completeness, color fidelity, audio support, audio-video synchronization, terminal correctness, sandboxing, production readiness, or release eligibility.
