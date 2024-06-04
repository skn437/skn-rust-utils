use best_skn_message::message::error;
use std::{
  io::Error,
  process::{Child, Command},
};

pub fn execute_command(command: &str, args: &[&str]) {
  let result: Result<Child, Error> = Command::new(command).args(args).spawn();

  match result {
    | Ok(_) => (),
    | Err(e) => println!("Error: {}", error(e.to_string().as_str())),
  }
}

pub fn gnome_execute_command(arg: &str) {
  execute_command("gnome-terminal", &["--", "bash", "-c", arg]);
}
