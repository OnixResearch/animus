# Application-host binding completion evidence

- `scripts/cairn/check-application-host-binding --root .`: pass.
- Focused runtime tests: 4 passed.
- Negative Nickel fixtures: 6 rejected as required.
- `cargo clippy -p animus-runtime --lib --no-deps --locked -- -D warnings`: pass.
- `cargo check -p animus-runtime --no-default-features --target wasm32-unknown-unknown --locked`: pass.
- `nix build path:$PWD#checks.x86_64-linux.application-host-binding -L --no-link`: pass.
- Cairn validation and proposal, design, and task gates: pass.

Animus freezes Kamacite conformance fixture BLAKE3
`d5a082fb3ee92bc0fdbccdd0a08bbc9e072b3eea95615df8a17be70fbf9c540c`.
The accepted Kamacite receipt identity is
`1bac624751d47bd89ecba53923eda72f9cf20cdcc31dc01186882abf8da1cbb2`.

The full `nix flake check path:$PWD -L` reaches an unrelated existing
failure in `clankers-product-surfaces`. It reports a rejected marker in
`crates/animus-std/src/codingsession.rs`. This change does not modify that
file. The application-host binding Nix check passes.
