## Phase 1: Versioned catalog and merge

- [x] [serial] r[animus.catalog.version] Add the typed Nickel catalog contract and checked runtime DTO projection.
- [x] [parallel] r[animus.catalog.version.merge] Add stable replace-by-ID and deterministic append fixtures.
- [x] [parallel] r[animus.catalog.version.duplicate] Add negative duplicate-ID, reserved-name, and unsupported-version fixtures.

## Phase 2: Credential references

- [x] [serial] r[animus.catalog.secret] Restrict connection credentials to environment-variable references.
- [x] [parallel] r[animus.catalog.secret.reference] Add a positive credential-reference fixture.
- [x] [parallel] r[animus.catalog.secret.value] Add a negative literal-secret and diagnostic-redaction fixture.

## Phase 3: Startup and selection policy

- [x] [serial] r[animus.catalog.failure] Add fail-loud catalog parsing with source and field-class diagnostics.
- [x] [parallel] r[animus.catalog.failure.field] Add malformed-field and no-silent-ignore fixtures.
- [x] [serial] r[animus.catalog.selection] Exclude custom connections from automatic retry and billing fallback candidates.
- [x] [parallel] r[animus.catalog.selection.manual] Add a positive exact manual-selection fixture.
- [x] [parallel] r[animus.catalog.selection.fallback] Add a negative automatic custom-connection fallback fixture.

## Phase 4: Verification

- [x] [serial] r[animus.catalog.verification] Add the focused positive, negative, and boundary matrix and run the rails.
