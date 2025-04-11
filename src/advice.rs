use crate::model::GregTechRecipe;
use crate::optimization_request::OptimizationRequest;
use serde::Serialize;
use std::collections::HashMap;
use std::iter::once;

/// Represents the optimized processing plan returned by the server.
#[derive(Serialize, Debug)]
pub struct OptimizedPattern {
    /// A list of `AdvisedItem`s representing the adjusted input materials.
    pub inputs: Vec<AdvisedItem>,

    /// A list of `AdvisedItem`s representing the expected output materials.
    pub outputs: Vec<AdvisedItem>,

    /// The number of ticks the optimized process is expected to take.
    pub duration: u64,
}

/// Represents an optimized item with adjusted batch size and metadata.
#[derive(Serialize, Debug)]
pub struct AdvisedItem {
    /// The unique identifier of the item (typically the registry name).
    pub id: String,

    /// The advised quantity of this item for the optimized batch.
    pub amount: u64,

    /// The metadata or durability value of the item.
    pub meta: u64,

    /// The item's associated NBT (if any), represented as a string.
    pub nbt: String,
}

pub fn advise(
    recipe: &GregTechRecipe,
    advised_batch: u64,
    duration: u64,
    request: &OptimizationRequest,
) -> OptimizedPattern {
    let meta_map = recipe
        .item_inputs
        .iter()
        .filter(|input| input.meta == 32767)
        .map(|input| (input.id.clone().unwrap(), input.meta))
        .collect::<HashMap<String, u64>>();

    let max_factor = recipe
        .item_inputs
        .iter()
        .chain(recipe.item_outputs.iter())
        .map(|item| item.amount)
        .chain(
            recipe
                .fluid_inputs
                .iter()
                .chain(recipe.fluid_outputs.iter())
                .map(|fluid| fluid.amount),
        )
        .map(|amount| i32::MAX as u64 / u64::max(amount, 1))
        .chain(once(advised_batch))
        .min()
        .unwrap();

    let item_inputs = recipe
        .item_inputs
        .iter()
        .filter(|item| !request.skip || item.amount > 0)
        .map(|item| AdvisedItem {
            id: item.id.clone().unwrap(),
            amount: u64::max(item.amount * max_factor, 1),
            meta: *meta_map
                .get(&item.id.clone().unwrap())
                .unwrap_or(&item.meta),
            nbt: item.nbt.clone().unwrap_or_default(),
        });

    let fluid_inputs = recipe.fluid_inputs.iter().map(|fluid| AdvisedItem {
        id: "ae2fc:fluid_drop".to_string(),
        amount: fluid.amount * max_factor,
        meta: 0,
        nbt: format!("{{Fluid: \"{}\"}}", fluid.id),
    });

    let item_outputs = recipe
        .item_outputs
        .iter()
        .filter(|recipe_item| {
            request.restore
                || request.outputs.iter().any(|request_item| {
                    recipe_item.id.as_ref() == Some(&request_item.name)
                        && (recipe_item.meta == request_item.damage || recipe_item.meta == 32767)
                })
        })
        .map(|item| AdvisedItem {
            id: item.id.clone().unwrap(),
            amount: u64::max(item.amount * max_factor, 1),
            meta: item.meta,
            nbt: item.nbt.clone().unwrap_or_default(),
        });

    let fluid_outputs = recipe
        .fluid_outputs
        .iter()
        .filter(|recipe_fluid| {
            request.restore
                || request.outputs.iter().any(|request_item| {
                    request_item
                        .fluid_drop
                        .as_ref()
                        .is_some_and(|fluid| fluid.name == recipe_fluid.id)
                })
        })
        .map(|fluid| AdvisedItem {
            id: "ae2fc:fluid_drop".to_string(),
            amount: fluid.amount * max_factor,
            meta: 0,
            nbt: format!("{{Fluid: \"{}\"}}", fluid.id),
        });

    OptimizedPattern {
        inputs: item_inputs.chain(fluid_inputs).collect(),
        outputs: item_outputs.chain(fluid_outputs).collect(),
        duration,
    }
}
