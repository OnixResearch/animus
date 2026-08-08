# Terminal image protocols

Animus provides bounded static-image planning under the non-default `terminal-image` feature. The reusable runtime, adapter, Steel, default, no-std, and Wasm profiles do not gain terminal-image dependencies.

## Selected Rats cohort

The implementation baseline fetched the canonical `git@github.com:brittonr/rats.git` default branch. Its observed head was `3b498c3d7c248c335d3e7faf12f133a18c87d632` on 2026-08-08. Cargo and Nix pin this exact revision.

The prior Animus pin was `e9cb7865fe400de6220aea8bfc08ddf4f75da3d3`. The selected range adds the combined Rats workspace, `rat-image`, reusable host-effect boundaries, typed limits, and many unrelated UI packages. Animus selects only `rat-image`, `rat-keymap`, and `rat-widgets` as optional TUI dependencies.

`rat-image` uses MIT workspace licensing. Its direct content dependencies are `base64 0.22.1` and `image 0.25.10`. Both use MIT OR Apache-2.0 licensing. The selected `image` release declares Rust 1.88.0. The optional terminal-image feature therefore needs Rust 1.88 or newer; default profiles retain the workspace Rust 1.85 declaration because they do not resolve this optional dependency.

The upstream API already has the required seam:

- `prepare_terminal_image` accepts explicit bytes, protocol, cell bounds, and typed limits.
- `write_prepared_terminal_image` accepts an explicit writer and reports exact or partial writes.
- It does not open `/dev/tty`, fall back to stdout, or inspect process environment.
- Errors use a closed content-free vocabulary.

No local Rats patch or copied protocol encoder is used.

## Boundary

`plan_terminal_image` is the functional core. It consumes safe metadata, an operation-level capability row, layout geometry, policy, intent, and bounded lifecycle state. It does not read image bytes, files, environment variables, terminals, clocks, processes, networks, or credentials.

`execute_terminal_image_plan` is the imperative shell. The host supplies:

1. `TerminalImageByteResolver` for one approved safe handle.
2. `TerminalImageLifecycleService` for metadata-only query, placement, replacement, deletion, and cleanup operations.
3. An explicit `std::io::Write` terminal sink.

For transmission, the shell rechecks byte count and BLAKE3 identity. It then uses the selected upstream Rats preparation and writer APIs. It does not retain source or prepared bytes after the call.

## Operation-level conformance

| Protocol | Query | Transmit | Place | Replace | Delete | Cleanup | Built-in payload path |
|---|---:|---:|---:|---:|---:|---:|---|
| Kitty | host capability | Rats | host capability | host capability | host capability | host capability | `rat-image` transmit and display |
| iTerm2 | no | Rats | no | no | no | no | `rat-image` inline transmit |
| Sixel | no | Rats | no | no | no | no | `rat-image` bounded preparation |
| Text | no | no | no | no | no | no | bounded alternative text |

A host must supply each capability as an exact fact. `parse_kitty_capability_response` reduces one bounded complete positive Kitty response to a support state without retaining response bytes. `rat-image` transmission does not imply query, independent placement, replacement, deletion, cleanup, animation, or complete protocol support. Unsupported destructive operations fail. A render request can use visible bounded text only when policy allows it.

A full Kitty static plan orders query, transmit, and placement. Replacement, deletion, and cleanup are separate plans. A failure after transmission keeps a cleanup obligation visible. Failed cleanup produces `cleanup-incomplete`; it never produces a successful render receipt.

## Limits and evidence

`config/terminal-image-policy.ncl` owns the image-byte, prepared-byte, capability-response, source dimension, source pixel, decoded-byte, terminal-cell, chunk, operation, lifecycle, and alternative-text limits. `config/generated/terminal-image-policy.json` is the checked runtime projection.

Receipts bind the selected Rats revision, protocol row, BLAKE3 content identity, byte counts, geometry, operation outcomes, final status, and explicit non-claims. They omit safe lookup handles, alternative text, raw image bytes, base64, escape payloads, private paths, environment values, terminal responses, provider payloads, and credentials.

## Verification

Run the focused rail:

```sh
scripts/cairn/check-terminal-image-protocols --root .
nix build .#checks.x86_64-linux.terminal-image-protocols --no-link -L
```

The rail covers positive and negative planning, Rats transmission through an injected writer, byte identity drift, bounds, unsupported rows, fallback denial, partial writes, failed cleanup, evidence leaks, dependency isolation, Nickel drift, no-default builds, and Wasm isolation.

These checks establish source shape, pure planning behavior, selected fixture bytes, injected writer behavior, and metadata-only projection. Passing the rail does not prove live-terminal behavior, emulator correctness, visual fidelity, decoder correctness, hostile-image safety, process isolation, remote-session transparency, video or audio playback, provider availability, production readiness, or release eligibility.
