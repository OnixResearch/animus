# Design: Add router policy and conformance contracts

## Context

Rust validates individual recipe fields, but the *permitted* set of recipes, role combos, and bound ranges is policy that should be Nickel-authored and reviewed, matching the repo's config-source-of-truth rule and the existing adapter-conformance-index pattern. Without it, "is this recipe combination allowed?" is answered only by ad hoc Rust checks that can drift from intent.

## Decisions

### 1. Nickel owns the router recipe policy

**Choice:** Add `router-recipe-policy.ncl` with typed contracts for allowed `RouteFamily` values, the valid `Role` set per family, `OutputContract` compatibility per role, bounds floors/ceilings, allowed `FallbackPolicy` labels, and backend allowlist labels. Runtime consumes checked-in generated policy JSON; no runtime Nickel execution.

**Rationale:** Matches the repo rule that Nickel owns policy/config/schema and the cairn skill's "Nickel owns policy/config/schema; runtime Rust consumes checked-in generated policy JSON."

### 2. A conformance-index fixture mirrors the adapter pattern

**Choice:** Add `cairn/router-conformance-index.ncl` with positive fixtures (one valid recipe per family) and negative fixtures (unknown family, invalid role combo, out-of-range bound, disallowed fallback, unknown backend), mirroring `cairn/adapter-conformance-index.ncl`.

**Rationale:** Reuses the proven conformance-index rail so router policy drift is caught the same way adapter drift is.

### 3. A check script and flake check enforce sync

**Choice:** Add `scripts/cairn/check-router-policy` and a `checks.x86_64-linux.router-policy` flake check that validate the Nickel source, regenerate fixtures, and fail on drift or policy violations.

**Rationale:** One command, run in pre-commit and `nix flake check`, keeps policy and fixtures in sync without manual regeneration.

### 4. Policy is build-time, not runtime

**Choice:** The Rust router consumes generated policy JSON at build/check time only; runtime recipe validation stays in Rust and references policy-defined constants. No Nickel evaluator runs at runtime.

**Rationale:** Matches the cairn skill's runtime-Nickel prohibition and keeps the no-std router free of Nickel dependencies.

## Risks / Trade-offs

- Policy and Rust constants can drift if the generator is skipped. The flake check and pre-commit hook mitigate this; the check fails closed on drift.
- Over-constraining policy can block valid future recipes. Keep the contract to families/roles/bounds/fallback/backends and leave recipe *content* to Rust validation.
- Adding a check expands `nix flake check` time. Keep the fixture set small and focused.
