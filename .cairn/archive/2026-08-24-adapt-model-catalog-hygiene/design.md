## Context

Animus resolves exact model targets through shell-owned registry data and typed runtime profiles. Repo-owned configuration uses Nickel as the source of truth. Optional user additions need deterministic precedence without granting hidden credential or billing authority.

`digitallyinduced/haskell-agent` uses stable model IDs, replace-by-ID user merge, environment-variable credential references, startup diagnostics, reserved built-in names, and manual-only custom connections.

## Goals

- Define one versioned typed catalog contract.
- Merge shipped and user entries deterministically by stable ID.
- Keep secret values outside catalog content and diagnostics.
- Fail loudly on malformed or ambiguous input.
- Keep custom connections outside automatic fallback.

## Non-goals

- Live model discovery or endpoint probing.
- Credential loading in pure catalog code.
- Model or provider authenticity claims.
- Silent compatibility repair for malformed catalogs.

## Decisions

### Decision: Nickel owns catalog contracts

**Choice:** Repo-owned defaults and catalog contracts are typed Nickel. Runtime hosts consume a checked deterministic projection and parse optional user content into the same DTO contract.

**Rationale:** One checked schema prevents independent JSON-shaped interpretations and matches Animus configuration ownership.

### Decision: Stable IDs own merge precedence

**Choice:** A user entry with the same stable model ID replaces the shipped entry. A new ID appends deterministically. Duplicate IDs inside one source, reserved built-in connection names, and unsupported catalog versions fail closed.

**Rationale:** Explicit replacement avoids order-dependent partial merges.

### Decision: Catalogs carry credential references only

**Choice:** An authenticated connection names the environment variable that contains its key. Catalogs, receipts, and diagnostics never contain the resolved secret value.

**Rationale:** Provider selection needs a credential reference, not credential bytes.

### Decision: Custom connections are manual-only

**Choice:** Operator-added connections can be selected by exact target identity. They are excluded from automatic retry and billing fallback candidate sets.

**Rationale:** The runtime cannot infer the spend, trust, or compatibility policy of an arbitrary endpoint.

### Decision: Invalid input fails loudly and safely

**Choice:** Startup reports the catalog source and invalid field class. It does not ignore the user catalog, continue with partial entries, or disclose secret values.

**Rationale:** Silent fallback to shipped defaults hides operator mistakes and can select the wrong provider.

## Dependency Direction

```text
Nickel catalog source -- checked projection -- pure merge and validation
                                                ^
                                                |
                                   provider-registry shell and credential host
```

Pure catalog code does not read files, environment variables, credentials, networks, or clocks.

## Adversarial Audit

The audit must try to show that Animus:

- merges duplicate IDs by file order;
- accepts an unsupported catalog version;
- permits a user catalog to redefine a reserved connection name;
- stores or prints a secret value;
- ignores a malformed user catalog and continues silently;
- includes a custom connection in automatic fallback;
- claims endpoint or model authenticity from catalog admission.

Any successful counterexample blocks closeout.

## Risks and Trade-offs

- Replace-by-ID semantics discard the shipped entry as one unit. That makes review clearer than field-wise implicit merge.
- Manual-only custom connections reduce automatic availability. They prevent unknown billing and trust transitions.
- Startup failure can block use after an invalid edit. The diagnostic must be exact enough for repair.

## Non-Claims

This change does not prove endpoint availability, provider identity, model identity, credential validity, billing correctness, transport safety, or schema compatibility beyond the selected catalog cohort.
