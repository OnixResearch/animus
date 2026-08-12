## Context

`scripts/ralph-drain-cairns.scm` discovers changes in one repository and can launch one Pi process per selected change. It does not aggregate sibling repositories, use fresh lifecycle snapshots between rounds, or reject a Pi run that deletes or archives more work than requested. Animus already documents Pi-style fresh-context task isolation, while Cairn exposes repository-local dependency ordering and deterministic validation.

The workspace root is not a monorepo or shared commit boundary. Every child repository owns its own instructions, lifecycle files, checks, VCS state, archive, and commit history.

## Decisions

### 1. Use Pi as the temporary imperative host

**Choice:** The new helper launches a fresh `pi -p` process for each round with stdin redirected from `/dev/null`, so print mode observes immediate EOF instead of waiting on an inherited queue or terminal input stream. It does not continue, resume, fork, or share a prior Pi session. Additional Pi options are passed as explicit argument values rather than a shell command string. A named one-hour host timeout sends `TERM` and then applies a bounded kill grace so a stalled provider call cannot block the stack indefinitely.

**Rationale:** Pi already loads repository context, can use the Cairn drain skill, runs the required implementation and validation tools, and preserves one session per attempted round for local inspection. A dedicated Animus host can replace this boundary later without changing the lifecycle state machine.

### 2. Keep plan mode as the default

**Choice:** Discovery and prompt rendering are effect-free by default. `--run` is required before the helper may spawn Pi or invoke post-archive Cairn validation.

**Rationale:** Stack-wide autonomous mutation needs an explicit operator action. Plan mode also provides comparison evidence before promotion.

### 3. Dispatch one repository-local change per Pi round

**Choice:** The supervisor selects one repository with active changes. Pi must consult that repository's Cairn dependency state, drain at most one ready change, and stop after either archive completion or a concrete blocker.

**Rationale:** Cairn, not lexical directory order in Steel, owns same-repository dependency readiness. Limiting scope to one change makes after-state checks meaningful and keeps fresh sessions bounded.

### 4. Derive progress from lifecycle state, not model text

**Choice:** The supervisor compares before/after active change names and unchecked task counts. One removed active change plus one newly created matching archive entry is archived progress. A decrease in unchecked tasks for exactly one still-active change is task progress. No lifecycle delta is no progress. Added changes, multiple removals, or changes across multiple active packages are scope breaches.

**Rationale:** Pi exit status and a model completion token are not lifecycle evidence. Filesystem-derived lifecycle state provides a small independent check without treating Steel as Cairn's validation engine.

### 5. Validate every observed archive with canonical Cairn

**Choice:** After archived progress, the helper invokes the canonical sibling Cairn flake against the repository root and requires a passing validation result before continuing.

**Rationale:** Archive-directory movement alone does not prove the resulting lifecycle tree is valid. Cairn remains the authority for lifecycle validation.

### 6. Block locally and fail closed on scope breaches

**Choice:** A non-zero Pi exit or no-progress result blocks that repository for the current run while independent repositories remain eligible. A scope breach, missing new archive, or failed post-archive validation stops the entire supervisor.

**Rationale:** Local implementation blockers should not stall unrelated work, but unexpected mutation invalidates the supervisor's assumptions and requires operator inspection.

### 7. Keep cross-repository ordering advisory

**Choice:** Repository discovery is deterministic and operator filters may provide an explicit order, but the helper does not claim that this order satisfies cross-repository dependencies.

**Rationale:** `cairn/changes/cross-repo-change-dependencies` is not yet implemented. Until explicit receipt-backed dependencies are available, sibling ordering is a scheduling preference only.

## Risks / Trade-offs

- Pi sessions can make code progress without changing task checkboxes; such a round may be conservatively classified as no progress and block the repository for the current run.
- Saved Pi sessions consume local storage, but preserve useful round-level audit context. Operators may pass Pi's `--no-session` option when persistence is not wanted.
- Canonical sibling Cairn may differ from a repository-pinned Cairn version. Pi still uses repository-local guidance for implementation; canonical Cairn is only the supervisor's post-archive validation rail.
- Serial execution is slower than cross-repository concurrency, but avoids overlapping mutation and produces simpler evidence.
