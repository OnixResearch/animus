# Clankers Tool Gateway delivery

Animus delivery is a policy-checked adapter boundary for tool-produced artifacts. Reusable crates validate safe metadata, plan adapter execution, record bounded outbox attempts, and hash receipts without sending files or owning platform integrations.

## Delivery requests

`ArtifactDeliveryRequest` normalizes an opaque artifact handle, BLAKE3 artifact hash, artifact kind, target kind, active context label, safe artifact label, toolset label, capability, policy status, attempt ID, retryability, and adapter outcome status.

Raw artifact bytes are rejected at normalization time. Hosts keep file contents, media bodies, paths, and platform payloads outside reusable request DTOs.

## Gateway policy

`evaluate_gateway_policy` approves only supported local/session or host-context targets with an active context and enabled toolset. It fails closed before adapter execution for unsupported platform targets, missing contexts, disabled toolsets, raw destinations, credentials, headers, cloud URLs, webhook URLs, and raw platform payloads.

## Adapter-backed attempts

`record_delivery_attempt` records safe outbox metadata after policy evaluation. A successful attempt requires an approved policy, an available injected adapter, and a successful adapter outcome. Missing, unavailable, denied, or malformed adapters produce typed failures and never emit success metadata.

## Outbox retry

`plan_delivery_retry` references a persisted attempt ID, checks whether the original failure is retryable, revalidates current policy, and requires current adapter availability before returning an executable retry plan. Policy-denied and other non-retryable attempts preserve their original safe failure class.

## Receipts

`DeliveryReceipt::validate` rejects forbidden destination URLs, tokens, headers, full paths, raw artifact bytes, and platform payloads. Valid receipts compute a deterministic BLAKE3 hash over attempt ID, status, retryability, target kind, artifact kind, safe artifact label, artifact hash, adapter availability, adapter outcome, safe platform handle, and error class.

## Non-claims

This contract does not implement Matrix delivery, webhooks, cloud storage, filesystem writes, credential handling, media upload, arbitrary platform sends, outbox persistence, or retry scheduling. Those remain host adapter opt-ins behind the safe DTO boundary.
