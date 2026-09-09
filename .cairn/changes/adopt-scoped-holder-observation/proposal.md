# Adopt scoped holder observations

## Why

The scoped dispatch callback exposes the original holder admission but the current Basalt pin lacks non-consuming rechecks. Repeating invocation verification spends replay authority rather than observing current token facts.

## Scope

Adopt published Basalt `b10a0465bc3ee966a4864d6e779184f4eae0cd61`, containing UCAN `3a2aa043068a82663cf72ec05ed5d56a7d65ecd5`. Prove that the existing scoped holder reference composes with the owner recheck API. Prefer existing public contracts over another wrapper or copied verifier.

No dispatch gate, effect permission, replay ownership, or generic no-std API is widened. Resource custody and effect-time release authority remain consumer responsibilities.
