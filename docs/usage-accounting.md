# Usage-cost accounting projection

The `animus-adapter-usage-accounting` crate owns a pure, non-authoritative per-agent usage-cost projection. It is a no-std decision core.

## Boundary

- `project_accounting` admits typed usage records, binds each record to an agent and session, and prices each record against the model that served it.
- A documented fallback price steps in when registry pricing is absent. Missing required prices fail the summary entry when configured.
- `project_subtrees` follows only authenticated same-session creator relationships. Unknown descendants are never invented into a subtree.
- Every projection carries the explicit `non-authoritative-estimate` label. It is not provider billing reconciliation.
- `build_accounting_receipt` emits a metadata-only receipt.

## Use

```rust
use animus_adapter_usage_accounting::{PricingPolicy, UsageRecord, project_accounting};
```

Attribution and pricing decisions stay pure. Hosts own usage ingestion, price catalogs, and billing reconciliation.

## Non-claims

- The figure is an eyeballed estimate, never a provider billing statement.
- Attribution depends on trustworthy creator relationships. Unattributed usage stays separate.
