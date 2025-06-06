use {
    serde::Deserialize,
    std::fmt::{
        Display,
        Formatter,
    },
};

/// A fluid drop associated with an item in an `OptimizationRequest`.
#[derive(Deserialize, Debug)]
#[allow(unused)]
pub struct FluidDrop {
    /// Localized name of the fluid.
    #[serde(rename = "label")]
    pub localized_name: String,

    /// Fluid ID or registry name.
    #[serde(rename = "name")]
    pub id: String,

    /// Amount in liters.
    #[serde(rename = "amount")]
    #[cfg(feature = "unused_fields")]
    pub amount: u64,

    /// Whether the fluid has NBT data.
    #[serde(rename = "hasTag")]
    #[cfg(feature = "unused_fields")]
    pub has_tag: bool,
}

impl Display for FluidDrop {
    #[rustfmt::skip]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Name:            {}", self.localized_name)?;
        writeln!(f, "ID:              {}", self.id)
    }
}
