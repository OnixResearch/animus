#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ReducerDecision {
    Continue,
    Stop,
}

fn pure_reducer(turn_count: u32, max_turns: u32) -> ReducerDecision {
    if turn_count >= max_turns {
        ReducerDecision::Stop
    } else {
        ReducerDecision::Continue
    }
}
