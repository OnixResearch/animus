# Design: Adapt Zerostack prompt mode profiles for Animus

## Context

Zerostack's prompt profiles are useful because they make task posture explicit at runtime. Animus should adopt the profile/directive/chain concepts while keeping model-visible text assembled by the Animus prompt assembly core with source attribution and policy checks.

## Decisions

### 1. Profiles are descriptors, not blobs

**Choice:** Represent each prompt profile as typed metadata plus bounded body text and source identity.

**Rationale:** Profiles influence model behavior and permission posture, so their authority and provenance must be visible before prompt assembly.

### 2. Directives parse before prompt inclusion

**Choice:** Parse mode and chain directives into metadata, strip directive syntax from model-visible body text, and validate directives against policy.

**Rationale:** This preserves the ergonomic Zerostack pattern without letting hidden text mutate permission state.

### 3. Chain transitions are continuation plans

**Choice:** Model chain-of-prompts as explicit transition plans with source phase, target phase, user decision, and optional extra instruction.

**Rationale:** A chain should be auditable and cancellable rather than hidden control flow.

### 4. Prompt mode cannot bypass permission resolution

**Choice:** Prompt profile mode requests are inputs to permission policy, not direct authority grants.

**Rationale:** A profile such as ask/read-only can restrict behavior, but no profile can grant missing authority.

## Risks

- Prompt profiles can become vendor-specific skills. Keep the descriptor format small and Animus-owned.
- Chain transitions can surprise users. Require visible host decisions or policy-enabled automatic transitions.
