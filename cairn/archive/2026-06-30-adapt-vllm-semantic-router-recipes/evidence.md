# Evidence: adapt vLLM Semantic Router recipes

## Scope

- Change: `adapt-vllm-semantic-router-recipes`
- Requirement IDs: `r[agent_harness_runtime.vllm_semantic_router_recipes.*]`
- Reference: vLLM Semantic Router micro-agent recipes are used as a bounded architectural reference only.
- Non-claims: no vLLM dependency, serving proxy, live provider fan-out, benchmark claim, raw worker trace exposure, or product workflow surface is claimed.

## Lifecycle gates

- `nix run .#cairn -- gate proposal adapt-vllm-semantic-router-recipes --root .`: PASS before implementation.
- `nix run .#cairn -- gate design adapt-vllm-semantic-router-recipes --root .`: PASS before implementation.
- `nix run .#cairn -- gate tasks adapt-vllm-semantic-router-recipes --root .`: PASS before implementation.

## Focused implementation checks

- `nix develop -c cargo test -p animus-adapters --locked router::`: PASS; 10 router tests passed covering supported recipe families, invalid recipe shapes, bounded signal projection, deterministic selection, unsafe model eligibility denial, invalid execution bounds, output-contract failure/fallback, safe receipt identity, and leak rejection.
- `nix develop -c cargo test -p animus-adapters --locked`: PASS; adapter package tests passed.
- `nix develop -c cargo clippy -p animus-adapters --all-targets --locked -- -D warnings`: PASS.
- `nix develop -c cargo fmt --check`: PASS after formatting was applied.

## Evidence safety

Router receipt leak fixtures reject raw prompts, provider JSON/payloads, worker outputs, judge rationales, hidden reasoning text, credentials, full paths, and unbounded traces. Durable evidence here contains only bounded command/status summaries.
