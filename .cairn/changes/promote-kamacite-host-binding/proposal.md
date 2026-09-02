## Why

Animus has a Kamacite-compatible application-host binding pilot. The pilot is non-default, fake-backed, and separate from a real product composition root.

The live headless agent already binds session authority, adapters, resources, model requests, tool stages, and runtime seals. These identities are not joined by one canonical application-host artifact.

Animus needs one real path that rejects a correct application paired with the wrong entrypoint, effect contract, host profile, adapter cohort, or resource profile.

## What Changes

- Pin one immutable Kamacite application-host binding and semantic operation cohort.
- Add a narrow Animus adapter for canonical `ApplicationHostBinding` artifacts and receipts.
- Bind one real `animus-live-steel-agent` composition to exact application, entrypoint, schema, effect, host, adapter, resource, and policy identities.
- Carry exact Kamacite `effect-operation` identities for model and tool dispatch requirements.
- Require the existing runtime authority seal and all current host rechecks after binding admission.
- Add a non-default rollout with no silent fallback to the legacy unbound path.
- Add positive and negative packaged host tests over the real composition root with controlled adapters.

## Dependencies

- Kamacite owns canonical application-host binding and semantic operation identity contracts.
- Animus owns application composition, adapter DTOs, session authority, runtime seals, dispatch, and observations.
- Basalt remains an optional separate authority gate. A Kamacite binding cannot replace it.

## Non-Goals

- Making Kamacite the Animus wire format, session ledger, policy engine, or runtime.
- Storing prompts, tool payloads, credentials, provider bodies, or host paths in binding artifacts.
- Granting model, tool, filesystem, process, network, credential, or sandbox authority.
- Enabling the new path by default before the packaged composition and migration gates pass.

## Impact

- **Adapters**: one narrow Kamacite binding adapter converts canonical artifacts into Animus-owned nominal DTOs.
- **Runtime**: one production composition root requires an exact binding before dependent model or tool dispatch.
- **Shell**: current authority, capability, resource, credential, sandbox, timeout, and cancellation checks remain final.
- **Evidence**: safe receipts retain binding and operation identities without private payloads.
- **Testing**: stale host, adapter, resource, operation, policy, and receipt identities fail before effects.
