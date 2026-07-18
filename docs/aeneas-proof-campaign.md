# Aeneas proof campaign recipe

Animus defines a metadata-only campaign recipe for coordinating bounded Aeneas
proof work without embedding a proof runtime. The canonical source is
[`config/aeneas-proof-campaign.ncl`](../config/aeneas-proof-campaign.ncl); its
runtime export is
[`config/generated/aeneas-proof-campaign.json`](../config/generated/aeneas-proof-campaign.json).
Runtime code never evaluates Nickel. Hosts load the generated profile, resolve
existing recipes, roles, model profiles, tool catalogs, injected services, and
safe-receipt policy, then pass already-loaded metadata into the pure contracts
in `animus_runtime::aeneas_proof_campaign`. This campaign proves selected
Rust-derived statements through external Aeneas/Lean providers; it is separate
from the optional [Lean 4 loop fabric](lean4-loop-fabric.md), whose maintained
theorems cover only its repository-owned encoded fabric kernel and do not
certify Aeneas campaigns or Rust/Lean semantic equivalence.

## Composition and authority

The profile composes these existing Animus contracts rather than creating a
second loop or provider runtime:

- `goose.recipe.workflow` for stage orchestration;
- `compat-default` for the admitted model-runtime profile;
- `animus.tool-catalog.v1` for capability vocabulary;
- `animus.safe-receipt.v1` for metadata projection and redaction;
- host-owned `octet-aeneas-generation`, `octet-aeneas-proof`, and
  `octet-artifact-verify` services;
- separate `host.authoritative-completion` admission.

The campaign has no authority of its own. Even `proved` is only a candidate
input to the unresolved host completion gate. Blocked, counterexample, invalid
statement, invalid model, budget exhaustion, tool failure, denial,
cancellation, and abstention remain distinct terminal outcomes.

## Specialized assignments

| Assignment | Existing class | Mutation | Ceiling |
|---|---|---|---|
| statement author | implementation | candidate statement only | source read, statement write |
| statement reviewer | review | none | source read, statement review |
| prover | implementation | maintained theorem only | source read, theorem write, proof request |
| verifier | verification | none | artifact read, verification request |

Statement review and final verification require distinct assignment, actor,
run/thread, context, workspace, model-profile, and mutation-lineage identities.
The statement and generated model remain immutable across each handoff, the
producer service differs from the verifier/check service, and the reviewer or
verifier cannot mutate the subject.

## Statement and artifact contracts

A proof request is admitted only after a reviewer approves the exact statement
BLAKE3, requirement, claim class, source BLAKE3, generated-model BLAKE3, and
type/overflow-model BLAKE3. The approval rejects vacuity, `sorry`/`admit`, and
an incomplete declaration of model inputs, assumptions, axioms, and opaque or
external definitions.

Generated models and generation manifests are immutable provider outputs with
fresh generation identities. Candidate statements are mutable only by the
statement author; maintained theorem source is mutable only by the prover
assignment. Source, statement review, proof receipt, verification receipt, and
bounded log roles remain independently identified. Campaign planning rejects a
role overlap, artifact role overlap, stale BLAKE3, unknown service, capability
widening, or generated/maintained artifact conflation.

## Budgets and evidence

The profile names finite proof-time, stage-call, service/tool-call, output-byte,
retrieval-item, retained-log-byte, and retained-artifact budgets. The host owns
measurement, timeout, cancellation, and service execution. The pure terminal
reducer converts any measured overrun to `budget-exhausted` and never upgrades
another terminal outcome.

Receipts contain only schemas; profile, skill, plan, role, assignment, subject,
statement, model, budget, stage, and evidence BLAKE3 identities; bounded counts;
safe issue classes; terminal outcomes; and explicit redaction markers. They
exclude raw prompts, hidden reasoning, credentials, private
paths, source/proof bodies, tool arguments/output, provider payloads, and
unbounded logs. A campaign receipt always records
`authoritative_completion=false`.

## Canonical skill and validation

The single repository-owned skill is
[`skills/aeneas-proof-campaign/SKILL.md`](../skills/aeneas-proof-campaign/SKILL.md).
The Nickel manifest binds its BLAKE3, and the focused checker rejects profile,
generated export, or skill drift. The skill provides progressive disclosure;
it does not copy Aeneas's Git workflow, hidden prompts, project-specific proof
tactics, runtime policy, or mutable thresholds.

Run the focused rail from the repository root:

```sh
nix develop --option builders '' --option secret-key-files '' \
  -c check-aeneas-proof-campaign --root .
```

The rail validates generated-profile freshness, canonical-skill freshness,
positive and negative Nickel fixtures, pure Rust contracts, safe receipts, and
non-authority boundaries. Its receipt contains deterministic identities and
fixture counts, not elapsed timing.
