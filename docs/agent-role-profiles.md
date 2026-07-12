# Capability-bound semantic agent roles

Animus represents agent roles as typed, validated capability ceilings rather than prompt labels. `config/agent-role-profiles.ncl` is the reviewed authoring source and exports `config/generated/agent-role-profiles.json`. Runtime code never evaluates Nickel: the `animus-std` host shell loads an explicit generated JSON path, and the reusable `animus-runtime::agent_roles` core only validates in-memory values and computes deterministic decisions.

## Role taxonomy

The catalog requires four baseline roles and supports narrower specializations:

| Role | Semantic class | Mutation posture | Output boundary |
|---|---|---|---|
| `implementer` | implementation | scoped mutation intent; later host mutation policy still decides | implementation candidate only |
| `reviewer` | review | immutable subject, read-only | advisory findings, warnings, blocks, tighten recommendations, or abstention |
| `verifier` | verification | supplied named verification service only | supplied check evidence or abstention |
| `judge` | judgment | none | rubric-bound ranking, selection, veto, tie, or abstention |
| `coordinator` | coordination | none | bounded coordination or handoff metadata |
| `observer` | observation | none | read-only observations or abstention |

A profile binds its semantic class, exact required capabilities, capability ceiling, eligible model profiles and targets, prompt/skill/memory/tool/workspace ceilings, class postures, input/output/evidence contract references, separation policy, finite budget, active policy hash, and canonical BLAKE3 profile identity. A role name does not grant authority, and role aliases cannot bypass the typed class invariants.

## Functional core and host shell

`animus-runtime::agent_roles` is the pure core. It does not read files, inspect environment variables, evaluate Nickel, invoke providers, execute tools, access credentials, inspect clocks, or persist data. It validates catalogs, composes resource ceilings, evaluates separation observations, intersects typed capabilities, binds immutable plans, classifies transitions, admits bounded outputs, and projects safe receipts.

The host shell owns:

- selecting and reading an explicit generated JSON path;
- validating current policy, grant, freshness, subject, actor, workspace, model, verifier, and rubric identities;
- UCAN and local-policy authorization;
- provider, tool, verification-service, filesystem, process, network, credential, clock, timeout, cancellation, persistence, and receipt I/O;
- mutation admission and authoritative completion gates.

The `animus-std` loader is behind the `agent-role-profiles` feature. Unknown JSON fields, unsupported enum values, stale profile hashes, malformed references, untrusted profiles, and invalid class postures fail closed.

## Authority is an exact intersection

Role assignment and activation require separate exact `GuardedAction::AgentRole` requests and matching allowed receipts over the same scoped resource. That resource binds role/profile, subject/task, actor/parent lineage, run/session/workspace/mutation lineage, agent definition, role-bound model plan, policy/grant/freshness, separation decision, and output-contract identities. Output, handoff, selection, and veto effects require their own action-specific resources and abilities; assignment or activation authority is never reused as later influence authority.

Effective capabilities are the intersection of the request and every independent ceiling:

1. verified UCAN grant and contract;
2. host/session policy and local caveats;
3. parent delegation;
4. task or recipe scope;
5. admitted agent definition;
6. semantic role profile;
7. admitted per-model runtime profile;
8. workspace and context scope;
9. tool catalog and executor admission;
10. explicit approval;
11. sandbox admission;
12. cancellation state;
13. timeout state; and
14. named finite budget admission.

A capability missing from any source is denied and records the exact denying source, including separate approval, sandbox, cancellation, timeout, and budget diagnostics. Required capabilities only make activation fail when absent; they never mint missing authority. Wildcards, duplicate capabilities, stale assignment bindings, and cross-assignment reuse fail closed.

## Model-profile composition

Role composition adds a semantic-role `ModelResourceCeiling` to the existing per-model invocation composition. The existing model-profile implementation remains authoritative for prompt assembly, progressive skill disclosure, exact memory-policy consent, model capability negotiation, native tool declarations, executor binding, and budgets.

The role can only narrow:

- eligible model profile and exact provider/model target;
- prompt mode and source labels;
- skill metadata or selected bodies already admitted elsewhere;
- existing validated memory-policy identities;
- tool names, tool capability labels, catalog entries, executors, and approvals;
- workspace and finite budget scopes.

Role and model identities are included in the role-bound model-plan hash. Model fallback recomposes the model-specific plan; role change and handoff recompose all role-specific state.

## Separation of duty

Review, verification, and judgment profiles require subject immutability plus at least one explicit distinct dimension. Named policies can check:

- assignment;
- actor;
- run or thread;
- session;
- model/runtime profile;
- workspace;
- mutation lineage;
- clean context reconstruction;
- immutable subject;
- immutable verifier/check identity;
- immutable rubric identity.

A host supplies observed identities. The pure core returns a bounded decision listing failed dimensions. Separation is scoped only to those checked dimensions; it does not claim human, organizational, statistical, or global independence.

Implementers cannot approve their own candidates merely by changing role labels. Reviewer, verifier, and judge contexts are reconstructed from admitted bounded records rather than inherited transcripts or hidden reasoning. Verification reports identify supplied checks but do not claim that a verifier is sound. Judgment is rubric-bound and cannot override or suppress deterministic policy, capability, sandbox, validation, or verification denials.

## Outputs, handoffs, and transitions

Role outputs are typed effects rather than free-form authoritative decisions. Every output is bound to the exact assignment, subject, role profile, role plan, model plan, capability decision, policy, separation decision, output contract, evidence identity, and output authorization receipt.

Forbidden output claims include authority grants, subject mutation, authoritative completion, verifier soundness, and unbounded independence. The durable envelope rejects raw prompts or instructions, hidden reasoning, provider payloads, credentials, private paths, verifier or hidden-test bodies, raw diffs/files, tool arguments/output, judge rationale, and transcripts.

Resume and replay require every immutable plan identity to match exactly. Role change, handoff, and model fallback return explicit recomposition decisions rather than carrying old authority. Handoffs contain only bounded evidence references, unresolved-intent labels, requested-action labels, and identities; they cannot carry transcripts, reasoning, credentials, grants, tool bodies, or pending tool calls.

## Receipt and completion boundary

`AgentRoleReceipt` is metadata-only evidence. It can record role class, profile/assignment/subject/actor/model/policy/grant/separation identities, capability counts, plan/handoff/output identities, and a redaction marker. It cannot contain raw role context or output material.

A successful role output is not authoritative task completion. Completion remains a separate host-owned decision that must validate required evidence, terminal state, policy, cancellation, mutation admission, and durable receipt consistency. The semantic role layer neither upgrades advisory findings nor converts a candidate, check report, or judgment into completion.

## Verification

Run the focused rail from the repository development shell:

```sh
scripts/cairn/check-agent-role-profiles --root .
```

The rail checks generated JSON parity, positive and negative Nickel fixtures, pure-core authority markers, the explicit host loader, focused Rust tests, and absence of ambient host authority in the reusable role module. It emits `target/cairn/focused-rails/agent-role-profiles.md`.

The rail proves only the checked catalog, composition, intersection, separation, transition, output, and receipt behavior. It does not prove provider or tool correctness, verifier soundness, human independence, authoritative completion, or whole-system correctness.
