use crate::model::{FurnaceRecipe, GregTechMachine};
use serde::Deserialize;

/// Represents a database of crafting recipes categorized by source.
#[derive(Deserialize, Debug)]
pub struct RecipeDatabase {
    /// Machine crafting recipes.
    pub machines: Vec<GregTechMachine>,

    /// Smelting recipes.
    pub smelting: Vec<FurnaceRecipe>,
}
