# Proposal: Adapt Zerostack permission kernel for Animus

## Why

Zerostack demonstrates a compact permission kernel with security modes, glob and regex rules, session allowlists, mode-specific default behavior, and repeated-call doom-loop detection. Animus already has authority-first policy work, but it needs a concrete UX-oriented permission resolution contract that remains subordinate to Nickel policy and UCAN-style authority.

Adapting the pattern gives hosts a predictable allow/ask/deny layer without copying Zerostack's GPL implementation, config files, or direct tool execution behavior.

## Scope

- Define Animus-owned permission modes inspired by Zerostack's restrictive, read-only, guarded, standard, and yolo UX modes.
- Add typed glob/regex-style match descriptors that map guarded action classes and resource labels to allow, ask, or deny recommendations.
- Add scoped session allowlist entries that never bypass authority, sandbox, or redaction policy.
- Add repeated identical action detection that can coach, ask, or deny before wasting model/tool budget.
- Emit bounded permission receipts with authority status, local policy status, repeated-call status, and prompt-decision metadata.

## Impact

- **Files**: future permission DTOs, policy resolver core, receipt DTOs, Nickel policy contracts, tests, and docs.
- **Testing**: positive tests for mode defaults, matching precedence, allowlist acceptance, and loop coaching; negative tests for missing authority, malformed patterns, stale allowlists, and repeated denied actions.
- **Non-claims**: this does not adopt Zerostack's config format, GPL code, exact prompt text, or direct filesystem/process execution model.
