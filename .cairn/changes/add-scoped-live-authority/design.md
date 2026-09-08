# Design

The std shell owns the live callback. Generic no-std adapter contracts remain
free of Basalt types. Existing deterministic mapping and policy stay unchanged.

A private-construction ScopedAuthority view borrows the actual live holder
identity, generic decision, safe committed receipt, and validated commit identity.
The view exposes only borrowed accessors. It has no Clone, serde, or Debug surface.
Its callback uses a higher-ranked lifetime so the view and its borrowed contents
cannot escape through the result. Ordinary copied metadata remains evidence,
never a new capability.

Both public dispatch APIs use one verification, gate intersection, and commit
path. No wrapper performs another replay reservation. Errors retain existing
receipt behavior. An effect error does not become successful execution.

The view denotes the admitted invocation at verification time, not continuous
revocation freshness or resource protection. Hosts own later rechecks, cancellation,
limits, custody, persistence, and effect meaning. No new ambient services exist.

Validation covers direct and delegated authorization, actual effective time bounds,
callback/commit order, single replay admission, provider/gate/commit denials,
non-escape, private construction, and existing API compatibility. The full focused
rail retains default, no-default, Wasm, Nickel, catalog, and connected checks.
Rollback removes the additive API and restores the old internal delegation.
