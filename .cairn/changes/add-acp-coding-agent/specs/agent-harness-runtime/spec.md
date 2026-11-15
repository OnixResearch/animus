# Agent Harness Runtime Specification Delta

## ADDED Requirements

### Requirement: The ACP coding agent is an optional crate

r[agent_harness_runtime.acp_coding_agent.boundary] Animus MUST provide the ACP coding agent as the optional crate `animus-acp-coding-agent` with the binary `animus-acp-agent`, outside the workspace's default members. Its admission rules MUST be pure functions without filesystem, process, network, clock or credential authority.

#### Scenario: Default builds do not carry the agent

- GIVEN the Animus workspace with its default members
- WHEN a default build is resolved
- THEN `animus-acp-coding-agent` is not built.

### Requirement: Workspace tools are confined to one root

r[agent_harness_runtime.acp_coding_agent.tools] The tool service MUST offer exactly `read_file`, `list_files`, `edit_file` and `run_check`. It MUST refuse a request whose path is absolute, empty, contains a `.`, `..` or empty component or a NUL byte, or exceeds the path bound, and any request whose sizes exceed their bounds. An `edit_file` request MUST be refused unless its `old` text occurs exactly once in the current file, except that an empty `old` creates a file that does not exist.

#### Scenario: A path outside the root is refused before any effect

- GIVEN a request to read `../secret` or `/etc/passwd`
- WHEN the core admits it
- THEN the request is refused and no file is opened.

#### Scenario: An ambiguous edit is refused

- GIVEN a file containing `x` twice
- WHEN an edit replaces `x`
- THEN the edit is refused and the file is unchanged.

### Requirement: Checks run only as declared

r[agent_harness_runtime.acp_coding_agent.checks] `run_check` MUST run only a check named in the declared checks file, with its exact argv, the workspace root as working directory, and its declared budget. The file MUST be refused at startup when its schema differs, an id is malformed or repeated, an argv is empty, or a budget is missing or above the crate ceiling.

#### Scenario: An undeclared check is refused

- GIVEN a checks file declaring `cargo-test`
- WHEN the agent asks to run `rm-all`
- THEN the request is refused and no process starts.

#### Scenario: A malformed checks file stops startup

- GIVEN a checks file with a repeated id
- WHEN the binary starts
- THEN it exits with an error before serving ACP.

### Requirement: The provider is explicit

r[agent_harness_runtime.acp_coding_agent.provider] The provider service MUST take its endpoint, model and credential file from explicit arguments and MUST NOT read credentials from the environment or stdin.

#### Scenario: Missing credential for a remote endpoint

- GIVEN an `https` endpoint and no credential file
- WHEN the binary starts
- THEN it exits with an error before serving ACP.

### Requirement: The session is served over ACP stdio

r[agent_harness_runtime.acp_coding_agent.acp] The binary MUST serve one coding session through `animus-acp-stdio-host`, write only ACP frames to stdout, and emit metadata-only receipts that bind the checks-file identity.

#### Scenario: Diagnostics stay off the protocol stream

- GIVEN a startup warning
- WHEN the binary writes it
- THEN it appears on stderr and stdout carries only ACP frames.

### Requirement: Harbor's ACP client completes a scripted session

r[agent_harness_runtime.acp_coding_agent.harbor] A test MUST run the binary under Harbor's ACP client with a fixture provider and complete a session that edits a file and runs a declared check, without network access.

#### Scenario: Offline scripted session

- GIVEN the fixture provider and a workspace with one declared check
- WHEN Harbor's ACP client sends a prompt
- THEN the session edits the file, runs the check, and ends with a completed stop reason.

### Requirement: Admission rules are tested both ways

r[agent_harness_runtime.acp_coding_agent.verification] Every admission rule MUST have positive and negative tests, and the focused checks `cargo-acp-coding-agent` and `acp-coding-agent-octet` MUST pass.

#### Scenario: Focused checks pass

- GIVEN the change
- WHEN the focused checks run
- THEN both pass.
