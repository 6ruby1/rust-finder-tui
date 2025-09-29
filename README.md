# rust-finder-tui

[![GitHub Actions Workflow Status](https://custom-icon-badges.demolab.com/github/actions/workflow/status/6ruby1/rust-finder-tui/build.yml?style=for-the-badge&logo=play&logoSize=auto&labelColor=%23323449&color=%23a48cf2)](https://github.com/6ruby1/rust-finder-tui/actions/workflows/build.yml)
![GitHub Tag](https://custom-icon-badges.demolab.com/github/v/tag/6ruby1/rust-finder-tui?include_prereleases&sort=semver&style=for-the-badge&logo=tag&logoSize=auto&label=Version&labelColor=%23323449&color=%23f16c75)
[![GitHub License](https://custom-icon-badges.demolab.com/github/license/6ruby1/rust-finder-tui?style=for-the-badge&logo=law&logoSize=auto&labelColor=%23323449&color=%23f1fc79)](./LICENSE)
[![Open Issues](https://custom-icon-badges.demolab.com/github/issues-raw/6ruby1/rust-finder-tui?style=for-the-badge&logo=issue-opened&logoColor=%233fb950&labelColor=%23323449&color=%23f7c67f)](https://github.com/6ruby1/rust-finder-tui/issues)
[![Closed Issues](https://custom-icon-badges.demolab.com/github/issues-closed-raw/6ruby1/rust-finder-tui?style=for-the-badge&logo=issue-closed&logoColor=%23ab7df8&labelColor=%23323449&color=%237081d0)](https://github.com/6ruby1/rust-finder-tui/issues?q=is%3Aissue%20state%3Aclosed)

---

## About This Project

**rust-finder-tui** is a terminal-based file finder built with Rust and the [`ratatui`](https://github.com/ratatui/ratatui) crate as a portfolio project. The project is focused on applying newly learned language concepts with an emphasis on project planning, maintainable architecture, and documentation.

## Learning Goals & Portfolio Value

This project was created to:
- **Learn Rust:** Practice modern Rust concepts and best practices.
- **Learn TUI Development:** Utilise modern libraries to build a functional interactive terminal user interface.
- **Planning and Process:** Demonstrate project planning and execution as a portfolio piece.

## Project Roadmap

The progression of **rust-finder-tui** is organised into [Issue Milestones](https://github.com/6ruby1/rust-finder-tui/milestones), each representing a major phase of development. Major features are tracked using [Epics](https://github.com/6ruby1/rust-finder-tui/issues?q=label%3Aepic), which remain independent from milestones.

Check out the ![GitHub Project](https://github.com/users/6ruby1/projects/1) for a detailed list of tasks!

### Milestone Completion

- [x] ![Milestone: Initial Setup - v0.1.0](https://github.com/6ruby1/rust-finder-tui/milestone/1)

  - Set up the initial project structure with essential tooling and configurations.
    
- [ ] ![Milestone: Minimum Viable Product - v0.2.0](https://github.com/6ruby1/rust-finder-tui/milestone/2)
      
  - Implement the core features required for the project to be usable.
    
- [ ] ![Milestone: User Interface Enhancements - v0.3.0](https://github.com/6ruby1/rust-finder-tui/milestone/3)
      
  - Improve the user interface with updated layouts, keybindings and features.
    
- [ ] ![Milestone: Advanced Search Features - v0.4.0](https://github.com/6ruby1/rust-finder-tui/milestone/4)
      
  - Extend the search functionality of the minimum viable product adding features such as:
    - Fuzzy search
    - File content matching
      
- [ ] ![Milestone: Configuration Capabilities - v0.5.0](https://github.com/6ruby1/rust-finder-tui/milestone/5)
      
  - Implement support for custom user configurations to modify the UI and keybindings.
    
- [ ] ![Milestone: Refactoring and Code Structure - v0.6.0](https://github.com/6ruby1/rust-finder-tui/milestone/6)
      
  - Restructure the codebase as needed to ensure maintainability and adherence to the project's architecture.
    
- [ ] ![Milestone: Documentation Refinement - v0.7.0](https://github.com/6ruby1/rust-finder-tui/milestone/7)
      
  - Ensure the project's documentation provides thorough coverage of architecture, system modules, and user instructions.
    
- [ ] ![Milestone: First Major Release v1.0.0](https://github.com/6ruby1/rust-finder-tui/milestone/8)
      
  - Confirm stability of system features and prepare for the first release.
     

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
