# Tasks

## Phase 1: Boundary and model

- [ ] [serial] Record Jido reference revision `548b2a345765ba33e687341c661bbbcbdda73d94` at `github.com/agentjido/jido`, license, and selected `cmd/2` invariants in repository references. r[animus.command_contract.boundary]
- [ ] [serial] Run and record current tool-contract, model-request, operation-contract, no-std, and Wasm baselines. r[animus.command_contract.verification]
- [ ] [serial] Add no-std command identity, action-unit identity, declared schema descriptor, generation, request, result, decision, issue, transition, limit, and receipt DTOs. r[animus.command_contract.model]
- [ ] [depends:animus.command_contract.model] Implement pure structural validation and domain-separated BLAKE3 identities over supplied safe values. r[animus.command_contract.admission]

## Phase 2: Admission and generations

- [ ] [depends:animus.command_contract.admission] Implement complete-before-apply admission with a post-command subject projection and an effect-intent-only plan, with fail-closed missing, duplicate, undeclared, mixed, stale, malformed, plan-mutation, and over-limit outcomes. r[animus.command_contract.admission]
- [ ] [depends:animus.command_contract.admission] Implement command replacement and handoff generation validation without consumer lifecycle policy. r[animus.command_contract.generation]
- [ ] [serial] Add optional Serde and Preserves codecs that re-enter core validation. r[animus.command_contract.model]

## Phase 3: Consumer mappings and evidence

- [ ] [depends:animus.command_contract.admission] Add frozen Lattice command-bound node mapping fixtures without a Lattice runtime dependency. r[animus.command_contract.consumers]
- [ ] [depends:animus.command_contract.admission] Add frozen Pi tool-host declared-schema dispatch mapping fixtures without a Pi runtime dependency. r[animus.command_contract.consumers]
- [ ] [parallel] Add positive fixtures for one current complete command, exact generation replacement, codec round trips, and consumer mappings. r[animus.command_contract.verification]
- [ ] [parallel] Add negative fixtures for undeclared schema, stale generation, duplicate command, missing input, malformed schema, plan mutation, over-limit commands, malformed codec data, and authority overclaims. r[animus.command_contract.verification]
- [ ] [serial] Update README consumer mapping and non-claims. Add focused Cairn and Nix checks. r[animus.command_contract.boundary] r[animus.command_contract.verification]
- [ ] [depends:animus.command_contract.verification] Run focused tests before and after core changes, workspace tests, Clippy, no-std, Wasm, codecs, Cairn validation and gates, and relevant Nix checks. r[animus.command_contract.verification]
