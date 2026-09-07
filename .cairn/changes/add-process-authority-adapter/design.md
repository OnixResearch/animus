# Process authority design

## Ownership

The generic no-std authority contracts remain unchanged and contain no Basalt or
UCAN type. The optional std adapter translates Exec/process-execute/process/execute
to the owner's ProcessExecute. The owner's public binder validates the exact
onix://process-execution/ resource and complete 64-lowercase-hex context reference.
The adapter uses the public operation-profile API, not copied private hashes.

## Policy

Document mappings retain the unchanged default policy identity. Exec requires the
separate published execution-policy byte identity. No arbitrary policy reference
becomes admitted. Caller-owned policy objects and services must match the selected
reviewed artifact. This mapping does not authenticate policy bytes or the context
preimage; the caller must authenticate them and retain custody through dispatch.

## Cohort

Both existing Basalt dependencies (optional root reference facade and optional std
adapter) select the same reviewed published revision. The root reference facade
gets explicit compatibility checks; this is not an unrelated workspace upgrade.
Cargo and Nix generate locks. No manual lock editing, crypto downgrade or sibling
product dependency. Vendor-source hash and public policy identity are measured
from the published source, with provenance recorded. Historical archives stay intact.

## Effect order

Keep raw holder verification, revocation and final replay, live grant lifetime,
every existing local gate, safe decision commit, and one effect callback unchanged.
Tests use test-key signatures and counted in-memory effects, not a real process.
Denial, failed commit and replay cannot dispatch. Receipts remain metadata-only.

## Verification

Baseline adapter tests precede edits. Repeat document tests on the new owner before
adding execution mapping. Positive execution plus purpose, policy, context, replay,
revocation, local-gate and commit negatives cover the new boundary. Run the focused
owner rail, default/reference/feature isolation, configured Nickel freshness and
committed lifecycle gates. No Neural completion or activation is claimed.
