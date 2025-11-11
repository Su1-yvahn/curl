pub enum MethodType{
    POST,
    GET,
}

impl MethodType{
    pub fn get_method_type(method_type:&str)->Result<Self, String>{
        match method_type{
            "GET" => Ok(MethodType::GET),
            "POST" => Ok(MethodType::POST),
            _=> Err(format!("Invalid method type: {}", method_type)),
        }
    }   
}





