## Phase 1: Receipt core

- [ ] [serial] Implement canonical receipt projection over action kind, adapter identity, request snapshot, response selection, and gate verdicts, backed by BLAKE3. r[animus.live_action_receipt.identity]
- [ ] [serial] Implement pure action classification and claim-boundary rules. r[animus.live_action_receipt.boundary]
- [ ] [serial] Implement deterministic verification rules with no store access. r[animus.live_action_receipt.verifiable]

## Phase 2: Live-loop publication

- [ ] [serial] Publish one receipt per admitted action when a session records. r[animus.live_action_receipt.universality]
- [ ] [depends:animus.live_action_receipt.authority] Record exact authority-gate identities on every receipt. r[animus.live_action_receipt.authority]
- [ ] [serial] Add the injected publish port and the session-scoped recording flag in the shell. r[animus.live_action_receipt.universality]

## Phase 3: Fixtures and evidence

- [ ] [parallel] Add positive fixtures for identity stability, full publication, metadata claims, verification, and gate identity. r[animus.live_action_receipt.verification]
- [ ] [parallel] Add negative and boundary fixtures for over-claims, tampering, missing gate identity, and malformed payloads. r[animus.live_action_receipt.verification]
- [ ] [serial] Document the claim boundary and the reuse of existing gate verdicts. r[animus.live_action_receipt.boundary]
- [ ] [serial] Run workspace, Clippy, formatting, Cairn, and Nix checks, then record evidence. r[animus.live_action_receipt.verification]
