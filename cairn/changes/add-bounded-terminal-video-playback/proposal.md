## Why

A terminal image adapter can display static frames, but video playback adds decoding, frame scheduling, backpressure, cancellation, resource cleanup, and terminal animation state. Putting those effects into the image adapter or model payload contract would combine unrelated authority. Animus needs a separate optional TUI playback boundary that consumes host-supplied bounded frames and uses admitted terminal image operations without owning codecs, files, URLs, processes, clocks, audio, or persistence.

## What Changes

- Add pure bounded playback planning over caller-supplied frame metadata, monotonic ticks, terminal capabilities, policy, and lifecycle events. r[agent_harness_runtime.terminal_video_playback.planning]
- Add an injected frame-source service that supplies bounded decoded frames from approved handles while media decoding and transport remain host-owned. r[agent_harness_runtime.terminal_video_playback.frame_source]
- Add a non-default TUI playback shell that uses admitted Kitty animation or bounded image replacement, with text or thumbnail fallback. r[agent_harness_runtime.terminal_video_playback.backends]
- Add explicit pause, resume, cancellation, backpressure, frame-drop, terminal-loss, and cleanup behavior. r[agent_harness_runtime.terminal_video_playback.lifecycle]
- Add metadata-only BLAKE3 receipts and positive and negative deterministic playback fixtures. r[agent_harness_runtime.terminal_video_playback.evidence] r[agent_harness_runtime.terminal_video_playback.verification]

## Impact

- **Files**: `crates/animus-tui`, root feature wiring, typed Nickel playback policy, fake frame sources, terminal backend fixtures, component and facade inventories, README links, documentation, and focused Cairn/Nix checks.
- **Dependencies**: terminal playback depends on the terminal-image protocol change; reusable and default profiles gain no video decoder, process, filesystem, network, audio, or clock implementation.
- **Testing**: pure scheduler and lifecycle tests, fake frame streams, backend command fixtures, cancellation and cleanup failures, bounds and backpressure, dependency isolation, safe receipts, Cairn validation and gates.

## Non-Goals

- Bundling FFmpeg, GStreamer, platform codecs, file or URL loaders, provider clients, audio playback, media persistence, or ambient wall-clock access.
- Guaranteed smooth playback, exact audio-video synchronization, color fidelity, terminal-emulator correctness, or full media-player behavior.
- Automatic execution of untrusted media or a claim that terminal graphics provide a security sandbox.
