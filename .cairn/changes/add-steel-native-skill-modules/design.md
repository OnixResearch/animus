# Design: Steel-native skill modules

## Context

Animus skill contracts separate manifest metadata from instruction-body loading. The model-facing RLM kernel can retain Steel helpers, but no contract binds those helpers to reviewed skill sources.

A programmatic skill must remain data until the host selects, validates, and loads it.

## Success Contract

A selected trusted skill can provide a bounded Steel module with named exports. The loader validates its source identity, imports, dependency graph, capability ceiling, and policy before evaluation.

The module can call host operations only through the same typed bridge as model-written code. It cannot load native extensions or discover files and packages.

A manifest with an arbitrary script path, automatic execution, direct FFI, or ambient package lookup is false completion.

## Decisions

### 1. Add a separate module descriptor

The existing skill manifest remains the discovery and guidance owner. An optional module descriptor adds:

- module schema and language cohort
- exact manifest and frozen-source identities
- private bounded Steel source body
- named exports
- exact module dependencies
- requested bridge capability labels
- evaluation bounds and trust state

### 2. Use reviewed Steel source only

The first cohort accepts one complete Steel source unit from an explicit host source service. It does not accept native modules, dynamic libraries, arbitrary include paths, or package-manager references.

Source acquisition remains host-owned and follows the accepted skill-source lifecycle.

### 3. Resolve dependencies before kernel load

A pure resolver checks exact identities, acyclic order, export conflicts, duplicate modules, version conflicts, capability ceilings, and cumulative bounds.

The loader evaluates modules in the resolved order inside staged kernel state.

### 4. Use the normal host bridge

Skill functions can call only bridge helpers that the effective skill and session capabilities admit. A skill cannot register a new host operation class.

Model-written code and skill code receive the same result, cancellation, and receipt rules.

### 5. Load only at safe points

Module selection occurs before a cell starts or through an explicit kernel transition. A failed load does not partially change the kernel.

Committed module identities enter the private kernel journal. Restore replays the exact source with effects disabled.

## Authority Flow

```text
validated skill manifest + frozen source
  -> pure module admission and dependency resolution
  -> staged Steel module evaluation
  -> explicit kernel module commit
  -> model calls named exports
  -> admitted resumable host bridge when required
```

## Alternatives Considered

### Put executable code in the existing manifest body

Rejected because instruction disclosure and program loading have different trust, capability, and failure rules.

### Allow arbitrary Steel native modules

Rejected because native modules can introduce undeclared host authority.

### Auto-load every discovered skill

Rejected because discovery does not grant activation or capability authority.

## Risks and Controls

- Skill source can be malicious. Trust, capability, source identity, bounds, and negative fixtures fail closed.
- Dependencies can conflict or cycle. Pure deterministic resolution rejects them before evaluation.
- A failed module can poison kernel state. Staged evaluation commits only after complete module success.
- Source and helper values can leak. Private journal and metadata-only evidence remain separate.

## Non-Claims

This design does not establish skill correctness, source authenticity beyond supplied provenance, native extension safety, sandboxing, automatic updates, task success, or production readiness.
