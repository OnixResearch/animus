# Design: Adapt Hermes skill manifests for Animus

## Context

Hermes distinguishes the operational skill store from versioned source skills, but both expose a compact `SKILL.md` contract that is easy for agents and tools to inspect. Animus needs the same lightweight manifest boundary as data, not a dependency on Hermes' filesystem layout.

Reference material inspected:

- `/home/brittonr/git/hermes/README.md`
- `/home/brittonr/.hermes/skills/software-development/hermes-agent-skill-authoring/SKILL.md`
- `/home/brittonr/git/hermes/_global/skills/*/SKILL.md`

## Decisions

### 1. Manifest data is Animus-owned

**Choice:** Add Animus skill manifest DTOs with name, description, version, source category, tags, related skill names, body identity, provenance, trust status, and compatibility notes.

**Rationale:** Animus can adapt the useful metadata shape without making Hermes a required dependency.

### 2. Host parses and supplies manifests

**Choice:** The host or std adapter reads `SKILL.md` content and passes parsed manifest descriptors to reusable runtime logic. No `no_std` crate scans `~/.hermes`, `~/.pi`, project directories, or package stores.

**Rationale:** Discovery is host policy. Core runtime consumes typed data.

### 3. Validation is pure and strict

**Choice:** Manifest validation runs over in-memory fields and returns typed diagnostics for malformed names, missing descriptions, invalid versions, missing bodies, duplicate tags, unresolved related skills, and unsafe handles.

**Rationale:** Bad manifests must fail before prompt construction and before resource-backed commands are advertised.

### 4. Progressive disclosure remains the prompt model

**Choice:** The manifest summary can enter the prompt, but full body content enters context only through an explicit load decision or trusted host policy.

**Rationale:** This preserves context and prevents untrusted project guidance from silently steering the runtime.

## Risks / Trade-offs

- Hermes category names may not fit every host. Treat category as metadata, not authorization.
- Related-skill links can be partial in user-local stores. Diagnostics should warn or mark unresolved links without forcing global installation.
- Description-length limits should be configurable at the host edge but have safe default bounds in Animus.
