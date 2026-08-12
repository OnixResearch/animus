## Phase 1: Baseline and upstream cohort

- [x] [serial] Run and record the current positive and negative `animus-tui`, multimodal image, dependency-isolation, no-std, Wasm, Nickel, and Cairn baselines before changing dependencies. r[agent_harness_runtime.terminal_image_protocols.verification]
- [x] [serial] Fetch the canonical Rats default branch, record its observed head, compare it with `e9cb7865fe400de6220aea8bfc08ddf4f75da3d3`, and review source, dependency, license, feature, MSRV, and protocol changes. r[agent_harness_runtime.terminal_image_protocols.rats_cohort]
- [x] [serial] Record a conformance inventory for the selected upstream `rat-image` API, including explicit protocol selection, writer ownership, payload bounds, error behavior, cleanup, tests, and unsupported operations. r[agent_harness_runtime.terminal_image_protocols.rats_cohort] r[agent_harness_runtime.terminal_image_protocols.protocols]
- [x] [serial] If the selected API lacks required explicit inputs or typed failures, land and verify the smallest reusable fix in Rats first; then select and pin the resulting latest canonical upstream revision without a local patch. r[agent_harness_runtime.terminal_image_protocols.rats_cohort] r[agent_harness_runtime.terminal_image_protocols.boundary]

## Phase 2: Pin and isolate the dependency

- [x] [serial] Update every Rats Cargo and Nix source reference to the same selected immutable revision, add optional `rat-image` wiring only to `animus-tui`, and let Cargo and Nix generate their lock changes. Never edit `flake.lock` manually. r[agent_harness_runtime.terminal_image_protocols.rats_cohort] r[agent_harness_runtime.terminal_image_protocols.boundary]
- [x] [parallel] Add a non-default terminal-image feature and prove that default, core, adapters, runtime, Steel, no-std, Wasm, and unrelated host profiles gain no image decoder, base64, terminal, environment, filesystem, or writer dependency. r[agent_harness_runtime.terminal_image_protocols.boundary]
- [x] [parallel] Add typed Nickel policy for named image-byte, encoded-byte, dimension, cell, chunk, operation, and lifecycle bounds; export checked runtime data and Rust parity fixtures. r[agent_harness_runtime.terminal_image_protocols.planning]

## Phase 3: Pure planning and host shell

- [x] [serial] Add pure capability, protocol-row, geometry, image-identity, lifecycle, fallback, cleanup, and typed-error contracts inside the TUI boundary. r[agent_harness_runtime.terminal_image_protocols.planning]
- [x] [serial] Add deterministic planning for query, transmit, placement, replacement, deletion, cleanup, and text fallback over explicit inputs without terminal, environment, filesystem, clock, or process access. r[agent_harness_runtime.terminal_image_protocols.planning] r[agent_harness_runtime.terminal_image_protocols.protocols]
- [x] [serial] Add a thin host shell that resolves approved image handles to bounded bytes and invokes upstream `rat-image` through the selected protocol and writer seam. r[agent_harness_runtime.terminal_image_protocols.boundary] r[agent_harness_runtime.terminal_image_protocols.protocols]

## Phase 4: Protocol conformance and evidence

- [x] [parallel] Add positive Kitty fixtures for capability response parsing, bounded chunk transmission, placement, replacement, deletion, and cleanup, plus negative malformed response, stale ID, oversized payload, partial write, and failed cleanup fixtures. r[agent_harness_runtime.terminal_image_protocols.protocols] r[agent_harness_runtime.terminal_image_protocols.verification]
- [x] [parallel] Add positive iTerm2, Sixel, and text-fallback fixtures plus negative unsupported protocol, malformed image, invalid geometry, and denied-fallback fixtures. r[agent_harness_runtime.terminal_image_protocols.protocols] r[agent_harness_runtime.terminal_image_protocols.verification]
- [x] [parallel] Add metadata-only BLAKE3 receipts and leak negatives for raw bytes, base64, escape sequences, private paths, environment values, terminal responses, and unsupported full-protocol claims. r[agent_harness_runtime.terminal_image_protocols.evidence] r[agent_harness_runtime.terminal_image_protocols.verification]
- [x] [serial] Update the component catalog, facade inventory, README, terminal-host docs, feature guidance, and a focused Cairn/Nix rail with exact protocol rows and non-claims. r[agent_harness_runtime.terminal_image_protocols.evidence]
- [x] [serial] Run focused tests, dependency and source-shape guards, generated-configuration checks, workspace checks warranted by the dependency update, Cairn validation, and proposal/design/tasks gates; record skipped live-terminal checks. r[agent_harness_runtime.terminal_image_protocols.verification]
