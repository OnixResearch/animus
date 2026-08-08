let initial_sequence = 1 in
let next_sequence = 2 in
{
  cases = [
    { name = "initial-replay", cursor = null, sequences = [initial_sequence, next_sequence], expected = "initial" },
    { name = "exact-top-up", cursor = initial_sequence, sequences = [initial_sequence, next_sequence], expected = "top_up" },
    { name = "no-op", cursor = next_sequence, sequences = [initial_sequence, next_sequence], expected = "no_op" },
    { name = "terminal-replay", cursor = next_sequence, sequences = [initial_sequence, next_sequence], terminal = "completed", expected = "terminal" },
  ],
}
