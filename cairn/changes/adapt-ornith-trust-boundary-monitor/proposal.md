# Proposal: Adapt Ornith trust-boundary monitoring for Animus

## Why

Ornith explicitly calls out the risk that self-authored scaffolds can satisfy verifiers without solving the task: reading oracle files, modifying verification scripts, hardcoding expected artifacts, or using allowed tools for intent-level gaming. Animus should adopt the defense pattern: keep the outer trust boundary immutable, enforce exact rules with a deterministic monitor, and treat any LLM judge as a veto rather than an authority grant.

This change turns those reward-hacking defenses into Animus policy contracts and negative fixtures.

## What Changes

- Define an immutable outer trust-boundary descriptor for environment authority, sanctioned tool surface, withheld paths, verifier files, sandbox limits, approval requirements, and evidence rules.
- Add a deterministic monitor contract that evaluates planned or observed actions against the boundary and fails closed on exact violations before evidence can count as successful.
- Define optional judge-veto input/output contracts that may reject suspicious trajectories but may not override deterministic monitor denials or grant new authority.
- Add anti-gaming fixtures for hidden-path reads, verifier modification, oracle copying, hardcoded expected outputs, tool-surface expansion, sandbox weakening, and approval suppression.
- Emit safe violation receipts with boundary policy hash, monitor issue codes, labels, and redaction status.

## Impact

- **Files**: future trust-boundary DTOs, pure monitor logic, judge-veto DTOs, violation receipt fixtures, tests, docs, and Cairn artifacts.
- **Testing**: positive allowed-action cases and negative withheld-path, verifier-mutation, oracle-copy, hardcoded-output, tool-expansion, sandbox-weakening, approval-bypass, judge-override, and receipt-leak cases.
- **Non-claims**: this does not add a live LLM judge, train a reward model, expose hidden tests, weaken sandbox policy, or allow runtime scaffolds to alter verifier or tool authority.
