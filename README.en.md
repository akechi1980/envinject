# envinject

**[中文说明](README.md)**

`envinject` is a cross-platform single-file tool for managing environment variables per *project* and temporarily injecting them when running project commands.

It runs as a CLI by default; running `envinject gui` opens a local graphical interface to edit configuration.

### Why keep sensitive data separate? (Especially in the AI era)

In the AI era, code is often shared with AI assistants, pasted into chat, or sent for Copilot-style completion and refactors. If API keys, database URLs, or `.env` files live inside the project or get committed, they can leak in a single copy-paste or “share whole repo” action. **Keeping sensitive configuration out of the project directory and in a single local store** (e.g. per-project config in your user config dir via envinject) lets you:

- **Share code with AI safely** — the repo only contains “read from env” patterns, not real secrets.
- **Open-source or share repos safely** — no `.env` or placeholder secrets in the repo; others clone and use their own envinject config to run.
- **Reduce leak surface** — secrets exist in one place on your machine and don’t travel with git, screenshots, or session logs.

envinject stores config in the system config directory and injects it at run time, so sensitive data stays separate from code on your local machine.

---

## Goals (MVP)

- Single executable (CLI + GUI in one binary).
- Default: CLI; `gui` subcommand opens the config editor.
- `run <project> -- <command...>` injects env and runs the command.
- Local config file first; keep the first version simple and stable.

---

## Features

- Store env vars per project (`project -> key/value`).
- Inject a project’s env when running a command.
- GUI: create project, select project, edit key/value pairs, delete keys, save config.
- CLI: `run <project> -- <command...>`, `gui`, `list`, `show <project>`.

---

## Tech stack

- Rust 2021
- `clap` — CLI
- `eframe` / `egui` — lightweight GUI
- `serde` + `serde_json` — config (JSON)
- `directories` — config dir (cross-platform)
- `anyhow` — errors

---

## Project layout

```text
envinject/
├── Cargo.toml
└── src/
    ├── main.rs    # entry + subcommands
    ├── config.rs  # load/save config (JSON)
    ├── exec.rs    # run command with env
    └── gui.rs     # config editor UI
```

---

## Quick start

### 1) Build

Debug:

```bash
cargo build
```

Release (recommended):

```bash
cargo build --release
```

Binary:

- Linux/macOS: `target/release/envinject`
- Windows: `target\release\envinject.exe`

### 2) Configure projects in the GUI

```bash
envinject gui
```

If the binary is not on your PATH:

```bash
./target/release/envinject gui
```

In the GUI: create a project (e.g. `myproj`), add keys (e.g. `API_URL`, `DB_HOST`), then click **Save Config**.

### 3) Run a command with a project’s env

Node:

```bash
envinject run myproj -- npm run dev
```

Python:

```bash
envinject run myproj -- python app.py
```

Rust:

```bash
envinject run myproj -- cargo run
```

---

## GitHub Release builds

The repo uses GitHub Actions to build and publish multi-platform binaries when you push a version tag.

### Trigger a release

```bash
git tag v0.1.0
git push origin v0.1.0
```

After pushing a `v*` tag, the workflow produces a Release with:

| Platform        | File name                    |
|----------------|-----------------------------|
| Linux (x86_64) | `envinject-linux-x86_64`    |
| Windows (x86_64) | `envinject-windows-x86_64.exe` |
| macOS (Apple Silicon) | `envinject-macos-aarch64` |
| macOS (Intel)  | `envinject-macos-x86_64`    |

Download the right asset from the **Releases** page; no Rust install needed.

---

## Install / deploy (Windows & Ubuntu)

Single-binary: build or download, put it on PATH.

### Windows (PowerShell)

**Option A — Build from source**

1. Install Rust (rustup).
2. In the project dir: `cargo build --release`
3. Install for current user:

```powershell
New-Item -ItemType Directory -Force "$env:LOCALAPPDATA\Programs\envinject" | Out-Null
Copy-Item ".\target\release\envinject.exe" "$env:LOCALAPPDATA\Programs\envinject\envinject.exe" -Force
```

4. Add to user PATH:

```powershell
[Environment]::SetEnvironmentVariable(
  "Path",
  $env:Path + ";$env:LOCALAPPDATA\Programs\envinject",
  "User"
)
```

5. Restart terminal, then: `envinject list`, `envinject gui`.

**Option B — From GitHub Release**

Download `envinject-windows-x86_64.exe` from **Releases**, rename to `envinject.exe`, put in e.g. `%LOCALAPPDATA%\Programs\envinject\`, add that dir to PATH.

### Ubuntu (bash)

**Option A — Build from source**

```bash
curl https://sh.rustup.rs -sSf | sh
source "$HOME/.cargo/env"
./build.sh release   # or: cargo build --release
sudo install -Dm755 ./target/release/envinject /usr/local/bin/envinject
envinject list && envinject gui
```

**Option B — From GitHub Release**

Download `envinject-linux-x86_64` from **Releases**, then:

```bash
mkdir -p "$HOME/.local/bin"
install -m755 ~/Downloads/envinject-linux-x86_64 "$HOME/.local/bin/envinject"
echo 'export PATH="$HOME/.local/bin:$PATH"' >> "$HOME/.bashrc"
source "$HOME/.bashrc"
```

**Option C — User install from source (no sudo)**

```bash
mkdir -p "$HOME/.local/bin"
install -m755 ./target/release/envinject "$HOME/.local/bin/envinject"
echo 'export PATH="$HOME/.local/bin:$PATH"' >> "$HOME/.bashrc"
source "$HOME/.bashrc"
```

**Deploy tips**

- Use a fixed install path; upgrade by replacing the binary.
- After upgrade, run `envinject list` to confirm config is still read.
- Config lives in the user config dir; **upgrading the binary does not wipe config**.

---

## CLI reference

### `envinject run <project> -- <command...>`

Load the project’s env and run the command with it.

```bash
envinject run <project> -- <command> [args...]
```

Example: `envinject run webapp -- npm run dev`. Everything after `--` is the command; stdin/stdout/stderr and exit code follow the child process.

### `envinject gui`

Opens the config editor (no terminal or log panel).

### `envinject list`

List all saved projects.

### `envinject show <project>`

Print all env vars for the project (`KEY=VALUE`).

---

## Config file

Config is stored as JSON in the platform config directory (`directories` crate).

Typical paths:

- Linux: `~/.config/envinject/config.json`
- macOS: `~/Library/Application Support/dev.envinject.envinject/...`
- Windows: `%APPDATA%\dev\envinject\envinject\config\config.json`

Example structure:

```json
{
  "projects": {
    "myproj": {
      "env": {
        "API_URL": "http://localhost:3000",
        "TOKEN": "abc123"
      }
    }
  }
}
```

---

## GUI (current)

- **Left:** project list; create/delete project.
- **Right:** selected project; add/edit/delete key-value pairs.
- **Top:** Save Config; status message.

---

## Use cases

- **Multi-project on one machine** — e.g. `frontend`, `backend`, `tools`; `run` injects the right project’s env.
- **No real `.env` in repo** — code reads from env; real values live in envinject config.
- **Switch env quickly** — e.g. `myproj-dev` vs `myproj-staging` as two projects.

---

## Caveats

- No separate “secret” vs “normal” vars in the UI; all key/value.
- No encryption or keychain in this version; suitable for local single-machine use.
- `show` prints values in plain text; use in a safe environment.

---

## Cross-platform

- Config path: `directories`.
- Execution: `std::process::Command`.
- GUI: `eframe`/`egui`; no heavy runtime.

---

## Possible extensions (not in MVP)

- Optional encryption / system keychain.
- Import/export config.
- Template validation (required keys).
- `run --print-env` for debugging.
- Project aliases and batch ops.

---

## Development

```bash
cargo run -- list
cargo run -- show myproj
cargo run -- run myproj -- env
cargo run -- gui
cargo check
```

---

## License

No license file in the repo yet; add one if you publish.
