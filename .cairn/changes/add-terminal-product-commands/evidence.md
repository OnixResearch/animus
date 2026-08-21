# Evidence

Date: 2026-08-21

Outcome: exact terminal controls, private event relay, and the persistent live-session shell pass focused functional, routing, rendering, strict Clippy, Cairn, and Nix rails. Existing strict Octet debt still blocks archive completion.

## Passing evidence

- The published base revision `1498832358a6cd960508b4709fecc4f271c4c90e` passed the pre-change focused baseline.
- `cargo test -p animus-local-daemon -p animus-live-terminal --all-targets --all-features --locked` passed 49 tests after the private relay change.
- `cargo test -p animus-live-steel-agent --all-targets --all-features --locked` passed 25 tests after the persistent live-session change.
- The live Steel and terminal focused host rails passed tests, strict Clippy, documentation tests, catalog checks, and runtime-facade inventory checks.
- `nix build .#checks.x86_64-linux.live-steel-agent-shell --no-link -L` passed.
- `nix build .#checks.x86_64-linux.live-terminal-agent-host --no-link -L` passed.
- Cairn validation and proposal, design, and task gates passed.
- Direct and daemon tests prove the same typed state event crosses both explicit connection modes.
- Positive tests prove exact command correlation, deterministic observation identity, metadata reduction, and terminal rendering.
- Negative tests reject unknown wire fields, private-field smuggling, malformed identities, blocked restore, stale worker generations, restore drift, unordered batches, forged observation identities, and queue-generation regression.
- The state observation identity binds the request correlation, root, sequence, worker state, queue state, and all bounded counts.
- Terminal completion receipts remain unchanged and omit inspected state.
- The completed live Steel output returns the exact ordered runtime-event snapshot, and its event count matches the metadata-only receipt.
- Direct and daemon tests route the same versioned Preserves runtime events for conversation, tool, and session status rendering.
- The supervisor validates private payload bounds, root, order, class, and BLAKE3 identity without decoding or retaining payload bytes.
- Negative tests reject empty and oversized payloads, unknown classes, malformed codecs, forged identities, stale roots, and unordered sequences.
- `WorkerPrivateEvent` and daemon-event debug output report byte counts only.
- `LiveSteelAgentComposition::open_session` moves the provider, snapshot store, coding-session store, Steel tool, kernel persistence, and validated configuration into one owner.
- Positive tests drive a complete tool continuation one effect at a time and produce the same private runtime events and metadata-only receipt.
- Interleaving tests queue and edit a follow-up while a provider effect remains pending, then cancel through the existing coding-session effect.
- Negative tests reject malformed correlations, stale queue generation and identity guards, zero event bounds, over-bound event batches, and stale cursors.
- The persistent host, private prompt DTOs, and event batch have no debug projection that can expose private content.
- `finish` and `close` attempt every session, snapshot, and kernel lease release before returning a close or projection error.

## Adversarial audit

The first metadata projection accepted a non-zero worker generation without matching it to the active generation. It also lacked request correlation and payload identity binding.

The corrected design now checks worker generation and restore status against supervisor and terminal state. It enforces strict batch order and queue-generation monotonicity. It binds each state payload to its request and ordered event identity with BLAKE3.

No provider, tool, Steel, session, queue, journal, persistence, credential, endpoint-discovery, or process authority moved into the terminal.

A separate private stream was rejected because its cursor can drift from safe events. Direct terminal store access was rejected because it adds persistence authority. Rebuilding the live host for each command was rejected because it loses pending effects. The accepted design uses one persistent owner, the existing ordered worker stream, and the versioned Preserves runtime contract.

The first relay file name added two Octet findings. Renaming `private_event.rs` to `privateevent.rs` and adding explicit post-validation assertions restored both focused package counts to the published baseline.

## Octet comparison

The exact pinned package checks were run against published private-relay baseline `f6d2bbb253bc1f5c8bc1ee6bad4cf887cc1d5f9d` and this change.

`animus-live-terminal` reported the same result for both revisions:

```text
Findings: 121
Warnings: 2
Errors: 119
Config hash: b3:4cb01362a9222b7c7606540c736888c68cac53a4282d33fda06bed3bfcf2158a
Profile hash: b3:f4f80c7b7038fd73adcc65248aed9a23d589af3ef143a5a863abdfb02e19e736
```

`animus-local-daemon` reported the same result for both revisions:

```text
Findings: 220
Warnings: 0
Errors: 220
Config hash: b3:c6b638a9ecea8fb957bb09e5422e428677189aa2b0fedc83cefede3b4c3d5383
Profile hash: b3:0b4ca52658f1d11e82ab7930dd54ef8f67d1be8a56f662811a34c0c4bf9ea6af
```

The current package-focused Octet run reports the same 372 `animus_live_steel_agent` errors as the checked base. No finding points into `session.rs`, `session/control.rs`, or `session/lifecycle.rs`:

```text
Findings: 373
Warnings: 1
Errors: 372
animus_live_steel_agent: 372
animus_std: 1
Config hash: b3:28969e21d70c877cbbac8e231282e8c319c4f71655d0967ffed087a1a2d10c1d
Profile hash: b3:da16b91cfb1614bba9194b9b5f9390a3a324a05152235a4c3df7e403f4c4a312
```

The one warning is an unrelated `animus_std` unknown-lint diagnostic. The persistent-session source adds no focused package finding. This is regression evidence only. It does not satisfy the deny-all gate.

## Blockers and non-claims

- Existing live Steel, terminal, and daemon Octet debt blocks the strict package gate.
- The change remains active because the required deny-all gate does not pass.
- This evidence does not claim product worker authorization correctness, approval or child-control composition, kernel-clear execution, full-flake acceptance, archive completion, product readiness, production readiness, or release eligibility.
- Provider and tool calls remain synchronous effects. The worker can interleave commands between effects, not inside one adapter call.
- The private relay does not claim remote security, end-to-end encryption, or protection from a compromised local process.
- The current standalone Cairn CLI rejected the repository's locked policy field `task_marker_policy.markers`; the repository-pinned Cairn CLI passed validation and all three change gates.
