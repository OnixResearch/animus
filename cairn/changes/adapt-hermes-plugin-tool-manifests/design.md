# Design: Adapt Hermes plugin and tool manifests for Animus

## Context

Hermes keeps review/gate workflows available both as skills and as a first-class plugin surface. Its manifest advertises tool names while implementation remains in scripts. Animus can reuse this boundary as a data contract for host-supplied toolsets.

Reference material inspected:

- `/home/brittonr/git/hermes/tools/hermes-review-gate-plugin/plugin.yaml`
- `/home/brittonr/git/hermes/tools/hermes-review-gate-plugin/scripts/*.py`
- `/home/brittonr/git/hermes/openspec/specs/agent-orchestration/spec.md`

## Decisions

### 1. Toolsets are declared before executors

**Choice:** Add plugin/toolset manifest DTOs that list provided tools, descriptions, parameter schema identities, capability labels, and result contract names before any executor is attached.

**Rationale:** Agents and prompts can reason about available actions without loading executable plugin code.

### 2. Executors are host-injected services

**Choice:** Runtime dispatch requires an injected executor matching the validated tool definition. If absent, dispatch fails with a typed missing-executor error.

**Rationale:** Manifest discovery must not become a backdoor for filesystem execution or shell fallback.

### 3. Capability labels are required

**Choice:** Each manifest-provided tool declares whether it is read-only, dry-run, metrics-writing, filesystem-mutating, process-spawning, networked, or otherwise capability-bearing.

**Rationale:** Prompt construction and middleware need clear side-effect boundaries.

### 4. Collision handling is deterministic

**Choice:** Duplicate toolset names or tool names are rejected or resolved by explicit host policy with diagnostics that avoid unsafe paths.

**Rationale:** Tool prompts and dispatch tables must be reproducible.

## Risks / Trade-offs

- Capability labels can become too coarse. Start with a small stable vocabulary and allow host-specific extension labels in metadata.
- YAML is a Hermes implementation detail. Animus should validate the data shape, not require YAML parsing in core.
- Some hosts may generate manifests dynamically. Dynamic entries must still pass the same validation before becoming active.
