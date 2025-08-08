# 🧠 3BLD-Coach

A command-line (or GUI) training tool for learning and mastering **3x3 Blindfolded Solving (3BLD)**.  
Simulates cube states, scrambles, buffer-based memo training, and generates corner and edge solutions to help you speed up your blindsolving game.

---

## 🚀 Features

- 🧩 Accurate 3x3 cube simulation
- 🔀 WCA-style scramble generator
- 🎯 Corner & edge memo assistant
- 🧠 Buffer-based solution generation
- 📈 Memory and execution practice modes
- ⚙️ Customizable buffer and sticker settings *(coming soon)*

---

## 📦 Installation

```bash
git clone https://github.com/yourusername/3bld-c.git
cd 3bld-c
make
./3bld-c
````

> Requires `gcc`, `make`, and a POSIX-compliant environment.

---

## 🕹️ Usage

Basic usage:

```bash
./3bld-c
```

Planned options:

```bash
./3bld-c --scramble
./3bld-c --solve corners
./3bld-c --solve edges
./3bld-c --memo trainer
```

---

## 🧠 How It Works

* Uses a 3D cube model internally to simulate state
* Parses and executes scramble strings
* Identifies cycles based on current buffer
* Outputs optimal or beginner-method memo sequences

---

## ✅ To-Do

* [ ] Visual representation of cube (CLI or TUI)
* [ ] Support multiple solving methods (3-style, OP, M2, etc.)
* [ ] JSON import/export of memo sessions
* [ ] Add unit tests
* [ ] GUI version (maybe in SDL or QT)
