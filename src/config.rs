use serde::{Deserialize, Serialize};
use std::{fs::File, io::Read};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct GatewayConfig {
    #[serde(rename = "socket_address")]
    pub socket_addr: SocketAddr,
    pub services: Vec<ServiceConfig>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SocketAddr {
    #[serde(rename = "ip_address")]
    pub ip: String,
    pub port: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ServiceConfig {
    pub path: String,
    pub target_service: String,
    pub ty: ServiceType,
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum ServiceType {
    Openai,
    Llama2,
    Test,
}

pub fn load_config(file_path: &str) -> GatewayConfig {
    let mut file = File::open(file_path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    serde_yaml::from_str(&contents).unwrap()
}
