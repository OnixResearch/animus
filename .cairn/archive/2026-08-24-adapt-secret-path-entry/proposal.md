## Why

Animus already resolves reviewed SecretSpec declarations through explicit environment, systemd-credential, and guarded SOPS workers. Interactive sessions still need a safe way to request a user-supplied token without placing the value in chat history, tool arguments, transcripts, or command text.

`digitallyinduced/haskell-agent` uses a narrow `ask_secret` pattern: a masked terminal prompt writes the value to a private temporary file and returns only the path to the model. This change adapts that pattern into the Animus secret host and terminal shell.

## What Changes

- Add an authority-gated interactive secret-entry request that returns a file path, never the secret value.
- Require masked terminal input and owner-only temporary files below the session scratch directory.
- Require cleanup when the tool runtime closes, with earlier caller cleanup permitted.
- Fail closed when no approved interactive terminal is present.
- Add explicit non-claims that path-only delivery is not a sandbox or isolation boundary.

## Impact

- **Files**: secret adapter DTOs, `animus-std-secretspec`, live-terminal or TUI composition, cleanup services, fixtures, documentation, and this lifecycle package.
- **Testing**: focused SecretSpec and live-terminal rails, workspace tests, Clippy, Octet, formatting, Cairn validation and gates, and `nix flake check`.
- **Dependencies**: none new. Source evidence: `digitallyinduced/haskell-agent` at `master`.

## Non-goals

- A password manager, credential broker, or operating-system sandbox.
- Persistent secret storage.
- Protection from unsandboxed commands that run as the same operating-system user.
- Secret entry for an unapproved headless or child session.
