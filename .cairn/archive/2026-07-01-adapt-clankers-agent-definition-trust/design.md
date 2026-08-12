# Design: Adapt Clankers agent definition trust for Animus

## Context

Named agent definitions are useful for reviewers, researchers, planners, and delegated workers. They are also risky because they bundle prompt text, model preferences, and tool access. Clankers distinguishes user and project sources; Animus should keep that trust boundary and make authority explicit through typed manifests and policy checks.

## Decisions

### 1. Manifests are typed data

**Choice:** Agent definitions use typed Nickel or Preserves-compatible manifests with name, description, model preference, prompt fragments, capability ceilings, toolset limits, source scope, and manifest identity.

**Rationale:** Frontmatter Markdown is convenient but weakly typed. Animus should validate manifest structure before activation.

### 2. Source scope affects trust

**Choice:** User/global and explicit host-provided definitions may be trusted according to host policy. Project/workspace definitions are untrusted by default and require confirmation or admission before use.

**Rationale:** Project files can be supplied by an untrusted repository. They must not silently alter model/tool behavior.

### 3. Capabilities are ceilings, not grants

**Choice:** A manifest may request or cap tools, model roles, memories, context references, or workflow abilities, but final authority still comes from capability policy, UCAN grants, and host confirmation.

**Rationale:** Agent definitions should narrow or describe behavior; they must not bypass the runtime authority model.

### 4. Activation records safe evidence

**Choice:** Activation receipts record definition name, source scope, trust/admission status, capability ceilings, selected model role label, manifest hash, policy hash, and redaction markers. Raw prompt bodies, project paths, credentials, and provider payloads are excluded.

**Rationale:** Operators need to know which persona influenced a turn without leaking prompt text or local paths.

### 5. Compatibility importers are optional adapters

**Choice:** A future Clankers/Markdown/YAML importer may map into Animus manifests, but canonical runtime contracts use Animus-owned typed DTOs.

**Rationale:** Animus should not inherit Clankers' file format as its authority boundary.

## Risks / Trade-offs

- Overly strict manifests can make small persona snippets cumbersome. Keep required fields minimal but typed.
- Trust prompts can become noisy. Hosts should cache explicit admissions by manifest hash and policy where appropriate.
- Agent definitions overlap agent teams. This change only defines manifest trust and activation, not worker orchestration.
