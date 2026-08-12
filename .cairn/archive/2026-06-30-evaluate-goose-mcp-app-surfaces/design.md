# Design: Evaluate Goose MCP Apps surface patterns for Animus

Goose MCP Apps are attractive because tool results can become interactive UI resources instead of text-only responses. Animus should evaluate this pattern before adopting it because it crosses multiple authority boundaries: extension descriptors, UI rendering, CSP, browser permissions, host callbacks, and message-to-agent flow.

## Decisions

### 1. This is evaluation-only

**Choice:** Produce an adoption matrix and safety constraints without adding runtime behavior.

**Rationale:** MCP Apps are experimental and depend on product UI surfaces Animus has not stabilized.

### 2. UI metadata is separable from UI execution

**Choice:** Evaluate a data-only descriptor shape for UI resource URI, MIME/profile tag, title, preferred border, size hints, CSP domains, permission requests, and callback channels.

**Rationale:** Metadata can be validated in reusable core, but rendering and callbacks belong to host adapters.

### 3. CSP and permission policy must be deny-by-default

**Choice:** Any future descriptor should default to no remote domains, no nested frames, no device permissions, no clipboard write, and bounded callbacks unless explicitly admitted.

**Rationale:** Interactive UIs can become data exfiltration or prompt-injection surfaces if permissions are broad.

### 4. Chat callbacks are host-mediated events

**Choice:** Any future UI-to-agent message must flow through host-control and permission policy as a typed event rather than directly appending model-visible messages.

**Rationale:** User intent from UI interactions is valuable, but untrusted UI code must not bypass prompt assembly or audit trails.

### 5. Promotion requires product-surface prerequisites

**Choice:** Implementation should wait for MCP/ACP interop, TUI/headless host surfaces, host-control protocol, and security review stack evidence.

**Rationale:** Interactive extension UIs should not bypass the safer standards and policy foundations.
