use serde::Deserialize;

/// An item in a recipe, including amount, metadata, and optional NBT.
#[derive(Deserialize, Debug, Clone)]
#[allow(unused)]
pub struct RecipeItem {
    /// Optional item registry ID.
    #[serde(rename = "id")]
    pub id: Option<String>,

    /// Optional human-readable name.
    #[serde(rename = "lN")]
    pub localized_name: Option<String>,

    /// Item quantity.
    #[serde(rename = "a")]
    pub amount: u64,

    /// Metadata or damage value.
    #[serde(rename = "m")]
    pub meta: u64,

    /// Optional NBT data.
    #[serde(rename = "nbt")]
    pub nbt: Option<String>,
}
