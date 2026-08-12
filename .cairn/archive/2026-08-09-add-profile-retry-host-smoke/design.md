# Design: Profile-bound provider-retry host smoke

## Context

`animus-std` loads generated model profiles and retry policy from explicit paths. `animus-runtime` resolves exact profiles, plans same-target retry, gates timer observations, and projects metadata-only receipts.

The current packaged smokes do not compose these seams. The new smoke must keep all decisions pure and use deterministic injected effects.

## Success Contract

The change is complete when one non-default binary loads both generated policies, resolves the requested exact target, binds that profile resolution into one retry invocation, and dispatches one admitted retry through fake services.

Completion evidence includes profile and retry identities, one timer call, one model call, one metadata-only output, a source-bound focused receipt with current-head metadata when available, positive and negative fixtures, the focused script, the focused Nix check, and Cairn gates.

Compilation, isolated contract tests, raw task markers, or a retry decision without admitted dispatch are false-completion cases.

## Decisions

### 1. Add a separate non-default smoke feature

**Choice:** Add `profile-retry-smoke`, `animus-profile-retry-smoke`, and a focused library module to `animus-std`.

**Rationale:** A separate feature keeps existing smoke claims stable and does not add provider behavior to default builds.

### 2. Require explicit policy paths and target inputs

**Choice:** Require profile policy, retry policy, provider, model, prompt, and failure class as exact arguments.

**Rationale:** Explicit inputs avoid ambient discovery and let negative fixtures replace either policy independently.

### 3. Bind the resolved profile into the retry invocation

**Choice:** Use the exact profile resolution hash as the retry invocation profile-plan identity. Bind all remaining request identities before planning retry.

**Rationale:** The retry core already rejects changed invocation identities. This composition proves that the loaded profile participates in the exact same-target gate.

### 4. Keep fake effects thin and deterministic

**Choice:** Use one injected timer and one injected model adapter. The pure core produces the retry decision, gate, and receipts before the shell renders output.

**Rationale:** The smoke proves host composition without credentials, clocks, sleeps, provider discovery, or network access.

### 5. Emit metadata-only line records

**Choice:** Render stable status classes, counts, profile and retry hashes, and explicit non-claims. Omit raw prompt, response, policy content, paths, and diagnostics.

**Rationale:** Operators can index the result without storing sensitive model content or private host paths.

## Portfolio Search Record

The exact goal is a packaged deterministic host path that binds loaded model profiles to admitted provider retry. Completion requires exact identity binding, one gated retry dispatch, safe receipts, and denial coverage.

One agent performed the following serial lenses, so the passes are correlated.

- **Separate deterministic smoke**: selected. It closes both roadmap gaps without changing existing smoke claims.
- **Extend the durable coding-session smoke**: rejected as broader than required because it adds storage and authority-seal state to retry validation.
- **Extend the live OpenAI smoke**: rejected because credentials and network behavior make deterministic closeout unavailable.
- **Only add another unit test**: falsified because it does not provide a packaged operator path or focused evidence rail.

The adversarial audit targets profile substitution, request drift, terminal failures, cancelled waits, missing services, malformed policies, raw prompt or response leaks, and unsupported live-provider claims.

The retrieval budget covers existing profile loaders, retry shells, packaged-smoke patterns, the dogfood roadmap, Nix checks, and lifecycle artifacts. The validation budget covers focused Cargo tests, the new script and Nix check, profile and retry rails, Cairn validation, and Cairn gates.

## Risks / Trade-offs

- The smoke uses deterministic fake timer and model services.
- One retry does not prove provider reliability or production retry behavior.
- The smoke resolves an exact profile but does not claim broader router or fallback behavior.
