# Completion policy coherence — 2026-09-06

## Cause and repair

Source `92914e2b08e8ba52582ad99c3f9725706d0a10de` aligns completion with the current repository policy.
The lifecycle upgrade changed the policy but retained old completion bindings.
The CLI correctly rejected the old positive fixture.
The embedded core accepted its explicitly supplied historical identity.

The reviewed policy identity is `302bc1f8bcb27920c37a6bdc21865e2ca1fbb5a38c268570b3fc326734e59942`.
The previous identity is `2c0b44345b9d2e2b3eff5fa7db7a14b9681c6bfcf7a4bb651f13bcadc1f91757`.
The change updates the runtime constant, Nickel registry, generated registry, and synthetic completion inputs together.
It retains the previous inputs as negative fixtures.

The current policy adds outcome-machine profiles, task-ordering metadata and markers, and traceability assurance metadata.
The completion status policy does not change.
The repository policy itself remains unchanged by this repair.

## Component scope

The embedded Cairn core remains pinned to `695124d459574ba7aeba6097310d237f393c243c`.
The lifecycle CLI remains pinned to `5e1fe08d62149b286ff583eb056e655fe92e3cf1`.
The selected `verified/agent_receipt/authoritative.rs` and `verified/evidence.rs` implementations are byte-identical across these sources.
The adjacent receipt test module changes one historical golden value.
This comparison does not establish compatibility for all Cairn APIs.
No Cargo manifest, Cargo lock, Nix lock, dependency override, or default feature changes.

Cairn's published `canonical_json_hash` computes the actual policy identity in the test shell.
Its published completion evaluator computes the expected complete report.
Animus copies neither implementation.
Runtime domain APIs gain no filesystem, environment, process, transport, or policy-loading dependency.

## Discriminating controls

The pre-repair package baseline passed two library and four real-child tests.
The broader completion rail then failed on the old positive fixture.
The new coherence test failed before repair with current identity `302bc1f8` versus runtime identity `2c0b4434`.

After repair, all five new integration controls passed:

1. The actual repository policy agrees with the runtime and generated registry.
2. Complete positive, stale-subject, and old-policy CLI reports agree with the embedded core.
3. Internally matching old-policy inputs still fail current admission.
4. A changed policy cannot borrow current completion evidence.
5. Altered report fields and removed non-claims fail the comparator.

The current positive completion identity is `78fc4f60a136e42f71deff7450ffecfd77d38476e42b12d7120561428ad3f199`.
Old-policy and stale-subject responses both report `valid: false` and `verdict: FAIL`, despite CLI exit zero.
The runtime rejects both arbitrary stale identities and the exact previous identity.
Five negative Nickel fixtures reject, including the exact previous identity.
Unknown completion-mode values reject before commands run.

The real-child package retains its existing six passing tests.
The live integrated completion rail, strict host-package Clippy, two lifecycle fixture tests,
and the source-bound real-check fixture rail passed.
The maintained full Nix Octet profile passed with zero findings, warnings, or errors.
This profile does not claim every optional feature combination.

## Mode and evidence boundaries

The integrated rail passes explicit repository and policy paths to the selected CLI.
It compares the complete nested completion report, not grep matches or exit status alone.
Ordinary Cargo runs use saved conformance responses.
The live rail supplies fresh CLI responses through an explicit test-only directory.
The rail records `cairn_response_mode` so saved replay cannot masquerade as live validation.
The maintained Nix completion check uses saved mode inside its sandbox.
That sandboxed Nix check passed in 2 minutes 49 seconds with all five parity controls.
Its saved-mode result is separate from live validation.
The live and source-bound fixture rails passed again at committed source `92914e2`.

The implementing assistant performed correlated serial reviews, not independent review.
Persistent logs remain under the primary Neural Stream `.pi/drains/2026-09-06-completion-cohort/` directory.
The previous capture worktree and primary user changes remain preserved.

No synthetic result proves actual check execution, native training, executable authenticity, or promotion authority.
Cairn reports `checks_executed: false` throughout this validation study.
Materialization requires separate authenticated native execution, evaluation, and live fallback evidence.
