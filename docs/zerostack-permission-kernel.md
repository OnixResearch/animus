# Zerostack permission kernel adaptation

Animus adapts Zerostack's compact permission-kernel shape as an Animus-owned, data-only contract. The reusable boundary is a pure resolver over supplied authority, sandbox, redaction, policy, prompt, session, and repeat-signal inputs. It does not load Zerostack configuration, execute tools, read files, contact providers, or grant authority.

## Permission modes

`PermissionMode` provides restrictive, read-only, guarded, standard, and yolo-style UX defaults. Mode defaults are advisory: `resolve_mode_default` checks supplied authority, sandbox, and redaction statuses before honoring a local allow. Even yolo mode returns a denial when authority is missing, the sandbox denies, or redaction policy fails.

## Pattern rules

`PatternRule` validates glob-like and regex-like descriptors with action class, safe resource pattern, source identity, trust scope, trust status, precedence, and allow/ask/deny outcome. Invalid match kinds, malformed regex-like patterns, unsupported outcomes, malformed action classes, missing sources, and untrusted descriptors fail before they can authorize or deny an action.

## Session allowlists

`SessionAllowlistEntry` binds a prompt token to action identity, resource hash, policy hash, and session label. `validate_session_allowlist` accepts only exact current-request matches and rejects invalidated, stale, mismatched-resource, mismatched-policy, mismatched-session, or mismatched-action entries.

## Repeated actions

`detect_repeated_action` evaluates repeated identical action identities using host-supplied counts, threshold, destructive classification, and policy-selected coach/ask/deny/ignore behavior. Detection is a pure pre-dispatch signal and never calls filesystem, process, network, model, tool, or provider adapters.

## Receipts

`KernelReceipt` records only bounded metadata: action class, BLAKE3 resource hash, authority status, mode outcome, optional matched-rule hash, repeated-action status, prompt status, final decision, and redaction markers. Raw prompts, credentials, full paths, provider payloads, and raw tool bodies remain excluded and are covered by receipt leak fixtures.

## Non-claims

This adaptation does not adopt Zerostack's GPL implementation, exact prompts, configuration files, filesystem model, direct process execution behavior, or provider dispatch behavior. Hosts may layer their own UI and storage around these DTOs, but reusable Animus crates keep those effects outside the pure permission core.
