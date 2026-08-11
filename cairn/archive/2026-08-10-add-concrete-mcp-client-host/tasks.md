## Phase 1: Baseline, cohort, and dependencies

- [x] [serial] B1 Run current MCP bridge, tool executor, SecretSpec, authority-seal, request-context, and dogfood-readiness baselines. r[agent_harness_runtime.mcp_client_host.verification]
- [x] [serial] B2 Select one exact MCP standard-input protocol cohort and record dependency, license, advisory, and feature-isolation review. r[agent_harness_runtime.mcp_client_host.boundary]
- [x] [parallel] B3 Record the pinned Autolith revision, ISC license, adapted failure scenarios, rejected transports, and no-runtime-dependency boundary. r[agent_harness_runtime.mcp_client_host.evidence]

## Phase 2: Configuration and pure lifecycle core

- [x] [serial] C1 Add typed Nickel configuration for server posture, executable descriptor, arguments, environment references, capability root, protocol, limits, and policy identities. r[agent_harness_runtime.mcp_client_host.configuration]
- [x] [serial] [depends:C1] C2 Export deterministic runtime data and add strict Rust revalidation with BLAKE3 configuration identity. r[agent_harness_runtime.mcp_client_host.configuration]
- [x] [serial] [depends:C2] C3 Add nominal server, generation, process-attempt, transport, credential-snapshot, discovery, call, and cancellation identities. r[agent_harness_runtime.mcp_client_host.lifecycle]
- [x] [serial] [depends:C3] C4 Add the pure configured-through-stopped lifecycle reducer with required and optional outcomes. r[agent_harness_runtime.mcp_client_host.lifecycle]
- [x] [serial] [depends:C4] C5 Add bounded discovery validation for counts, names, descriptions, schemas, capabilities, and diagnostics. r[agent_harness_runtime.mcp_client_host.discovery]

## Phase 3: Standard-input shell and composition

- [x] [serial] [depends:C4] H1 Create non-default `animus-std-mcp` with a thin process and framing shell and no shell-command fallback. r[agent_harness_runtime.mcp_client_host.boundary] r[agent_harness_runtime.mcp_client_host.lifecycle]
- [x] [serial] [depends:H1] H2 Add initialization, discovery, call, cancellation, and teardown over exact named limits. r[agent_harness_runtime.mcp_client_host.lifecycle]
- [x] [serial] [depends:C5] [depends:H2] H3 Map discovered tools through the existing MCP bridge and trusted tool catalog. r[agent_harness_runtime.mcp_client_host.discovery]
- [x] [serial] [depends:H3] H4 Bind calls to current model plans, authority seals, approvals, capability grants, mutation plans, schemas, and output bounds. r[agent_harness_runtime.mcp_client_host.authority]
- [x] [serial] [depends:H2] H5 Resolve operation-scoped credentials through injected services and restart only after terminal old-generation evidence. r[agent_harness_runtime.mcp_client_host.lifecycle]
- [x] [serial] [depends:H3] H6 Route server instructions through untrusted current-request contributions without durable retention. r[agent_harness_runtime.mcp_client_host.context]

## Phase 4: Fake servers and adversarial evidence

- [x] [parallel] [depends:H2] [depends:H3] [depends:H4] [depends:H5] [depends:H6] T1 Add a fixture child server for successful initialization, bounded discovery, one call, cancellation, and clean teardown. r[agent_harness_runtime.mcp_client_host.verification]
- [x] [parallel] [depends:H2] [depends:H3] T2 Add malformed frame, duplicate tool, schema drift, oversized catalog, timeout, early exit, and unsupported transport fixtures. r[agent_harness_runtime.mcp_client_host.verification]
- [x] [parallel] [depends:H4] T3 Add untrusted server, denied capability, missing approval, stale model plan, stale server generation, child-grant expansion, and mutation-conflict fixtures. r[agent_harness_runtime.mcp_client_host.authority] r[agent_harness_runtime.mcp_client_host.verification]
- [x] [parallel] [depends:H5] T4 Add credential drift, resolver failure, stale-process survival, cancellation timeout, and blocked restart fixtures. r[agent_harness_runtime.mcp_client_host.lifecycle] r[agent_harness_runtime.mcp_client_host.verification]
- [x] [parallel] [depends:H6] T5 Add receipt and context leak fixtures for raw frames, arguments, results, instructions, credentials, environment values, paths, and unrestricted errors. r[agent_harness_runtime.mcp_client_host.evidence] r[agent_harness_runtime.mcp_client_host.verification]

## Phase 5: Packaged smoke and closeout

- [x] [serial] [depends:T1] [depends:T2] [depends:T3] [depends:T4] [depends:T5] D1 Add a packaged non-default standard-input smoke that uses no network and no live credential. r[agent_harness_runtime.mcp_client_host.verification]
- [x] [serial] [depends:D1] D2 Document configuration, lifecycle, transport cohort, authority order, cancellation, evidence ladder, and non-claims. r[agent_harness_runtime.mcp_client_host.boundary] r[agent_harness_runtime.mcp_client_host.evidence]
- [x] [serial] [depends:D2] D3 Update request-lifecycle, contracts, components, std-host, dogfood-readiness, facade, roadmap, and README indexes. r[agent_harness_runtime.mcp_client_host.boundary]
- [x] [serial] [depends:D3] D4 Add focused Cairn and Nix rails for policy drift, feature isolation, fake servers, authority, cancellation, and safe receipts. r[agent_harness_runtime.mcp_client_host.verification]
- [x] [serial] [depends:D4] D5 Run focused tests, formatting, Clippy, dependency isolation, Cairn validation and gates, and relevant Nix checks. r[agent_harness_runtime.mcp_client_host.verification]
- [x] [serial] [depends:D5] D6 Review dry-run sync and archive plans, validate accepted requirement promotion, and archive only with current evidence. r[agent_harness_runtime.mcp_client_host.verification]
