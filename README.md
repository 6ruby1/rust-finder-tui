# rust-finder-tui

[![GitHub Actions Workflow Status](https://custom-icon-badges.demolab.com/github/actions/workflow/status/6ruby1/rust-finder-tui/build.yml?style=for-the-badge&logo=play&logoSize=auto&labelColor=%23323449&color=%23a48cf2)](https://github.com/6ruby1/rust-finder-tui/actions/workflows/build.yml)
[![GitHub License](https://custom-icon-badges.demolab.com/github/license/6ruby1/rust-finder-tui?style=for-the-badge&logo=law&logoSize=auto&labelColor=%23323449&color=%23f1fc79)](./LICENSE)
[![Open Issues](https://custom-icon-badges.demolab.com/github/issues-raw/6ruby1/rust-finder-tui?style=for-the-badge&logo=issue-opened&logoColor=%233fb950&labelColor=%23323449&color=%23f7c67f)](https://github.com/6ruby1/rust-finder-tui/issues)
[![Closed Issues](https://custom-icon-badges.demolab.com/github/issues-closed-raw/6ruby1/rust-finder-tui?style=for-the-badge&logo=issue-closed&logoColor=%23ab7df8&labelColor=%23323449&color=%237081d0)](https://github.com/6ruby1/rust-finder-tui/issues?q=is%3Aissue%20state%3Aclosed)

---

## About This Project

**rust-finder-tui** is a terminal-based file finder built in Rust as a portfolio project, focused on applying newly learned language concepts with an emphasis on project planning, maintainable architecture, and documentation.

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
