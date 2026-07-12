# Ornith trust-boundary monitor

Animus adapts Ornith's outer trust-boundary defense as immutable typed policy data plus a deterministic monitor. Scaffolds, Steel scripts, model decisions, and tool requests may shape work inside the boundary, but they cannot mutate verifier authority, sandbox limits, withheld materials, approval requirements, or evidence acceptance rules.

The reusable runtime surface lives in `animus-runtime::trust_boundary` and is re-exported by `animus-runtime`. Hosts own filesystem access, sandbox enforcement, verifier execution, approval UX, provider calls, optional judge execution, and receipt storage.

## Boundary descriptor

A trust-boundary descriptor records:

- sanctioned tool labels;
- withheld path/oracle labels;
- verifier artifact identity and BLAKE3 hash;
- sandbox profile identity;
- approval-required action classes;
- evidence acceptance policy;
- reviewed boundary policy hash;
- whether an inner runtime path attempted boundary mutation.

Descriptors use safe labels and hashes instead of raw hidden paths, verifier bodies, prompts, provider payloads, credentials, or full path dumps.

## Deterministic monitor

The monitor evaluates planned or observed action descriptors before evidence can count as success. Exact denials include withheld-path reads, verifier modification, unsanctioned tools, sandbox weakening, approval suppression, evidence-policy bypass, oracle copying, hardcoded expected outputs, visible-test overfitting, and allowed-tool intent gaming.

Monitor denials are final. Optional judge output is reject-only: a judge may veto suspicious trajectories, but judge approval cannot override deterministic monitor denial, grant a tool, weaken sandboxing, suppress approval, or mark denied evidence successful. When represented as a semantic agent role, that judge additionally requires a rubric-bound `animus_runtime::agent_roles` assignment, explicit separation, immutable subject/evidence identities, and its own judgment or veto capability; the router/display label alone has no influence.

## Receipt safety

Boundary violation receipts carry only safe metadata such as policy hash, issue code, sanitized labels, counts, and redaction status. Receipt validation rejects hidden paths, verifier bodies, prompts, tool bodies, provider payloads, credential-looking values, and full path dumps.

## Focused verification

Run the focused tests with:

```sh
nix develop -c cargo test -p animus-runtime trust_boundary
```

The tests cover valid and invalid boundary descriptors, exact monitor denials, judge reject-only binding, anti-gaming fixtures, and receipt leak rejection without provider calls or hidden tests.
