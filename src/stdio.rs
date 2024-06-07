use std::io::{stdin, Error};

/// Prompts the user for an input and returns the value
///
/// ## Params:
///
/// - **_prompt_** : The prompt message as String Slice (&str)
///
/// ## Returns:
///
/// `Result<String, Error>`: If successful then the input as String, else `Error`
///
/// ## Since:
///
/// v1.0.0
///
/// ## Usage:
///
/// - For prompt message, use `:` at the end to look better
///
/// ## Example:
///
/// ```rust
/// use best_skn_utils::stdio::read_line;
/// use std::io::Error;
///
/// let input: Result<String, Error> = read_line("Write your name:");
///
/// match input {
///   | Ok(value) => println!("Your name is: {}", value),
///   | Err(e) => println!("Error: {}", e),
/// }
/// ```
pub fn read_line(prompt: &str) -> Result<String, Error> {
  println!("{}", prompt);

  let mut input: String = String::new();

  let _result: usize = stdin().read_line(&mut input)?;

  Ok(input)
}
