# Versioned Model Catalog Hygiene

## ADDED Requirements

### Requirement: Model catalogs are versioned and merge by stable ID

r[animus.catalog.version] The model catalog MUST have a supported version and stable model IDs. A user entry with an existing ID MUST replace the shipped entry as one unit. A new ID MUST append deterministically. Duplicate IDs within one source, reserved connection names, and unsupported versions MUST fail closed.

#### Scenario: User entry replaces one shipped entry

r[animus.catalog.version.merge]
- GIVEN a shipped entry and a valid user entry have the same stable ID
- WHEN the catalogs merge
- THEN the user entry MUST replace the shipped entry as one unit
- AND unrelated entries MUST retain deterministic order

#### Scenario: Ambiguous catalog fails

r[animus.catalog.version.duplicate]
- GIVEN a source has duplicate IDs, a reserved connection name, or an unsupported version
- WHEN catalog admission runs
- THEN admission MUST fail without producing a partial catalog

### Requirement: Catalogs store credential references, not secret values

r[animus.catalog.secret] An authenticated custom connection MUST name the environment variable that contains its credential. Catalog content, receipts, and diagnostics MUST NOT contain the resolved secret value.

#### Scenario: Credential reference is admitted

r[animus.catalog.secret.reference]
- GIVEN a custom connection names an environment-variable credential reference
- WHEN catalog admission runs
- THEN the reference MAY be retained without reading the credential value

#### Scenario: Literal secret fails

r[animus.catalog.secret.value]
- GIVEN a catalog contains a literal credential value
- WHEN catalog admission runs
- THEN admission MUST fail
- AND the diagnostic MUST NOT repeat the value

### Requirement: Malformed catalogs fail loudly

r[animus.catalog.failure] A malformed user catalog MUST be reported at startup with the catalog source and invalid field class. Animus MUST NOT silently ignore the user catalog or continue with partial entries.

#### Scenario: Invalid field is named

r[animus.catalog.failure.field]
- GIVEN a user catalog contains an invalid field
- WHEN startup parses the catalog
- THEN startup MUST fail and name the source and field class
- AND MUST NOT disclose credential values

### Requirement: Custom connections are manual-only

r[animus.catalog.selection] An operator-added connection MUST be selected only by exact manual target identity. It MUST NOT enter automatic retry or billing fallback candidate sets.

#### Scenario: Exact manual selection succeeds

r[animus.catalog.selection.manual]
- GIVEN a valid custom connection and model ID exist
- WHEN the operator selects that exact target
- THEN resolution MAY admit the target through its declared profile

#### Scenario: Automatic custom fallback fails

r[animus.catalog.selection.fallback]
- GIVEN an automatic retry or billing fallback is selecting candidates
- WHEN a custom connection is present
- THEN the custom connection MUST remain excluded

### Requirement: Model-catalog verification is complete

r[animus.catalog.verification] Positive, negative, and boundary fixtures MUST cover catalog versions, replace-by-ID merge, duplicates, reserved names, credential references, malformed input, manual selection, and automatic-fallback exclusion.

#### Scenario: Model-catalog matrix passes

r[animus.catalog.verification.matrix]
- GIVEN the catalog rails, fixtures, and documentation are complete
- WHEN the focused validation runs
- THEN conforming catalogs MUST behave as declared
- AND every version, ambiguity, secret, malformed-input, or fallback violation MUST fail as declared
