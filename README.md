# SKN Rust Utility Library

<img width="150px" src="https://firebasestorage.googleapis.com/v0/b/skn-ultimate-project-la437.appspot.com/o/GitHub%20Library%2F07-Rust-SRU.svg?alt=media&token=7f4940f6-d18f-46fb-88ac-e21f6bf27d89" alt="rust" />

> Rust

[![Crates IO](https://img.shields.io/crates/v/best_skn_utils)](https://crates.io/crates/best_skn_utils) [![MIT License](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/license/mit)

&nbsp;

## **_RustDocs:_**

### Read the Rustdoc for the main Modules

- [env](https://docs.rs/best_skn_utils/1.1.0/best_skn_utils/env/index.html)
- [execution](https://docs.rs/best_skn_utils/1.1.0/best_skn_utils/execution/index.html)
- [stdio](https://docs.rs/best_skn_utils/1.1.0/best_skn_utils/stdio/index.html)
- [random](https://docs.rs/best_skn_utils/1.1.0/best_skn_utils/random/index.html)

&nbsp;

## **_Introduction:_**

### This is a simple Rust Library for some essential utility functions

### I made this library so that I can use it in all of my rust projects without writing the same codes over and over again

### The main Modules of this library are `env`, `execution` & `stdio`

&nbsp;

## **_Details:_**

### **`env` Module:**

- It has 1 function which builds external config file functionality to get data from outside the program
- `init_config` function takes generic
- The file name is `rustenv.toml` from which you can get data after invoking the function
- The config file `rustenv.toml` must be placed in the root directory where `Cargo.toml` file is
- To read values from the file properly you need [serde](https://crates.io/crates/serde) crate
- See `Usage` section to get an example of how to use it

### **`execution` Module**

- It has 2 functions which help to execute commands in the terminal
- `execute_command` function can run any command in the terminal
  - it takes the command name as first argument
  - it takes a reference of string slice (&str) array as command arguments
- `gnome_execute_command` function can open a new gnome terminal and executes commands in it
  - it takes only one argument which is a string slice (&str)
  - if multiple command needs to be executed then the commands must be separated by `;`
- See `Usage` section to get an example of how to use it

### **`stdio` Module**

- It has 1 function which reads user input and returns it
- `read_line` function returns String if it successfully reads the input else returns Error
- See `Usage` section to get an example of how to use it

### **`random` Module**

- It has 1 function that generates random number from a given range
- `gen_random_number` function takes two parameters to set a range. One is `low` and the other one is `high`
- The parameters can either be `Integer` or `Float`
- parameter `low` and `high` must be of same type i.e. you cannot set a range of say from 1 to 10.1
- The second parameter `high` is inclusive i.e. a range of 1 and 10 will mean the range includes from 1 to 10
- See `Usage` section to get an example of how to use it

&nbsp;

## **_Use Case:_**

- Rust

&nbsp;

## **_Requirements:_**

- 💀 Minimum Rust Version: `1.78.0`
- 💀 Crates:
  - [serde](https://crates.io/crates/serde): `1.0.203`

&nbsp;

## **_Usage:_**

### To install the package, type the following in console

> ```zsh
> cargo add best_skn_utils
> ```

### Inside your Rust Code, import the package like this

> ```rust
> use best_skn_utils::{env, execution, stdio, random};
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

#### (4) For `random` module, you can use like this

> ```rust
> use best_skn_utils::random::gen_random_number;
>
> let num1: i32 = gen_random_number(1, 10);
> let num2: f64 = gen_random_number(1.5, 7.5);
> ```

&nbsp;

## **_Dedicated To:_**

- 👩‍🎨`Prodipta Das Logno` & 🧛‍♀️`Atoshi Sarker Prithula`: The two most special ladies of my life. I
  can't thank them
  enough for always treasuring me a lot. I am lucky that I met with these two amazing ladies. They
  have two special
  places in my heart and no other girl can ever replace them.
- 💯`My Parents`: The greatest treasures of my life ever.
