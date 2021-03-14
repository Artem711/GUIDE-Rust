use serde_derive::{ Serialize, Deserialize };
mod yaml;

fn main() {
    let config = yaml::ServerConfig {
        workers: 100,
        ignore: false,
        auth_server: Some("auth.server.io".to_string()),
    };

    yaml::yaml_serialization(config);
}
