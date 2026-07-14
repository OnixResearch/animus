## Context

Animus uses Nix-owned generation commands to export Nickel-authored TOML, YAML, JSON, and raw projections into checked destinations. The standalone `nickel-export` shell now provides those generic mechanics, including exact dependency and output identities, selector handling, structured diagnostics, and freshness checks. Animus must continue to own contract meaning and all agent/runtime policy.

## Decisions

### 1. The Nix graph pins one immutable tool revision

The flake and lock select `github.com/OnixResearch/nickel-export` revision `257fafc1c746f1faf156207043a4c826bfb16d49` plus the declared evaluator cohort. Workspace-relative overrides may be used for development diagnostics only and cannot satisfy release evidence.

### 2. A Nix-owned wrapper is the only new shell boundary

The wrapper chooses the repository root, evaluator binary, request files, declared dependencies, contracts/selectors, and checked destinations. It invokes the standalone shell and then runs existing Animus-owned semantic checks. Reusable core and adapter crates do not gain process, filesystem, environment, credential, provider, or persistence authority.

### 3. Product contracts and destination authority remain local

Animus continues to define agent/runtime profile contracts, provider policy, role composition, generated destination paths, and release gates. A successful standalone export cannot admit a runtime profile or provider by itself.

### 4. Dual-run covers every current output family

Legacy and standalone generation run over identical sources, dependencies, evaluator versions, selectors, contracts, formats, and expected destinations. Exact comparison covers Cargo and lint config, pre-commit YAML, recipe catalog exports, and runtime/role profile JSON. Negative contract fixtures compare rejection class and absence of success receipts or destination mutation.

### 5. Rollback is fail-closed

Current Nix generation commands stay authoritative until all positive and negative destinations complete one full validation cycle. Unexplained drift preserves both outputs and identities, blocks the cutover, and requires paired regression fixtures before retry.

### 6. Shared logic stays a functional core

Normalization, exact identity, admission, receipt/manifest construction, and freshness remain pure operations in the standalone core. Nix evaluation, external process execution, file access, writes, config semantics, and release orchestration remain in Animus-owned shells.

## Risks / Trade-offs

- Selector and raw-output behavior can reveal previously hidden newline or scalar-format differences.
- Dual-run adds temporary Nix check cost.
- Exact output agreement does not prove profile or provider-policy correctness.
- A conservative secret heuristic cannot prove that generated configuration is safe to publish.

## Non-Goals

- Changing agent-harness runtime behavior or provider admission.
- Adding Nickel evaluation to ordinary runtime crates.
- Moving destination or release authority to the shared repository.
- Treating export compatibility as runtime correctness, deployability, or release eligibility.
