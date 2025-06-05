use serde::Deserialize;

/// Represents a database of crafting recipes categorized by source.
#[derive(Deserialize, Debug)]
pub struct RecipeDatabase {
    /// Machine crafting recipes.
    pub machines: Vec<Machine>,

    /// Smelting recipes.
    pub smelting: Vec<FurnaceRecipe>,
}

/// A machine and its processing recipes.
#[derive(Deserialize, Debug)]
pub struct Machine {
    /// Machine name or identifier.
    #[serde(rename = "n")]
    pub name: String,

    /// Recipes this machine can perform.
    #[serde(rename = "recs")]
    pub recipes: Vec<GregTechRecipe>,
}

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
