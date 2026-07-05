# Design: Adapt Pi skills and resource discovery for Animus

## Context

Pi discovers AGENTS/context files, skills, prompt templates, themes, and extensions from global and project-local locations. Animus should not copy that whole product surface, but the resource descriptors, progressive disclosure, and project-trust ordering are valuable.

## Decisions

### 1. Resources are typed descriptors

**Choice:** Define resource descriptor DTOs with name, description, content handle or content hash, source scope, provenance, trust status, and resource kind.

**Rationale:** Prompt construction and runtime policy need to know what a resource is and where it came from without requiring core code to walk files.

### 2. Discovery belongs to the shell or host

**Choice:** Hosts discover files, packages, or application-provided resources and pass typed descriptors to Animus. The runtime does not scan directories.

**Rationale:** Animus remains embeddable and avoids filesystem policy in the core.

### 3. Project trust gates dynamic local resources

**Choice:** Project-local skills, prompt templates, context files, and future executable resources are ignored or marked unavailable until the host supplies a positive trust decision.

**Rationale:** Project guidance can steer the model or request unsafe actions. Trust sequencing must happen before prompt influence.

### 4. Skills use progressive disclosure

**Choice:** The model sees available skill names/descriptions and can request full content through an explicit host-mediated load path.

**Rationale:** This preserves context while still making specialized workflows discoverable.

### 5. Collisions produce deterministic diagnostics

**Choice:** Duplicate resource names are resolved by policy order and accompanied by diagnostics rather than nondeterministic map overwrites.

**Rationale:** Prompt surfaces and commands must be reproducible and reviewable.

## Risks / Trade-offs

- Automatic skill matching can over-load irrelevant guidance. Keep matching host-policy-owned and test prompt budget impact.
- Project trust can be awkward in headless mode. Provide explicit trust inputs and fail closed by default.
- Resource descriptors can leak full paths. Store safe provenance labels and hashes; leave display paths to shell policy.
