# Standalone Nickel export cutover

Animus dual-runs its repository-owned Nickel generation through the immutable `OnixResearch/nickel-export` revision `257fafc1c746f1faf156207043a4c826bfb16d49` and Nickel `1.17.0`.

The standalone shell owns generic request validation, evaluator invocation, exact-byte BLAKE3 identities, manifests, structured diagnostics, and freshness checks. Animus continues to own every contract, selector, destination, generated artifact, runtime policy, provider and role meaning, and release decision. No reusable Rust crate depends on or invokes the exporter.

`config/nickel-export-requests.ncl` is the typed request source. Checked request JSON and canonical manifests live under `config/generated/nickel-export-requests/` and `config/generated/nickel-export-manifests/`. The first cutover cycle covers Cargo, Clippy, Dylint, pre-commit, recipe-workflow, model-runtime-profile, and agent-role-profile outputs in TOML, YAML, and JSON.

Run:

```text
scripts/config/check-nickel-configs
scripts/cairn/check-nickel-export-cutover --root .
```

The legacy direct Nickel generation remains authoritative during this validation cycle. Any byte, evaluator, selector, contract, dependency, diagnostic, or failure-class drift blocks cutover. Rollback removes only the consumer adapter/input while retaining both identities and adding paired regression fixtures.

Passing evidence proves exact declared inputs and outputs under the recorded evaluator descriptor. It does not prove evaluator equivalence, semantic correctness, deployability, consumer-policy correctness, production readiness, or release eligibility. Requests that opt into conservative secret markers do so only because reviewed policy source contains credential vocabulary; generated outputs remain subject to Animus-owned redaction and leak checks.
