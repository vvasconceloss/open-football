# Open-Source Football Management Simulator

## Overview

**Open Football** is an open-source football management simulator inspired by *Football Manager*, built for speed, modularity, and transparency. The project aims to deliver a community-driven, extensible, and lightweight experience for players and developers alike — free to use, modify, and distribute under the **MIT License**.

## Vision

Create a modern, local-first football management experience that focuses on:

* Realistic simulation depth without unnecessary complexity
* Lightweight and efficient Rust-based architecture
* Complete modularity for extensibility and custom engines
* Transparent development with community contributions

## Tech Stack

* **Language:** Rust
* **Architecture:** Domain-Driven Design (DDD)
* **Testing:** Cargo test suite
* **Version Control:** Git + Semantic Commits
* **License:** MIT

## Getting Started

### Prerequisites

* Install [Rust](https://www.rust-lang.org/tools/install)
* Install [Git](https://git-scm.com/)

### Clone the repository

```bash
git https://github.com/vvasconceloss/open-football.git
cd open-football
```

### Run tests

```bash
cargo test --all
```

### Run (Development Environment)
```bash
cargo tauri dev
```

### Build project

```bash
cargo tauri build -- --release
```

## Contributing

We welcome all contributions — from code improvements to documentation, design, or testing.
Please read our [CONTRIBUTING.md](./CONTRIBUTING.md) and [CODE_OF_CONDUCT.md](./CODE_OF_CONDUCT.md) before making a pull request.

## License

This project is licensed under the [MIT License](./LICENSE).

---

**Open Football** — Community-driven, transparent, and open by design.
