use crate::advice::{advise, OptimizedPattern};
use crate::machines::advised_batch;
use crate::model::RecipeDatabase;
use crate::optimization_request::OptimizationRequest;
use serde_json::{json, Deserializer};
use std::collections::HashMap;
use std::io::Write;
use std::net::TcpStream;

pub enum RecipeLookupResult {
    Found(OptimizedPattern),
    RecipeNotFound,
    MachineNotFound,
    WrongCategory,
}

pub fn handle_client(mut stream: TcpStream, recipes: &RecipeDatabase) {
    println!("Client connected: {}", stream.peer_addr().unwrap());

    for request in Deserializer::from_reader(stream.try_clone().unwrap()).into_iter() {
        let request = match request {
            Ok(r) => r,
            Err(err) => {
                eprintln!("Failed to parse request JSON: {err}");
                continue;
            }
        };

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
            RecipeLookupResult::WrongCategory => {
                let error_message = json!({"error": "Category does not exist"});
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
    for source in &recipes.sources {
        if source.category != "gregtech" {
            return RecipeLookupResult::WrongCategory;
        }
        for machine in &source.machines {
            if !request.machine.recipes.contains(&machine.name) {
                continue;
            }
            if let Some(recipe) = machine.recipes.iter().find(|recipe| {
                request.inputs.iter().all(|request_item| {
                    if let Some(fluid_drop) = &request_item.fluid_drop {
                        recipe
                            .fluid_inputs
                            .iter()
                            .any(|recipe_fluid| recipe_fluid.id == fluid_drop.name)
                    } else {
                        recipe.item_inputs.iter().any(|recipe_item| {
                            recipe_item.id.as_ref() == Some(&request_item.name)
                                && (recipe_item.meta == request_item.damage
                                    || recipe_item.meta == 32767)
                        })
                    }
                })
            }) {
                let meta_map = recipe
                    .item_inputs
                    .iter()
                    .filter(|input| input.meta == 32767)
                    .map(|input| (input.id.clone().unwrap(), input.meta))
                    .collect::<HashMap<String, u64>>();
                let (advised_batch, duration) =
                    advised_batch(&request.machine, request.ticks, recipe);
                let optimized_pattern = advise(&meta_map, recipe, advised_batch, duration);
                return RecipeLookupResult::Found(optimized_pattern);
            }
            return RecipeLookupResult::RecipeNotFound;
        }
    }
    RecipeLookupResult::MachineNotFound
}
