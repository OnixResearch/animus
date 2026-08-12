# Proposal: Negotiate model capability matrix

## Why

Model compatibility depends on knowing what a selected model can actually do before dispatch. Animus already records provider/model metadata for API kind, modality, budgets, reasoning, thinking levels, cost, and streaming in Rust, but compatibility decisions need a broader matrix for canonical message requests: role support, content parts, tool calls, structured output, streaming channels, reasoning controls, context/output limits, and local/privacy classes.

This change makes capability matching explicit and fail-closed so unsupported required features do not silently downgrade or fall through to another provider.

## What Changes

- Extend provider/model capability metadata and Nickel-owned registry examples to describe canonical request features such as roles, content part classes, tool-call support, JSON/structured output, stream channels, reasoning controls, budget classes, and local/privacy posture.
- Add pure capability negotiation over canonical request requirements, selected model capabilities, and downgrade policy.
- Return typed admitted, downgraded, or denied decisions with safe evidence suitable for model dispatch planning receipts.
- Add positive and negative tests for supported features, optional downgrade, required unsupported features, missing capability metadata, stale Nickel config, and denied fallback.

## Impact

- **Files**: `animus-std` provider capabilities, Nickel provider registry config/fixtures, runtime dispatch planning inputs, provider hardening docs, and facade inventory.
- **Testing**: pure negotiation tests, config drift tests, and fail-closed unsupported-feature tests.
- **Non-claims**: this does not add live provider calls, provider discovery, price scraping, credentials, OAuth, or concrete network clients.
