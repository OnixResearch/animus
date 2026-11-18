//! The budget-matched random-ranking baseline route.

/// `SplitMix64` increment.
const GOLDEN_GAMMA: u64 = 0x9E37_79B9_7F4A_7C15;

/// A route that ranks every listed candidate uniformly at random within declared bounds.
///
/// Draws come from `SplitMix64` over the declared seed, the call index, and the candidate's brief
/// position, so one seed repeats exactly. Like the scorer, it adds no offers.
pub struct RandomRankRoute {
    identity: String,
    seed: u64,
    bounds: super::RankBounds,
    calls: u64,
}

/// Where one draw happens.
struct DrawPoint {
    seed: u64,
    call: u64,
    position: u64,
}

impl RandomRankRoute {
    /// A random-ranking route for `seed` within `bounds`.
    #[must_use]
    pub fn new(seed: u64, bounds: super::RankBounds) -> Self {
        Self {
            identity: format!("model:random-rank:seed-{seed}"),
            seed,
            bounds,
            calls: 0,
        }
    }

    /// The route's Harbor model-route identity.
    #[must_use]
    pub fn identity(&self) -> &str {
        &self.identity
    }

    /// The route's reply to one rendered brief.
    // r[impl agent_harness_runtime.campaign_manager_eval.local_scorer.baseline]
    pub fn reply(&mut self, prompt: &str) -> String {
        let view = super::super::reading::read_prompt(prompt);
        let span_ranks = self
            .bounds
            .max()
            .abs_diff(self.bounds.min())
            .saturating_add(1);
        let scores: Vec<String> = view
            .candidates
            .iter()
            .zip(0_u64..)
            .map(|(listed, position)| {
                let drawn = draw(&DrawPoint {
                    seed: self.seed,
                    call: self.calls,
                    position,
                });
                let rank = drawn
                    .checked_rem(span_ranks)
                    .map_or(self.bounds.min(), |offset_ranks| {
                        self.bounds.min().saturating_add_unsigned(offset_ranks)
                    });
                debug_assert!(self.bounds.contains(rank));
                let ranks = super::super::reading::ranks(rank, view.dimensions);
                format!(r#"{{"candidate":"{}","ranks":{ranks}}}"#, listed.handle)
            })
            .collect();
        self.calls = self.calls.saturating_add(1);
        debug_assert_eq!(scores.len(), view.candidates.len());
        format!(r#"{{"scores":[{}],"add":[]}}"#, scores.join(","))
    }
}

impl
    animus_adapters::ComponentAdapter<animus_adapters::ModelRequest, animus_adapters::ModelResponse>
    for RandomRankRoute
{
    type Error = super::super::RouteFailure;

    fn call(
        &mut self,
        request: &animus_adapters::ModelRequest,
    ) -> Result<animus_adapters::ModelResponse, Self::Error> {
        let reply = self.reply(request.prompt());
        animus_adapters::ModelResponse::new(&reply)
            .map_err(|_| super::super::RouteFailure::Malformed)
    }
}

/// One `SplitMix64` draw chained over the seed, the call, and the position.
fn draw(at: &DrawPoint) -> u64 {
    mix(mix(mix(at.seed) ^ at.call) ^ at.position)
}

fn mix(state: u64) -> u64 {
    let mut value = state.wrapping_add(GOLDEN_GAMMA);
    value = (value ^ (value >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    value = (value ^ (value >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    value ^ (value >> 31)
}
