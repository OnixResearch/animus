## Why

The selected-check host hashes bounded stdout and stderr, then drops their bytes.
Consumers cannot inspect a complete native record without a second capture path.
File redirection would bypass the host's stream bounds.

## What Changes

- Add an opt-in private capture object beside the existing metadata observation.
- Retain exact bounded bytes, truncation, completion, and disposition from one run.
- Add explicit cancellable capture without changing the existing service API.
- Keep raw output out of ordinary observations, Debug output, and smoke receipts.

## Impact

- **Files**: `crates/animus-std-check-exec`, its focused rail, and host documentation.
- **Testing**: real fixture children, compatibility controls, Clippy, the focused rail, and Cairn gates.
- **Non-claims**: capture does not authenticate executables, callers, output meaning, sandboxing, native builds, or promotion.
