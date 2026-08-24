## Phase 1: Path-only result

- [x] [serial] r[animus.secret.path] Add the authority-free request and path-only result DTOs.
- [x] [parallel] r[animus.secret.path.masked] Add a positive fixture that returns a path after masked input.
- [x] [parallel] r[animus.secret.path.value_leak] Add a negative fixture that rejects secret bytes in the result or transcript.

## Phase 2: Private file lifetime

- [x] [serial] r[animus.secret.file] Add the session-local owner-only file host and tracked cleanup plan.
- [x] [parallel] r[animus.secret.file.private] Add positive and negative mode, escape, symlink, and existing-target fixtures.
- [x] [parallel] r[animus.secret.file.cleanup] Add close and earlier-consumption cleanup fixtures.

## Phase 3: Interactive authority

- [x] [serial] r[animus.secret.scope] Compose the root-session terminal capability with the secret host.
- [x] [parallel] r[animus.secret.scope.root] Add a positive approved-root-session fixture.
- [x] [parallel] r[animus.secret.scope.headless] Add negative headless and child-session fixtures.

## Phase 4: Verification

- [x] [serial] r[animus.secret.verification] Add the focused positive, negative, and boundary matrix and run the rails.
