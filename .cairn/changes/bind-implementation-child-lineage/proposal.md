# Attempt-bound implementation lineage

## Why

Fleet needs genuine runtime parent/child relationships, not a topology inferred from a plan or raw stream.

## What Changes

Animus must export bounded metadata-only child lineage from the selected implementation parent’s own lifecycle events.

## Impact

Non-default local qualification only; no new broader production grant or deployment.
