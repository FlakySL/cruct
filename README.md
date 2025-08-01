![cruct-readme](https://raw.githubusercontent.com/FlakySL/cruct/refs/heads/main/.github/cruct_banner.png)

[![Crates.io](https://badges.ws/crates/v/cruct)](https://crates.io/crates/cruct)
[![Docs.rs](https://badges.ws/crates/docs/cruct)](https://docs.rs/cruct)
[![License](https://badges.ws/crates/l/cruct)](https://docs.rs/cruct)
[![Downloads](https://badges.ws/crates/dt/cruct)](https://crates.io/crates/cruct)
[![Codecov](https://img.shields.io/codecov/c/github/FlakySL/cruct)](https://app.codecov.io/gh/FlakySL/cruct)
![tests](https://github.com/FlakySL/cruct/actions/workflows/overall-coverage.yml/badge.svg)
[![Discord](https://badges.ws/discord/online/1344769456731197450)](https://discord.gg/AJWFyps23a)

A procedural macro for loading configuration files into Rust structs with compile‑time validation and type safety.

## Table of Contents 📖
- [Features](#features-)
- [Installation](#installation-)
- [Basic Usage](#basic-usage-)
- [Use‑Case: Environment‑Variable Override](#usecase-environmentvariable-override-)
- [Advanced](#advanced-)
- [License](#license-)

## Features 👀

- **Multi‑format support**: TOML, YAML, JSON (via Cargo feature flags)
- **Merge & override**: CLI args, environment variables, config files, defaults
- **Compile‑time safety**: Missing or mismatched fields become compile or runtime errors
- **Nested structures**: Automatically derive for nested custom types

## Installation 📦

Add to your `Cargo.toml`:

```toml
[dependencies]
cruct = "1.0.0"
````

Enable only the formats you need:

```toml
[dependencies.cruct]
version = "1.0.0"
default-features = false
features = ["toml", "json"]  # only TOML and JSON support
```

## Basic Usage 🔍

Annotate your config‐struct with `#[cruct]`, pointing at one or more sources:

```rust
use cruct::cruct;

#[cruct(load_config(path = "config/settings.toml"))]
struct AppConfig {
    #[field(default = 8080)]
    http_port: u16,
    database_url: String,
}

fn main() -> Result<(), cruct::ParserError> {
    let cfg = AppConfig::loader()
        .with_config()
        .load()?;
    println!("Listening on port {}", cfg.http_port);
    Ok(())
}
```

## Use‑Case: Environment‑Variable Override 💡

Often you want a default in your file, but allow ops to override via env vars. For example, given `tests/fixtures/test_config.toml`:

```toml
http_port = 8080
```

You can override `http_port` at runtime:

```rust
use cruct::cruct;

#[cruct(load_config(path = "tests/fixtures/test_config.toml"))]
#[derive(Debug, PartialEq)]
struct TestEnv {
    #[field(env_override = "TEST_HTTP_PORT")]
    http_port: u16,
}

fn main() {
    // Simulate setting the env var:
    unsafe { std::env::set_var("TEST_HTTP_PORT", "9999"); }

    let config = TestEnv::loader()
        .with_config()
        .load()
        .unwrap();

    assert_eq!(config.http_port, 9999);
    println!("Overridden port: {}", config.http_port);
}
```

This pattern is drawn directly from our end‑to‑end tests.

## Advanced 🥷

* **Multiple files & priority**: Chain `load_config` calls with explicit `priority`
* **Case‑insensitive keys**: Use `#[field(name = "HTTP_PORT", insensitive = true)]`
* **Default values**: Supply literals, expressions, or functions for `default`

See the full [API docs](https://docs.rs/cruct) for details on all options.

## License 📜

This repository is dual licensed, TLDR. If your repository is open source, the library
is free of use, otherwise contact [licensing@flaky.es](mailto:licensing@flaky.es) for a custom license for your
use case.

For more information read the [license](./LICENSE) file.
