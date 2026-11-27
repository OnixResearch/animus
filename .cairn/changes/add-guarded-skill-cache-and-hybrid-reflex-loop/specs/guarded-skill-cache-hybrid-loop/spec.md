# Guarded skill cache and hybrid reflex loop requirements

## ADDED Requirements

### Requirement: Shadow probation and live hit sampling against silent distribution shift

Compiled procedural skills admitted into the Animus runtime cache SHALL start in `ProbationShadow`, promote to `ActiveSampled` only after `K` consecutive shadow matches against System 2, and subject `ActiveSampled` hits to deterministic seeded dual-execution sampling. Any silent mismatch (exit code `0` with divergent output) SHALL transition the skill immediately to `SuspendedDrift` and route subsequent requests to System 2 fallback.

#### Scenario: Silent distribution shift triggers automatic suspension

- **GIVEN** an `ActiveSampled` cached skill whose environment semantics silently shift so it returns exit code `0` with an incorrect result
- **WHEN** the deterministic live-hit sampler evaluates a sampled hit against System 2 and detects output divergence
- **THEN** the skill SHALL transition immediately to `SuspendedDrift`, emit a drift receipt with input/output digests and explicit non-claims, and route all subsequent matching calls to System 2 fallback

### Requirement: Attenuated hybrid reflex routing with clean fallback-on-miss

`animus-frozen-policy` reflex outputs SHALL be checked against attenuated host capabilities (`animus-std-basalt-authority`) and strict skill applicability guards before execution. Any capability violation, guard rejection, or missing skill SHALL fail closed to System 2 fallback without executing the cached skill or expanding agent authority.

#### Scenario: Misfiring reflex or rephrased out-of-contract input falls back safely

- **GIVEN** a bounded System 1 reflex emitting an out-of-scope action or a rephrased input failing a cached skill's applicability guard
- **WHEN** the hybrid loop evaluates the turn
- **THEN** the runtime SHALL refuse privilege expansion, skip cached skill execution, record the structured refusal/miss receipt, and route execution cleanly to System 2 fallback
