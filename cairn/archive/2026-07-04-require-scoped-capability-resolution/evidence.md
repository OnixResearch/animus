# Evidence: require-scoped-capability-resolution

r[verify agent_harness_runtime.scoped_capabilities.verification]

## Implementation evidence

- Added runtime capability descriptors and registries with stable labels, service kinds, optional safe scopes, duplicate-label rejection, unsafe-scope rejection, and documented default model/tool labels.
- Runtime model/tool dispatch uses exact label/service/scope resolution before adapter calls; unrelated services fail closed without fallback.
- Added compatibility constructors that bind existing model/tool adapters to default capabilities.
- Added positive and negative tests for default labels, explicit scoped labels, empty labels, unsupported service kinds, unsafe scopes, duplicates, unrelated services, and missing capabilities.
- Updated runtime composition docs and facade inventory entries.

## Checks

- `nix develop -c cargo test -p animus-runtime caps --locked` — covered by focused crate suite.
- `nix develop -c cargo test -p animus-runtime admission --locked` — covered by focused crate suite.
- `nix develop -c cargo test -p animus-runtime shell --locked` — covered by focused crate suite.
- `nix develop -c cargo test -p animus-adapters -p animus-runtime -p animus-preserves -p animus-std --locked` — passed.
- `nix develop -c cargo check -p animus-runtime --no-default-features --locked` — passed.
- Gates: `nix run path:$PWD#cairn -- gate proposal/design/tasks require-scoped-capability-resolution --root .` — passed.

## Blocked rails

- Crate-wide clippy remains blocked by existing runtime/std lint debt outside this focused evidence; next best validation is the passing focused runtime tests and no-default runtime check.
