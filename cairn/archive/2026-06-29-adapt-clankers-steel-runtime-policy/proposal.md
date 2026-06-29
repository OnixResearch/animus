# Proposal: Adapt Clankers Steel runtime policy for Animus

## Why

Animus already has a Steel loop fabric, but runtime policy is still implicit in Rust code and tests. Clankers demonstrates a safer pattern: Nickel-authored policy declares script identity, hashes, budgets, host actions, fallback behavior, and receipt redaction, while Rust enforces exported policy before Steel can influence a turn. Animus should adopt that pattern in its own smaller terms.

## What Changes

- Add `policy/steel-runtime/` Nickel contracts and checked exported data for Steel runtime profiles.
- Declare profile identity, script bindings, BLAKE3 script hashes, runtime budgets, allowed host actions, fallback mode, and receipt redaction policy.
- Enforce selected profile, script hash, budget, action allowlist, and fallback behavior before Steel evaluation.
- Add invalid-policy fixtures for missing scripts, bad names, non-positive budgets, unknown actions, hash mismatches, path escapes, and unsupported fallback modes.
- Add focused drift and activation rails with reviewable receipts.

## Impact

- **Files**: Nickel policy, generated JSON if needed, scripts/checker, runtime tests, docs, Cairn artifacts.
- **Testing**: positive and negative policy validation; enforcement before Steel turn evaluation.
- **Non-claims**: this does not grant Steel ambient filesystem, shell, network, provider, credential, daemon, or session mutation authority.