## Development Setup

### 1. Prerequisites

Before you begin, ensure you have the following installed:

* **Rust** ≥ 1.80 (managed via `rustup`)
* **Node.js** ≥ 20
* **pnpm** ≥ 9
* **SQLite3**
* **Tauri dependencies** (Linux example):

  ```bash
  sudo apt install libwebkit2gtk-4.1-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev patchelf
  ```

### 2. Clone the Repository

```bash
git clone https://github.com/openfootball/openfootball.git
cd openfootball
```

### 3. Install Dependencies

```bash
cd src/tauri
pnpm install
```

### 4. Set Up Pre-commit Hooks

Install **pre-commit** globally or run it through `uv`:

```bash
uv run pre-commit install
```

Alternatively:

```bash
pip install pre-commit
pre-commit install
```

### 5. Code Quality Checks

Run all formatters and linters before committing:

```bash
cargo fmt --all
cargo clippy --all-targets --all-features
```

### 6. Run the Project

To start the application in development mode:

```bash
cd src/tauri/
pnpm tauri dev
```

### 7. Troubleshooting

If you encounter issues:

* Run `cargo clean` and `pnpm install` again.
* Verify that all Rust and Node versions meet the required versions.
* Ensure Tauri dependencies are installed correctly for your OS.

---

**Result:**
A standardized, reproducible local development environment. Contributors can clone, set up, and run the project with minimal friction.
