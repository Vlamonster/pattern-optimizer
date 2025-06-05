use serde::Serialize;

/// An optimized item with batch size and metadata.
#[derive(Serialize, Debug)]
pub struct OptimizedItem {
    /// Item registry ID.
    pub id: String,

    /// Optimized item quantity.
    pub amount: u64,

    /// Metadata (e.g. damage value).
    pub meta: u64,

    /// Associated NBT data.
    pub nbt: String,
}
