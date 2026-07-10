# Goose recipe workflow adaptation

Animus adapts Goose-style recipes as Animus-owned workflow contracts. Nickel is the source of truth in `config/recipe-workflows.ncl`; `policy/recipe-workflows/generated/recipe-workflows.json` is a derived interchange artifact checked by `scripts/config/check-nickel-configs`.

The reusable adapter layer validates recipe manifests as data only. It does not read files, execute retry checks, load toolsets, select providers, schedule DAG steps, run steps in parallel, persist workflow state, or grant permissions.

## Runtime boundary

- `RecipeWorkflow` validates identity, instructions, prompt template, activities, parameters, model profile, capability ceilings, toolsets, validation gates, retry policy, subrecipes, response schema, and provenance.
- `bind_recipe_parameters` validates `{{variable}}` references against declared parameters and safe built-ins before prompt assembly.
- File parameters must be required and supplied as host content-reference labels; they cannot carry defaults.
- Select parameters must declare allowed options, and defaults or supplied values must match those options.
- `validate_recipe_output` records success only after required structured fields match the declared schema.
- `plan_recipe_retry` records host-effect requests with capability, sandbox, timeout, cleanup, and receipt labels. Shell effects are rejected before planning.
- `RecipeTrustReceipt` uses BLAKE3 identities. Pending or untrusted recipes have no influence over prompt, tools, model selection, memory, or workflow permissions.

## Optional recipe DAGs

Recipe manifests may declare `dag_steps` as safe workflow data. Each step has a step ID, kind, dependencies, declared parameter or upstream `step.output` input references, declared output references, validation gate references, retry check references, subrecipe references, and capability ceilings.

The reusable core validates DAG structure before effects: duplicate steps, duplicate edges, unknown dependencies, self-edges, cycles, unsafe labels, over-limit graphs, undeclared fan-in outputs, untrusted subrecipes, capability widening, undeclared gates, and retry bypasses fail closed. Accepted DAGs produce deterministic topological batches and safe receipts with BLAKE3 identity hashes, bounded counts, issue codes, and redaction markers only.

`plan_recipe_dag` accepts host admission denials for capabilities, validation gates, and subrecipes and returns blocked-step explanations plus recomputed ready batches. It is still a plan-only function: hosts own scheduling, concurrency, cancellation, provider dispatch, tool execution, cleanup, and persistence.

## Verification

Focused verification for this surface:

```sh
cargo test -p animus-adapters flow --locked
scripts/config/check-nickel-configs
nix run .#cairn -- validate --root .
nix run .#cairn -- gate proposal adapt-goose-recipe-workflows --root .
nix run .#cairn -- gate design adapt-goose-recipe-workflows --root .
nix run .#cairn -- gate tasks adapt-goose-recipe-workflows --root .
```

The adapter tests include positive workflow, binding, output, retry, trust, linear-compatibility, parallel DAG, fan-in, validation-gate, subrecipe, retry-plan, and blocked-step cases plus negative malformed-template, unsafe file/default, invalid select, missing output, retry-bypass, untrusted-provenance, duplicate-step, unknown-step, self-edge, cycle, undeclared-output, untrusted-subrecipe, capability-widening, unsafe-label, and receipt-leak cases.
