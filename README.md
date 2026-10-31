# ORBIT Terminal — Development

### Building a universal terminal experience, one commit at a time.

<p align="center">

**Explore. Execute. Evolve.**

An open-source development project focused on creating a modern, lightweight, and extensible desktop terminal application.

[![Repository](https://img.shields.io/badge/Repository-Development-a4f36c?style=for-the-badge)](https://github.com/OtakuSamaRS/ORBIT-Terminal-Development)
[![Application](https://img.shields.io/badge/Application-ORBIT%20Terminal-181818?style=for-the-badge)](https://github.com/OtakuSamaRS/ORBIT-Terminal)
[![Built with Rust](https://img.shields.io/badge/Built%20with-Rust-orange?style=for-the-badge&logo=rust)](https://www.rust-lang.org/)
[![Tauri](https://img.shields.io/badge/Framework-Tauri%202-24C8DB?style=for-the-badge&logo=tauri)](https://tauri.app/)

</p>

---

## 📖 About the Project

**ORBIT Terminal** is a desktop terminal application designed to unify multiple command-line environments within a single, modern interface.

The goal is to create a terminal that automatically detects available shells, understands the host environment, and provides users with a consistent command-line experience across different operating systems.

This repository is the development home of ORBIT Terminal.

It contains the application source code, development resources, implementation changes, and the foundation for future features and community contributions.

> This repository is intended for developers and contributors. For stable downloads, visit the main application repository.

## 🎯 Project Vision

The long-term vision of ORBIT Terminal is to build a universal terminal experience.

We want ORBIT to move beyond being just another terminal emulator by focusing on:

- Automatic operating system detection.
- Automatic shell discovery and configuration.
- Unified access to different command-line environments.
- A consistent experience across Windows and Linux.
- Lightweight performance and efficient resource usage.
- A clean, modern, developer-focused interface.
- Simple installation and setup.

## ✨ Current Capabilities

The current development baseline includes:

| Feature | Status |
|---|---|
| Windows PowerShell 5.1 | Implemented |
| Windows Command Prompt | Implemented |
| Git Bash | Implemented |
| Shell selection interface | Implemented |
| Terminal session management | Implemented |
| Start and stop terminal sessions | Implemented |
| Back navigation | Implemented |
| Dark ORBIT interface | Implemented |
| Linux support | Planned |
| macOS support | Planned |
| Automatic cross-platform setup | Planned |

*Implementation status reflects the current development baseline and may change as development progresses.*

## 🛠️ Technology Stack

ORBIT Terminal uses a combination of native and web technologies.

| Technology | Role |
|---|---|
| Rust | Native backend and process management |
| Tauri 2 | Desktop application framework |
| JavaScript | Frontend application logic |
| xterm.js | Terminal rendering |
| Vite | Frontend development and bundling |
| Cargo | Rust dependency and build management |
| npm | JavaScript dependency management |

## 📂 Project Structure

```text
ORBIT-Terminal-Development/
│
├── .vscode/
│   └── extensions.json
│
├── src/
│   ├── assets/
│   ├── index.html
│   ├── main.js
│   └── styles.css
│
├── src-tauri/
│   ├── capabilities/
│   │   └── default.json
│   ├── icons/
│   ├── src/
│   │   ├── lib.rs
│   │   └── main.rs
│   ├── Cargo.toml
│   ├── Cargo.lock
│   ├── build.rs
│   └── tauri.conf.json
│
├── .gitignore
├── package.json
├── package-lock.json
└── vite.config.js
```

### Architecture Overview

**Frontend (`src/`)**

Responsible for the user interface, shell selection, terminal display, and frontend interactions.

**Backend (`src-tauri/src/`)**

Responsible for native application functionality, terminal process management, and communication between the frontend and operating system.

**Tauri configuration (`src-tauri/`)**

Contains desktop application configuration, permissions, icons, and Rust build settings.

## 🚀 Getting Started

Want to contribute or run ORBIT Terminal locally?

Follow the instructions below.

### 1. Prerequisites

Install the following development tools:

- [Git](https://git-scm.com/downloads)
- [Node.js](https://nodejs.org/)
- [Rust and Cargo](https://www.rust-lang.org/tools/install)
- [Tauri 2 prerequisites](https://v2.tauri.app/start/prerequisites/)

On Windows, make sure the required Microsoft C++ Build Tools and WebView2 prerequisites are installed.

### 2. Clone the Repository

```bash
git clone https://github.com/OtakuSamaRS/ORBIT-Terminal-Development.git
```

Navigate into the project directory:

```bash
cd ORBIT-Terminal-Development
```

### 3. Install Dependencies

```bash
npm install
```

### 4. Start Development Mode

```bash
npm run tauri -- dev
```

This launches the ORBIT Terminal desktop application in development mode.

### 5. Build the Application

```bash
npm run tauri -- build
```

The compiled application will be generated in the Tauri build output directory under:

```text
src-tauri/target/release/
```

The exact installer and executable locations depend on the operating system and Tauri build configuration.

## 🧪 Development Guidelines

To keep the project maintainable:

- Keep frontend and backend responsibilities separated.
- Write clear and understandable code.
- Avoid unnecessary dependencies.
- Follow existing naming and formatting conventions.
- Test changes before submitting pull requests.
- Do not commit build artifacts, local environment files, or personal configuration.
- Document new functionality where appropriate.

### Useful Commands

| Command | Purpose |
|---|---|
| `npm install` | Install frontend dependencies |
| `npm run build` | Build the frontend |
| `npm run tauri -- dev` | Run the desktop application |
| `npm run tauri -- build` | Build the desktop application |
| `cargo check` | Check the Rust backend |
| `cargo fmt` | Format Rust source code |

## 🗺️ Development Roadmap

ORBIT Terminal is being developed with a long-term focus on portability, extensibility, and usability.

### Phase 1 — Core Foundation

- [x] Establish Tauri 2 application structure.
- [x] Build the ORBIT desktop interface.
- [x] Implement Windows shell selection.
- [x] Integrate terminal rendering.
- [x] Implement terminal session lifecycle.
- [x] Establish the development repository.

### Phase 2 — Universal Environment

- [ ] Automatic operating system detection.
- [ ] Automatic shell discovery.
- [ ] Linux shell integration.
- [ ] macOS shell integration.
- [ ] Environment and capability detection.
- [ ] Automatic configuration profiles.

### Phase 3 — Productivity

- [ ] Multiple terminal tabs.
- [ ] Custom shell profiles.
- [ ] Customizable themes.
- [ ] Keyboard shortcuts.
- [ ] Improved terminal settings.
- [ ] Better session management.

### Phase 4 — Distribution & Ecosystem

- [ ] Automated cross-platform builds.
- [ ] Automated release pipeline.
- [ ] Installation and update management.
- [ ] Plugin or extension architecture.
- [ ] Expanded documentation.
- [ ] Community-driven development.

*Roadmap items are planned objectives, not promises of delivery dates.*

## 🤝 Contributing

ORBIT Terminal welcomes developers, testers, designers, and open-source enthusiasts.

Whether you want to fix a bug, improve the interface, suggest a feature, or help expand platform support, your contribution is appreciated.

### How to Contribute

1. Fork this repository.
2. Create a new branch for your changes.
3. Make your changes.
4. Test your implementation.
5. Commit your changes with a descriptive message.
6. Push your branch to your fork.
7. Open a Pull Request.

### Branch Naming

Suggested branch naming conventions:

```text
feature/feature-name
fix/bug-description
docs/documentation-update
refactor/component-name
```

### Commit Message Examples

```text
feat: add automatic shell detection
fix: resolve terminal session cleanup
docs: update development instructions
refactor: improve process management
```

### Before Submitting a Pull Request

- Ensure the application builds successfully.
- Run `cargo check`.
- Run `npm run build`.
- Test the affected functionality.
- Explain what changed and why.
- Include screenshots for significant UI changes.

## 🐛 Issues & Discussions

Found a bug? Have a feature request? Want to discuss an implementation?

- [Report a Bug](https://github.com/OtakuSamaRS/ORBIT-Terminal-Development/issues)
- [Request a Feature](https://github.com/OtakuSamaRS/ORBIT-Terminal-Development/issues)
- [Browse Source Code](https://github.com/OtakuSamaRS/ORBIT-Terminal-Development)

Please provide enough information to reproduce reported issues, including your operating system, relevant shell, and error output.

## 🔒 Security

Please do not publish security vulnerabilities as public issues.

For sensitive security concerns, contact the project maintainer privately through their GitHub profile.

## 📦 Stable Releases

Development builds and stable application releases are maintained separately.

For ready-to-use downloads, visit:

**[ORBIT Terminal — Main Repository](https://github.com/OtakuSamaRS/ORBIT-Terminal)**

**[Download Latest Release](https://github.com/OtakuSamaRS/ORBIT-Terminal/releases/latest)**

## 📜 License

A license has not yet been specified for this repository.

Until a license is added, do not assume that the source code is available for unrestricted reuse, modification, or redistribution.

## 👨‍💻 Maintainer

**OtakuSamaRS**

Creator and maintainer of ORBIT Terminal.

- GitHub: [@OtakuSamaRS](https://github.com/OtakuSamaRS)
- Main Application: [ORBIT Terminal](https://github.com/OtakuSamaRS/ORBIT-Terminal)
- Development Repository: [ORBIT Terminal Development](https://github.com/OtakuSamaRS/ORBIT-Terminal-Development)

---

<p align="center">

# ORBIT TERMINAL

### Explore. Execute. Evolve.

**Built for developers. Designed for freedom.**

</p>
