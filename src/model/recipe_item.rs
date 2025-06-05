use serde::Deserialize;
use std::fmt::{Display, Formatter};

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

impl Display for RecipeItem {
    #[rustfmt::skip]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "ID:            {}", self.id.as_deref().unwrap_or("None"))?;
        writeln!(f, "Name:          {}", self.localized_name.as_deref().unwrap_or("None"))?;
        writeln!(f, "Amount:        {}", self.amount)?;
        writeln!(f, "Meta:          {}", self.meta)?;
        writeln!(f, "NBT:           {}", self.nbt.as_deref().unwrap_or("None"))
    }
}
