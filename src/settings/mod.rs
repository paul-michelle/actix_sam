use serde::Deserialize;
use serde_aux::field_attributes::deserialize_number_from_string;

#[derive(Deserialize)]
pub struct Settings {
    pub app: AppSetings,
}

#[derive(Deserialize)]
pub struct AppSetings {
    pub host: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
}

pub fn get_local_settings() -> Settings {
    let project_root =
        std::env::current_dir().expect("Failed to determine project's root directory.");
    let settings_dir = project_root.join("src").join("settings");
    config::Config::builder()
        .add_source(config::File::from(settings_dir.join("local.yaml")))
        .add_source(
            config::Environment::with_prefix("APP")
                .prefix_separator("_")
                .separator("__"),
        )
        .build()
        .expect("Failed to build project settings.")
        .try_deserialize::<Settings>()
        .expect("Failed to deserialize project settings.")
}
