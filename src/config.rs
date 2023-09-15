use serde::{Deserialize, Serialize};
use serde_yaml;
use std::{fs::File, io::Read};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct GatewayConfig {
    pub services: Vec<ServiceConfig>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ServiceConfig {
    pub path: String,
    pub target_service: String,
}

pub fn load_config(file_path: &str) -> GatewayConfig {
    let mut file = File::open(file_path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    serde_yaml::from_str(&contents).unwrap()
}
