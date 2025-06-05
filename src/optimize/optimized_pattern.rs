use {
    crate::optimize::OptimizedItem,
    serde::Serialize,
};

/// An optimized pattern returned by the server.
#[derive(Serialize, Debug)]
pub struct OptimizedPattern {
    /// Optimized input items.
    pub inputs: Vec<OptimizedItem>,

    /// Optimized output items.
    pub outputs: Vec<OptimizedItem>,

    /// Expected duration in ticks.
    pub duration: u64,
}
