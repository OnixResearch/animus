# OpenAI Codex guidance and skill budget adaptation

Animus adapts Codex-style layered guidance and progressive skill disclosure as typed adapter data, not as ambient filesystem behavior.

## Layered guidance

Hosts pass validated `GuidanceSource` values into `assemble_guidance_sources`. Each source carries:

- a stable name and safe origin label;
- a scope and precedence rank;
- override semantics;
- a lowercase BLAKE3 content hash;
- a non-zero prompt-budget cost;
- an optional host redaction decision.

Assembly is deterministic: lower precedence ranks are recorded first, duplicate active `(scope, precedence)` pairs fail closed, and the ledger records included, truncated, skipped, and redacted sources with consumed budget and source attribution.

## Progressive skill disclosure

Hosts validate ordinary `SkillManifest` descriptors first, then derive `SkillDiscoveryMetadata` through `SkillDiscoveryDraft`. Metadata exposes only name, summary, source, invocation policy, dependency hints, and metadata budget cost. Full instructions, references, assets, and scripts stay out of the initial model context.

`decide_skill_loading` is the pure decision point for deferred loading. It denies missing skills, duplicate names, disabled skills, implicit loads without policy approval, and missing dependencies before a host may provide full instructions.

## Skill packages and scripts

`SkillPackageDescriptor` records instructions, optional scripts, references, assets, dependencies, and trust status as data. Runtime core never executes package scripts directly. `admit_runtime_core_skill_script` always returns a denial; trusted scripts must go through host shell, hook, and tool policy, while untrusted script packages are denied before that handoff.

## Non-claims

This contract does not make Codex file names the Animus default, scan home/project directories, load skill bodies by path, execute scripts in reusable core, or install external plugins.
