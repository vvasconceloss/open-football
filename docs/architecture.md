# Architecture Overview

## 1. Design Principles

OpenFootball follows a **modular, layered architecture** guided by **Domain-Driven Design (DDD)** principles. The objective is to maintain **clarity**, **testability**, and **independence** between layers.

The project is structured to allow future integration with GUI (Tauri + React) and data persistence (SQLite) without breaking core domain logic.

---

## 2. Layered Structure

```
openfootball/
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

## 3. Domain Layer

* Defines **Entities**, **Value Objects**, and **Aggregates**.
* Contains pure logic and must remain independent of frameworks or I/O operations.
* Example entities: `Club`, `Player`, `Contract`, `Fixture`, `MatchResult`.
* Example value objects: `PlayerAttributes`, `Position`, `Overall`.

**Key Rule:** Domain code must never depend on external crates beyond core Rust or math utilities.

---

## 4. Application Layer

* Coordinates use cases by combining domain logic and infrastructure.
* Exposes high-level functions for match simulation, player transfers, and club management.
* Does not handle UI or persistence directly — delegates to infrastructure adapters.

**Example:**

```rust
pub fn simulate_match(home: &Club, away: &Club) -> MatchResult {
    let home_strength = home.calc_strength();
    let away_strength = away.calc_strength();
    ResultGenerator::generate(home_strength, away_strength)
}
```

---

## 5. Infrastructure Layer

* Implements repositories, adapters, and data persistence.
* Includes SQLite integration (via `rusqlite` or `sea-orm` in the future).
* Handles serialization (`serde`) and configuration management.

**Example modules:**

* `repositories/club_repository.rs`
* `repositories/player_repository.rs`

---

## 6. Presentation Layer

* Handles communication with the user interface.
* For the MVP: may be a CLI or minimal web view.
* For later phases: integrates with **Tauri + React (TypeScript)** frontend.

**Frontend stack:**

* React + TypeScript for UI logic.
* Tauri for native packaging.
* SQLite for embedded persistence.

---

## 7. Future Extensions

* **AI Engine:** Player progression, scouting, tactical adaptation.
* **Modding API:** Allow users to create data packs, leagues, and rulesets.

---

## 8. Summary

The architecture balances **performance** (Rust), **portability** (Tauri), and **maintainability** (DDD).
It supports iterative feature expansion while maintaining strict separation of concerns and long-term scalability.
