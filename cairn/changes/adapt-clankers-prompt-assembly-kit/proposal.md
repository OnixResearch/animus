# Proposal: Adapt Clankers prompt assembly kit for Animus

## Why

Animus is accumulating guidance, skills, memories, context references, policy profiles, and agent definitions. Clankers' embedding docs show a useful prompt-assembly brick: hosts provide explicit prompt sources, reusable logic assembles bounded attributed sections, and embedders can disable ambient filesystem discovery by default. Animus needs that umbrella contract so future features do not each invent a separate prompt precedence and receipt model.

## What Changes

- Define typed prompt-source DTOs for host context, policy profiles, skills, memories, agent definitions, context references, tool metadata, and run objectives.
- Add pure deterministic prompt assembly over in-memory sources, trust/admission decisions, precedence policy, section budgets, and redaction policy.
- Keep filesystem discovery, project-file loading, network retrieval, and host-specific source discovery behind injected adapters.
- Emit prompt assembly ledgers that record included, skipped, truncated, redacted, denied, and unsupported sources with safe source identity and BLAKE3 body hashes.
- Add positive and negative fixtures for precedence, disabled ambient discovery, over-budget sources, conflicting sources, untrusted project sources, unsupported source kinds, and receipt leaks.

## Impact

- **Files**: future prompt-source DTOs, assembly core, policy fixtures, ledgers/receipts, tests, docs, and Cairn artifacts.
- **Testing**: positive assembly fixtures and negative ambient-discovery/conflict/trust/budget/leak fixtures.
- **Non-claims**: this does not copy Clankers prompt text, enable ambient project-file discovery for embedders by default, or make any one prompt source an authority grant.
