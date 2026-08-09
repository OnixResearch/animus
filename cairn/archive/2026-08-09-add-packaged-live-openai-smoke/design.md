# Design: Packaged live OpenAI smoke

## Context

`animus-std-openai` owns the exact OpenAI Responses endpoint, request projection, bounded response normalization, and one-attempt HTTPS shell. Its ignored integration test duplicates smoke setup and cannot serve as an operator command.

The release evidence schema already classifies `live-providers` as optional. Its validator requires a passing optional check, but it does not require that check to match the recorded head.

## Success Contract

The change is complete when a packaged binary can run one fixed text prompt through the existing live host from explicit model and credential inputs. A successful run must print only safe receipt metadata.

Deterministic completion evidence includes positive receipt projection, negative input and leak tests, stale optional-receipt rejection, the focused Rust rail, the focused Nix rail, and Cairn gates.

Compilation, an ignored test, a fake transport, a task marker, or a stale optional receipt is not live-provider evidence. An actual live-provider claim still requires an explicit credential-backed run on the indexed head.

## Decisions

### 1. Keep the binary in the authority-scoped crate

**Choice:** Add `animus-live-openai-smoke` to `animus-std-openai` behind `reqwest-client`.

**Rationale:** The binary needs the existing network and credential authority. Adding it to `animus-std` would widen another host package.

### 2. Use bounded standard input for the credential

**Choice:** Require `--model MODEL` and `--credential-stdin`. Read one bounded API key value from standard input.

**Rationale:** Command-line credentials leak through process listings. Environment lookup is an ambient dependency. Standard input supplies an explicit channel without storing or printing the value.

### 3. Keep planning and evidence projection pure

**Choice:** Pure functions build the fixed smoke dispatch plan and render metadata-only evidence. A thin shell constructs the HTTPS host and performs one request.

**Rationale:** Deterministic tests can validate all setup and output rules without network access.

### 4. Reuse the existing host contract

**Choice:** Keep the official endpoint, fixed prompt, named timeout, response limit, non-streaming text cohort, and no-retry behavior.

**Rationale:** The smoke must exercise the accepted host. It must not create a second provider client or broaden the request cohort.

### 5. Bind live claims to the recorded head

**Choice:** Treat `live-providers` as claimable only when its optional check passes and `head_match` is true.

**Rationale:** A stale live result is useful historical evidence, but it cannot prove current-head behavior.

## Portfolio Search Record

The exact goal is a bounded packaged live path with current-head evidence. Completion requires a safe operator binary, deterministic denial coverage, and stale-receipt rejection.

The serial architecture lenses are correlated because one agent performed them.

- **Authority-scoped packaged binary**: selected. It reuses the existing host and keeps authority isolated.
- **Retain only the ignored integration test**: falsified. That test already exists and leaves the packaged roadmap gap open.
- **Add the smoke to `animus-std`**: rejected as stronger than required because it widens a separate host package.
- **Use a provider SDK**: rejected because it can add hidden retries, discovery, and request-shape changes.
- **Pass the API key as an argument**: rejected because process metadata can expose it.
- **Read an implicit environment key**: rejected because the packaged path must require an explicit credential channel.

The adversarial audit targets credential leaks, raw response leaks, unbounded input, model substitution, network contact before admission, hidden retries, stale evidence, and unsupported claim expansion.

The retrieval budget is limited to the current host, release evidence validator, packaged smoke patterns, lifecycle files, and focused checks. The validation budget includes focused Rust and Nix rails plus Cairn validation and gates. The actual live run is allowed to remain blocked when explicit credentials are absent.

## Risks and Trade-offs

- The fixed smoke prompt proves only one text request shape.
- Standard input is explicit but still depends on the operator protecting the input source.
- OpenAI wire behavior can change after deterministic checks pass.
- A successful run does not prove provider reliability, quality, billing correctness, or production readiness.
