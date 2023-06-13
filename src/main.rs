use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs;
use std::net::SocketAddr;
use std::str;

use bevy::prelude::*;

#[derive(Deserialize, Default, Serialize, PartialEq, Debug)]
struct AudioSettings {
    default_bgm: String,
    bgm_volume: f32,
    fx_volume: f32,
}

#[derive(Deserialize, Serialize, PartialEq, Debug)]
struct LoginServer {
    name: String,
    endpoint: SocketAddr,
}

impl Default for LoginServer {
    fn default() -> Self {
        LoginServer {
            name: "localhost".parse().expect("Failed to set default hostname"),
            endpoint: "127.0.0.1:6900"
                .parse::<SocketAddr>()
                .expect("Failed to parse default endpoint"),
        }
    }
}

#[derive(Deserialize, Default, Serialize, PartialEq, Debug)]
struct Config {
    version: String,
    audio: AudioSettings,
    login_servers: Vec<LoginServer>,
}

#[derive(Resource, Default)]
struct State {
    config: Config,
    bgm: Handle<AudioSink>,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<State>()
        .add_startup_system(load_config)
        .add_startup_system(load_bgm)
        .run()
}

fn load_config(mut state: ResMut<State>) {
    let config_yaml = fs::read_to_string("assets/cracl.cfg").expect("Failed to load config file");
    state.config = serde_yaml::from_str(&config_yaml).unwrap();
}

fn load_bgm(mut state: ResMut<State>, asset_server: Res<AssetServer>, audio: Res<Audio>) {
    let music = asset_server.load(&state.config.audio.default_bgm);
    state.bgm = audio.play_with_settings(
        music,
        PlaybackSettings::LOOP.with_volume(state.config.audio.bgm_volume),
    );
}
