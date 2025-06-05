use crate::model::{RecipeFluid, RecipeItem};
use serde::Deserialize;

/// A machine processing recipe, including inputs, outputs, and processing details.
#[derive(Deserialize, Debug)]
#[allow(unused)]
pub struct GregTechRecipe {
    /// Whether the recipe is enabled.
    #[serde(rename = "en")]
    pub enabled: bool,

    /// Duration in ticks.
    #[serde(rename = "dur")]
    pub duration: u64,

    /// Energy usage per tick (EU/t).
    #[serde(rename = "eut")]
    pub energy_usage: u64,

    /// Special parameter for custom logic.
    #[serde(rename = "sp")]
    pub special: isize,

    /// Required item inputs.
    #[serde(rename = "iI")]
    pub item_inputs: Vec<RecipeItem>,

    /// Required fluid inputs.
    #[serde(rename = "fI")]
    pub fluid_inputs: Vec<RecipeFluid>,

    /// Produced item outputs.
    #[serde(rename = "iO")]
    pub item_outputs: Vec<RecipeItem>,

    /// Produced fluid outputs.
    #[serde(rename = "fO")]
    pub fluid_outputs: Vec<RecipeFluid>,
}
