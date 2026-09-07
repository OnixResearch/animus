# Exact process authority adapter

## Why

The published Basalt owner now exposes a distinct process execution action and an
explicit opt-in policy. Animus still maps only document actions and pins the old
owner. Document authority must never stand in for execution authority.

## Scope

Qualify published Basalt `27547d7870fdb5ca21208d583b58a518ba820b72` in the non-default
adapter and the existing optional root Basalt reference facade. Keep all unrelated
pins and default dependency isolation. Add one closed Exec mapping and admit only
its reviewed execution policy identity. Reuse the owner's binder for resource shape.

No native process implementation, context authentication, immutable custody,
resource enforcement, signing service, credential discovery or production activation.
The caller retains current authority services and complete invocation-context custody.
