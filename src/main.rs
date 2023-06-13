use serde::Deserialize;
use std::fs;

use bevy::prelude::*;

#[derive(Deserialize, PartialEq, Debug)]
struct Config {
    version: String,
}

fn main() {
    let config_yaml: String = fs::read_to_string("data/config.yaml").unwrap();
    let config: Config = serde_yaml::from_str(&config_yaml).unwrap();
    println!("Version: {}", config.version);
    App::new().add_plugins(DefaultPlugins).run()
}
