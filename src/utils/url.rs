use url::{Url, ParseError};

pub fn is_valid(url: &str) -> bool {
    match Url::parse(url){
        Ok(url) => {
            let scheme = url.scheme();
            if scheme != "http" && scheme != "https" {
                println!("Error: The URL does not have a valid base protocol.");
                false
            }else{
                true
            }
        },

        Err(e) => {
            match e {
                ParseError::InvalidIpv4Address => {println!("Error: The URL contains an invalid IPv4 address.");},
                ParseError::InvalidIpv6Address => {println!("Error: The URL contains an invalid IPv6 address.");},
                ParseError::RelativeUrlWithoutBase => {println!("Error: The URL does not have a valid base protocol.");},
                ParseError::InvalidPort => {println!("Error: The URL contains an invalid port number.");},
                _ => {println!("Error: Failed to parse the URL ({:?}).", e);},
            }
            false
        },
    }
   
}


#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn test_is_valid() {
        let mut url = "www.eecg.toronto.edu";
        assert_eq!(is_valid(url), false);
        url = "data://www.eecg.toronto.edu";
        assert_eq!(is_valid(url), false);
    }


}