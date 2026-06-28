# Cairn policy

`generated/cairn-policy.json` is the repo-local Cairn runtime policy artifact used by default lifecycle validation. Runtime validation consumes this generated JSON; it does not execute Nickel while validating Animus lifecycle artifacts.

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

When refreshing the generated policy, record the Cairn revision or package identity, the drift-check result, the policy hash reported by Cairn validation, and the validation command used so lifecycle evidence can be reproduced.
