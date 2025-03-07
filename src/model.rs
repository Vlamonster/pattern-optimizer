use serde::Deserialize;

/// Represents a database of crafting recipes, categorized by different sources.
///
/// # Fields
/// - `sources`: A list of `Source` structures, each representing a crafting method
///   and its associated recipes.
#[derive(Deserialize, Debug)]
pub struct RecipeDatabase {
    pub machines: Vec<Machine>,
    pub smelting: Vec<FurnaceRecipe>,
}

/// Represents a specific machine and its associated processing recipes.
///
/// # Fields
/// - `name`: The name or identifier of the machine.
/// - `recipes`: A list of `Recipe` structures defining the machine's possible operations.
#[derive(Deserialize, Debug)]
pub struct Machine {
    #[serde(rename = "n")]
    pub name: String,
    #[serde(rename = "recs")]
    pub recipes: Vec<GregTechRecipe>,
}

/// Represents a machine processing recipe, including inputs, outputs, and processing details.
///
/// # Fields
/// - `enabled`: Whether the recipe is enabled (`true`) or disabled (`false`).
/// - `duration`: The time required to process the recipe (typically in ticks).
/// - `energy_usage`: The energy consumption per tick (measured in EU/t).
/// - `special`: A special processing parameter (used for custom behaviors or tiering).
/// - `item_inputs`: A list of required item inputs for the recipe.
/// - `item_outputs`: A list of produced item outputs from the recipe.
/// - `fluid_inputs`: A list of required fluid inputs for the recipe.
/// - `fluid_outputs`: A list of produced fluid outputs from the recipe.
#[derive(Deserialize, Debug)]
#[allow(unused)]
pub struct GregTechRecipe {
    #[serde(rename = "en")]
    pub enabled: bool,
    #[serde(rename = "dur")]
    pub duration: u64,
    #[serde(rename = "eut")]
    pub energy_usage: u64,
    #[serde(rename = "sp")]
    pub special: isize,
    #[serde(rename = "iI")]
    pub item_inputs: Vec<RecipeItem>,
    #[serde(rename = "iO")]
    pub item_outputs: Vec<RecipeItem>,
    #[serde(rename = "fI")]
    pub fluid_inputs: Vec<RecipeFluid>,
    #[serde(rename = "fO")]
    pub fluid_outputs: Vec<RecipeFluid>,
}

#[derive(Deserialize, Debug)]
pub struct FurnaceRecipe {
    #[serde(rename = "input")]
    pub input: RecipeItem,
    #[serde(rename = "output")]
    pub output: RecipeItem,
}

/// Represents an item entry in the recipe database, including quantity, metadata, and NBT data.
///
/// # Fields
/// - `id`: The optional registry identifier of the item.
/// - `localized_name`: The optional human-readable name of the item.
/// - `amount`: The number of items required or produced.
/// - `meta`: The item's metadata or damage value.
/// - `nbt`: Optional NBT data associated with the item.
#[derive(Deserialize, Debug, Clone)]
#[allow(unused)]
pub struct RecipeItem {
    #[serde(rename = "id")]
    pub id: Option<String>,
    #[serde(rename = "lN")]
    pub localized_name: Option<String>,
    #[serde(rename = "a")]
    pub amount: u64,
    #[serde(rename = "m")]
    pub meta: u64,
    #[serde(rename = "nbt")]
    pub nbt: Option<String>,
}

/// Represents a fluid entry in the recipe database, including quantity and identifiers.
///
/// # Fields
/// - `id`: The registry identifier of the fluid.
/// - `localized_name`: The human-readable name of the fluid.
/// - `amount`: The quantity of fluid required or produced in millibuckets.
#[derive(Deserialize, Debug, Clone)]
#[allow(unused)]
pub struct RecipeFluid {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "lN")]
    pub localized_name: String,
    #[serde(rename = "a")]
    pub amount: u64,
}
