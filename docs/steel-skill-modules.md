# Steel skill modules

Animus can load reviewed Steel helpers into the model-facing kernel. This support is non-default. Enable it with `steel-skill-modules`.

## Boundary

A module descriptor binds these values:

- one validated skill-manifest identity;
- one frozen skill-source identity;
- one complete private Steel source unit and its BLAKE3 identity;
- one reviewed version and language cohort;
- named exports and exact dependencies;
- requested existing bridge capability labels;
- explicit trust and evaluation bounds.

The descriptor is data. Validation does not evaluate its source and does not grant authority.

## Admission and dependencies

The host selects modules explicitly. The resolver checks exact identities, versions, dependencies, exports, trust, bounds, and capability ceilings before evaluation.

Dependency order is deterministic. The resolver rejects missing modules, identity drift, cycles, duplicate modules, and export conflicts.

The source cohort accepts a complete Steel source unit. It rejects imports, native modules, FFI, dynamic libraries, arbitrary paths, package managers, ambient lookup, dynamic evaluation, and forged bridge requests.

Source can call only declared bridge helper names. The loader also checks each requested capability against the current kernel policy.

## Kernel lifecycle

The loader builds a candidate sandboxed engine. It evaluates modules in dependency order, with a timeout, and checks every named export. It commits the candidate only after all modules pass.

A failed load leaves the prior kernel and module set unchanged. Compaction preserves the committed engine.

The private kernel journal stores exact module source and identity data. Restore revalidates every descriptor and dependency before effect-disabled replay. Safe receipts expose only identities, counts, omission markers, and non-claims.

## Authority

A requested bridge capability must be present in all of these current ceilings:

1. session;
2. model profile;
3. selected skill;
4. current host.

Modules cannot register new host operation classes. Host effects still use the resumable Steel bridge and its normal correlation, cancellation, timeout, and receipt rules.

## Selection

`animus_runtime::select_steel_skill_modules` admits candidates only when the composed model invocation plan contains the matching skill and a selected progressive-disclosure body.

No automatic activation occurs. Discovery, a manifest, or a frozen source does not load executable code.

## Verification

Run:

```text
scripts/cairn/check-steel-skill-modules --root .
```

The rail covers descriptor and policy parity, dependency order, atomic loading, helper reuse, compaction, durable restore, no-std and Wasm isolation, Clippy, and root facade wiring.

## Non-claims

This support does not prove source authenticity, skill correctness, native extension safety, sandboxing, task success, or production readiness.
