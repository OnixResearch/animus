# Design: Adapt Goose security review stack for Animus

Goose's security stack is valuable because it layers pattern detection, user permission modes, and a context-aware adversary reviewer before risky tools execute. Animus should adapt the stack as deterministic findings and advisory review inputs that remain below hard authority, sandbox, confirmation, and redaction constraints.

## Decisions

### 1. Deterministic scanners produce safe findings

**Choice:** Define a pure scanner over bounded tool-call text, action class, target labels, and recent context summaries. Findings carry stable IDs, categories, confidence, safe explanations, and redacted evidence hashes.

**Rationale:** Deterministic findings are testable, auditable, and can run without external services.

### 2. Classifiers and adversary reviewers are optional policy inputs

**Choice:** Optional ML classifiers or adversary reviewers return allow, warn, or block recommendations with explanations. Their output can tighten policy but cannot grant authority, bypass sandboxing, suppress confirmation, or expose secrets.

**Rationale:** Context-aware review is useful but should not become a privileged oracle.

### 3. Fail behavior is risk-class specific

**Choice:** High-risk action review failures may fail closed according to policy. Low-risk or explicitly advisory classifiers may degrade to deterministic scanner-only review with a receipt.

**Rationale:** Goose's fail-open adversary mode is convenient, but Animus needs configurable failure semantics for sensitive workflows.

### 4. Review outcomes bind through the permission kernel

**Choice:** Security findings flow into permission resolution and confirmation prompts as evidence. The final decision still requires authority, capability, sandbox, and confirmation checks.

**Rationale:** A security scanner should explain risk and block dangerous paths, not create an alternate execution path.

### 5. Receipts never include raw suspicious content

**Choice:** Findings and review receipts expose category, confidence, action class, bounded snippets only when public, and BLAKE3 hashes for raw tool arguments or context.

**Rationale:** Security logs are themselves sensitive and must not become a leakage channel.
