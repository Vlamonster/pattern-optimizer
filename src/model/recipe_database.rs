use {
    crate::model::{
        FurnaceRecipe,
        GregTechMachine,
    },
    serde::Deserialize,
    std::fmt::{
        Display,
        Formatter,
    },
};

/// Represents a database of crafting recipes categorized by source.
#[derive(Deserialize, Debug)]
pub struct RecipeDatabase {
    /// Machine crafting recipes.
    pub machines: Vec<GregTechMachine>,

    /// Smelting recipes.
    pub smelting: Vec<FurnaceRecipe>,
}

impl Display for RecipeDatabase {
    #[rustfmt::skip]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Machines:        {}", self.machines.len())?;
        writeln!(f, "Smelting:        {}", self.smelting.len())
    }
}
