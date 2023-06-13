use serde::{Deserialize, Serialize};
use std::fs;
use std::net::SocketAddr;

use bevy::prelude::*;

#[derive(Deserialize, Serialize, PartialEq, Debug)]
struct LoginServer {
    name: String,
    endpoint: SocketAddr,
}

#[derive(Deserialize, Serialize, PartialEq, Debug)]
struct Config {
    version: String,
    login_servers: Vec<LoginServer>,
}

fn main() {
    let config_yaml: String = fs::read_to_string("data/config.yaml").unwrap();
    let config: Config = serde_yaml::from_str(&config_yaml).unwrap();
    println!("Version: {}", config.version);
    println!("Number of login servers: {}", config.login_servers.len());
    App::new().add_plugins(DefaultPlugins).run()
}
