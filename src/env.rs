use config::{Config, ConfigError, File};
use serde::de::Deserialize;

fn config_builder() -> Result<Config, ConfigError> {
  let config_build: Config = Config::builder()
    .add_source(File::with_name("./rustenv"))
    .build()?;

  Ok(config_build)
}

pub fn init_config<'a, T: Deserialize<'a>>() -> Result<T, ConfigError> {
  let config = config_builder()?.try_deserialize()?;

  Ok(config)
}
