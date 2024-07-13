use std::env;

/// Provides the arguments passed in command line interface
///
/// ## Returns:
///
/// `Vec<String>`: A vector of String
///
/// ## Since:
///
/// v1.2.0
///
/// ## Example:
///
/// ```rust
/// use best_skn_utils::args::get_args;
///
/// let args: Vec<String> = get_args();
///
/// println!("{:?}", args);
/// ```
pub fn get_args() -> Vec<String> {
  let args: Vec<String> = env::args().collect();
  return args;
}
