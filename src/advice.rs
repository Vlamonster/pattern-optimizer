use crate::model::Recipe;
use serde::Serialize;
use std::collections::HashMap;

/// Represents the optimized processing plan returned by the server.
///
/// # Fields
/// - `inputs`: A list of `AdvisedItem`s representing the adjusted input materials.
/// - `outputs`: A list of `AdvisedItem`s representing the expected output materials.
#[derive(Serialize, Debug)]
pub struct OptimizedPattern {
    pub inputs: Vec<AdvisedItem>,
    pub outputs: Vec<AdvisedItem>,
}

/// Represents an optimized item with adjusted batch size and metadata.
///
/// # Fields
/// - `id`: The unique identifier of the item (typically the registry name).
/// - `amount`: The advised quantity of this item for the optimized batch.
/// - `meta`: The metadata or durability value of the item.
/// - `nbt`: The item's associated NBT (if any), represented as a string.
#[derive(Serialize, Debug)]
pub struct AdvisedItem {
    pub id: String,
    pub amount: u64,
    pub meta: u64,
    pub nbt: String,
}

pub fn advise(
    damage_map: &HashMap<String, u64>,
    recipe: &Recipe,
    advised_batch_size: u64,
) -> OptimizedPattern {
    let item_inputs = recipe.item_inputs.iter().map(|item| AdvisedItem {
        id: item.id.clone().unwrap(),
        amount: u64::max(item.amount * advised_batch_size, 1),
        meta: damage_map[&item.localized_name.clone().unwrap()],
        nbt: item.nbt.clone().unwrap_or_default(),
    });

    let fluid_inputs = recipe.fluid_inputs.iter().map(|fluid| AdvisedItem {
        id: "ae2fc:fluid_drop".to_string(),
        amount: fluid.amount * advised_batch_size,
        meta: 0,
        nbt: format!("{{Fluid: \"{}\"}}", fluid.id),
    });

    let item_outputs = recipe.item_outputs.iter().map(|item| AdvisedItem {
        id: item.id.clone().unwrap(),
        amount: u64::max(item.amount * advised_batch_size, 1),
        meta: item.meta,
        nbt: item.nbt.clone().unwrap_or_default(),
    });

    let fluid_outputs = recipe.fluid_outputs.iter().map(|fluid| AdvisedItem {
        id: "ae2fc:fluid_drop".to_string(),
        amount: fluid.amount * advised_batch_size,
        meta: 0,
        nbt: format!("{{Fluid: \"{}\"}}", fluid.id),
    });

    OptimizedPattern {
        inputs: item_inputs.chain(fluid_inputs).collect(),
        outputs: item_outputs.chain(fluid_outputs).collect(),
    }
}
