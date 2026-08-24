## Context

Animus keeps credential authority in `animus-std-secretspec` and keeps terminal authority in live-terminal or TUI product shells. An interactive secret-entry flow crosses both boundaries. The model-facing result must not contain the secret bytes.

`digitallyinduced/haskell-agent` reads a secret through a masked prompt, writes it to an owner-only file below the session scratch directory, and returns only that path. The file is removed when the tool runtime closes.

## Goals

- Keep secret values out of conversation history, tool arguments, transcripts, and command text.
- Return only a capability-bound path to the approved model turn.
- Bind the file to the session scratch directory and tool-runtime lifetime.
- Fail closed outside an approved interactive root session.

## Non-goals

- Host-user isolation or resistance to same-user process access.
- Secret persistence, rotation, validation, or remote delivery.
- Automatic injection into arbitrary command environments.
- Ambient terminal discovery in reusable crates.

## Decisions

### Decision: The model receives only the path

**Choice:** The secret-entry adapter result contains path metadata and no secret bytes. The terminal shell owns masked input. The secret host owns bounded file publication and cleanup.

**Rationale:** Returning the value would copy it into every normal tool and transcript surface.

### Decision: Secret files are private and session-local

**Choice:** The host creates the file below the admitted session scratch directory with owner-only permissions. It refuses symlink, escape, or preexisting-target cases.

**Rationale:** The path is a temporary delivery mechanism, not general filesystem publication.

### Decision: Runtime close removes the file

**Choice:** The tool runtime tracks every issued secret path and removes each remaining file during close. Callers can remove files earlier after consumption.

**Rationale:** Bounded lifetime reduces accidental persistence without claiming secure erasure.

### Decision: Non-interactive use fails closed

**Choice:** Interactive secret entry requires an approved root-session terminal capability. Headless and child sessions receive a typed denial unless a separate authority contract explicitly admits them.

**Rationale:** A model cannot safely improvise secret collection through normal text tools.

## Dependency Direction

```text
live terminal shell ---- masked input
          ^
          |
secret-entry DTOs -- animus-std-secretspec file host
```

Reusable contracts do not read terminals, files, environment variables, or credentials.

## Adversarial Audit

The audit must try to show that Animus:

- returns secret bytes in a tool result or transcript;
- places the secret in a command line or environment without separate authority;
- writes outside the admitted scratch directory;
- creates a group-readable or world-readable secret file;
- leaves a tracked secret file after runtime close;
- accepts a headless or child request without explicit authority;
- claims same-user process isolation from path-only delivery.

Any successful counterexample blocks closeout.

## Risks and Trade-offs

- File-path delivery remains readable by authorized same-user processes. The non-claim stays explicit.
- Cleanup can fail after host damage or abrupt process death. Restore must identify and remove admitted leftovers before reuse.
- Some terminal hosts cannot mask input. They must deny the request instead of echoing the secret.

## Non-Claims

This change does not prove secure erasure, host-user isolation, process sandboxing, credential validity, remote-service acceptance, or protection from commands with equivalent operating-system authority.
