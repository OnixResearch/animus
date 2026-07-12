# Nickel runtime-policy contracts

r[impl agent_harness_runtime.nickel_policy_contracts.authority_boundary]

Animus rejects malformed human-authored runtime policy before projection while retaining runtime authority in Rust and host shells. `policy/contracts/primitives.ncl` supplies shared exact-schema, bounded-integer, closed-enum, lowercase-BLAKE3, safe-path, and uniqueness contracts with stable diagnostic codes. Family source files add whole-value invariants.

## Ownership inventory

| Family | Nickel source | Checked projection | Rust DTO / semantic owner | Focused checker |
| --- | --- | --- | --- | --- |
| Model routing | `policy/model-routing/policy.ncl` | `policy/model-routing/generated/model-routing-policy.json` | `animus-adapters` `ModelRoutingPolicy` validation and pure `route_model`; hosts supply catalogs and perform provider dispatch | `scripts/cairn/check-nickel-policy-contracts --root .` plus generated-policy tests in `animus-adapters` |
| Process jobs | `policy/process-jobs/policy.ncl` | `policy/process-jobs/generated/process-job-profile-policy.json` and `sample-profile-manifest.json` | `animus-runtime::background` pure profile resolution; hosts own discovery, process/queue execution, clocks, storage, and notifications | `scripts/cairn/check-process-job-profiles --root .` and `scripts/cairn/check-nickel-policy-contracts --root .` |
| Steel runtime | `policy/steel-runtime/policy.ncl` | `policy/steel-runtime/generated/steel-runtime-policy.json` | `animus-steel` profile activation plus `animus-runtime` adapter-boundary dispatch; hosts own model/tool effects | `scripts/cairn/check-steel-runtime-policy --root .` and `scripts/cairn/check-nickel-policy-contracts --root .` |
| Tool catalog | `policy/tool-catalog/catalog.ncl` | `policy/tool-catalog/generated/tool-catalog-schema.json` and `tool-catalog.json` | `animus-adapters::tool` metadata validation and `animus-runtime::tool_executor_kit` final admission; hosts inject executors | `scripts/cairn/check-tool-catalog --root .` and `scripts/cairn/check-nickel-policy-contracts --root .` |

## Checked projection identity

Run:

```sh
scripts/config/export-policy-projections --execute
scripts/config/export-policy-projections --check
```

The export binds every generated artifact to:

- its Nickel source and family contract BLAKE3 identity;
- the shared prelude BLAKE3 identity;
- the exact exported field and `nickel-json-export-v1` evaluator profile; and
- the generated JSON BLAKE3 identity.

The checked sidecar is `policy/generated/policy-projection-receipts.json`. The focused rail verifies the sidecar and generated JSON are fresh, executes positive and negative Nickel fixtures, and runs Rust tests that deserialize projections and compare their normalized DTO meaning.

## Family invariants

- Model routing requires exact schema identity, unique ordered roles, lower-score downgrade targets, an acyclic downgrade graph, ordered thresholds, integer non-negative weights, closed behaviors, and a lowercase BLAKE3 policy identity.
- Process jobs require one execution form, admitted backends, confined lexical paths, allowed unique environment and metadata keys, positive bounded resources, and deterministic profile identity.
- Steel runtime requires unique profile and script identities, exactly one enabled default, positive turn budgets, closed host actions, fail-closed fallback, hash-only receipts, safe script paths, and lowercase BLAKE3 script identities.
- Tool catalogs require closed vocabularies, unique names and capabilities, schema references, effect/capability consistency, prompt-or-deny approval for dangerous capabilities, and redaction for secret-adjacent tools.

## Authority boundary and non-claims

Nickel contract success establishes authoring-time shape and the declared source invariants only. Reusable Rust crates do not evaluate Nickel, read policy files, inspect the environment, discover providers or tools, execute processes, or emit authoritative host receipts. Rust remains responsible for external-data revalidation and runtime admission; host shells remain responsible for files, processes, provider/tool calls, clocks, credentials, persistence, and execution evidence. A fresh projection does not prove authorization, confinement, provider truth, tool safety, successful execution, release eligibility, or whole-system correctness.
