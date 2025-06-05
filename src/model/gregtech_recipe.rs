use {
    crate::model::{
        FurnaceRecipe,
        RecipeFluid,
        RecipeItem,
    },
    itertools::Itertools,
    serde::Deserialize,
    std::fmt::{
        Display,
        Formatter,
    },
};

/// A machine processing recipe, including inputs, outputs, and processing details.
#[derive(Deserialize, Debug, Clone)]
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

impl Display for GregTechRecipe {
    #[rustfmt::skip]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Enabled:         {}", self.enabled)?;
        writeln!(f, "Duration:        {} ticks", self.duration)?;
        writeln!(f, "Energy Usage:    {} EU/t", self.energy_usage)?;
        writeln!(f, "Special:         {}", self.special)?;
        writeln!(f, "Item Inputs:   \n{}", self.item_inputs.iter().map(|item| format!("-{item}")).join("\n"))?;
        writeln!(f, "Fluid Inputs:  \n{}", self.fluid_inputs.iter().map(|fluid| format!("-{fluid}")).join("\n"))?;
        writeln!(f, "Item Outputs:  \n{}", self.item_outputs.iter().map(|item| format!("-{item}")).join("\n"))?;
        writeln!(f, "Fluid Outputs: \n{}", self.fluid_outputs.iter().map(|fluid| format!("-{fluid}")).join("\n"))
    }
}

impl From<&FurnaceRecipe> for GregTechRecipe {
    fn from(recipe: &FurnaceRecipe) -> Self {
        GregTechRecipe {
            enabled: true,
            duration: 512,
            energy_usage: 4,
            special: 0,
            item_inputs: vec![recipe.input.clone()],
            item_outputs: vec![recipe.output.clone()],
            fluid_inputs: vec![],
            fluid_outputs: vec![],
        }
    }
}
