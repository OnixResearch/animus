# Evidence

Date: 2026-08-20

Outcome: the focused implementation passes functional, routing, rendering, strict Clippy, Cairn, and Nix rails. Existing strict Octet debt still blocks archive completion.

## Passing evidence

- The published base revision `1498832358a6cd960508b4709fecc4f271c4c90e` passed the pre-change focused baseline.
- `cargo test -p animus-local-daemon -p animus-live-terminal --all-targets --all-features --locked` passed 49 tests after the private relay change.
- `cargo test -p animus-live-steel-agent --all-targets --all-features --locked` passed 22 tests after the private event-source change.
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

## Adversarial audit

The first metadata projection accepted a non-zero worker generation without matching it to the active generation. It also lacked request correlation and payload identity binding.

The corrected design now checks worker generation and restore status against supervisor and terminal state. It enforces strict batch order and queue-generation monotonicity. It binds each state payload to its request and ordered event identity with BLAKE3.

No provider, tool, Steel, session, queue, journal, persistence, credential, endpoint-discovery, or process authority moved into the terminal.

A separate private stream was rejected because its cursor can drift from safe events. Direct terminal store access was rejected because it adds persistence authority. The accepted design uses the existing ordered worker stream and versioned Preserves runtime contract.

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

`animus-live-steel-agent` also reported the same result on `f6d2bbb253bc1f5c8bc1ee6bad4cf887cc1d5f9d` and this change:

```text
Findings: 372
Warnings: 0
Errors: 372
Config hash: b3:28969e21d70c877cbbac8e231282e8c319c4f71655d0967ffed087a1a2d10c1d
Profile hash: b3:da16b91cfb1614bba9194b9b5f9390a3a324a05152235a4c3df7e403f4c4a312
```

The comparisons found no new finding count or lint-category count. This is regression evidence only. It does not satisfy the deny-all gate.

## Blockers and non-claims

- Existing live Steel, terminal, and daemon Octet debt blocks the strict package gate.
- The change remains active because the required deny-all gate does not pass.
- This evidence does not claim worker-side authorization correctness, kernel-clear execution, full-flake acceptance, archive completion, product readiness, production readiness, or release eligibility.
- The private relay does not claim remote security, end-to-end encryption, or protection from a compromised local process.
- The current standalone Cairn CLI rejected the repository's locked policy field `task_marker_policy.markers`; the repository-pinned Cairn CLI passed validation and all three change gates.
