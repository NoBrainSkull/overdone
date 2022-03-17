use bevy::prelude::*;
use config::Config;
use std::collections::HashMap;

pub struct LoadConfigPlugin;

#[derive(Default)]
pub struct GameConfig {
    pub settings: HashMap<String, String>,
}

impl GameConfig {
    pub fn fetch(&self, prop: &str) -> String {
        println!("{:?}", self.settings);
        self.settings
            .get(prop)
            .expect("At least one setting is not configured")
            .to_owned()
    }
}

fn load_config(mut config: ResMut<GameConfig>) {
    let settings = Config::builder()
        .add_source(config::File::with_name("config/Settings"))
        .add_source(config::Environment::with_prefix("GAME"))
        .build()
        .expect("File Settings not found at config/Settings.toml")
        .try_deserialize::<HashMap<String, String>>()
        .expect("Config file could not be parsed to HashMap<String, String>");

    config.settings = settings;
}

impl Plugin for LoadConfigPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameConfig>().add_system(load_config);
    }
}
