use reqwest::json::get;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: curl <URL>");
        process::exit(1);
    }

    let url = &args[1];
    println!("Requesting URL: {}", url);
    println!("Method: GET");

    match get(url) {
        Ok(response) => {
            let body = response.text().unwrap_or_else(|_| String::from("Failed to read response body."));
            println!("Response body:\n{}", body);
        }

        Err(err) => {
            eprintln!("Error: {}", err);
        }

    }
}
