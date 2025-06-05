use serde::Deserialize;

/// A fluid in a recipe, including amount and identifiers.
#[derive(Deserialize, Debug, Clone)]
#[allow(unused)]
pub struct RecipeFluid {
    /// Fluid registry ID.
    #[serde(rename = "id")]
    pub id: String,

    /// Human-readable name.
    #[serde(rename = "lN")]
    pub localized_name: String,

    /// Fluid amount in liters.
    #[serde(rename = "a")]
    pub amount: u64,
}
