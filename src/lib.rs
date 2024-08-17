//! # SKN Rust Utility Library
//!
//! ## Introduction:
//!
//! A simple rust library for convenient utility functions
//!
//! ## Features:
//!
//! - This crate's `env` module has 1 function which provides highly convenient way to build an
//! external config
//! - This crate's `execution` module has 2 functions to help execute commands in terminal
//! - This crate's `stdio` module has 1 function to read input value in the terminal
//! - This crate's `random` module has 1 function that generates a random number in a given range
//! - This crate's `args` module has 1 function that returns a collection of arguments passed in command line interface
//!
//! ## Minimum Requirements:
//!
//! - 💀 Rust Version: **1.80.0**
//! - 💀 Crates:
//!     - serde: **1.0.208**
//!     - num: **0.4.3**
//!     - rand: **0.8.5**
//!     - config: **0.14.0**
//!
//! ## Version:
//!
//! 1.2.2

/// This module provides functionality for building an external configuration
///
/// ## `env` module
///
/// - It has 1 function that returns a deserialized value read from an external config file
/// `rustenv.toml`
///
/// ## Since:
///
/// 2024-06-07
pub mod env;

/// This module provides functionality for executing commands in the terminal
///
/// ## `execution` module
///
/// - It has 1 function that executes any kind of command in the terminal
/// - It has 1 function that opens a new gnome-terminal and runs command inside it
///
/// ## Since:
///
/// 2024-06-07
pub mod execution;

/// This module provides functionality for reading user input
///
/// ## `stdio` module
///
/// - It has 1 function that reads user input in the terminal with prompt
///
/// ## Since:
///
/// 2024-06-07
pub mod stdio;

/// This module provides functionality for generating random number
///
/// ## `random` module
///
/// - It has 1 function that generates random number from a given range
///
/// ## Since:
///
/// 2024-06-19
pub mod random;

/// This module provides functionality for getting arguments passed in command line interface
///
/// ## `args` module
///
/// - It has 1 function that returns a collection of arguments in command line interface
///
/// ## Since:
///
/// 2024-07-12
pub mod args;
