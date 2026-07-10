# ast-grep Agent Boundary Audits

Animus uses ast-grep as an inventory-only structural audit rail for syntax shapes that can threaten reusable agent boundaries. These scans are candidate review evidence, not proof of harness safety, adapter correctness, provider behavior, Steel policy enforcement, or release readiness.

## Profiles

| Profile | Scope | Posture | Non-claims |
|---|---|---|---|
| core | `crates/animus-core/src/**/*.rs` | inventory-only | not-a-safety-proof, not-release-readiness |
| adapter-dto | `crates/animus-adapters/src/**/*.rs` | inventory-only | not-adapter-correctness, not-provider-behavior |
| runtime | `crates/animus-runtime/src/**/*.rs` | inventory-only | not-runtime-safety, not-provider-behavior |
| std-shell | `crates/animus-std/src/**/*.rs` | inventory-only | not-shell-correctness, not-release-readiness |
| steel-loop-fabric | `crates/animus-steel/src/**/*.rs` | inventory-only | not-steel-policy-enforcement, not-runtime-safety |
| tui-host | `crates/animus-tui/src/**/*.rs` | inventory-only | not-terminal-safety, not-provider-behavior |
| generated-config | `docs/**/*.md cairn/**/*.ncl` | inventory-only | not-config-correctness, not-release-readiness |

## Inventory rules

| Rule | Category | Severity | Promotion guard |
|---|---|---|---|
| ambient-filesystem-call | ambient-authority | hint | positive and negative fixtures must pass before warning/blocking posture |
| ambient-process-command | ambient-authority | hint | positive and negative fixtures must pass before warning/blocking posture |
| unsafe-block | unsafe-code | hint | positive and negative fixtures must pass before warning/blocking posture |
| panic-macro | panic-path | hint | positive and negative fixtures must pass before warning/blocking posture |
| direct-provider-client | provider-boundary | hint | positive and negative fixtures must pass before warning/blocking posture |

## Receipt identity

`check-ast-grep-agent-boundary-audits` captures ast-grep version, BLAKE3 rule-bundle identity, scan scopes, bounded finding counts, fixture outcomes, and non-claim labels. Receipts intentionally omit raw source payloads, provider payloads, credentials, private paths, and unbounded output.

## Validation

Run:

```sh
scripts/cairn/check-ast-grep-agent-boundary-audits --root .
nix build .#checks.x86_64-linux.ast-grep-agent-boundary-audits --no-link -L
```

The rail validates Nickel profile/doc drift, ast-grep rule fixtures, BLAKE3 bundle identity, safe receipt fields, and inventory scan execution.