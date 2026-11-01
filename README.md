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

git clone https://github.com/Strengthnaxy/RoofWm.git
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

Bindings

Optional helpers in other languages live under bindings/:

    bindings/cpp — CMake project, main.cpp

    bindings/csharp — .NET 8 project, Program.cs

    bindings/zig — build.zig + main.zig

They are independent from the rust compositor and are built separately:
bash

cmake -S bindings/cpp -B bindings/cpp/build && cmake --build bindings/cpp/build
dotnet build bindings/csharp/RoofWm.csproj
zig build --build-file bindings/zig/build.zig

Layout
text

RoofWm/
├── .github/
│   ├── ISSUE_TEMPLATE/bug_report.md
│   ├── PULL_REQUEST_TEMPLATE.md
│   ├── dependabot.yml
│   └── workflows/ci.yml
├── bindings/
│   ├── cpp/{CMakeLists.txt,main.cpp}
│   ├── csharp/{RoofWm.csproj,Program.cs}
│   └── zig/{build.zig,main.zig}
├── config/roof.rs
├── src/{compositor,config,input,layout,main,output}.rs
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

License

MIT. See LICENSE.
text


---

## 3. Как запустить на «11 Wayland» — два прочтения

Вы написали «на 11 wayland». Это можно понять двояко:

### Вариант A: Wayland **1.11** (старая версия протокола/библиотеки)

Wayland 1.11 — это 2016 год. Smithay 0.4 и `wayland-server 0.31` **не соберутся** с libwayland 1.11: им нужны более новые заголовки (`wlr-*`, `xdg-shell` из staging и т.д.). Минимально нужен **libwayland ≥ 1.20** (лучше 1.22+).

Если у вас действительно старая система с Wayland 1.11 — вариантов два:

1. **Обновить libwayland** (обычно через обновление дистрибутива).
2. **Собрать композитор в winit-режиме** — он не требует DRM/KMS и работает поверх *существующего* Wayland/X11-сеанса:
   ```bash
   cargo run --features winit
