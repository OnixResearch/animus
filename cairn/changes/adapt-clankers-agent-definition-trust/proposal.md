# Proposal: Adapt Clankers agent definition trust for Animus

## Why

Clankers supports named agent definitions with a simple trust distinction between user-level and project-level definitions. Animus needs a more typed version of that pattern before agent teams, delegated workers, and workflow commands can safely consume reusable agent personas. Agent manifests should be data with capability ceilings and trust metadata, not ambient prompt files that silently grant tools or model authority.

Animus should adopt the trust model while using Nickel or Preserves-backed typed manifests instead of Clankers' Markdown/YAML frontmatter format.

## What Changes

- Define typed agent-definition DTOs for name, description, model preference, prompt fragments, capability ceilings, toolset limits, source scope, trust status, and manifest identity.
- Add Nickel/Preserves manifest contracts for user, project, and explicit host-provided definitions with deterministic precedence and duplicate-name rejection.
- Require project/workspace definitions to be untrusted by default and require host confirmation or policy admission before they influence prompt assembly or tool/model access.
- Treat agent definition tools and model preferences as ceilings or requests, never as authority grants beyond UCAN/policy capability checks.
- Add safe discovery/activation receipts and negative fixtures for malformed manifests, duplicate names, untrusted project definitions, capability escalation, unsafe prompt bodies, and receipt leaks.

## Impact

- **Files**: future agent-definition DTOs, Nickel/Preserves contracts, discovery/activation policy, tests, docs, and Cairn artifacts.
- **Testing**: positive user/explicit manifest activation fixtures and negative project-trust/duplicate/escalation/malformed/leak fixtures.
- **Non-claims**: this does not implement full agent teams, subagent execution, Clankers Markdown/YAML compatibility, or implicit project prompt authority.
