use crate::advice::{advise, OptimizedPattern};
use crate::machines::advised_batch;
use crate::model::{FurnaceRecipe, GregTechRecipe, RecipeDatabase, RecipeFluid, RecipeItem};
use crate::optimization_request::{OptimizationRequest, RequestItem};
use serde_json::{json, Deserializer};
use std::collections::HashMap;
use std::io::Write;
use std::net::TcpStream;

pub enum RecipeLookupResult {
    Found(OptimizedPattern),
    RecipeNotFound,
    MachineNotFound,
    NotEnoughEnergy(u64, u64),
}

pub fn handle_client(mut stream: TcpStream, recipes: &RecipeDatabase) {
    let peer_addr = stream.peer_addr().unwrap();
    println!("Client connected: {peer_addr}");

    for request in Deserializer::from_reader(stream.try_clone().unwrap()).into_iter() {
        let request = match request {
            Ok(r) => r,
            Err(err) => {
                eprintln!("Failed to parse request JSON: {err}");
                continue;
            }
        };

        println!("Received a request from {peer_addr}:");
        println!("{request}");

        match process_request(&request, recipes) {
            RecipeLookupResult::Found(response) => {
                if let Err(error) =
                    stream.write_all((serde_json::to_string(&response).unwrap() + "\n").as_bytes())
                {
                    eprintln!("Failed to write to socket: {error}");
                    continue;
                }
                stream.flush().unwrap();
            }
            RecipeLookupResult::RecipeNotFound => {
                let error_message = json!({"error": "Recipe not found for the given inputs"});
                if let Err(error) = stream.write_all((error_message.to_string() + "\n").as_bytes())
                {
                    eprintln!("Failed to write to socket: {error}");
                    continue;
                }
                stream.flush().unwrap();
            }
            RecipeLookupResult::MachineNotFound => {
                let error_message = json!({"error": "Machine not found"});
                if let Err(error) = stream.write_all((error_message.to_string() + "\n").as_bytes())
                {
                    eprintln!("Failed to write to socket: {error}");
                    break;
                }
                stream.flush().unwrap();
            }
            RecipeLookupResult::NotEnoughEnergy(provided, required) => {
                let error_message = json!({"error": format!("Not enough energy. Provided: {provided}, Required: {required}")});
                if let Err(error) = stream.write_all((error_message.to_string() + "\n").as_bytes())
                {
                    eprintln!("Failed to write to socket: {error}");
                    break;
                }
                stream.flush().unwrap();
            }
        }
    }
}

pub fn process_request(
    request: &OptimizationRequest,
    recipes: &RecipeDatabase,
) -> RecipeLookupResult {
    let mut machine_present = false;

    // Check Multi Smelter recipes
    if request
        .machine
        .recipes
        .contains(&"Multi Smelter".to_string())
    {
        machine_present = true;

        if let Some(recipe) = recipes
            .smelting
            .iter()
            .map(furnace_to_gregtech_recipe)
            .find(|recipe| matches_request(recipe, request))
        {
            return optimize_recipe(request, &recipe);
        }
    }

    // Check other GregTech machines
    for machine in &recipes.machines {
        if !request.machine.recipes.contains(&machine.name) {
            continue;
        }
        machine_present = true;

        if let Some(recipe) = machine
            .recipes
            .iter()
            .find(|recipe| matches_request(recipe, request))
        {
            println!("{recipe:?}\n{request:?}");
            return optimize_recipe(request, recipe);
        }
    }

    if machine_present {
        RecipeLookupResult::RecipeNotFound
    } else {
        RecipeLookupResult::MachineNotFound
    }
}

fn furnace_to_gregtech_recipe(recipe: &FurnaceRecipe) -> GregTechRecipe {
    GregTechRecipe {
        enabled: true,
        duration: 512,
        energy_usage: 4,
        special: 0,
        item_inputs: vec![recipe.input.clone()],
        item_outputs: vec![recipe.output.clone()],
        fluid_inputs: vec![],
        fluid_outputs: vec![],
    }
}

fn matches_item(request_item: &RequestItem, recipe_items: &[RecipeItem]) -> bool {
    recipe_items.iter().any(|recipe_item| {
        recipe_item.id.as_ref() == Some(&request_item.name)
            && (recipe_item.meta == request_item.damage || recipe_item.meta == 32767)
    })
}

fn matches_fluid(request_item: &RequestItem, recipe_fluids: &[RecipeFluid]) -> bool {
    request_item.fluid_drop.as_ref().is_some_and(|fluid| {
        recipe_fluids
            .iter()
            .any(|recipe_fluid| recipe_fluid.id == fluid.name)
    })
}

fn matches_request(recipe: &GregTechRecipe, request: &OptimizationRequest) -> bool {
    let inputs_match = request.inputs.iter().all(|item| {
        matches_item(item, &recipe.item_inputs) || matches_fluid(item, &recipe.fluid_inputs)
    });

    let outputs_match = request.outputs.iter().all(|item| {
        matches_item(item, &recipe.item_outputs) || matches_fluid(item, &recipe.fluid_outputs)
    });

    inputs_match && outputs_match
}

fn optimize_recipe(request: &OptimizationRequest, recipe: &GregTechRecipe) -> RecipeLookupResult {
    if request.machine.energy_usage < recipe.energy_usage
        && !request.machine.id.starts_with("Helio")
    {
        return RecipeLookupResult::NotEnoughEnergy(
            request.machine.energy_usage,
            recipe.energy_usage,
        );
    }

    let meta_variants = recipe
        .item_inputs
        .iter()
        .filter(|input| input.meta == 32767)
        .map(|input| (input.id.clone().unwrap(), input.meta))
        .collect::<HashMap<String, u64>>();

    let (batch_size, duration) = advised_batch(&request.machine, request.ticks, recipe);
    let optimized_pattern = advise(&meta_variants, recipe, batch_size, duration, request.skip);
    RecipeLookupResult::Found(optimized_pattern)
}
