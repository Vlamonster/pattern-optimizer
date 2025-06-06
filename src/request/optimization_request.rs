use {
    crate::{
        model::{
            GregTechRecipe,
            RecipeDatabase,
        },
        request::{
            MachineConfiguration,
            RequestItem,
        },
        MainError,
    },
    itertools::Itertools,
    serde::Deserialize,
    std::{
        fmt::{
            Display,
            Formatter,
        },
        ops::{
            BitAnd,
            BitOr,
        },
    },
};

/// Represents a crafting or processing request sent by the client to the server.
#[derive(Deserialize, Debug)]
#[allow(unused)]
pub struct OptimizationRequest {
    /// The version to use for recipes.
    #[serde(rename = "version", default = "default_version")]
    pub version: String,

    /// The name of the machine that should process the pattern.
    #[serde(rename = "machine")]
    pub machine: MachineConfiguration,

    /// The number of ticks to run the machine for at least.
    #[serde(rename = "ticks")]
    pub ticks: u64,

    /// Indicates whether patterning non-consumed items should be skipped.
    #[serde(rename = "skip", default)]
    pub skip: bool,

    /// Indicates whether missing outputs should be restored.
    #[serde(rename = "restore", default)]
    pub restore: bool,

    /// If present, skips optimization and patterns `multiplier` times the base recipe.
    #[serde(rename = "multiplier")]
    pub multiplier: Option<u64>,

    /// A list of input `Item`s required for the recipe.
    #[serde(rename = "inputs")]
    pub inputs: Vec<RequestItem>,

    /// A list of expected output `Item`s.
    #[serde(rename = "outputs")]
    pub outputs: Vec<RequestItem>,
}

fn default_version() -> String {
    "2.7.3".into()
}

impl OptimizationRequest {
    fn matches(&self, recipe: &GregTechRecipe) -> bool {
        self.inputs
            .iter()
            .all(|request_item| {
                recipe
                    .item_inputs
                    .iter()
                    .any(|recipe_item| request_item == recipe_item)
                    .bitor(recipe.fluid_inputs.iter().any(|recipe_fluid| request_item == recipe_fluid))
            })
            .bitand(self.outputs.iter().all(|request_item| {
                recipe
                    .item_outputs
                    .iter()
                    .any(|recipe_item| request_item == recipe_item)
                    .bitor(recipe.fluid_outputs.iter().any(|recipe_fluid| request_item == recipe_fluid))
            }))
    }

    pub fn query(&self, recipe_database: &RecipeDatabase) -> Result<GregTechRecipe, MainError> {
        println!("Starting a query");

        let mut machine_present = false;

        // Check if the request is a Furnace Recipe.
        if self.machine.recipes.contains(&"Multi Smelter".to_string()) {
            machine_present = true;

            let query = recipe_database.smelting.iter().map(Into::into).find(|recipe| self.matches(recipe));

            if let Some(matched_recipe) = query {
                return Ok(matched_recipe.clone());
            }
        }

        // Check if the request matches any of the GregTech machines.
        for machine in &recipe_database.machines {
            if self.machine.recipes.contains(&machine.name) {
                machine_present = true;

                let query = machine.recipes.iter().find(|recipe| self.matches(recipe));

                if let Some(matched_recipe) = query {
                    return Ok(matched_recipe.clone());
                }
            }
        }

        if machine_present {
            Err(MainError::MachineNotFound)
        } else {
            Err(MainError::RecipeNotFound)
        }
    }
}

impl Display for OptimizationRequest {
    #[rustfmt::skip]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Version:                    {}", self.version)?;
        writeln!(f, "Machine:                  \n{}", self.machine)?;
        writeln!(f, "Ticks:                      {}", self.ticks)?;
        writeln!(f, "Skip NCs:                   {}", self.skip)?;
        writeln!(f, "Restore Missing:            {}", self.restore)?;
        writeln!(f, "Multiplier:                 {}", self.multiplier.as_ref().map_or("None".into(), u64::to_string))?;
        writeln!(f, "Inputs:                   \n{}", self.inputs.iter().join("\n------------------------------------------------------\n"))?;
        writeln!(f, "Outputs:                  \n{}", self.outputs.iter().join("\n------------------------------------------------------\n"))
    }
}
