## Why

The live session planner hard-codes `low` reasoning even after the exact local-dspark configuration validates `none`. The local server then emits hidden-reasoning events, and the production parser correctly closes the request.

## What Changes

- Expose the validated reasoning effort as metadata from the live configuration.
- Project `none` as disabled canonical reasoning and preserve `low` for existing cohorts.
- Reject every unsupported reasoning value before provider contact.
- Preserve daemon event sequence on decoded private runtime events so terminal polling remains coherent.
- Add positive and negative planning and cursor tests.

## Impact

- **Files**: live-agent configuration, provider planning, terminal event projection, tests, docs, and lifecycle evidence
- **Testing**: focused live-agent tests, strict Clippy, formatting, focused Nix checks, and downstream zero-finding product verification
