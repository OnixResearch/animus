# Fixture qualification: consume-serving-readiness-admission

- Subject: uncommitted serving-admission work on top of `c5fe338ed2168583ee2c5456baa956e93780417f` (Animus #15), itself a child of `365b0ee6`.
- Evidence label: `fixture`. `live_qualification: false`. No live serving owner, provider endpoint, model, GPU, or host was exercised.
- Toolchain for the cargo rows below: host nightly `rustc 1.99.0-nightly (3d50c25bc 2026-07-16)`, not the flake-pinned toolchain.

## Results observed before the full flake run

| Check | Result |
|---|---|
| `scripts/config/export-nickel-configs` into a temporary root, compared with every tracked output | identical |
| `scripts/cairn/check-serving-admission --root .` | pass: 44 core tests, 12 host-path tests, 4 Mesh tests, clippy clean, `--locked` builds |
| `scripts/cairn/check-component-catalog --root .` | pass, 0 issues |
| `scripts/cairn/check-runtime-facade-inventory --root .` | pass |
| `scripts/cairn/check-evidence-safety --root .` | pass |
| `cargo check --workspace --all-targets --locked` | pass |
| `cargo test --workspace --locked` with `ANIMUS_RLM_HOST_PROCESS_FIXTURE` set to coreutils `printf` | pass: 190 suites, 2369 passed, 0 failed, 5 ignored |
| `cargo clippy --keep-going --workspace --all-targets -- -D warnings` | fail: 6 findings, none in files this change touches |
| `scripts/cairn/check-roadmap --root .` | exit 1 with one issue: the accepted pre-existing `ROADMAP_UNCLASSIFIED_ACTIVE_CHANGE add-acp-coding-agent`. This change is classified in `.cairn/roadmap.ncl`. |
| `cairn validate --root .` via the packaged `/nix/store/6k3x42szvrrmyxb1yyw6g54vx2g4zmya-cairn/bin/cairn` (`nix run .#cairn` stalled under host load) | valid, 0 issues, 0 change issues |

The six clippy findings are `manual_assert_eq` and `map_or_identity` in `animus-adapter-weave/src/adapter.rs`, `animus-runtime/src/aeneasproofcampaign.rs` (two sites), `animus-runtime/src/pendinginputvault/tests.rs`, and two in `animus-frozen-policy` from #15. The weave site and both `aeneasproofcampaign.rs` sites also fail on a clean `365b0ee6` export with the same host toolchain.

## Accepted pre-existing roadmap failure

The owner accepted `ROADMAP_UNCLASSIFIED_ACTIVE_CHANGE add-acp-coding-agent` as pre-existing. The #15 change reproduced it on a clean export of `365b0ee6` with:

```sh
rm -rf /tmp/s1-head && mkdir /tmp/s1-head && git archive HEAD | tar -x -C /tmp/s1-head && cd /tmp/s1-head && TMPDIR=/tmp nix develop --builders '' "path:/home/brittonr/.cargo-target/producer-work/animus-system-one" -c sh scripts/cairn/check-roadmap --root .
```

`HEAD` was `365b0ee6` when that ran. This change also reproduced it on `git archive 365b0ee6 | tar -x -C /tmp/animus-clean-365b` with `sh scripts/cairn/check-roadmap --root .`: exit 1, the same single issue.

## Full flake result

`nix flake check -L --keep-going --builders '' --max-jobs 2 --cores 8 path:$PWD` on this tree, committed unchanged as `1c370d5029a2667051c46bdfa1919e78e65e2f4b`: exit 0; 145 x86_64-linux checks, 151 derivations built, 0 failures, 8023 s. aarch64-linux outputs are not evaluated by default. The flake's own clippy check on the pinned toolchain passes; the six findings above come only from the newer host nightly.

## Non-claims

Fixture results do not establish live endpoint behavior, GPU scheduling, production availability, provider honesty, capacity reservation, or pricing. Task 3.3 (live endpoint qualification) is not performed.
