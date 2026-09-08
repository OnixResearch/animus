# Scoped live Basalt authority

## Why

The guarded effect currently receives no arguments. Consumers cannot inspect the
actual live holder binding and effective token time bounds inside that callback.
Metadata receipts cannot replace the live authorization.

## What Changes

Add a non-default callback API with an owner-created borrowed authority view.
Preserve the existing API through delegation to the same dispatch implementation.
Preserve raw verification, gate intersection, commit ordering, and one replay
admission. Add positive, denial, redaction, and compile-fail controls.

## Impact

Only animus-std-basalt-authority and its documentation and focused rail change.
No new dependency, authority discovery, credential creation, continuing-authority
recheck, resource custody, consumer activation, or receipt schema change occurs.
