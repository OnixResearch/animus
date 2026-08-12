# Tasks

- [x] [serial] Define an optional gateway protocol profile and bounded-reference note that names OpenClaw as inspiration while rejecting daemon, WebSocket, channel, provider, and dashboard adoption. r[agent_harness_runtime.openclaw_gateway.scope]
- [x] [depends:scope] Add connect handshake DTOs for challenge evidence, protocol range negotiation, client identity, role, scopes, feature list, and safe hello snapshots. r[agent_harness_runtime.openclaw_gateway.handshake]
- [x] [depends:handshake] Add request, response, event, error, sequence, state-version, and snapshot frame validators with positive and negative fixtures. r[agent_harness_runtime.openclaw_gateway.frames]
- [x] [depends:frames] Require idempotency descriptors for side-effecting gateway methods and reject missing or mismatched retry metadata before host dispatch. r[agent_harness_runtime.openclaw_gateway.idempotency]
- [x] [depends:scope,handshake,frames,idempotency] Update docs, facade inventory, README reference, and focused validation evidence for the OpenClaw gateway adaptation. r[agent_harness_runtime.openclaw_gateway.verification]
