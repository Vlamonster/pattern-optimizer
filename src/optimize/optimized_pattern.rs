use {
    crate::optimize::OptimizedItem,
    serde::Serialize,
};

/// An optimized pattern returned by the server.
#[derive(Serialize, Debug)]
pub struct OptimizedPattern {
    /// Optimized input items.
    #[serde(rename = "inputs")]
    pub inputs: Vec<OptimizedItem>,

    /// Optimized output items.
    #[serde(rename = "outputs")]
    pub outputs: Vec<OptimizedItem>,

    /// Expected duration in ticks.
    #[serde(rename = "duration")]
    pub duration: u64,
}
