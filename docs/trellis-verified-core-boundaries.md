# Trellis verified-core boundaries

Animus uses Trellis as a bounded reference for proof-oriented treatment of small pure decision cores. This note is the inventory and proof-scope record for `r[agent_harness_runtime.trellis_verified_core.candidate_inventory]` and `r[agent_harness_runtime.trellis_verified_core.obligations]`.

## Candidate inventory

| Candidate | Pure core inputs | Pure core outputs | Excluded shell behavior | Initial status |
| --- | --- | --- | --- | --- |
| Capability/effect authorization | Typed capability request, reviewed contract fixture, verified grant summaries, local caveat decisions, sandbox state | Authorization issue/status and safe receipt metadata | Filesystem, process, provider, network, credential, terminal, clock, adapter dispatch | Selected for model equivalence |
| Ledger graph validation | In-memory entry IDs, parent links, roles, and payload classes | Typed graph acceptance or duplicate/missing-parent/cycle/unsupported-entry errors | Storage, filesystem paths, session lookup, clocks, provider payloads | Candidate only |
| Branch replay | In-memory neutral entries and replay options | Model-visible context plus safe counts or typed replay error | Provider calls, tool calls, persistence, credentials | Candidate only |
| Receipt redaction classification | Safe receipt draft fields and leak-class fixtures | Accepted safe receipt or typed redaction error | Raw prompt/model/tool/provider capture, unbounded output storage | Candidate only |
| Schema compatibility | Versioned DTO schema labels and fixture payload classes | Compatibility acceptance or unsupported-version/type errors | Runtime Nickel evaluation, filesystem discovery, adapter dispatch | Candidate only |
| Compaction cut points | In-memory ledger/event classes, safe counts, and budget hints | Deterministic cut-point choice or typed refusal | Tokenizer/provider calls, wall-clock reads, storage mutation | Candidate only |
| Roadmap dependency checks | Active change IDs, accepted requirement IDs, roadmap dependency records | Valid/blocked/error issue classes | Cairn mutation, archive/sync execution, network, clocks | Candidate only |

## Proof obligations

Selected candidates must declare these obligations before proof or model artifacts count as release evidence:

- Inputs and outputs are explicit in Rust types or test fixtures.
- Invalid inputs return typed rejection classes instead of panics or silent acceptance.
- Contract, authority, redaction, and graph invariants state both what must happen and what cannot happen.
- Equivalence tests compare model or verified logic with production Rust behavior for positive and negative fixtures when both exist.
- Counterexample fixtures remain replayable without live host services.

## Selected boundary: capability/effect authorization

`animus-adapters` now contains a pure model of the authorization issue-selection boundary. The model consumes normalized booleans/counts/state derived from an `AuthorizationContext` and predicts the production `AuthorizationIssue` ordering:

1. contract/resource/ability mismatch denies first;
2. local deny caveats fail closed before missing-grant handling;
3. missing verified grants deny before sandbox dispatch;
4. malformed or unavailable required sandboxes deny after authority checks;
5. only matching contract, matching verified grant, non-denying caveats, and enforceable/no sandbox allow dispatch.

Equivalence tests cover allowed model/exec paths and rejected contract mismatch, local denial, missing grant, malformed sandbox, and unavailable sandbox counterexamples. This satisfies `r[agent_harness_runtime.trellis_verified_core.model_logic]`, `r[agent_harness_runtime.trellis_verified_core.equivalence]`, and the selected-candidate portion of `r[agent_harness_runtime.trellis_verified_core.negative_fixtures]`.

## Dependency isolation

Trellis remains an optional root feature (`trellis-reference`) and is not required by default `animus-adapters`, `animus-core`, or root no-default builds. The focused rail checks the optional-feature declaration and runs a no-default root build to guard `r[agent_harness_runtime.trellis_verified_core.dependency_isolation]`.

## Non-claims

This work does not formally verify all Animus logic, does not make Trellis a default runtime dependency, does not replace ordinary Rust tests or code review, and does not claim verified coverage for candidate boundaries whose model/equivalence tests are not yet implemented.
