# Design: TUI session context status rail

## Context

Animus already defines session-tree-ledger DTOs (`adapt-pi-session-tree-ledger`), model-dispatch and provider-registry DTOs (`adapt-clankers-cost-model-routing`, `adapt-pi-provider-registry-shell`), retrieval budgets (`adapt-hermes-retrieval-budgets`), process-job profiles (`adapt-clankers-process-job-profiles`), and semantic status authority (`adapt-agent-status-authority`). A status rail should project these into a persistent UI region without re-owning any of them.

## Decisions

### 1. Pickers project ledger and registry DTOs

**Choice:** The session picker projects session-tree-ledger DTOs and the model picker projects provider-registry/model-dispatch DTOs. Switching issues typed commands to the host; the TUI does not mutate sessions or models directly.

**Rationale:** Keeps ownership in the std shell and ledger; the TUI is a view and command issuer.

### 2. Context gauge and cost are bounded projections

**Choice:** The context gauge renders retrieval-budget DTOs and the cost overlay renders cost-model-routing DTOs, both bounded and redacted.

**Rationale:** Avoids unbounded numeric or payload growth and keeps credential/provider bodies out.

### 3. Status follows semantic authority

**Choice:** The status bar follows status-authority precedence for busy/idle/streaming/waiting states rather than inventing TUI-local status.

**Rationale:** Keeps the TUI consistent with headless and receipt surfaces.

### 4. Auxiliary panels are read-only projections

**Choice:** Git/branch, environment, and todo panels are read-only projections of process-job-profile and existing status surfaces; mutating actions issue typed commands to the host.

**Rationale:** No ambient filesystem, git, or process authority in the TUI.

### 5. Downstream-only composition

**Choice:** The rail consumes accepted DTOs as-is. No new field is added to a reusable crate.

**Rationale:** Keeps the TUI strictly downstream.

## Risks / Trade-offs

- Cost/usage numbers can drift from provider reality; render them as display-only with freshness metadata.
- Model selection could leak credentials if it reads the registry naively; project labels and capabilities only.
- Context gauge can overflow on huge contexts; bound and normalize the projection.
