use config::Config;
use serde::Deserialize;
use std::path::Path;

use entity::university;

#[derive(Debug, Deserialize)]
pub struct General {
    pub database_uri: String,
    pub assets_root: String,
}

#[derive(Debug, Deserialize)]
pub struct Base {
    title: String,
    colour: String,
    longitude: f32,
    latitude: f32,
}
impl From<Base> for university::Model {
    fn from(val: Base) -> Self {
        university::Model {
            id: -1,
            title: val.title,
            icon: String::new(),
            colour: val.colour,
            longitude: val.longitude,
            latitude: val.latitude,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub general: General,
    pub base: Base,
}

impl Settings {
    #[must_use]
    pub fn new(location: &Path) -> Self {
        let config = Config::builder()
            .add_source(config::File::with_name(&location.to_string_lossy()))
            .build()
            .expect("Failed to read config");
        config.try_deserialize().expect("Failed to parse config")
    }
}
