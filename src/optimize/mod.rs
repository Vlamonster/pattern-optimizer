pub mod optimized_item;
pub mod optimized_pattern;
pub mod overclock;

use {
    crate::{
        machines::optimize_batch_size,
        model::GregTechRecipe,
        request::OptimizationRequest,
        MainError,
    },
    std::{
        collections::HashMap,
        iter::once,
    },
};
pub use {
    optimized_item::OptimizedItem,
    optimized_pattern::OptimizedPattern,
    overclock::Overclock,
};

/// Wildcard value for item metadata (matches any damage/meta).
pub const WILDCARD: u64 = (1 << 15) - 1;

pub fn optimize(request: &OptimizationRequest, recipe: &GregTechRecipe) -> Result<OptimizedPattern, MainError> {
    println!("Start optimizing");

    if request.machine.energy_usage < recipe.energy_usage {
        return Err(MainError::NotEnoughEnergy(request.machine.energy_usage, recipe.energy_usage));
    }

    let (optimal_batch_size, duration) = match request.multiplier {
        None => optimize_batch_size(request, recipe),
        Some(multiplier) => (multiplier, 0),
    };

    let meta_map = recipe
        .item_inputs
        .iter()
        .filter(|input| input.meta == WILDCARD)
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
        .chain(once(optimal_batch_size))
        .min()
        .unwrap();

    let item_inputs = recipe
        .item_inputs
        .iter()
        .filter(|item| !request.skip || item.amount > 0)
        .map(|item| OptimizedItem {
            id: item.id.clone().unwrap(),
            amount: u64::max(item.amount * max_factor, 1),
            meta: *meta_map.get(&item.id.clone().unwrap()).unwrap_or(&item.meta),
            nbt: item.nbt.clone().unwrap_or_default(),
        });

    let fluid_inputs = recipe.fluid_inputs.iter().map(|fluid| OptimizedItem {
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
                    recipe_item.id.as_ref() == Some(&request_item.id)
                        && (recipe_item.meta == request_item.meta || recipe_item.meta == WILDCARD)
                })
        })
        .map(|item| OptimizedItem {
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
                || request
                    .outputs
                    .iter()
                    .any(|request_item| request_item.fluid_drop.as_ref().is_some_and(|fluid| fluid.id == recipe_fluid.id))
        })
        .map(|fluid| OptimizedItem {
            id: "ae2fc:fluid_drop".to_string(),
            amount: fluid.amount * max_factor,
            meta: 0,
            nbt: format!("{{Fluid: \"{}\"}}", fluid.id),
        });

    Ok(OptimizedPattern {
        inputs: item_inputs.chain(fluid_inputs).collect(),
        outputs: item_outputs.chain(fluid_outputs).collect(),
        duration,
    })
}
