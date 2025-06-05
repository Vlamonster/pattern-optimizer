use {
    crate::model::RecipeItem,
    serde::Deserialize,
    std::fmt::{
        Display,
        Formatter,
    },
};

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

impl Display for FurnaceRecipe {
    #[rustfmt::skip]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Input:           {}", self.input)?;
        writeln!(f, "Output:          {}", self.output)
    }
}
