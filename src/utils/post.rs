use super::url::is_valid;
use reqwest::{Result, blocking::*};
use reqwest::header::CONTENT_TYPE;
use serde_json::{from_str, Value, to_string_pretty};

fn sort_keys(value: Value) -> Value {
    match value {
        Value::Object(map) => {
            let mut sorted = serde_json::Map::new();
            let mut keys: Vec<_> = map.keys().collect();
            keys.sort();
            for k in keys {
                sorted.insert(k.clone(), sort_keys(map[k].clone()));
            }
            Value::Object(sorted)
        }
        v => v,
    }
}

fn manage_response(response: Result <Response>){
    match response{
        Ok(response) => {
            if !response.status().is_success(){
                println!("Error: Request failed with status code: {}", response.status());
            }else{
                let body = &response.text().unwrap();
                if let Ok(response) = from_str::<Value>(&body) {
                    
                    let sorted = sort_keys(response);
                    println!("Response body (JSON with sorted keys):");
                    println!("{}", to_string_pretty(&sorted).unwrap());
                }else{
                    println!("Response body: (JSON with sorted keys):");
                    println!("{}", body);
                }
            } 
        },
        Err(_) => {println!("Error: Unable to connect to the server. Perhaps the network is offline or the server hostname cannot be resolved.");},
    }
}

pub fn url_post(url: &str, data: & Option<String>, json: & Option< String> ){

    let data = match data {
        Some(data) => data,
        None => "no data",
    };
    let json = match json {
        Some(j) => j,
        None => "no json",
    };

    if data != "no data"{
        println!("Data: {}",data);
    }
    if json != "no json"{
        println!("JSON: {}",json);
    }
    

    let client = Client::new();
    let url_is_valid = is_valid(url);

    if url_is_valid {

        if data != "no data"{
            let response = client.post(url).header(CONTENT_TYPE, "application/x-www-form-urlencoded").body(data.to_string()).send();
            manage_response(response);

        }else if json != "no json"{
            let response = client.post(url).header(CONTENT_TYPE, "application/json").body(json.to_string()).send();
            manage_response(response);
        }    

        
        
    }
}
