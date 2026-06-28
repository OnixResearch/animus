# agent-harness-runtime Change Spec: Clankers remote admission receipt adaptation

## Requirements

### Requirement: Typed remote/headless admission requests

r[agent_harness_runtime.clankers_remote_admission.requests] Animus MUST represent remote or headless entrypoint admission as typed requests with audience, actor, entrypoint, session or workspace scope, requested action, resource, ability, credential evidence, local caveats, policy hash, and expiry or window metadata.

#### Scenario: Prompt submission admission normalizes

r[agent_harness_runtime.clankers_remote_admission.requests.prompt]
- GIVEN a headless or remote client requests prompt submission for a session
- WHEN admission normalization runs
- THEN Animus produces a typed admission request with audience, actor, entrypoint, session scope, and prompt-submit ability
- AND the raw prompt body is not embedded in the admission receipt.

### Requirement: Admission before side effects

r[agent_harness_runtime.clankers_remote_admission.before_effects] Admission checks MUST complete before session create, attach or resume, prompt submission, tool execution, memory mutation, artifact delivery, or workflow command side effects occur.

#### Scenario: Missing credential blocks session mutation

r[agent_harness_runtime.clankers_remote_admission.before_effects.missing_credential]
- GIVEN an entrypoint requires a credential or session grant
- AND no credential evidence is supplied
- WHEN admission evaluates the request
- THEN admission denies the request before session state changes
- AND no prompt, tool, memory, delivery, or workflow side effect is invoked.

### Requirement: UCAN-aligned authority and local caveats

r[agent_harness_runtime.clankers_remote_admission.authority] Admission MUST evaluate credential audience, expiry, revocation or verifier status, requested resource/ability, and local caveats, and local caveats MUST NOT expand authority beyond verified grants.

#### Scenario: Wrong audience denied

r[agent_harness_runtime.clankers_remote_admission.authority.wrong_audience]
- GIVEN credential evidence targets a different audience than the current runtime or session entrypoint
- WHEN admission authority validation runs
- THEN admission denies the request with a wrong-audience reason
- AND the local caveat layer cannot override the denial into an allow.

### Requirement: Transport and credential stores are host-owned

r[agent_harness_runtime.clankers_remote_admission.host_owned] Network transports, daemon sockets, iroh, Matrix, MCP, HTTP servers, OAuth stores, credential persistence, revocation databases, and verifier backends MUST remain host-owned adapter behavior outside reusable admission core logic.

#### Scenario: Admission core does not read credential store

r[agent_harness_runtime.clankers_remote_admission.host_owned.no_store]
- GIVEN admission validation receives typed credential evidence
- WHEN reusable admission core evaluates the request
- THEN it validates the supplied evidence and policy inputs
- AND it does not read credential files, open sockets, contact OAuth endpoints, or query revocation storage directly.

### Requirement: Safe admission receipts

r[agent_harness_runtime.clankers_remote_admission.receipts] Admission receipts MUST record safe metadata such as verdict, reason class, entrypoint label, resource and ability labels, audience hash, actor hash, credential hash, caveat summary, policy hash, expiry class, verifier status, and BLAKE3 receipt hash, and MUST NOT include raw tokens, private keys, raw prompts, provider payloads, transport headers, Matrix room IDs, full paths, or credentials.

#### Scenario: Admission receipt leak rejected

r[agent_harness_runtime.clankers_remote_admission.receipts.rejects_leaks]
- GIVEN an admission receipt fixture contains a raw token, private key, prompt body, provider JSON, transport header, Matrix room ID, full path, or credential body
- WHEN receipt validation runs
- THEN validation rejects the fixture with a typed leak diagnostic.

### Requirement: Remote admission verification

r[agent_harness_runtime.clankers_remote_admission.verification] The adaptation MUST include focused positive and negative tests for request normalization, before-effect gating, authorized admission, missing/expired/revoked/wrong-audience credentials, verifier-unavailable status, local caveat denial, host-owned store/transport boundaries, unsupported entrypoints, and receipt redaction.

#### Scenario: Closeout evidence covers admission fixtures

r[agent_harness_runtime.clankers_remote_admission.verification.closeout]
- GIVEN maintainers intend to archive the remote admission receipt adaptation
- WHEN closeout validation runs
- THEN evidence names focused request, before-effect, authority, host-boundary, receipt, workspace, Cairn validation, and Cairn gate checks
- AND any skipped rail includes a concrete blocker and next-best validation.
