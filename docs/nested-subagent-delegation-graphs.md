# Nested subagent delegation graphs

Nested delegation is represented as explicit graph data: safe node IDs, parent/child edges, ancestry proofs, depth policy, host-admission receipts, authority/budget descriptors, terminal statuses, evidence references, and safe receipt hashes.

Reusable logic validates acyclicity, parent/child consistency, host admission, depth bounds, monotonic authority, budget narrowing, cancellation, fan-in, and influence rollups over already-supplied DTOs.

Non-claims: automatic recursive spawning, default subagent workers, provider calls, background schedulers, queue selection, retries, polling, persistence, UI tree controls, raw transcript import, and workspace mutation remain host-owned.
