use crate::model::RecipeItem;
use serde::Deserialize;

/// A smelting recipe with one input and one output item.
#[derive(Deserialize, Debug)]
pub struct FurnaceRecipe {
    /// Required item input.
    #[serde(rename = "input")]
    pub input: RecipeItem,

    /// Produced item output.
    #[serde(rename = "output")]
    pub output: RecipeItem,
}
