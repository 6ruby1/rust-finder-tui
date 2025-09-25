# rust-finder-tui

A Rust-based terminal user interface (TUI) application.

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (latest stable recommended)
- [cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) (comes with Rust)

### Running the Project

Clone the repository:
```sh
git clone https://github.com/6ruby1/rust-finder-tui.git
cd rust-finder-tui
```

Build and run:
```sh
cargo run
```

## Development Instructions

To contribute or work on this project:

### 1. Install Rust development tools

Make sure you have Rust and cargo installed:
```sh
rustup update
```

### 2. Set up development environment

Install development dependencies:

#### pre-commit

[pre-commit](https://pre-commit.com/) to manage and run Git hooks.

Install `pre-commit`:
```sh
pip install pre-commit
```

#### gitlint

[gitlint](https://jorisroovers.com/gitlint/) is used by `pre-commit` to ensure commit messages follow conventions.

Install `gitlint`:
```sh
pip install gitlint
```

#### gitleaks

[gitleaks](https://github.com/gitleaks/gitleaks) is used by `pre-commit` to detects secrets included in a commit.

Install `gitleaks`:
```sh
brew install gitleaks
```
Or download from [releases](https://github.com/gitleaks/gitleaks/releases).

### 3. Install Git hooks using pre-commit

Before committing, manually run checks:
```sh
pre-commit install
pre-commit install --hook-type commit-msg
```

### 4. Running tests

Run Rust tests:
```sh
cargo test
```
