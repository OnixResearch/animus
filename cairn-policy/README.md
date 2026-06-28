# Cairn policy

`generated/cairn-policy.json` is the repo-local Cairn runtime policy artifact used by default lifecycle validation.

Source: the canonical Cairn policy export from `github.com/OnixResearch/cairn`.

Refresh from a local Cairn checkout:

```sh
cd /path/to/cairn
nix run path:$PWD#cairn -- policy export
cp cairn-policy/generated/cairn-policy.json /path/to/animus/cairn-policy/generated/cairn-policy.json
```

Validate this repository with the checked-in policy:

```sh
cd /path/to/animus
nix run path:/path/to/cairn#cairn -- validate --root .
```

When refreshing the generated policy, record the Cairn revision or package identity used so lifecycle evidence can be reproduced.
