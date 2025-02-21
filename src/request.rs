use crate::advice::{advise, OptimizedPattern};
use crate::machines::get_advised_batch;
use crate::model::RecipeDatabase;
use crate::optimization_request::OptimizationRequest;
use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::TcpStream;

pub enum RecipeLookupResult {
    Found(OptimizedPattern),
    RecipeNotFound,
    MachineNotFound,
    WrongCategory,
}

pub fn handle_client(mut stream: TcpStream, recipes: &RecipeDatabase) {
    println!("Client connected: {}", stream.peer_addr().unwrap());
    let mut buffer = [0; 8192];

    loop {
        let bytes_read = match stream.read(&mut buffer) {
            Ok(0) => {
                println!("Client disconnected: {}", stream.peer_addr().unwrap());
                return;
            }
            Ok(bytes) => bytes,
            Err(error) => {
                eprintln!("Failed to read from socket: {error}");
                break;
            }
        };

        let request = match serde_json::from_slice(&buffer[..bytes_read]) {
            Ok(req) => req,
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
                let error_message = r#"{"error": "Recipe not found for the given inputs"}"#;
                if let Err(error) = stream.write_all((error_message.to_string() + "\n").as_bytes())
                {
                    eprintln!("Failed to write to socket: {error}");
                    continue;
                }
                stream.flush().unwrap();
            }
            RecipeLookupResult::MachineNotFound => {
                let error_message = r#"{"error": "Machine not found"}"#;
                if let Err(error) = stream.write_all((error_message.to_string() + "\n").as_bytes())
                {
                    eprintln!("Failed to write to socket: {error}");
                    break;
                }
                stream.flush().unwrap();
            }
            RecipeLookupResult::WrongCategory => {
                let error_message = r#"{"error": "Category does not exist"}"#;
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
                return RecipeLookupResult::Found(advise(
                    &meta_map,
                    recipe,
                    get_advised_batch(&request.machine, request.ticks, recipe),
                ));
            }
            return RecipeLookupResult::RecipeNotFound;
        }
    }
    RecipeLookupResult::MachineNotFound
}
