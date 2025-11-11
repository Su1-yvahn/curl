use reqwest::blocking::get;
use super::url::is_valid;

pub fn url_get(url:&str){

    let url_is_valid = is_valid(url);
    if url_is_valid {
        match get(url) {
            Ok(response) =>  {
                if !response.status().is_success(){
                    println!("Error: Request failed with status code: {}", response.status());
                }else{
                    if let Ok(body) = response.text() {
                        println!("Response body:");
                        println!("{}", body)
                    }     
                } 
            },
            Err(_) => {println!("Error: Unable to connect to the server. Perhaps the network is offline or the server hostname cannot be resolved.");},
        } 
    }
}