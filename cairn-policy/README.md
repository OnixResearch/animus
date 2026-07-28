# Cairn policy

`generated/cairn-policy.json` is the repository-local Cairn runtime policy artifact.

Default lifecycle validation consumes this generated JSON. It does not execute Nickel while it validates Animus lifecycle artifacts.

Source: the canonical Cairn policy export from `github.com/OnixResearch/cairn`, pinned in this repository's flake input as `cairn`.

Refresh from a local Cairn checkout:

```sh
cd /path/to/cairn
nix run path:$PWD#cairn -- policy export --check
cp cairn-policy/generated/cairn-policy.json /path/to/animus/cairn-policy/generated/cairn-policy.json
```

Check that the checked-in policy matches a canonical generated source when the source is available:

```sh
cd /path/to/animus
scripts/cairn/check-policy --root .
# or
CAIRN_POLICY_SOURCE=/path/to/cairn/cairn-policy/generated/cairn-policy.json scripts/cairn/check-policy --root .
```

Validate this repository with the checked-in policy through the pinned wrapper:

```sh
cd /path/to/animus
nix run .#cairn -- validate --root .
```

Validate with an explicit repo-local policy path:

```sh
cd /path/to/animus
nix run .#cairn -- validate --root . --policy cairn-policy/generated/cairn-policy.json
```

When you refresh the policy, record these items:

- Cairn revision or package identity
- drift-check result
- policy hash from Cairn validation
- validation command

These records make the lifecycle evidence reproducible.
