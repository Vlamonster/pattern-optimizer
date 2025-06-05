mod machines;
mod model;
mod optimize;
mod request;

use {
    crate::{
        model::RecipeDatabase,
        optimize::optimize,
        request::OptimizationRequest,
    },
    serde_json::{
        json,
        Deserializer,
    },
    std::{
        collections::HashMap,
        io::{
            BufReader,
            Write,
        },
        net::TcpListener,
        sync::Arc,
        thread,
    },
    tap::Pipe,
};

pub enum MainError {
    RecipeNotFound,
    MachineNotFound,
    NotEnoughEnergy(u64, u64),
}

fn main() -> Result<(), std::io::Error> {
    let listener = TcpListener::bind("0.0.0.0:3939")?;
    println!("Server listening on port 3939");

    let recipes = Arc::new(HashMap::from([
        (
            "2.7.3".to_string(),
            serde_json::from_str::<RecipeDatabase>(include_str!("resources/recipes-2.7.3.json"))?,
        ),
        (
            "n987".to_string(),
            serde_json::from_str::<RecipeDatabase>(include_str!("resources/recipes-n987.json"))?,
        ),
    ]));

    for stream in listener.incoming() {
        let mut stream = match stream {
            Ok(stream) => stream,
            Err(error) => {
                println!("Connection failed: {error}");
                continue;
            },
        };

        let recipes = Arc::clone(&recipes);

        thread::spawn(move || {
            let peer_addr = stream.peer_addr().unwrap();
            println!("Client connected: {peer_addr}");

            let requests = stream
                .try_clone()
                .unwrap()
                .pipe(BufReader::new)
                .pipe(Deserializer::from_reader)
                .into_iter::<OptimizationRequest>();

            for request in requests {
                let request = match request {
                    Ok(request) => request,
                    Err(err) => {
                        println!("Failed to parse request JSON: {err}");
                        let response = json!({"error": "Internal Server Error: Parsing\n"});
                        stream.write_all(response.to_string().as_bytes()).unwrap();
                        continue;
                    },
                };

                println!("Received a request from {peer_addr}:\n{request}");

                let optimized_pattern = request
                    .query(&recipes[&request.version])
                    .and_then(|matched_recipe| optimize(&request, &matched_recipe));

                let response = match optimized_pattern {
                    Ok(optimized_pattern) => serde_json::to_string(&optimized_pattern).unwrap(),
                    Err(MainError::RecipeNotFound) => json!({"error": "Recipe not found for the given inputs\n"}).to_string(),
                    Err(MainError::MachineNotFound) => json!({"error": "Machine not found\n"}).to_string(),
                    Err(MainError::NotEnoughEnergy(provided, required)) =>
                        json!({"error": format!("Not enough energy. Provided: {provided}, Required: {required}\n")}).to_string(),
                };

                stream.write_all(response.as_bytes()).unwrap();
            }
        });
    }

    unreachable!("`listener.incoming()` will never return None.")
}
