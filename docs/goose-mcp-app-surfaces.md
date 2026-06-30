# Goose MCP Apps surface evaluation

Animus treats Goose MCP Apps as a bounded product-surface reference for interactive tool-result UI metadata. This evaluation does not adopt iframe rendering, browser permission requests, JavaScript bridges, persistent app windows, or direct UI-to-model message injection.

## Evaluation matrix

| Axis | Animus evaluation | Default decision |
| --- | --- | --- |
| UI resource identity | Record a stable `ui://`-style resource URI or descriptor identity as data only. | Data-only metadata may be adopted. |
| MIME/profile tags | Record MIME/profile labels for future host dispatch without loading content. | Data-only metadata may be adopted. |
| CSP domains | Classify declared resource domains; wildcard or insecure domains are rejected. | No remote domains unless explicit host policy admits them. |
| Permission requests | Classify camera, microphone, geolocation, screen-capture, and clipboard-write as sensitive. | Sensitive browser/device permissions are denied by default. |
| Host-context sync | Allow bounded labels such as theme or sizing; reject ambient host-control labels. | Data-only labels may be adopted. |
| Callback channels | Require bounded host-mediated callback labels and lifecycle policy. | No callbacks unless explicit host policy admits them. |
| Sizing | Require bounded or host-fit sizing policy. | Unbounded windows are not adopted. |
| Lifecycle bounds | Require passive or bounded-callback lifecycle policy. | Unbounded callbacks are rejected. |
| Adoption risk | Return a data-only, needs-host-policy, or reject-default classification with a risk reason. | Reject unsafe defaults before UI execution exists. |

## Descriptor sketch

`animus-adapters` exposes a data-only `McpAppDescriptorDraft` with resource identity, MIME/profile tag, title, CSP domains, permission labels, host-context labels, callback labels, sizing policy, lifecycle policy, and explicit policy-admission flags. Validation inspects only descriptor data.

Reusable validation must not render HTML, run JavaScript, load remote resources, request browser permissions, dispatch callbacks, or execute host-control behavior. Future TUI/headless hosts may consume accepted descriptors only after their own policy, sandbox, confirmation, and host-control gates run.

## Deny-by-default constraints

- Wildcard or insecure CSP domains are rejected for default behavior.
- Camera, microphone, geolocation, screen capture, and clipboard write require future explicit host policy and are denied by default.
- Direct model-visible chat-message callbacks are rejected; future UI-originated intent must flow through typed host-control events and prompt-assembly policy.
- Ambient host-control labels such as shell or host-command access are rejected.
- Callback channels require bounded lifecycle policy and host admission.
- Evaluation fixtures are evidence only; they do not claim runtime UI execution.

## Focused checks

```sh
cargo test -p animus-adapters --locked mcp_app::
cargo test -p animus-adapters --locked
cargo clippy -p animus-adapters --all-targets --locked -- -D warnings
```
