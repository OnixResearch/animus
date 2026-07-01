# Goose recipe workflow adaptation

Animus adapts Goose-style recipes as Animus-owned workflow contracts. Nickel is the source of truth in `config/recipe-workflows.ncl`; `policy/recipe-workflows/generated/recipe-workflows.json` is a derived interchange artifact checked by `scripts/config/check-nickel-configs`.

The reusable adapter layer validates recipe manifests as data only. It does not read files, execute retry checks, load toolsets, select providers, or grant permissions.

## Runtime boundary

- `RecipeWorkflow` validates identity, instructions, prompt template, activities, parameters, model profile, capability ceilings, toolsets, validation gates, retry policy, subrecipes, response schema, and provenance.
- `bind_recipe_parameters` validates `{{variable}}` references against declared parameters and safe built-ins before prompt assembly.
- File parameters must be required and supplied as host content-reference labels; they cannot carry defaults.
- Select parameters must declare allowed options, and defaults or supplied values must match those options.
- `validate_recipe_output` records success only after required structured fields match the declared schema.
- `plan_recipe_retry` records host-effect requests with capability, sandbox, timeout, cleanup, and receipt labels. Shell effects are rejected before planning.
- `RecipeTrustReceipt` uses BLAKE3 identities. Pending or untrusted recipes have no influence over prompt, tools, model selection, memory, or workflow permissions.

## Verification

Focused verification for this surface:

```sh
cargo test -p animus-adapters --locked
scripts/config/check-nickel-configs
nix run .#cairn -- validate --root .
nix run .#cairn -- gate proposal adapt-goose-recipe-workflows --root .
nix run .#cairn -- gate design adapt-goose-recipe-workflows --root .
nix run .#cairn -- gate tasks adapt-goose-recipe-workflows --root .
```

The adapter tests include positive workflow, binding, output, retry, and trust cases plus negative malformed-template, unsafe file/default, invalid select, missing output, retry-bypass, and untrusted-provenance cases.
