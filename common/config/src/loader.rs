use std::path::PathBuf;
use derive_builder::Builder;

use figment::Figment;
use figment::providers::{Format, Toml, Env, Serialized};

#[derive(Debug, Builder)]
pub struct ConfigLoader {
    #[builder(setter(into, strip_option), default)]
    pub toml_path: Option<PathBuf>,
    #[builder(setter(into, strip_option), default)]
    pub env_prefix: Option<String>,
}

impl ConfigLoader {
    pub fn load_with_extras<'de, Config, F>(&self, extras: F) -> Config
        where
            Config : Default + std::fmt::Debug
                + serde::ser::Serialize
                + serde::de::Deserialize<'de>,
            F: FnOnce(Figment) -> Figment,
    {
        let mut figment = Figment::new();
        figment = figment.merge(Serialized::defaults(Config::default()));
        if let Some(path) = &self.toml_path {
            if !path.exists() {
                tracing::warn!(config = std::any::type_name::<Config>(), path = path.to_string_lossy().to_string(), "config file not exist");
            } else {
                tracing::info!(config = std::any::type_name::<Config>(), path = path.to_string_lossy().to_string(), "loading config from file");
            }
            figment = figment.merge(Toml::file(path));
        }
        if let Some(env_prefix) = &self.env_prefix {
            tracing::info!(config = std::any::type_name::<Config>(), prefix = env_prefix, "loading config from env");
            figment = figment.merge(Env::prefixed(&env_prefix));
        }
        figment = extras(figment);
        let result : Config = figment
            .extract()
            .unwrap();
        tracing::debug!(config = std::any::type_name::<Config>(), "config loaded: \n{:#?}", result);
        result
    }

    pub fn load<'de, Config>(&self) -> Config
        where
            Config : Default + std::fmt::Debug
                + serde::ser::Serialize
                + serde::de::Deserialize<'de>,
    {
        self.load_with_extras(|figment| { figment })
    }

    pub fn load_with_args<'de, Config, Args>(&self, args: &Args) -> Config
        where
            Config : Default + std::fmt::Debug
                + serde::ser::Serialize
                + serde::de::Deserialize<'de>,
            Args : Default + serde::ser::Serialize,
    {
        self.load_with_extras(|figment| {
            figment.merge(Serialized::defaults(args))
        })
    }
}