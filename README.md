# SKN Rust Utility Library

<img width="150px" src="https://firebasestorage.googleapis.com/v0/b/skn-ultimate-project-la437.appspot.com/o/GitHub%20Library%2F07-Rust-SRU.svg?alt=media&token=7f4940f6-d18f-46fb-88ac-e21f6bf27d89" alt="rust" />

> Rust

[![Crates IO](https://img.shields.io/crates/v/best_skn_utils)](https://crates.io/crates/best_skn_utils) [![MIT License](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/license/mit)

&nbsp;

## **_RustDocs:_**

### Read the Rustdoc for the main Modules

- [env](https://docs.rs/best_skn_utils/1.0.0/best_skn_utils/env/index.html)
- [execution](https://docs.rs/best_skn_utils/1.0.0/best_skn_utils/execution/index.html)
- [stdio](https://docs.rs/best_skn_utils/1.0.0/best_skn_utils/stdio/index.html)

&nbsp;

## **_Introduction:_**

### This is a simple Rust Library for some essential utility functionalities

### I made this library so that I can use it in all of my rust projects without writing the same codes over and over again

### The main Modules of this library are `env`, `execution` & `stdio`

&nbsp;

## **_Details:_**

### **`Message` Module:**

- It has 3 functions which return colored Strings as output

  - error (takes 1 argument & returns a red colored String with cross mark)
  - success (takes 1 argument & returns a green colored String with tick mark)
  - info (takes 1 argument & returns a blue colored String with book info mark)

- It has 3 functions which outputs formatted static String notifications

  - action_failure (takes 1 argument as 'action name' & outputs an action failure message)
  - action_complete (takes 1 argument as 'action name' & outputs an action complete message)
  - action_notify (takes 2 arguments as 'action name', 'notification info' & outputs an action info message)

- The String returned by each function, doesn't contain new line character i.e. '\n'

&nbsp;

## **_Use Case:_**

- Rust

&nbsp;

## **_Requirements:_**

- 💀 Minimum Rust Version: `1.78.0`
- 💀 Crates:
  - serde: `1.0.203`

&nbsp;

## **_Usage:_**

### To install the package, type the following in console

> ```zsh
> cargo add best_skn_utils
> ```

### Inside your Rust Code, import the package like this

> ```rust
> use best_skn_utils::{env, execution, stdio};
> ```

### Use the modules like the following (Just an example)

#### (1) For `env` module, you can use like this

##### **(a) Suppose the `rustenv.toml` file contains the data like this**

> ```toml
> [author]
> name = "SKN"
> email = "skn437physx@gmail.com"
> ```

##### **(b) Then the usage of the module can be like this**

> ```rust
> use best_skn_utils::env::init_config;
> use serde::Deserialize;
>
> #[derive(Debug, Deserialize)]
> struct Author {
>   name: String,
>   email: String,
> }
>
> impl Author {
>   fn new() -> Self {
>     Self {
>       name: String::new(),
>       email: String::new(),
>     }
>   }
> }
>
> #[derive(Debug, Deserialize)]
> struct ConfigData {
>   author: Author,
> }
>
> impl ConfigData {
>   fn new() -> Self {
>     let config = init_config::<Self>();
>
>     match config {
>       | Ok(value) => value,
>       | Err(e) => {
>         println!("Error: {}", e);
>         Self {
>           author: Author::new(),
>         }
>       }
>     }
>   }
> }
>
> let config_data: ConfigData = ConfigData::new();
>
> println!("Name: {}, Email: {}", config_data.author.name, config_data.author.email);
> ```

#### (2) For `execution` module, you can use like this

> ```rust
> use best_skn_utils::execution::{execute_command, gnome_execute_command};
>
> execute_command("cargo", &["doc", "--open"]);
>
> gnome_execute_command("printf 'Hello SKN! \n'; printf 'Build was successful! ✅ \n'; read -n 1 KEY");
> ```

#### (3) For `stdio` module, you can use like this

> ```rust
> use best_skn_utils::stdio::read_line;
> use std::io::Error;
>
> let input: Result<String, Error> = read_line("Write your name:");
>
> match input {
>   | Ok(value) => println!("Your name: {}", value),
>   | Err(e) => println!("Error: {}", e),
> }
> ```

&nbsp;

## **_Dedicated To:_**

- 👩‍🎨`Prodipta Das Logno` & 🧛‍♀️`Atoshi Sarker Prithula`: The two most special ladies of my life. I can't thank them
  enough for always treasuring me a lot. I am lucky that I met with these two amazing ladies. They have two special
  places in my heart and no other girl can ever replace them.
- 💯`My Father & Mother`: The greatest treasures of my life ever.
