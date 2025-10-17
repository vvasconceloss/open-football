# CONTRIBUTING

Thank you for taking the time to read this and for showing interest in contributing to **Open Football**.

This project exists to bring a **lightweight**, **open-source**, and **community-driven** football management simulator to life. Inspired by the depth of *Football Manager* but built for **performance**, **transparency**, and **freedom**, Open Football is a project for developers and football fans who believe in open simulation systems.

Whether you're a programmer, designer, writer, or just passionate about football and open-source projects, your help is valuable.

---

## How You Can Contribute

There are many ways to contribute:

* **Give a star on GitHub**
* **Share the project** on Reddit, Twitter, YouTube, or anywhere else
* **Refer Open Football** in your own projects
* **Play the game** and give feedback
* **Report issues** or **suggest features**
* **Contribute code, documentation, or design**

---

## Reporting Issues and Proposing Features

* **Bug Reports:**

  * Use the Issue template for bug reports
  * Include clear steps to reproduce, expected vs. actual behavior, and any relevant logs or screenshots

* **Feature Requests:**

  * Clearly describe your idea and why it improves the project
  * Reference examples from other games or simulations if helpful

Before opening a new issue, **check if a similar one already exists**.

---

## Submitting Code

**Open Football** is licensed under **MIT**. Any code submitted must:

* Be **original** or **MIT-compatible**
* Not include copyrighted or patented dependencies
* Be submitted under the same license terms

All submissions go through a **Fork & Pull** workflow.

### Fork & Pull Workflow

1. Fork the repository
2. Clone your fork locally
3. Create a feature branch from `develop`
4. Commit your changes (following commit conventions)
5. Push to your fork
6. Open a Pull Request targeting the `develop` branch

PRs will be reviewed and may require revisions before merging.

If your PR adds a new feature, please open an Issue describing it before submitting.

---

## Commit Message Guidelines

We follow the **[Conventional Commits](https://www.conventionalcommits.org/)** specification.

Format: `type(scope): subject`

**Types:**

* `feat`: new feature
* `fix`: bug fix
* `docs`: documentation
* `refactor`: refactoring
* `style`: code style only
* `test`: adding or modifying tests
* `chore`: build tools or maintenance

**Examples:**

```
feat(player): add player position weight calculation
fix(engine): correct probability distribution in match results
docs(README): add setup instructions
refactor(domain): simplify team lineup generator
```

---

## Code Style and Practices

* Code is written in **Rust**
* Follow **Rustfmt** conventions; auto-format before committing
* Prefer **explicit types** for clarity
* Use **modules** and **struct encapsulation** to express domain boundaries
* Follow **domain-driven design** principles where possible
* Keep **functions pure** and testable
* Write **unit tests** for all new logic

---

## Testing

Testing ensures stability and accuracy of simulation results.

Use **cargo test** to run all tests.

**Guidelines:**

* All new features must include tests
* Strive for 80%+ coverage on new code
* Prefer behavior-based tests, not internal implementation
* Keep tests small and deterministic

---

## Branching Strategy

We follow a simplified **GitFlow** model:

* `main` – Stable, production-ready
* `develop` – Active development
* `feature/*` – New features
* `bugfix/*` – Fixes for develop
* `hotfix/*` – Urgent production fixes

**Branch naming conventions:**

```
feature/player-stats
bugfix/match-simulation-freeze
```

---

## Development Environment Setup

### Prerequisites

* Rust (latest stable)
* Cargo
* Git
* Node LTS

### Setup

```bash
git clone https://github.com/vvasconceloss/open-football.git
cd open-football
cargo build
```

To run tests:

```bash
cargo test
```

To run the application:

```bash
cd src/tauri
pnpm install
pnpm run tauri dev
```

---

## Project Structure

```
Open Football/
├── src/
│   ├── applicaion/                  # Services and Use-cases
│   ├── domain/                      # Core domain logic (entities, value objects, etc.)
│   ├── engine/                      # Simulation engine logic
│   ├── infrastructure/              # Infrastructure layer (persistence, adapters)
│   ├── tauri/                       # UI or CLI interface
├── docs/                            # Documentation and design notes
└── Cargo.toml                       # Rust project configuration
```

---

## Code of Conduct

All contributors must follow our **[Code of Conduct](CODE_OF_CONDUCT.md)**.

**Principles:**

* Be respectful and inclusive
* Focus on constructive feedback
* Use clear, inclusive language
* No harassment or discrimination of any kind

To report violations, contact **[vvasconcelos.dev@gmail.com](mailto:vvasconcelos.dev@gmail.com)**.

---

## Documentation and Tutorials

If you find unclear documentation, propose improvements via PR or Issue. Tutorials, diagrams, and code examples are encouraged contributions.

---

## Translations

Open Football will support internationalization in future releases. Translation contributions will be handled via external tools once available.

---

## Thank You

Every contribution makes Open Football better. Whether you fix a typo, write a new system, or share feedback — you are part of this open football simulation movement.
