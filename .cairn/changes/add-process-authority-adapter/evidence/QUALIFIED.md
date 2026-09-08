# Qualified execution adapter evidence

Base: `eff17736a6330cfe8f5d8fec2d986b65ff885f8d` (published Animus main).
Source: `e1442bc` (mapping and cohort), `fb2e504` (checked projection),
`0cba188` (standalone manifest repair).

## Cohort qualification

Both Basalt manifest pins select published `3f1fd1ed63f2b55f07a27f662e5a720518662b7e`
with `default-features = false` (the owner-qualified minimal profile): the optional
root reference facade and the std authority adapter. The Nix basalt input rev and
flake.lock were updated by Nix only. `cargo update -p zeroize` regenerated the lock
normally to the required 1.9.0; no blanket update, no precise override, no lock edit.
The adapter normal dependency graph contains none of bounded-http, bounded-http-core,
replay-ledger-core, or revocation-view-core.

Measured from the published source (store `s6n40n82yj0zhzjwnbyl2sl4b3ipkwq1-source`):
default policy `5475d5473ec98cbf052a83512836b0739f5dad3e7b9b0ed201eb3094b1cf39bf`
(unchanged), execution policy `c90838cfc1f3c8f5f926f9ee98a05d203d2c0dc2199657514537c37bff3d8c7d`,
support catalog `fe77fd63076eaef555b74e702e4c14e483bdba70151ccc955c75b17d97412427`,
UCAN revision `927dc32641af6cd1b17202aff44c35acef4540fa`. No private hash was copied.

## Mapping and policy boundary

`plan_basalt_request` admits exactly Exec/process-execute/process/execute in addition
to the two unchanged document rows. It requires the execution policy identity for
Exec, rejects non-`onix://process-execution/` resources before verification, and
keeps the exact operation-profile check. `required_policy_identity(action)` is pure
and public. Shell preflight is action-aware; a supplied default policy with an
execution request is denied PolicyDenied before verification or replay.

Two upstream API changes were absorbed: the new `BasaltUcanProtocolProfile` request
argument (fixed to the compact profile) and `ProtocolProfileDenied`, mapped to
UnsupportedAction. No caller surface changed.

The reviewed execution policy artifact is embedded only as a test fixture; the test
asserts its BLAKE3 equals `BASALT_PROCESS_EXECUTION_POLICY_IDENTITY`. Production
policy selection stays caller-owned.

## Controls

15 adapter tests passed on pinned rustc 1.95.0 (8 prior + 7 new): execution fixture
identity binding; exact purpose/policy/profile mapping denials; signed execution
dispatch after gates and commit with one replay; default policy denied before
replay; changed context invalidates the holder signature with zero replay calls;
document capability cannot authorize execution; revocation, replayed nonce, and a
fresh-invocation local-gate denial with zero effects. The first revoked/replay run
failed because one test reused a spent nonce for the gate phase; corrected with a
fresh nonce rather than weakening the assertion. Strict all-target Clippy passed
after ordinary fixes (match-arm merge, helper arg struct, lifetimes). The execution
policy fixture and changed-context signature reuse mirror Basalt's own published
process tests.

## Rails

The focused `basalt-authority-adapter` rail passed (5m44s): marker cohort,
Nickel fixtures, adapter and host-authority suites, wasm/no-default isolation,
root-facade leak denials, connected clash/coding-session/runtime-seal rails,
component catalog, facade inventory, and evidence safety.

The first rail run failed at the Nickel Nix check: upstream commit `92914e2`
changed `config/agent-check-registry.ncl` without regenerating the standalone
export manifests, so published main's check was already stale. The owner
`check-standalone-nickel-exports --write` path regenerated four manifests; the
diff records only the new dependency identity and manifest identities with no
output changes. The checked Basalt projection was refreshed through the owner
export script. Both are maintenance repairs, not authority changes.

## Non-claims

Test keys and counters are not operator authority. Execution admission binds only
the supplied 64-hex context reference; it does not authenticate the context
preimage, executable, argv, environment, inputs, services, limits, or custody, and
no process ran. No downstream consumer, Neural activation, trainer, sandbox
confinement, storage durability, or release readiness is established.
