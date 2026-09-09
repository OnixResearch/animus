# Design

## Boundary

Keep `ScopedAuthority` privately constructed inside the existing higher-ranked callback. Its `holder()` accessor already supplies the borrowed original authorization required by UCAN. Adopt the published Basalt dependency and call its re-exported UCAN recheck API from consumer-facing tests before adding any new public method.

The caller supplies the same token and explicit time, key, proof, revocation, caveat, profile, and limit facts. The returned observation borrows the original authorization and cannot substitute for dispatch authority. No replay port is supplied to a recheck.

## Verification

Run the existing authority tests before changing pins. Add direct and delegated callback observations, repeated observations without extra replay or commit, expired/revoked/substituted input denial, and a compile-fail callback-result escape control. Preserve existing gate and commit denial tests and exact time bounds.

Run the connected authority rail, strict Clippy, default/no-default isolation, and workspace compilation. Record source-bound evidence before lifecycle closure. Synchronization, archive, and publication are separate observed operations.

## Non-claims

An observation does not refresh Animus or Basalt policy, authenticate backend freshness, enforce monotonic successive observations, establish persistent custody, or authorize atomic release. No issuer grants, training, or promotion are in scope.
