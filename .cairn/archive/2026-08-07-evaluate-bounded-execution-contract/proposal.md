## Why

Animus, Octet, Nickel Export, and Mantle all describe bounded process execution, but their guarantees are not interchangeable. Animus resolves backend-neutral process-job and sandbox descriptors while host adapters retain effects. Octet's proof rails use isolated work roots, cleared allowlisted environments, bounded output, timeouts, and denied networking. Nickel Export runs an explicit evaluator with typed resource, process-time, output, and replay bounds. Mantle combines sandbox execution with protected-exec and declared host-tool evidence.

Those surfaces may share a reusable request, observation, failure, and receipt vocabulary. They may also differ materially in what `network denied`, `isolated root`, `timeout`, `cleanup`, or `sandboxed` means. Extracting a repository before measuring those differences could create a lowest-common-denominator contract that launders weaker mechanisms into stronger claims. Animus therefore needs a bounded cross-stack feasibility evaluation before any repository extraction or consumer cutover.

## What Changes

- Inventory the exact execution-plan, environment, filesystem, network, resource, timeout, cancellation, output, cleanup, mechanism, receipt, and retained-authority semantics in Animus, Octet, Nickel Export, and Mantle at immutable reviewed source identities.
- Define a typed Nickel evaluation profile and matrix that distinguishes requested, configured, enforced, observed, unavailable, unsupported, and not-applicable capability states instead of collapsing them into a `sandboxed` boolean.
- Freeze positive and negative cross-product fixtures for successful execution, nonzero exit, launch denial, missing executable, environment rejection, root or path escape, timeout, cancellation, signal termination, output overflow, network-policy differences, cleanup failure, and unsupported mechanism claims.
- Add a pure deterministic evaluator that classifies mappings as lossless, explicitly extended, or blocked and emits a BLAKE3-bound decision receipt with terminal status `extract`, `incubate`, or `blocked`.
- Require at least two independent consumers, a product-neutral authority-free core, no dependency cycle, lossless mandatory mappings, and independently useful release cadence before `extract` can be emitted.
- Keep repository creation, code movement, source pinning, compatibility facades, and consumer cutover in a successor Cairn that is created only if this evaluation emits `extract`.

## Impact

- **Animus surfaces**: cross-stack inventory documentation, typed Nickel evaluation policy and generated projection, conformance fixtures, a repository-owned deterministic checker, bounded decision evidence, contracts index, and focused verification guidance.
- **External repositories**: Octet, Nickel Export, and Mantle are measured inputs only. This change does not edit them, transfer their execution authority, or make workspace-relative paths durable behavior.
- **Runtime behavior**: no Animus process executor, sandbox backend, public runtime DTO, default feature, or product authority changes in this evaluation.
- **Testing**: positive and negative matrix/fixture tests, decision-classification self-tests, stale-source and lossy-mapping rejection, existing process-job and sandbox focused rails, Nickel projection freshness, Cairn validation/gates, and component/contract metadata checks.

## Out of Scope

- Creating a universal sandbox implementation or claiming semantic equivalence between Bubblewrap, Landlock, seccomp, Nix/Mantle build sandboxes, evaluator snapshots, or proof-runner isolation.
- Moving process, filesystem, environment, network, clock, signal, child-reaping, timeout, or cleanup authority into reusable Animus crates.
- Creating an independent repository or opening downstream adoption changes before the feasibility receipt supports extraction.
