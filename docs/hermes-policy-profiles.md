# Hermes policy profile adaptation

Animus adapts the useful Hermes operating-posture pattern as typed policy/profile data, not as a monolithic prompt. Hosts parse policy sources, then reusable crates consume validated descriptors for deterministic prompt-context assembly and review evidence.

## Descriptor shape

`animus-adapters` exposes `PolicyProfileDraft` / `PolicyProfile` for:

- policy kind: global posture, project overlay, or tool guidance;
- source scope: global, session, project, or tool;
- trust status and deterministic priority;
- goal-first text, success criteria, constraints, validation rules, stop rules, provenance, and optional snippets.

Profiles are bounded and validated before prompt assembly. Prompt hygiene rejects process-heavy scripts, unbounded instructions, stale model-specific guidance, missing validation rules, missing stop rules, and oversized policy text.

## Composition rules

Policy context composition is pure and deterministic:

1. Global profiles precede session, project, and tool profiles.
2. Higher priority wins within a source scope.
3. Provenance labels break remaining ties.
4. Untrusted project profiles are omitted unless the host admits trusted project policy.
5. A global destructive-action approval constraint defeats a project-local implicit destructive allow, with a safe diagnostic naming only the project provenance.

Prompt segments always carry provenance attribution so operators can review which policy source contributed each rule.

## Enforcement boundary

Policy descriptors are guidance/configuration input only. They do not replace typed runtime enforcement for capability denial, read-only mode, mutation serialization, sandbox policy, credential redaction, or receipt safety. A read-only policy profile still requires runtime capability guards to deny mutation requests.

## Focused checks

```sh
cargo test -p animus-adapters --locked policy_profile::
cargo test -p animus-adapters --locked
cargo clippy -p animus-adapters --all-targets --locked -- -D warnings
```
