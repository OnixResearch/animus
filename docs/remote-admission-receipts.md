# Remote admission receipts

Animus adapts the Clankers remote-admission pattern as a transport-neutral adapter contract. The reusable core validates typed admission inputs and emits safe receipts; hosts own transports, credential stores, revocation lookups, verifier backends, and persistence.

## Request shape

`RemoteAdmissionRequest` contains:

- audience and actor labels;
- entrypoint (`headless_rpc`, `mcp`, `remote_session`, `automation`, or `unsupported`);
- session/workspace scope;
- action (`session_create`, `attach_resume`, `prompt_submit`, `tool_execute`, `memory_mutate`, `artifact_deliver`, or `workflow_command`);
- resource and ability labels;
- optional credential evidence hash plus credential audience/resource/ability, expiry, and verifier status;
- local caveat status (`allow`, `deny`, or `prompt_required`);
- optional policy hash.

## Evaluation rules

1. Unsupported entrypoints deny before side effects.
2. Missing policy or missing credential evidence denies before side effects.
3. Wrong audience, expired credentials, revoked credentials, verifier-unavailable status, and missing resource/ability grant deny before local caveats can allow.
4. Local caveats may deny or require prompt, but they never expand authority beyond verified credential evidence.
5. Allowed decisions and denied decisions both produce safe receipts.

## Receipt safety

`RemoteAdmissionReceipt` records verdict, reason, entrypoint, action, resource, ability, audience hash, actor hash, credential hash, caveat status, policy hash, expiry status, verifier status, and a deterministic BLAKE3 receipt hash. It does not render raw tokens, private keys, prompt bodies, provider payloads, transport headers, Matrix room IDs, full paths, or credential bodies.

## Non-claims

This contract does not add a daemon, P2P transport, Matrix bridge, remote auth CLI, OAuth store, revocation database, or mandatory Basalt dependency. Those remain host adapter decisions.
