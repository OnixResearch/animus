# Receipt-authoritative agent completion

Animus treats completion text, queue emptiness, process exit, task counts, and archive appearance as requests or observations. Verified completion requires a current repository-owned check selection, exact pass-only host evidence, and a matching passing Cairn completion-validation receipt for the final subject.

## Pinned authority

The embedded completion core pins Cairn `695124d459574ba7aeba6097310d237f393c243c`.
The lifecycle CLI pins `5e1fe08d62149b286ff583eb056e655fe92e3cf1`.
Both use schema `cairn.agent-completion.v2` and the reviewed repository policy identity
`302bc1f8bcb27920c37a6bdc21865e2ca1fbb5a38c268570b3fc326734e59942`.
The selected completion algorithm and canonical identity implementation are unchanged across these revisions.
The complete-report parity rail checks their compatibility. Animus does not redefine Cairn's pass-only contract.

`config/agent-check-registry.ncl` owns the bounded check registry. `config/generated/agent-check-registry.json` is the checked runtime input. The generated registry records exact argv vectors, working roots, prerequisites, trigger classes, costs, timeouts, evidence classes, and conservative fallback checks. Runtime code never interprets shell strings or discovers commands from the environment.

## Functional core and host shell

The pure `animus_runtime::authoritative_completion` core:

1. validates the checked registry;
2. selects checks from host-supplied changed-artifact classes;
3. closes prerequisites and orders checks deterministically;
4. admits exact registry-bound execution requests;
5. accepts only fresh, successful, `passed` result summaries;
6. projects a completion candidate and validates a supplied Cairn summary; and
7. emits a metadata-only Animus receipt with BLAKE3 identities.

Every validation status except `passed` requires follow-up. `skipped`, `blocked`, `unsupported`, `timed-out`, cancelled, missing, stale, malformed, and failed observations cannot satisfy completion.

`animus-std` supplies the imperative shell. Embedders inject workspace-snapshot, subject-hash, check-execution, receipt-persistence, and Cairn-validation services. The shell does not discover or substitute checks, and failures do not fall back to ambient process execution.

## Completion candidates and continuation

The Steel `ralph-loop` profile recognizes only an exact trimmed `COMPLETE` response. Incidental prose such as `not COMPLETE`, `COMPLETE later`, and larger messages containing that token are not completion candidates. Even an exact candidate is non-authoritative: it emits candidate status and the controller waits for required authoritative evidence when no other work is queued.

The headless protocol exposes `CompletionCandidateObserved`; runtime telemetry records the candidate without claiming verified completion. Hosts may present this state while the selected checks and Cairn validation run.

## Cairn handoff

r[verify agent_harness_runtime.authoritative_completion.receipt_gate.host_adapter]

The positive fixture pair under `fixtures/authoritative-completion/` binds the selected check set, BLAKE3 registry and selection identities, exact command arrays, final subject, passing result receipts, Cairn schema, and pinned policy. The real pinned command is:

```sh
nix run --option builders '' --option secret-key-files '' .#cairn -- \
  agent-receipt complete \
  --contract fixtures/authoritative-completion/cairn-contract.json \
  --results fixtures/authoritative-completion/cairn-results.json \
  --subject-hash cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc
```

The checked positive receipt has completion identity `78fc4f60a136e42f71deff7450ffecfd77d38476e42b12d7120561428ad3f199`.
The stale-subject fixture fails with `verification_contract.subject_stale`.
The retained `previous-policy/` inputs fail with `verification_contract.policy_drift`.
All inputs are synthetic conformance fixtures. Production hosts must construct evidence for the current real subject.

`tests/policy-cohort.rs` in `animus-std-check-exec` compares complete core reports with CLI responses.
It also checks the actual repository policy identity and rejects altered receipt fields.
The integrated rail supplies fresh responses. Ordinary Cargo tests use checked fixture responses.
`CAIRN_COMPLETION_MODE=saved` is explicit fixture replay, not fresh CLI validation.
The rail receipt records that mode and never claims that Cairn ran the selected checks.

Run the integrated rail with:

```sh
nix develop --option builders '' --option secret-key-files '' -c \
  scripts/cairn/check-authoritative-agent-completion --root .
```

## Workspace consumer handoff

r[impl agent_harness_runtime.authoritative_completion.workspace_supervisor]
r[verify agent_harness_runtime.authoritative_completion.workspace_supervisor.progress]
r[verify agent_harness_runtime.authoritative_completion.workspace_supervisor.archive]
r[verify agent_harness_runtime.authoritative_completion.workspace_supervisor.non_authority]

Workspace supervisors such as `../ralph-drain-stack.scm` remain operator tools. They may use archived lifecycle progress to decide what to schedule next, but they must not promote model text, process exit, task deltas, or archive appearance to verified completion. A future workspace integration must consume a current Animus completion receipt and the matching Cairn `cairn.agent-completion.v2` receipt, preserve repository and subject identities, and validate each repository independently before making a bounded workspace claim.

## Non-claims

This rail does not prove check-oracle soundness, honest execution beyond supplied host evidence, model correctness, whole-product correctness, dependency satisfaction, release eligibility outside the selected contract, production readiness, performance, or deployment safety. It grants no push, pull-request, signing, release, shell, filesystem, network, clock, provider, or credential authority.
