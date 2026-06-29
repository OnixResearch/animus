/// Swappable loop-fabric boundary used by the runtime shell.
pub trait LoopFabric {
    /// Error returned by the concrete loop fabric.
    type Error: core::fmt::Display;

    /// Produces one validated loop decision for the supplied turn input.
    ///
    /// # Errors
    ///
    /// Returns [`Self::Error`] when the fabric cannot produce a valid decision.
    fn decide(
        &mut self,
        turn: &animus_steel::LoopTurn,
    ) -> Result<animus_steel::LoopDecision, Self::Error>;
}

impl LoopFabric for animus_steel::SteelLoopFabric {
    type Error = animus_steel::LoopError;

    fn decide(
        &mut self,
        turn: &animus_steel::LoopTurn,
    ) -> Result<animus_steel::LoopDecision, Self::Error> {
        animus_steel::SteelLoopFabric::decide(self, turn)
    }
}
