# Trajectory transcript adapter

Animus provides a non-default adapter for `@letta-ai/trajectory` transcript normalization. The reviewed cohort is package `0.2.0` at revision `a665556089d6a3c59f5806ee109b4d36f6c2ac83` under Apache-2.0.

The adapter supports transcript strings from `claude-code`, `codex`, `hermes`, `letta-code`, `openclaw`, `openhands`, and `pi`. It does not support local-store listing, home-directory scans, Deep Agents checkpoints, SQLite, the Python wrapper, canonical-v1 Cloud records, or chunked-upload identities.

## Contract

1. A caller loads the transcript and supplies an immutable source-generation identity.
2. Pure policy admission checks the exact helper and trajectory-v1 schema identities and all finite bounds.
3. The host supplies an implementation-free normalizer service or an injected bounded executor. Animus does not search `PATH`, inspect environment variables, install packages, or use a fallback normalizer.
4. The host sends one bounded JSON request through transient stdin. Raw stdout and diagnostic messages remain transient.
5. Pure Rust validates one leading meta record, source, roles, timestamps, content, JSON-object tool arguments, call uniqueness, result correlation, diagnostics, and bounds.
6. Pure projection creates existing neutral-ledger entries and batch drafts. It does not persist a session, dispatch a batch, emit dataset rows, or run ATIF evaluation.
7. A metadata-only receipt records the cohort, policy, helper, service, counts, loss posture, omissions, and projection class.

## Privacy defaults

The default profile omits reasoning, tool arguments, tool results, cwd, branch, model metadata, and diagnostic prose. Tool content needs a separate explicit policy change and the same finite validation bounds. Source timestamps remain descriptive. They do not establish identity, freshness, order authority, or causality.

BLAKE3 projection identities bind the caller's source-generation identity, record position, and validated record content. Safe receipts omit those content-derived identities because they can become correlation metadata.

## Update procedure

1. Pin one package version and source revision.
2. Review the transcript-only API, all supported adapters, diagnostics, schema, bundled helper, runtime requirement, and license.
3. Update the Nickel cohort and exact BLAKE3 schema and helper identities.
4. Run all seven upstream synthetic tool-call fixtures through the packaged helper.
5. Run positive and negative Rust, Nickel, isolation, facade, receipt, Cairn, and Nix checks.
6. Sync and archive only when tracked requirements and non-claims remain exact.

## Non-claims

Normalization does not prove source authenticity, transcript completeness, timestamp truth, tool execution, semantic equivalence, privacy, model correctness, dataset fitness, production readiness, or release eligibility.
