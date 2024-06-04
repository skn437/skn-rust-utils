use std::io::{stdin, Error};

pub fn read_line(prompt: &str) -> Result<String, Error> {
  println!("{}", prompt);

  let mut input: String = String::new();

  let _result: usize = stdin().read_line(&mut input)?;

  Ok(input)
}
