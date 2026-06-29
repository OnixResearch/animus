# Proposal: Adapt Zerostack context file layering for Animus

## Why

Zerostack automatically layers global and ancestor-discovered context files such as `AGENTS.md`, `CLAUDE.md`, and `ARCHITECTURE.md`, then injects them into the prompt. The ergonomic pattern is useful, especially the companion architecture file, but Animus must make discovery, trust, precedence, and prompt inclusion explicit.

Adapting this pattern gives hosts a predictable context-loading contract without silently granting project files instruction authority.

## Scope

- Define context file class descriptors for agent guidance, architecture guidance, project notes, and host-provided context.
- Define ancestor-walk discovery plans that return safe source identities and precedence ordering without doing ambient traversal in core.
- Define trust/admission policy before discovered files can influence model-visible prompt sections.
- Define prompt attribution for included files and omission diagnostics for disabled, untrusted, oversized, stale, or unsafe files.
- Add receipts with BLAKE3 body hashes and redacted source identifiers.

## Impact

- **Files**: context reference DTOs, host discovery interfaces, prompt assembly integration, Nickel policy contracts, tests, and docs.
- **Testing**: positive precedence and attribution tests; negative untrusted file, oversized file, stale file, unsafe path, disabled discovery, and secret-looking body tests.
- **Non-claims**: this does not make Zerostack's file names mandatory, treat `CLAUDE.md` as canonical, or silently inject arbitrary ancestor files into reusable runtime core.
