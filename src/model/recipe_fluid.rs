use {
    serde::Deserialize,
    std::fmt::{
        Display,
        Formatter,
    },
};

/// A fluid in a recipe, including amount and identifiers.
#[derive(Deserialize, Debug, Clone)]
#[allow(unused)]
pub struct RecipeFluid {
    /// Localized name of the fluid.
    #[serde(rename = "lN")]
    pub localized_name: String,

    /// Fluid registry ID.
    #[serde(rename = "id")]
    pub id: String,

    /// Fluid amount in liters.
    #[serde(rename = "a")]
    pub amount: u64,
}

impl Display for RecipeFluid {
    #[rustfmt::skip]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Name:            {}", self.localized_name)?;
        writeln!(f, "ID:              {}", self.id)?;
        writeln!(f, "Amount:          {}L", self.amount)
    }
}
