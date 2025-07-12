use serde::Serialize;

/// An optimized item with batch size and metadata.
#[derive(Serialize, Debug)]
pub struct OptimizedItem {
    /// Item registry ID.
    #[serde(rename = "id")]
    pub id: String,

    /// Localized name of the item.
    pub ln: String,

    /// Optimized item quantity.
    #[serde(rename = "amount")]
    pub amount: u64,

    /// Metadata (e.g. damage value).
    #[serde(rename = "meta")]
    pub meta: u64,

    /// Associated NBT data.
    #[serde(rename = "nbt")]
    pub nbt: String,
}
