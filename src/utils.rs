extern crate dotenv;
use dotenv::dotenv;
use std::env;

pub fn pd(message: &str) {
    // Load environment variables from the .env file (if it exists).
    dotenv().ok();

    // Check the value of the DEBUG environment variable.
    if let Ok(debug_value) = env::var("DEBUG") {
        if debug_value == "1" {
            println!("{}", message);
        }
    }
}
