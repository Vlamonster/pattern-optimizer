mod advice;
mod machines;
mod model;
mod optimization_request;
mod request;

use crate::model::RecipeDatabase;
use crate::request::handle_client;
use std::net::TcpListener;
use std::sync::Arc;
use std::thread;

fn main() -> Result<(), std::io::Error> {
    let listener = TcpListener::bind("0.0.0.0:3939")?;
    println!("Server listening on port 3939");

    let recipes = Arc::new(
        serde_json::from_str::<RecipeDatabase>(include_str!("resources/recipes.json")).unwrap(),
    );

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let recipes = Arc::clone(&recipes);
                thread::spawn(move || {
                    handle_client(stream, &recipes);
                });
            }
            Err(error) => {
                eprintln!("Connection failed: {error}");
            }
        }
    }
    Ok(())
}
