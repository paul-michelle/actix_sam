use serde::Deserialize;
use serde_aux::field_attributes::deserialize_number_from_string;
use std::env;

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

pub enum Environment {
    Local,
    Production,
}

impl Environment {
    pub fn as_str(&self) -> &'static str {
        match self {
            Environment::Local => "local",
            Environment::Production => "production",
        }
    }
}

impl TryFrom<String> for Environment {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.to_lowercase().as_str() {
            "local" => Ok(Environment::Local),
            "production" => Ok(Environment::Production),
            unsupported => Err(format!("{unsupported} environment not supported")),
        }
    }
}

pub fn figure_environment() -> Environment {
    env::var("APP_ENVIRONMENT")
        .unwrap_or_else(|_| "local".into())
        .try_into()
        .expect("Failed to parse APP_ENVIRONMENT variable.")
}

pub fn get_settings(env: &Environment) -> Settings {
    let project_root =
        std::env::current_dir().expect("Failed to determine project's root directory.");
    let settings_dir = project_root.join("src").join("settings");
    let settings_file_name = format!("{}.yaml", env.as_str());

    config::Config::builder()
        .add_source(config::File::from(settings_dir.join("base.yaml")))
        .add_source(config::File::from(settings_dir.join(settings_file_name)))
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
