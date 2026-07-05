# Proposal: Adapt Zerostack prompt mode profiles for Animus

## Why

Zerostack's prompt system packages task posture into named profiles such as code, plan, review, debug, ask, brainstorm, simplify, and write-prompt. It also supports prompt directives that switch permission mode and chain prompts from one phase to another. Animus needs the same ergonomic shape, but as typed, attributable prompt profile contracts rather than copied Markdown prompts.

Adapting this pattern lets hosts offer familiar task modes while preserving Animus-owned prompt assembly, policy, memory, and context admission boundaries.

## Scope

- Define prompt profile descriptors with purpose, allowed action posture, required capabilities, prompt-inclusion policy, and source identity.
- Define a first-line directive contract for mode and chain hints that is parsed into typed metadata before prompt assembly.
- Define chain-of-prompts transitions such as brainstorm to plan, plan to code, and code to review as policy-controlled continuation plans.
- Keep profile content attributable and bounded when it enters model-visible prompt sections.
- Add validation and negative tests for malformed directives, unsafe transitions, untrusted profile sources, and permission-mode bypass attempts.

## Impact

- **Files**: future prompt profile DTOs, prompt assembly core, Nickel profile contracts, host loader adapters, tests, and docs.
- **Testing**: positive profile assembly and chain transition tests; negative malformed directive, unknown mode, untrusted source, and disallowed transition tests.
- **Non-claims**: this does not copy Zerostack prompt text, adopt its config directories as canonical, or let prompts weaken permission or authority policy.
