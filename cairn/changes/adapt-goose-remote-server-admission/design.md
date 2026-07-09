# Design: Adapt Goose remote server admission

Goose's remote server setup is valuable because it makes remote backend use explicit: URL, TLS, shared secret, and certificate fingerprint. Animus should adapt that admission shape without letting reusable crates open sockets, store secrets, or own certificate verification.

### 1. Remote backends are descriptors

**Choice:** Define descriptors for backend identity, endpoint label or hash, transport kind, protocol role, platform role, TLS requirement, fingerprint pin policy, secret source label, capability ceilings, workspace/session scope, and provenance.

**Rationale:** Descriptors can be reviewed, imported, and admitted without network effects.

### 2. Handshakes are host-supplied summaries

**Choice:** Hosts perform network/TLS/auth probes and provide bounded summaries: observed TLS state, certificate fingerprint hash, pin match status, auth outcome, protocol version, accepted features, remote audience, and safe error class.

**Rationale:** Reusable validation can make deterministic decisions over safe evidence while transports remain host-owned.

### 3. Pinning and TOFU are explicit policy states

**Choice:** Admission supports pinned fingerprint, trust-on-first-use pending, trust-on-first-use accepted, rotated pin, and mismatch states. TOFU never silently admits a remote backend without a receipt and policy path.

**Rationale:** Goose's fingerprint ergonomics are useful, but Animus should preserve reviewable security evidence.

### 4. Secrets are labels, never values

**Choice:** Descriptors and receipts may name secret source labels and availability classes, but raw shared secrets, tokens, headers, keyring locators, and environment values are rejected from durable state.

**Rationale:** Remote backend admission needs auth evidence without leaking credentials.

### 5. No remote fallback without policy

**Choice:** A denied or unavailable remote backend cannot fall back to a local provider, different remote, or ambient host service unless an explicit policy-approved fallback descriptor is supplied and admitted separately.

**Rationale:** Remote admission must not weaken capability or privacy boundaries after a connection failure.

### Risks

- Trust-on-first-use can normalize unsafe first connections. Mitigate with explicit pending/accepted states and safe receipts.
- Endpoint labels can reveal private infrastructure. Mitigate with hash-only endpoint identity and redacted display labels.
- Remote availability failures can tempt silent fallback. Mitigate with deny-by-default fallback classes.
