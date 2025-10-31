# Threadweaver – Bevy Web Survivor

Threadweaver is a browser-first survivor prototype powered by [Bevy](https://bevyengine.org/) and WebGL2 (WebGPU-ready). The project ships a single code base for desktop, web, and touch devices with adaptive UI, persistent progression, and high-juice combat.

![Rust](https://img.shields.io/badge/rust-2021-orange.svg)
![Bevy](https://img.shields.io/badge/bevy-0.14-blue.svg)
![Target](https://img.shields.io/badge/target-webgl2%20%7C%20wasm-blueviolet.svg)

## Feature Highlights
- **Web build baseline** - currently WebGL2 via Bevy 0.14 (Trunk + wasm-opt), with WebGPU roadmap tracked in docs.
- **Responsive UI system** – HUD and modal shop scale from phones (<720px) to ultrawide monitors, with live breakpoints and focus highlights.
- **Universal input support** – mouse/keyboard, touch drag, and gamepad navigation map to the same pointer + action vocabulary.
- **Persistent progression** – localStorage on the web and filesystem snapshots on native keep currency and upgrades in sync.

## Workspace Layout
- `threadweaver-core` – ECS components, resources, constants, and shop data models.
- `threadweaver-gameplay` – combat loops, spawning, FX, and persistence glue.
- `threadweaver-ui` – adaptive HUD + upgrade shop with theming and accessibility.
- `threadweaver-platform` – storage backends (web and native shims).
- `threadweaver-launcher` – thin binary that wires plugins and WebGL2-friendly defaults.

## Quick Start

### Prerequisites
```bash
rustup target add wasm32-unknown-unknown
cargo install trunk wasm-bindgen-cli --locked   # run once
```

### Web (WASM / WebGL2)
```bash
make web-dev       # trunk serve --release with hot reloads
make web-release   # optimized dist/ with wasm-bindgen + wasm-opt
```

### Native Desktop Smoke (optional)
```bash
cargo run -p threadweaver-launcher
```

## Controls & Inputs
- **Mouse / Touch drag** – steer the Weaver (touch is auto-detected).
- **Gamepad left stick** – analog steering.
- **Click / Space / South button** – confirm interactions.
- **Tab / West button** – weapon switch (if unlocked).
- **Esc / East button** – pause or close the shop overlay.

## Responsive UI & Accessibility
- HUD snaps to a compact layout below 720px width and re-centres on tablets/phones.
- Shop modal wraps cards, stretches to full width on mobile, and stays scroll-safe under tall listings.
- Focus state and controller navigation update the same highlighted card, ensuring parity across mouse, touch, and gamepad.
- Palette + typography come from a shared theme resource so dark/light variants, contrast tweaks, or localisation can be dropped in rapidly.

## Helpful Commands
```bash
make fmt          # cargo fmt (requires Rust toolchain)
make check        # cargo check -p threadweaver-launcher
make web-release  # build optimized WASM bundle
```

## Licensing
Project assets and code follow the repository’s root LICENSE unless otherwise noted in the `assets/` tree.

