use best_skn_message::message::error;
use std::{
  io::Error,
  process::{Child, Command},
};

/// Executes a command in the terminal
///
/// ## Params:
///
/// - **_command_** : The name of the command as String Slice (&str)
/// - **_args_** : The arguments in the command as reference of String Slice (&str) Array
///
/// ## Since:
///
/// v1.0.0
///
/// ## Usage:
///
/// - No need to handle the error
/// - If any error occurs the command will not execute and will print an error message
///
/// ## Example:
///
/// ```rust
/// use best_skn_utils::execution::execute_command;
///
/// execute_command("cargo", &["build"]);
/// ```
pub fn execute_command(command: &str, args: &[&str]) {
  let result: Result<Child, Error> = Command::new(command).args(args).spawn();

  match result {
    | Ok(_) => (),
    | Err(e) => println!("Error: {}", error(e.to_string().as_str())),
  }
}

/// Opens a new `Gnome Terminal` & executes a command in it
///
/// ## Params:
///
/// - **_arg_** Command as a String Slice (&str)
///
/// ## Since:
///
/// v1.0.0
///
/// ## Usage:
///
/// - if there are several commands, then each command must be separated by `;`
/// - for multiple commands, you can add `read -n 1 KEY` at the end to keep the terminal alive
///
/// ## Example:
///
/// ```rust
/// use best_skn_utils::execution::gnome_execute_command;
///
/// gnome_execute_command("printf 'Hello SKN! \n'");
///
/// gnome_execute_command("printf 'Hello SKN! \n'; printf 'Build was successful! ✅ \n'; read -n 1
/// KEY");
/// ```
pub fn gnome_execute_command(arg: &str) {
  execute_command("gnome-terminal", &["--", "bash", "-c", arg]);
}
