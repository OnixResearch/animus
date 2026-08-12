# Evidence: Trajectory transcript adapter

## Baseline

Pueue task `14116` passed the focused pre-change baseline:

- 16 neutral-ledger positive and negative tests.
- 10 batch trajectory tests.
- 7 dataset-export tests, including reasoning and tool-content denial.
- 18 model-profile and reasoning-boundary tests.
- 7 process-job profile tests.
- 5 safe artifact and receipt tests.
- Proposal gate receipt `921e9175413b73b66a3653b370d6793668e78ac4747f8bc522aa2e21a7316e6d`.
- Design gate receipt `a7c50182e790aafa64d1902eb073549dc227226d708204351bc599aa006c48df`.
- Tasks gate receipt `d5f9bb3a2caf312c6cec844556200b196b10ae4180c558510c000f6cb79f369b`.

## Architecture search

The bounded search compared three mechanisms:

1. A Rust port of all seven native transcript decoders. This duplicates upstream repair semantics and does not prove cohort parity.
2. Ambient Node integration with store discovery. This adds process, filesystem, environment, SQLite, and mutable package authority.
3. The selected exact bundled helper behind an injected bounded executor, followed by pure Rust revalidation and existing-owner projection.

An adversarial pass checked helper mismatch, missing service, malformed protocol, unknown fields, nonzero and truncated outcomes, unknown diagnostics, malformed records, call and result ambiguity, sensitive receipt fields, and overclaims. The selected mechanism fails closed for each class.

## Reviewed cohort

The reviewed source is `@letta-ai/trajectory` `0.2.0` at revision `a665556089d6a3c59f5806ee109b4d36f6c2ac83` under Apache-2.0.

Reviewed transcript sources are `claude-code`, `codex`, `hermes`, `letta-code`, `openclaw`, `openhands`, and `pi`. The adapter excludes local listing, home-directory scans, Deep Agents checkpoints, SQLite, the Python wrapper, canonical-v1 Cloud records, source-context chunking, and automatic persistence.

Exact BLAKE3 identities:

- Bundled helper: `1ef86c7687ff8f7354cb08e515e391e66f4e894f892aa807597d4ce399f5de20`.
- Trajectory-v1 schema: `4193e80e761f1aaa9e59545f356de02d2aa33443b4430fae4d1d82e29bca1212`.
- Package metadata: `61f8fffe75383a098b530c91291e314eb379fb474ea869069ec3415b40f3aabc`.
- License bytes: `f4327b5abcba99316794919b8bbc8e9df5206cd9f20f0d88f8fb3ef215a63df1`.

The Nix source lock has `narHash = sha256-xD3HxZxOenM2tsMYsylYPi0U4MNoG8+ddEkQSRFJE/k=`.

## Implemented boundary

- `animus-adapters` owns implementation-free request, result, record, diagnostic, and normalizer-service contracts.
- `animus-runtime` owns pure exact-policy validation, trajectory-v1 validation, diagnostic classification, BLAKE3 projection, neutral-ledger drafts, batch drafts, and metadata-only receipts.
- `animus-std` owns strict generated-policy and helper-protocol parsing over one injected bounded executor.
- Nix packages the reviewed bundled helper with Node 22. Rust crates do not depend on Node or upstream TypeScript.
- The default profile omits reasoning, tool arguments, tool results, cwd, branch, model metadata, and diagnostic prose.

## Policy and fixtures

Exact BLAKE3 identities:

- Nickel policy source: `19fcdd6a1cab00a5a52f83b61667b1eed9ef9a48c569d6d9f90092bae1aa3b79`.
- Generated runtime policy: `4dd9c1c9cb635fb1ef88878aea9364660c326d7b07e6b4fc1b8507b27d9b7a10`.
- Positive Nickel fixture: `b108d4a918ec83322e4a1efc238393260b22456a660659f55f9282c6491d8f44`.
- Negative Nickel fixture: `aa5ccb57b6ceaba974d786c193e569da284eb10f0a4961e6e59572a1f1f71d01`.

## Validation

Focused Rust and Clippy checks passed in Pueue tasks `14123`, `14124`, `14037`, `14038`, and `14039`. Pueue task `14048` passed the complete focused rail with exact helper and schema identities plus all seven upstream synthetic source fixtures. Pueue task `14053` passed workspace check, workspace Clippy, formatting, and diff validation. Pueue task `14054` passed the final full Nix check. The focused rail receipt is `target/cairn/focused-rails/trajectory-transcript-adapter.md`.

## Final Cairn gates and sync plan

- Proposal gate receipt: `11d3189358bd5a0fda8f541270c7e9e413eaf59dc9b5f524662c61c983c7d67d`.
- Design gate receipt: `c6ab262e6a5b3b45666a1c1fe1c5fa0ea330b90372ab74ce6543fe1af110763f`.
- Tasks gate receipt: `616282bf8a72b6f0c88b5500595e4c2a17c8efc6af92d86b387c5887a4ee9245`.
- Unblocked accepted-spec dry-run plan: `69ae3f7798a5d207d631c73a1684bd724121bc39a2949fc318fa7c6fbfbade29`.
- Dry-run receipt: `9e72b6f09a8f3e6344502a07556f9674f87db708e1be86083c40ee6b00489b3e`.

The plan adds all seven `agent_harness_runtime.letta_trajectory.*` requirements without changing existing accepted requirements. Sync execution used plan `f2dab59c44b4c76f61fe9a056defdf4f0cc8d85835f49a73cfa89ee64cbbb770` and receipt `dfaf0579a25ca62001f592fecaed82f8217acdb1d5c0f7e4ce46732499258749`. The accepted specification hash is `320ad67bb13c92bf12a6c97fe2dfbb2ba3f9572a04df7d04958452988909014a`.

## Non-claims

This evidence does not prove source authenticity, transcript completeness, timestamp truth, tool execution, semantic equivalence, privacy, model correctness, dataset fitness, production readiness, or release eligibility.
