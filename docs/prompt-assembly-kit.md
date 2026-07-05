# Prompt assembly kit

Animus prompt assembly is a pure adapter contract over host-supplied, in-memory sources. It does not discover files, memories, skills, tools, URLs, or providers by itself.

## Sources

Hosts validate `PromptSource` values before assembly. Each source records kind, scope, trust/admission status, required/optional rule, priority, budget class, redaction class, safe label, section, BLAKE3 body hash, and requested budget units.

Supported source kinds cover host context, policy profiles, skills, memories, agent definitions, context references, tool metadata, run objectives, and explicit unsupported entries. Memory and context-reference sources require host discovery/admission outside reusable assembly.

## Assembly policy

`PromptAssemblyPolicy` defines host-context-only vs admitted-source mode, deterministic section order, per-section budget, and policy hash. `assemble_prompt_sources` sorts by section order, source priority, kind, and label. Conflicting sources at the same section and priority resolve deterministically: the selected source is included first, and later conflicting sources are ledgered without leaking body text.

## Ledger and sections

Assembly returns section metadata plus a safe ledger. Ledger states include included, skipped, truncated, redacted, denied, conflicting, and unsupported. Entries record safe labels, source kind, trust/admission status, section, budget class, requested/consumed units, BLAKE3 body hash, and reason.

Host-context-only mode records discovery-backed memory/context sources as unsupported instead of falling back to ambient filesystem, memory-store, provider, URL, skill-store, or project discovery.

## Receipts

`PromptAssemblyReceipt` keeps receipt metadata bounded: source label, source kind, trust status, section, consumed units, truncation flag, optional safe conflict reason, policy hash, and body hash. Receipt validation rejects raw prompt bodies, credentials, full paths, provider payloads, and hidden memory bodies.

## Non-claims

This contract does not copy Clankers prompt text, grant authority from prompt sources, read project files, query memories, retrieve URLs, load skill bodies, discover tools, execute providers, or make ambient discovery a default behavior.
