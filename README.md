<p align="center">
  <h1 align="center">
    rge
  </h1>

  <p align="center">
    <a href="https://github.com/claymcleod/rge/actions/workflows/CI.yml" target="_blank">
      <img alt="CI: Status" src="https://github.com/claymcleod/rge/actions/workflows/CI.yml/badge.svg" />
    </a>
    <a href="https://crates.io/crates/rge" target="_blank">
      <img alt="crates.io version" src="https://img.shields.io/crates/v/rge">
    </a>
    <img alt="crates.io downloads" src="https://img.shields.io/crates/d/rge">
    <a href="https://github.com/claymcleod/rge/blob/master/LICENSE-APACHE.md" target="_blank">
      <img alt="License: MIT" src="https://img.shields.io/badge/license-Apache 2.0-blue.svg" />
    </a>
    <a href="https://github.com/claymcleod/rge/blob/master/LICENSE-MIT.md" target="_blank">
      <img alt="License: MIT" src="https://img.shields.io/badge/license-MIT-blue.svg" />
    </a>
  </p>


  <p align="center">
    Reference genome explorer command-line tool.
    <br />
    <br />
    <a href="https://github.com/claymcleod/rge/issues/new?assignees=&labels=&template=feature_request.md&title=Descriptive%20Title&labels=enhancement">Request Feature</a>
    ·
    <a href="https://github.com/claymcleod/rge/issues/new?assignees=&labels=&template=bug_report.md&title=Descriptive%20Title&labels=bug">Report Bug</a>
    ·
    ⭐ Consider starring the repo! ⭐
    <br />
  </p>
</p>


## 🎨 Features

* **Pseudoautosomal Region detection.** Detects whether the pseudoautosomal region of chromosomes X and Y is present within the reference genome and, if so, what the coordinates of the PARs are.

## 📚 Getting Started

### Installation

To install the latest released version, you can simply use `cargo`.

```bash
cargo install rge
```

To install the latest version on `main`, you can use the following command.

```bash
cargo install --locked --git https://github.com/claymcleod/rge.git
```

## 🖥️ Development

To bootstrap a development environment, please use the following commands.

```bash
# Clone the repository
git clone git@github.com:claymcleod/rge.git
cd rge

# Run the command line tool using cargo.
cargo run -- -h
```

## 🚧️ Tests

```bash
# Run the project's tests.
cargo test

# Ensure the project doesn't have any linting warnirge.
cargo clippy

# Ensure the project passes `cargo fmt`.
cargo fmt --check
```

## Minimum Supported Rust Version (MSRV)

The minimum supported Rust version for this project is 1.64.0.

## 🤝 Contributing

Contributions, issues and feature requests are welcome! Feel free to check
[issues page](https://github.com/claymcleod/rge/issues).

## 📝 License

Copyright © 2021-Present Clay McLeod. This project is [MIT][license-mit] or [Apache 2.0][license-apache] licensed at your discretion.

[contributing-md]: https://github.com/claymcleod/rge/blob/master/CONTRIBUTING.md
[license-mit]: https://github.com/claymcleod/rge/blob/master/LICENSE-MIT
[license-apache]: https://github.com/claymcleod/rge/blob/master/LICENSE-APACHE

