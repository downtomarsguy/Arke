use dotenv::dotenv;
use std::env;

fn main() {
    dotenv().ok();
    let api_key = env::var("API_KEY");

    match api_key {
        Ok(val) => println!("API_KEY: {:?}", val),
        Err(e) => println!("Error API_KEY: {}", e),
    }
}
