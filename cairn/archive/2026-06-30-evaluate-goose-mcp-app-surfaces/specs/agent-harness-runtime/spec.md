# agent-harness-runtime Change Spec: Goose MCP Apps surface evaluation

## Requirements

### Requirement: MCP Apps evaluation matrix is explicit

r[agent_harness_runtime.goose_mcp_app_surfaces.evaluation_matrix] Animus MUST evaluate Goose MCP Apps patterns with an explicit matrix covering UI resource identity, MIME/profile tags, CSP domains, permission requests, host-context sync, callbacks, sizing, lifecycle bounds, and adoption risk.

#### Scenario: Unsafe UI metadata is classified

r[agent_harness_runtime.goose_mcp_app_surfaces.evaluation_matrix.fixtures]
- GIVEN evaluation fixtures include safe UI metadata and unsafe broad-domain, broad-permission, unbounded-callback, or ambient-host-control descriptors
- WHEN the MCP Apps evaluation matrix is applied
- THEN each fixture is classified with an adoption decision and risk reason
- AND no fixture claims runtime UI execution behavior.

### Requirement: UI metadata descriptors remain data-only

r[agent_harness_runtime.goose_mcp_app_surfaces.metadata_descriptor] Animus MAY define a data-only UI metadata descriptor sketch for future tool-result UI resources, but reusable validation MUST NOT render HTML, run JavaScript, open remote resources, request browser permissions, or send chat callbacks.

#### Scenario: Descriptor validation has no UI execution

r[agent_harness_runtime.goose_mcp_app_surfaces.metadata_descriptor.no_execution]
- GIVEN a metadata descriptor includes a UI resource URI, MIME/profile tag, CSP domains, permissions, and callback names
- WHEN reusable validation runs
- THEN validation inspects only descriptor data
- AND HTML rendering, JavaScript execution, remote resource loading, browser permission requests, and chat callbacks are absent.

### Requirement: Future MCP App surfaces are deny-by-default

r[agent_harness_runtime.goose_mcp_app_surfaces.security_constraints] Any future MCP App-style surface SHOULD default to no remote domains, no nested frames, no device permissions, no clipboard write, no direct model-visible message injection, and bounded host-context callbacks unless explicitly admitted by policy.

#### Scenario: Broad UI permission request is denied in evaluation

r[agent_harness_runtime.goose_mcp_app_surfaces.security_constraints.denied]
- GIVEN a candidate UI metadata descriptor requests wildcard resource domains, camera access, clipboard write, and direct chat-message callbacks without policy admission
- WHEN evaluation applies the security constraints
- THEN the descriptor is classified as rejected for future default behavior
- AND the evaluation records the required host-policy gates before reconsideration.

### Requirement: MCP Apps evaluation closeout is evidenced

r[agent_harness_runtime.goose_mcp_app_surfaces.verification] MCP Apps surface evaluation MUST include the adoption matrix, positive and negative metadata fixtures, runtime-neutrality checks, security-constraint documentation, Cairn validation, and Cairn gates before archive.

#### Scenario: MCP Apps evaluation evidence is complete

- GIVEN the Goose MCP Apps surface evaluation change is ready to archive
- WHEN closeout validation runs
- THEN evidence includes the evaluation matrix, safe and unsafe fixtures, runtime-neutrality checks, security-constraint docs, Cairn validation, and Cairn gates
- AND skipped checks record a blocker and next-best validation.
