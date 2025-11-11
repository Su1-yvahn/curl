mod utils;

use crate::utils::cli;
use crate::utils::method::MethodType;
use structopt::StructOpt;
use crate::utils::get::url_get;
use crate::utils::post::url_post;


fn main() {
    let input = cli::Opt::from_args();
    println!("Requesting URL: {}", input.url);
    let mut update_method = input.method;
    if input.json != None || input.data != None {
        update_method = "POST".to_string();
    }
    let method: MethodType = match MethodType::get_method_type(&update_method){
        Ok(m) => m,
        Err(e) => {eprintln!("Err: {}", e); return;},
    };
    println!("Method: {}", update_method);

    match method {
        MethodType::GET => {url_get(input.url.as_str());}
        MethodType::POST => {url_post(input.url.as_str(), &input.data, &input.json)}
    }
    
}
