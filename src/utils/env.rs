use std::env;
use crate::types::errors::DefaultError;

pub fn get(key: &str, fallback: Option<&str>) -> Result<String, DefaultError> {
    
    match find(key) {
        Ok(value) => Ok(value),
        Err(_) => match fallback {
            Some(value) => Ok(value.to_string()),
            None => Err(DefaultError { message: "Env key not found".to_string() })
        }
    }
    
}

pub fn get_int(key: &str, fallback: Option<usize>) -> Result<usize, DefaultError> {
    
    match find(key) {
        Ok(value) => Ok(value.parse::<usize>().unwrap()),
        Err(_) => match fallback {
            Some(value) => Ok(value),
            None => Err(DefaultError { message: "Env key not found".to_string() })
        }
    }
    
}

pub fn get_bool(key: &str, fallback: Option<bool>) -> Result<bool, DefaultError> {
    
    match find(key) {
        Ok(value) => Ok(value.parse::<bool>().unwrap()),
        Err(_) => match fallback {
            Some(value) => Ok(value),
            None => Err(DefaultError { message: "Env key not found".to_string() })
        }
    }
    
}

pub fn find(key: &str) -> Result<String, DefaultError> {
    match env::vars().find(|x| x.0 == key.to_string()) {
        Some(x) => Ok(x.1),
        None => Err(DefaultError { message: "".to_string() })
    }
}