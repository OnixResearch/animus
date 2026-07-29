# Tasks

## 1. Pin and baseline

- [ ] [serial] 1.1 Run the focused credential and adapter tests before core changes and record baseline failures. r[credentials.secretspec.provider_conformance]
- [ ] [serial] 1.2 Pin SecretSpec `v0.17.0` and tag commit `a8794e46ec9664a0e1a3869cc3105d0853937e48` through Cargo and Nix dependency inputs. r[credentials.secretspec.explicit_resolution]
- [ ] [serial] 1.3 Enable only the provider features required by the accepted Animus profiles. r[credentials.secretspec.explicit_resolution]
- [ ] [serial] 1.4 Record the upstream license, source identity, feature set, and SOPS child-process limitation. r[credentials.secretspec.bounded_provider_execution]

## 2. Build the functional core

- [ ] [serial] 2.1 Add typed credential requirements, profile and scope selection, provider allowlists, and stable rejection categories. r[credentials.secretspec.explicit_resolution]
- [ ] [serial] 2.2 Reject implicit profiles, unknown declarations, undeclared scopes, provider fallback, and write requests. r[credentials.secretspec.explicit_resolution] r[credentials.secretspec.read_only]
- [ ] [serial] 2.3 Add positive tests for valid selections and negative tests for each rejection path. r[credentials.secretspec.provider_conformance]
- [ ] [serial] 2.4 Add assertions that public metadata cannot contain secret values or provider credentials. r[credentials.secretspec.no_durable_secret_material]

## 3. Build the imperative shell

- [ ] [serial] 3.1 Add the optional `std` SecretSpec SDK adapter and keep it out of `no_std` crates. r[credentials.secretspec.functional_boundary]
- [ ] [serial] 3.2 Add an internal resolver worker for providers that can launch child processes. r[credentials.secretspec.bounded_provider_execution]
- [ ] [serial] 3.3 Execute that worker through `bounded-exec` with named deadline, input, output, and teardown limits. r[credentials.secretspec.bounded_provider_execution]
- [ ] [serial] 3.4 Return secret material in secrecy-aware containers and clear temporary buffers where supported. r[credentials.secretspec.no_durable_secret_material]
- [ ] [serial] 3.5 Disable SecretSpec write operations and plaintext caches for this adapter. r[credentials.secretspec.read_only] r[credentials.secretspec.no_durable_secret_material]

## 4. Verify providers and failures

- [ ] [serial] 4.1 Add positive environment, systemd credential, and SOPS provider fixtures. r[credentials.secretspec.provider_conformance]
- [ ] [serial] 4.2 Add negative fixtures for missing manifests, profiles, scopes, declarations, provider credentials, and executables. r[credentials.secretspec.provider_conformance]
- [ ] [serial] 4.3 Add malformed SOPS, nonzero exit, timeout, cancellation, and oversized-output tests. r[credentials.secretspec.bounded_provider_execution] r[credentials.secretspec.provider_conformance]
- [ ] [serial] 4.4 Prove through tests that diagnostics, transcripts, receipts, and snapshots contain no secret or value-derived hash. r[credentials.secretspec.no_durable_secret_material]

## 5. Integrate and document

- [ ] [serial] 5.1 Wire resolved credentials into one representative host adapter without changing its pure core. r[credentials.secretspec.functional_boundary]
- [ ] [serial] 5.2 Document profile, scope, provider, deployment, rotation, and rollback procedures. r[credentials.secretspec.explicit_resolution]
- [ ] [serial] 5.3 Document that SecretSpec and SOPS resolve values but do not establish caller authority. r[credentials.secretspec.no_durable_secret_material]
- [ ] [serial] 5.4 Publish the immutable validated revision required by Lattice. r[credentials.secretspec.provider_conformance]

## 6. Validate

- [ ] [serial] 6.1 Run `cargo fmt --all -- --check`. r[credentials.secretspec.provider_conformance]
- [ ] [serial] 6.2 Run focused positive and negative credential tests. r[credentials.secretspec.provider_conformance]
- [ ] [serial] 6.3 Run `cargo check --workspace --all-targets`. r[credentials.secretspec.functional_boundary]
- [ ] [serial] 6.4 Run `cargo clippy --workspace --all-targets -- -D warnings`. r[credentials.secretspec.functional_boundary]
- [ ] [serial] 6.5 Run `check-nickel-configs` and the relevant `nix flake check` targets. r[credentials.secretspec.explicit_resolution]
- [ ] [serial] 6.6 Run Cairn validation, requirement coverage, design gate, and tasks gate. r[credentials.secretspec.provider_conformance]
