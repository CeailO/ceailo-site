use std::{fs, io::Error as IoError};

use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct Config {
    #[allow(dead_code)]
    server_address: String,
    #[allow(dead_code)]
    server_port: String,
    #[allow(dead_code)]
    username: String,
    #[allow(dead_code)]
    password: String,
    #[allow(dead_code)]
    service_token: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ConfigToml {
    server: Option<ConfigTomlServer>,
    database: Option<ConfigTomlDatabase>,
    api: Option<ConfigTomlApi>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ConfigTomlServer {
    server_address: Option<String>,
    server_port: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ConfigTomlDatabase {
    username: Option<String>,
    password: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ConfigTomlApi {
    service_token: Option<String>,
}

impl Config {
    pub fn _new() -> Self {
        let config_path: [&str; 3] = ["./Config.toml", "./config.toml", "~/.config"];

        let mut content = "".to_owned();

        for path in config_path {
            let result: Result<String, IoError> = fs::read_to_string(path);

            while result.is_ok() {
                content = result.unwrap();
                break;
            }
        }
        println!("{}", content);
        let config_toml: ConfigToml = toml::from_str(&content).unwrap_or_else(|_| {
            println!("Failed to create ConfigToml Object out of config file");
            ConfigToml {
                server: None,
                database: None,
                api: None,
            }
        });

        let (server_address, server_port) = match config_toml.server {
            Some(server) => {
                let server_address = server.server_address.unwrap_or_else(|| {
                    println!("Missing field server address in table server.");
                    "unknown".to_owned()
                });
                let server_port = server.server_port.unwrap_or_else(|| {
                    println!("Missing field server port in table server.");
                    "unknown".to_owned()
                });
                (server_address, server_port)
            }
            None => {
                println!("Missing table server");
                ("Unknown".to_owned(), "Unknown".to_owned())
            }
        };
        // println!("{}: {}", server_address, server_port);

        let (username, password) = match config_toml.database {
            Some(database) => {
                let username = database.username.unwrap_or_else(|| {
                    println!("Missing field username in table database.");
                    "unknown".to_owned()
                });
                let password = database.password.unwrap_or_else(|| {
                    println!("Missing field password in table database.");
                    "unknown".to_owned()
                });
                (username, password)
            }
            None => {
                println!("Missing table database");
                ("Unknown".to_owned(), "Unknown".to_owned())
            }
        };
        // println!("{} {}", username, password);

        let service_token = match config_toml.api {
            Some(key) => key.service_token.unwrap_or_else(|| {
                println!("Missing service token in table api");
                "Unknown".to_owned()
            }),
            None => {
                println!("Missing table api");
                "Unknown".to_owned()
            }
        };
        // println!("{}", service_token);

        Self {
            server_address: server_address,
            server_port: server_port,
            username: username,
            password: password,
            service_token: service_token,
        }
    }
}
