# Proposal: Add router policy and conformance contracts

## Why

Router recipes, roles, output contracts, bounds, and fallback policies are validated in Rust, but the policy that says *which* recipe families, role combinations, bound floors/ceilings, and backend allowlists are permitted is not Nickel-authored. The repo rule makes Nickel the source of truth for policy/config/schema, and the existing `cairn/adapter-conformance-index.ncl` already models adapter families. Router recipes deserve the same treatment: a reviewed Nickel contract defining allowed families, valid role combos per family, bounds floors/ceilings, fallback policies, and backend allowlists, plus a conformance-index fixture so drift is caught before runtime use.

## What Changes

- Add a Nickel contract `router-recipe-policy.ncl` defining allowed route families, the valid role set per family, output-contract compatibility per role, bounds floors and ceilings (max candidates, parallelism, quorum, synthesis/repair attempts), allowed fallback policies, and backend allowlist labels.
- Add a router conformance-index fixture under `cairn/` mirroring `adapter-conformance-index.ncl`, with positive and negative recipe/role/bounds/fallback fixtures.
- Add a `check-router-policy` script and flake check so Nickel source and generated fixtures stay in sync, rejecting unknown families, invalid role combos, out-of-range bounds, and disallowed fallback.
- Add positive and negative fixtures for each family and each policy violation.

## Impact

- **Files**: `cairn-policy/` or `config/` Nickel source, `cairn/router-conformance-index.ncl` and fixtures, `scripts/cairn/check-router-policy`, `flake.nix` check, docs, and Cairn evidence.
- **Testing**: positive policy fixtures per family; negative fixtures for unknown family, invalid role combo, out-of-range bounds, disallowed fallback, and unknown backend.
- **Non-claims**: this does not execute Nickel at runtime, add live provider checks, or change Rust recipe validation; it adds reviewed policy and drift fixtures consumed at build/check time.
