use config::{Config, ConfigError, File};
use serde::de::Deserialize;

/// Builds an external config file to get data from outside the program
///
/// ## Generics:
///
/// - **_T_** : The type of data structure in the external config file
///
/// ## Returns:
///
/// `Result<T, ConfigError>`: If successful then the generic data type, else `ConfigError`
///
/// ## Since:
///
/// v1.0.0
///
/// ## Usage:
///
/// You must use `serde` to deserialize structs
///
/// ## Example:
///
/// ### Suppose the `rustenv.toml` file has data like this:
///
/// ```toml
/// [author]
/// name = "SKN"
/// email = "skn437physx@gmail.com"
/// ```
///
/// ### Then to read the data, do the following:
///
/// ```rust
/// use best_skn_utils::env::init_config;
/// use serde::Deserialize;
///
/// #[derive(Debug, Deserialize)]
/// struct Author {
///   name: String,
///   email: String,
/// }
///
/// impl Author {
///   fn new() -> Self {
///     Self {
///       name: String::new(),
///       email: String::new(),
///     }
///   }
/// }
///
/// #[derive(Debug, Deserialize)]
/// struct ConfigData {
///   author: Author,
/// }
///
/// impl ConfigData {
///   fn new() -> Self {
///     let config = init_config::<Self>();
///
///     match config {
///       | Ok(value) => value,
///       | Err(e) => {
///         println!("Error: {}", e);
///         Self {
///           author: Author::new(),
///         }
///       }
///     }
///   }
/// }
///
/// let config_data: ConfigData = ConfigData::new();
///
/// println!("Name: {}, Email: {}", config_data.author.name, config_data.author.email);
/// ```
pub fn init_config<'a, T: Deserialize<'a>>() -> Result<T, ConfigError> {
  let config: T = Config::builder()
    .add_source(File::with_name("./rustenv"))
    .build()?
    .try_deserialize()?;

  Ok(config)
}
