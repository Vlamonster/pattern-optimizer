use {
    crate::{
        model::{
            RecipeFluid,
            RecipeItem,
        },
        optimize::WILDCARD,
        request::FluidDrop,
    },
    serde::Deserialize,
    std::fmt::{
        Display,
        Formatter,
    },
};

/// An input or output item in an `OptimizationRequest`.
#[derive(Deserialize, Debug)]
pub struct RequestItem {
    /// Localized name of the item.
    #[serde(rename = "label")]
    pub name: String,

    /// Item ID or registry name.
    #[serde(rename = "name")]
    pub id: String,

    /// Max stack size.
    #[serde(rename = "maxSize")]
    pub max_size: u64,

    /// Metadata (e.g. damage value).
    #[serde(rename = "damage")]
    pub meta: u64,

    /// Optional fluid source details.
    #[serde(rename = "fluidDrop")]
    pub fluid_drop: Option<FluidDrop>,

    /// Number of items requested.
    #[serde(rename = "size")]
    #[cfg(feature = "unused_fields")]
    pub amount: u64,

    /// Maximum damage.
    #[serde(rename = "maxDamage")]
    #[cfg(feature = "unused_fields")]
    pub max_damage: u64,

    /// Whether the item has an NBT tag.
    #[serde(rename = "hasTag")]
    #[cfg(feature = "unused_fields")]
    pub has_tag: bool,

    /// Optional capacity (e.g. fluid container).
    #[serde(rename = "capacity")]
    #[cfg(feature = "unused_fields")]
    pub capacity: Option<u64>,

    /// Optional fluid contents.
    #[serde(rename = "fluid")]
    #[cfg(feature = "unused_fields")]
    pub fluid: Option<Fluid>,
}

impl PartialEq<RecipeItem> for RequestItem {
    fn eq(&self, other: &RecipeItem) -> bool {
        other.id.as_deref().is_some_and(|id| self.id == id) && self.meta == other.meta || other.meta == WILDCARD
    }
}

impl PartialEq<RecipeFluid> for RequestItem {
    fn eq(&self, other: &RecipeFluid) -> bool {
        self.fluid_drop.as_ref().is_some_and(|fluid_drop| fluid_drop.id == other.id)
    }
}

impl Display for RequestItem {
    #[rustfmt::skip]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "  Name:            {}", self.name)?;
        writeln!(f, "  ID:              {}", self.id)?;
        writeln!(f, "  Max Stack Size:  {}", self.max_size)?;
        writeln!(f, "  Meta:            {}", self.meta)?;
        writeln!(f, "  Fluid Drop:      {}", self.fluid_drop.as_ref().map_or("None".into(), FluidDrop::to_string))
    }
}
