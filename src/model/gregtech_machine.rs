use {
    crate::model::GregTechRecipe,
    serde::Deserialize,
    std::fmt::{
        Display,
        Formatter,
    },
};

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

impl Display for GregTechMachine {
    #[rustfmt::skip]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Machine:         {}", self.name)?;
        writeln!(f, "Recipes:         {}", self.recipes.len())
    }
}
