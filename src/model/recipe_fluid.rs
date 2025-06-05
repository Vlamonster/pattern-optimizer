use serde::Deserialize;
use std::fmt::{Display, Formatter};

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

impl Display for RecipeFluid {
    #[rustfmt::skip]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "ID:             {}", self.id)?;
        writeln!(f, "Name:           {}", self.localized_name)?;
        writeln!(f, "Amount:         {}L", self.amount)
    }
}
