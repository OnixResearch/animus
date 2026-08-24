# Adapt Secret Path Entry Specification

## Purpose

Defines the `adapt-secret-path-entry` capability.

## Requirements

### Requirement: Interactive secret entry returns only a path

r[animus.secret.path] An approved interactive secret-entry request MUST read the value through masked terminal input and MUST return only a path to the model. The result MUST NOT contain the secret value.

#### Scenario: Masked entry returns a path

r[animus.secret.path.masked]
- GIVEN an approved root session requests a secret
- WHEN the user enters the value through the masked prompt
- THEN the result MUST contain only the admitted secret-file path
- AND the conversation record MUST NOT contain the value

#### Scenario: Secret value leakage fails

r[animus.secret.path.value_leak]
- GIVEN an implementation places secret bytes in a result, argument, transcript, or command text
- WHEN the secret-entry record is inspected
- THEN the inspection MUST report the leak

### Requirement: Secret files are private and session-local

r[animus.secret.file] The secret host MUST create each secret file below the admitted session scratch directory with owner-only permissions. It MUST reject path escape, symlink, and preexisting-target cases. It MUST remove tracked files when the tool runtime closes.

#### Scenario: Private file is admitted

r[animus.secret.file.private]
- GIVEN an approved scratch directory and a new secret entry
- WHEN the secret host publishes the file
- THEN the file MUST remain below the scratch directory
- AND its permissions MUST admit only the owner

#### Scenario: Runtime close removes the file

r[animus.secret.file.cleanup]
- GIVEN a secret file remains tracked by the tool runtime
- WHEN the runtime closes
- THEN the host MUST remove the file or report the exact cleanup failure

### Requirement: Interactive authority is explicit

r[animus.secret.scope] Secret entry MUST require an approved interactive root-session terminal capability. A headless or child session MUST fail closed unless a separate authority contract explicitly admits secret entry.

#### Scenario: Approved root session can request a secret

r[animus.secret.scope.root]
- GIVEN a root session has the terminal and secret-entry capabilities
- WHEN it requests a secret
- THEN the request MAY proceed through the masked path-only flow

#### Scenario: Headless request fails

r[animus.secret.scope.headless]
- GIVEN a headless or child session has no explicit secret-entry authority
- WHEN it requests a secret
- THEN the request MUST fail closed without prompting or returning a value

### Requirement: Secret-entry verification is complete

r[animus.secret.verification] Positive, negative, and boundary fixtures MUST cover masked input, value non-disclosure, path admission, file permissions, cleanup, root-session authority, and headless denial.

#### Scenario: Secret-entry matrix passes

r[animus.secret.verification.matrix]
- GIVEN the secret-entry rails, fixtures, and documentation are complete
- WHEN the focused validation runs
- THEN conforming inputs MUST behave as declared
- AND every value-leak, path-escape, permission, cleanup, or authority violation MUST fail as declared
