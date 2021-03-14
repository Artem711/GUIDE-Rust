
use serde_yaml;
use serde_derive::{ Serialize, Deserialize };

pub fn yaml_serialization(config: ServerConfig) {
    println!("YAML Serialization");

    let serialized = serde_yaml::to_string(&config).unwrap();
    println!("Serialized: {}", serialized);

    let deserialized: ServerConfig = serde_yaml::from_str(&serialized).unwrap();
    println!("Deserialized: {:?}", deserialized)
}

// We will serialize/deserialize instances of this struct
#[derive(Serialize, Deserialize, Debug)]
pub struct ServerConfig {
    pub workers: u64,
    pub ignore: bool,
    pub auth_server: Option<String>,
}
