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
//!
//! ## Minimum Requirements:
//!
//! - 💀 Rust Version: **1.78.0**
//! - 💀 Crates:
//!     - serde: **1.0.203**

/// This module provides functionality for building an external configuration
///
/// ## `env` module
///
/// - It has 1 function that returns a deserialized value read from an external config file
/// `rustenv.toml`
///
/// ## Version:
///
/// 1.0.0
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
/// ## Version:
///
/// 1.0.0
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
/// ## Version:
///
/// 1.0.0
///
/// ## Since:
///
/// 2024-06-07
pub mod stdio;
