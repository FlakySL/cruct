//! `cruct` - Ease the creation of configuration files in Rust
//!
//! A procedural macro for loading configuration files into Rust structs
//! with compile-time validation and type safety.
//!
//! # Example
//! ```
//! use cruct::cruct;
//!
//! #[cruct(load_config(path = "tests/fixtures/test_config.toml"))]
//! struct Config {
//!     #[field(default = 8080)]
//!     http_port: u16,
//!     database_url: String,
//! }
//!
//! let cfg = Config::loader()
//!     .with_config()
//!     .load()
//!     .unwrap();
//! ```

pub use cruct_proc::cruct;
pub use cruct_shared::*;
