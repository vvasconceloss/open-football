# OpenFootball Roadmap

## 1. Overview

This roadmap defines the planned development stages for the OpenFootball project, from its inception to the first public MVP release. It is designed to maintain transparency and guide contributors through the project’s priorities.

---

## 2. Development Phases

### Phase 1 — Foundation (MVP Core)

**Goal:** Establish a solid technical and domain foundation.

**Deliverables:**

* Core domain model (Player, Club, Fixture, MatchResult, Manager, Contract)
* Basic match simulation engine (match outcome generator)
* Database integration (SQLite)
* CLI or minimal UI for match visualization
* Basic project documentation (README, CONTRIBUTING, CODE_OF_CONDUCT)

**Status:** In progress

---

### Phase 2 — Infrastructure Layer

**Goal:** Build the infrastructure to support persistent data and modular expansion.

**Deliverables:**

* Data persistence via SQLite + Rust ORM layer
* Repository pattern implementation
* Configuration management system
* Integration tests for database layer
* Infrastructure-level error handling and logging

**Dependencies:** Phase 1 completed

---

### Phase 3 — Match Engine Expansion

**Goal:** Improve realism and add simulation depth.

**Deliverables:**

* Detailed tactical model (formations, positions, manager strategies)
* Player performance variability per match
* Match events (goals, assists, cards, substitutions)
* Match statistics output
* Unit and integration tests for engine logic

**Dependencies:** Phase 2 completed

---

### Phase 4 — Frontend and UX

**Goal:** Provide a modern and responsive interface.

**Deliverables:**

* Tauri + React (TypeScript) interface
* Tailwind CSS integration for styling
* Real-time match viewer (text-based or minimal animation)
* Data visualization (player stats, match results)
* Local state management (Redux or Zustand)

**Dependencies:** Phase 3 completed

---

### Phase 5 — Community and Extensibility

**Goal:** Open the project for broader community contribution and modding.

**Deliverables:**

* Public plugin/mod API (content extensions)
* Translation (i18n) framework
* Documentation portal (API + modding guides)
* Contribution templates and governance setup
* CI/CD setup for automated builds and testing

**Dependencies:** Phase 4 completed

---

### Phase 6 — Public MVP Release

**Goal:** Deliver the first playable and testable public version.

**Deliverables:**

* MVP build for Windows, Linux, macOS
* Minimal tutorial or onboarding flow
* Feedback collection mechanism
* GitHub release with changelog and installer

**Dependencies:** All prior phases completed

---

## 3. Future Considerations

* Player transfer and scouting systems
* Financial management (club budgets, sponsorships)
* Procedural content generation for leagues
* AI-driven tactical assistant

---

## 4. Versioning and Tracking

* **Issues** and **Pull Requests** should reference roadmap phases
* **Semantic versioning** is used (v0.x for pre-MVP)

---

## 5. License

This roadmap and the OpenFootball project are licensed under the MIT License.
