# Proposal: Adapt Clankers runtime patterns for Animus

## Why

Animus now has typed core, adapter, Preserves, Steel, standard-shell, and Nickel configuration foundations. Clankers has already proven several adjacent agent-harness patterns: Nickel-governed Steel runtime policy, explicit request-lifecycle ownership, redacted receipts, neutral session ledgers, provider/tool dispatch separation, and focused drift rails. Adapting those ideas now will keep Animus small and embeddable while preventing the runtime work from growing hidden provider, persistence, or shell coupling.

## What Changes

- Add Nickel-authored Steel runtime policy for profiles, script bindings, budgets, allowed host actions, fallback behavior, and receipt redaction.
- Document the Animus request lifecycle and boundary ownership from runtime input through Steel decisions, adapter dispatch, events, reports, and receipts.
- Introduce safe receipt DTOs for runtime and adapter observations, using BLAKE3 hashes for content identity and redacting raw prompts, provider payloads, credentials, full paths, and tool bodies.
- Add a neutral session ledger/resume contract that can rebuild model-visible context without depending on concrete storage, daemon, UI, or provider crates.
- Tighten provider and tool dispatch boundaries so runtime code consumes injected services and fails closed when services are absent or unavailable.
- Add focused verification rails for Nickel policy drift, Steel profile validity, receipt redaction invariants, session-ledger behavior, and provider/tool dispatch boundaries.

## Impact

- **Files**: new `policy/steel-runtime/` Nickel policy, possible generated policy JSON, docs under `docs/`, possible receipt and session-ledger modules or crates, runtime/adapter tests, flake checks, and Cairn lifecycle artifacts.
- **Testing**: positive and negative tests for policy parsing, budget validation, receipt redaction, session replay, missing-service failures, provider/tool dispatch, and focused Nix checks.
- **Non-claims**: this change does not add a TUI, daemon, Matrix/P2P, worktree isolation, browser/voice/process-job tools, real model providers, or autonomous self-evolution. Those remain out of scope unless a later change explicitly adopts them.
