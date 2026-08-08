let initial_sequence = 1 in
let duplicate_sequence = initial_sequence in
let gap_sequence = 3 in
let oversized_event_count = 257 in
{
  cases = [
    { name = "duplicate-sequence", sequences = [initial_sequence, duplicate_sequence], expected = "duplicate" },
    { name = "gap", sequences = [initial_sequence, gap_sequence], expected = "gap" },
    { name = "stale-generation", generation = "generation:old", expected = "rebuild" },
    { name = "branch-drift", active_leaf = "leaf:old", expected = "rebuild" },
    { name = "live-only-delta", class = "live_only_delta", expected = "omitted" },
    { name = "oversized-range", event_count = oversized_event_count, expected = "truncated" },
    { name = "unsafe-payload", event_identity = "not-a-blake3-hash", expected = "invalid_event_identity" },
    { name = "ambiguous-rebuild", sequences = [initial_sequence, gap_sequence], generation = "generation:old", expected = "gap" },
  ],
}
