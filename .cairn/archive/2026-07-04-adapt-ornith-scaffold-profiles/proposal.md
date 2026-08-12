# Proposal: Adapt Ornith scaffold profiles for Animus

## Why

Ornith-1.0's transferable idea is that the agent scaffold is part of the policy surface, not an invisible prompt blob. Animus should adopt that as a typed, reviewed runtime strategy object while rejecting live self-mutation or executable model-authored harnesses.

First-class scaffold profiles let Animus compare task-category strategies, support small/local models with explicit orchestration help, and record which strategy influenced a turn without leaking transcripts.

## What Changes

- Define typed `ScaffoldProfile` and `ScaffoldRevision` descriptors for identity, task class, allowed strategy knobs, provenance, review status, route constraints, and BLAKE3 content hashes.
- Add a two-stage planning boundary where a scaffold is selected or proposed before a task turn runs, then Rust revalidates all resulting model, tool, memory, summary, and response actions.
- Require host admission for model-proposed scaffold revisions before persistence or future prompt influence.
- Record scaffold influence in safe runtime/evaluation receipts without raw prompts, hidden policy text, provider payloads, tool bodies, credentials, or full paths.
- Keep scaffold profiles model-route neutral so compact/local routes can use reviewed strategies without requiring frontier-only provider behavior.

## Impact

- **Files**: future DTOs for scaffold profiles/revisions, pure validation and selection logic, admission/reporting tests, docs, and Cairn artifacts.
- **Testing**: positive profile/selection/receipt tests and negative malformed profile, unsupported knob, unreviewed revision, route mismatch, and receipt leak tests.
- **Non-claims**: this does not add RL training, benchmark claims, executable generated harnesses, autonomous policy mutation, provider-specific scaffolds, or automatic adoption of model-proposed changes.
