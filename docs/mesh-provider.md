# Mesh LLM provider adapter

The non-default `mesh-provider` feature integrates a private Mesh LLM gateway through an injected, caller-owned bounded transport. Animus does not start Mesh, construct HTTP or Iroh clients, discover endpoints or peers, read credentials or invite tokens, configure relays, or persist endpoint identity.

The checked Nickel profile selects the Responses API explicitly and requires an exact model selected by Animus. `auto`, the experimental `mesh` virtual model, cross-model provider fallback, missing targets, unknown required capabilities, and response model substitution fail before acceptance. Physical placement remains opaque provider metadata and cannot change the semantic model.

The host loads `/v1/models`; the no-std pure core validates private posture, safe unique IDs, bounded rows, explicit capability support or unknown state, token bounds, freshness metadata, and deterministic ordering. Dispatch requires existing capability, privacy, budget, authority, remote-admission, deadline, cancellation, and profile decisions before the injected transport may run. Retry remains same-target and externally owned; the adapter has no fallback or retry engine.

Receipts contain only safe labels and domain-separated BLAKE3 identities for endpoint configuration, catalog, request, response, authority, status, and the exact model. They omit prompts, responses, tool bodies, reasoning, credentials, invites, keys, headers, relays, addresses, paths, and raw diagnostics.

Mesh receipts do not establish remote runtime integrity, peer honesty, physical-route truth, model correctness, exact model bytes, split parity, confidentiality, performance, production readiness, or release eligibility. Optional live smoke and OnixOS deployment evidence remain separate from deterministic conformance.
