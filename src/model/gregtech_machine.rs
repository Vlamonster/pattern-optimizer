use crate::model::gregtech_recipe::GregTechRecipe;
use serde::Deserialize;

/// A machine and its processing recipes.
#[derive(Deserialize, Debug)]
pub struct GregTechMachine {
    /// Machine name or identifier.
    #[serde(rename = "n")]
    pub name: String,

    /// Recipes this machine can perform.
    #[serde(rename = "recs")]
    pub recipes: Vec<GregTechRecipe>,
}
