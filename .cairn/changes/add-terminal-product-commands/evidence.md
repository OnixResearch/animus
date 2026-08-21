# Evidence

Date: 2026-08-20

Outcome: implementation passes focused functional gates but remains blocked by existing strict Octet debt and unpublished source.

## Passing evidence

- The pre-change `animus-live-terminal` baseline passed all 22 focused tests in the Nix development shell.
- `cargo test -p animus-live-terminal --all-targets --all-features --locked` passed 23 tests after the change.
- Positive tests prove exact `kernel_clear` and `state_inspection` classes, valid BLAKE3 correlation identities, serialization, and direct-service routing.
- Negative tests prove unknown command kinds and malformed kernel-clear correlation identities fail before worker delivery.
- Direct and daemon routing tests prove both exact product command classes pass the existing private-payload boundary.
- The pure control test proves descriptor construction does not alter the terminal view or execute an effect.
- Strict focused Clippy passed with `-D warnings`.
- `scripts/cairn/check-live-terminal-agent-host --root .` passed terminal, daemon, TUI, catalog, facade, authority, and leak checks.
- `nix build .#checks.x86_64-linux.live-terminal-agent-host --no-link -L` passed.
- Cairn validation and proposal, design, and task gates passed before implementation.

## Corrected failure

An adversarial daemon-path audit found that the wire allowlist initially omitted both new exact classes. The follow-up change adds them and routes both commands through the daemon dispatcher test. No compatibility class or generic fallback was added.

## Octet comparison

The pinned package-level Octet command was run against both the published base revision `e90c55d9aae274b103196573ea7f868fd26d5114` and this change.

Both runs reported the same result:

```text
Findings: 118
Warnings: 3
Errors: 115
animus_live_terminal: 117
animus_std: 1
Config hash: b3:4cb01362a9222b7c7606540c736888c68cac53a4282d33fda06bed3bfcf2158a
Profile hash: b3:f4f80c7b7038fd73adcc65248aed9a23d589af3ef143a5a863abdfb02e19e736
```

The comparison found no new Octet finding count or lint-category count. This is regression evidence only. It does not satisfy the deny-all gate.

## Blockers and non-claims

- Existing `animus-live-terminal` Octet debt blocks the strict package gate. This change does not baseline, suppress, or repair unrelated debt.
- The change is not archived because the required deny-all gate does not pass.
- The local branch is not published. A downstream immutable GitHub pin cannot consume it until publication is explicitly authorized.
- This evidence does not claim full-flake acceptance, archive completion, publication, product authorization, state redaction correctness, or kernel-clear execution.
