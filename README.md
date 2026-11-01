# RoofWm

Tiling wayland compositor written in rust.

## Status

alpha / experimental

## Stack

- rust + smithay + calloop
- linux (drm/kms + libinput + libseat)

## Features

- drm/kms output
- libinput input handling
- libseat seat management
- xdg-shell
- tiling layouts (dwindle, master)
- ron-based config at ~/config/roof/roof.rs
- winit backend for debugging

Planned: wlr-layer-shell, screencopy, idle-inhibit.

## Build

Arch:

bash
sudo pacman -S --needed base-devel rustup mesa libdrm libinput seatd libgbm wayland
Fedora:
bash

sudo dnf install gcc make rust cargo mesa-libEGL-devel libdrm-devel \
                 libinput-devel seatd-devel libgbm-devel wayland-devel

Debian/Ubuntu:
bash

sudo apt install build-essential rustc cargo libegl1-mesa-dev \
                 libdrm-dev libinput-dev libseat-dev libgbm-dev libwayland-dev

Then:
bash

git clone https://github.com/YOUR_NAME/RoofWm.git
cd RoofWm
cargo build --release

Binary: ./target/release/roofwm
Run

From a tty:
bash

./target/release/roofwm

For debugging without leaving the current session:
bash

cargo run --features winit

Config

File: ~/config/roof/roof.rs
Format: ron
ron

(
    layout:   "dwindle",
    gaps:     8,
    border:   2,
    terminal: "foot",
    modifier: "Super",
)

Fields:

    layout — "dwindle" or "master"

    gaps — inner gap in pixels

    border — border width in pixels

    terminal — default terminal command

    modifier — modifier key for keybindings

Layout
text

RoofWm/
├── .github/
│   ├── ISSUE_TEMPLATE/bug_report.md
│   ├── PULL_REQUEST_TEMPLATE.md
│   ├── dependabot.yml
│   └── workflows/ci.yml
├── config/roof.rs
├── src/
│   ├── compositor.rs
│   ├── config.rs
│   ├── input.rs
│   ├── layout.rs
│   ├── main.rs
│   └── output.rs
├── .gitignore
├── Cargo.toml
├── LICENSE
└── README.md

Contributing

    fork

    git checkout -b feat/your-thing

    git commit -m "feat: add your thing"

    git push origin feat/your-thing

    open a pull request

Before pushing:
bash

cargo fmt
cargo clippy --all-targets --all-features -- -D warnings
cargo test
