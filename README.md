<div align="center">

# 🧱 Breakout: Rust Edition

A classic Breakout clone written in Rust with [macroquad](https://github.com/not-fl3/macroquad).
I made it to learn Rust, just for fun.

[![Release](https://img.shields.io/github/v/release/yme11ow/breakout-rs?style=flat-square)](https://github.com/yme11ow/breakout-rs/releases/latest)
![Rust](https://img.shields.io/badge/Rust-2024_edition-orange?style=flat-square&logo=rust)
![Platforms](https://img.shields.io/badge/platforms-Windows%20%7C%20macOS%20%7C%20Linux-blue?style=flat-square)

</div>

---

## 🎮 Gameplay

Use the paddle to bounce the ball and break all **320 bricks**. You have **5 lives**, and you lose one every time the ball gets past your paddle. Clear the whole wall to win.

Bricks higher up the wall are worth more points:

| Row | Color   | Points |
|:---:|---------|-------:|
| 1   | 🟥 Red    | 100 |
| 2   | 🟧 Orange | 90  |
| 3   | 🟨 Yellow | 80  |
| 4   | 🟡 Gold   | 70  |
| 5   | 🟩 Green  | 60  |
| 6   | 🟦 Blue   | 50  |
| 7   | 🟪 Purple | 40  |
| 8   | 🩷 Magenta | 30  |
| 9   | 🌸 Pink   | 20  |
| 10  | ⬜ Gray   | 10  |

A perfect run scores **17,600** points.

## ⌨️ Controls

| Key | Action |
|-----|--------|
| <kbd>←</kbd> <kbd>→</kbd> | Move the paddle |
| <kbd>Enter</kbd> | Start / restart the game |
| <kbd>Esc</kbd> | Pause / resume |
| <kbd>M</kbd> | Mute / unmute audio |
| <kbd>Space</kbd> | Show the FPS counter |

## 📦 Download

Grab the latest build for your platform from the [**Releases**](https://github.com/yme11ow/breakout-rs/releases/latest) page:

- **Windows:** `x86_64-pc-windows-msvc`
- **macOS:** `x86_64-apple-darwin` (Intel; runs on Apple Silicon through Rosetta)
- **Linux:** `x86_64-unknown-linux-gnu`

The game is a single executable with all the audio built in. Extract it and run it.

> [!NOTE]
> The builds aren't code-signed, so your OS may warn you the first time you open the game.
> On **Windows**, click *More info → Run anyway*. On **macOS**, right-click the app and choose *Open*.
> On **Linux**, the game needs the ALSA runtime library (`libasound2`), which most desktop distros already include.

## 🛠️ Building from source

You'll need a recent [Rust toolchain](https://rustup.rs/) (1.85 or newer, for the 2024 edition).

```bash
git clone https://github.com/yme11ow/breakout-rs.git
cd breakout-rs
cargo run --release
```

On Linux, install the ALSA development headers first:

```bash
# Debian / Ubuntu
sudo apt install libasound2-dev

# Fedora
sudo dnf install alsa-lib-devel
```

## 🎵 Credits

**Music**
- Title screen, gameplay and game over music by **chippy01302**
- Win screen music by **Megumi Riyu**

**Built with**
- [macroquad](https://github.com/not-fl3/macroquad), a simple and easy-to-use Rust game library
