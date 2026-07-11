# Agent Notes

## Operating loop

- Work from the Animus repo root. This checkout is git-only, so use `git` here unless `.jj/` is introduced later.
- Read this file, `README.md`, and the docs for the touched surface before editing.
- Use native Cairn lifecycle as the spec and plan surface. Accepted requirements live in `cairn/specs/agent-harness-runtime/spec.md`; active work lives in `cairn/changes/<slug>/`.
- Treat `cairn/changes/<slug>/tasks.md` as the implementation plan for Ralph-style work. Do not create root `specs/` or `IMPLEMENTATION_PLAN.md` unless a future accepted change explicitly adopts them.
- If Cairn validation reports an active change that normal file listings hide, inspect `cairn/changes` with ignored/untracked files included before assuming the lifecycle is clean.

## Implementation constraints

- Preserve functional-core / imperative-shell boundaries. Reusable crates must not gain ambient filesystem, process, network, credential, clock, terminal, provider, or persistence authority.
- Keep non-obvious numeric values named in code and scripts. Use BLAKE3 for stack-owned identities unless an interoperability surface requires a different algorithm.
- Keep positive and negative tests or fixtures together for validators, DTOs, adapters, policies, receipts, and lifecycle rails.
- Capture progress, blockers, evidence, and status in Cairn tasks/evidence/docs, not here. Keep this file operational and compact.

## Backpressure ladder

- Docs or lifecycle edits: run `nix run .#cairn -- validate --root .` plus the relevant `scripts/cairn/check-* --root .` rail named by the changed surface.
- Rust core or adapter edits: run the focused existing test before changes when practical, then after changes run at least `cargo test -p <crate>`; widen to `cargo check --workspace --all-targets` and clippy when the touched surface warrants it.
- Nickel config edits: change `config/repo.ncl`, regenerate with `scripts/config/export-nickel-configs`, then run `check-nickel-configs` or `nix run .#check-nickel-configs`.
- Steel policy or Ralph profile edits: run `scripts/cairn/check-steel-runtime-policy --root .`. For `scripts/ralph-drain-cairns.scm` or `scripts/ralph-drain-stack.scm`, use Pi's Steel runner with the matching file and `args: ["--self-test"]`; a host-provided Steel wrapper is acceptable only when it supplies Pi's argument prelude. Stack-helper plan checks must omit `--run`.
- Release, dogfood, receipt, and conformance surfaces have named rails in `README.md`; run the smallest relevant rail and record concrete blockers instead of widening claims.

## Commit hygiene

- Preserve user changes. Do not `git add -A` when unrelated modified or untracked files are present.
- Use concise why-focused commit messages. Do not push unless explicitly asked. Never open or manage pull requests.
