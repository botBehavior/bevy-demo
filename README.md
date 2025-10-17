# 🎮 Bevy Survivor Demo

A fast-paced, arcade-style survival game built with [Bevy Engine 0.14](https://bevyengine.org/). Fight off waves of enemies, collect power-ups, and see how long you can survive!

![Rust](https://img.shields.io/badge/rust-2021-orange.svg)
![Bevy](https://img.shields.io/badge/bevy-0.14-blue.svg)
![License](https://img.shields.io/badge/license-MIT-green.svg)

## 🚀 Quick Start

### Native (Recommended)
```bash
cargo run --release
```

### Web (WASM)
```bash
# Install trunk if you haven't
cargo install trunk

# Serve locally
trunk serve --release

# Build for deployment
trunk build --release
```

## 🎯 How to Play

### Controls
- **Mouse Movement**: Aim and move your character
- **Left Click / Space**: Activate trail weapon (default)
- **Tab**: Toggle wave weapon
- **Esc**: Pause/unpause game (releases cursor)

### Objective
Survive as long as possible against endless waves of enemies. Kill enemies to increase your combo multiplier and score. Collect power-ups to become stronger!

### Weapons

#### 🌀 Trail Weapon (Default)
A deadly trail follows your cursor. Enemies that touch it take continuous damage and are knocked back.
- **Damage**: High (3 base + upgrades)
- **Range**: Short to medium
- **Best for**: Close combat, crowd control

#### 🌊 Wave Weapon (Tab to switch)
Cast powerful ocean-wave projectiles that curve outward from your movement.
- **Damage**: Medium (2 per wave)
- **Range**: Long
- **Best for**: Ranged attacks, keeping distance

### Power-Ups

Power-ups drop randomly when you kill enemies (15% chance). Each has a distinct color and shape:

| Icon | Type | Color | Effect | Duration |
|------|------|-------|--------|----------|
| 🔴 Circle | **Heart** | Bright Red | Restore 1 health | Instant |
| 🔵 Tall Rect | **Shield** | Bright Cyan | Invincibility | 4 seconds |
| 🟡 Square | **Damage** | Bright Gold | +50% damage per stack | Permanent |
| 🟣 Circle | **Accuracy** | Bright Purple | Faster, snappier movement | Permanent |

## ⚙️ Game Mechanics

### Progression
- **Enemies spawn faster** as your score increases
- **Enemy speed increases** gradually
- **Combo system**: Kill enemies quickly to build multiplier (1s window)
- **Power fantasy**: Stack damage and accuracy upgrades to become overpowered

### Health System
- Start with 4 health
- Lose 1 health per enemy collision
- Shield power-up grants temporary invincibility
- Heart power-up restores 1 health

### Scoring
```
Base Score: 10 points per kill
Combo Bonus: +50% per streak level
Example: 5x combo = 10 × (1 + 2.5) = 35 points
```

## 🎨 Game Feel Features

This game is designed with "game juice" in mind:

- **Screen Shake**: Intensity increases with combo
- **Knockback**: Enemies get pushed back when hit
- **Hit Freeze**: Brief pause on kills for impact
- **Particle Effects**: Death explosions, wave trails
- **Color Coding**: Instant visual recognition
- **Smooth Movement**: Lerped player acceleration/deceleration
- **Enemy AI**: Steering behaviors with prediction

## 🛠️ Technical Stack

### Core
- **Engine**: Bevy 0.14.2
- **Language**: Rust 2021 Edition
- **Physics**: Avian2D (for future expansion)

### Architecture
- **ECS**: Entity-Component-System architecture
- **State Management**: Bevy resources and components
- **Cross-Platform**: Native (Windows/Mac/Linux) + WASM

### Key Systems
- Dynamic enemy spawning around camera
- Infinite parallax background tiling
- Combo and scoring system
- Particle effects system
- Screen shake and hit freeze
- Power-up drop system with weighted randomization

## 📁 Project Structure

```
bevy-demo/
├── src/
│   └── main.rs          # Main game code (~1800 lines)
├── assets/
│   ├── fonts/
│   │   └── FiraSans-Bold.ttf
│   └── 240_F...jpg      # Background image (4K)
├── docs/
│   └── spec.md          # Game design specification
├── scripts/
│   └── vercel-build.sh  # WASM deployment script
├── Cargo.toml           # Dependencies
├── Trunk.toml           # WASM build config
├── vercel.json          # Vercel deployment config
├── CHANGELOG.md         # Version history
└── README.md            # This file
```

## 🔧 Development

### Building
```bash
# Debug build (fast compilation, slower runtime)
cargo build

# Release build (optimized)
cargo build --release

# WASM build
trunk build --release
```

### Linting
```bash
cargo clippy
```

### Formatting
```bash
cargo fmt
```

## 🌐 Deployment

### Vercel (WASM)
This project is configured for automatic deployment to Vercel:

1. Push to main branch
2. Vercel automatically builds with `scripts/vercel-build.sh`
3. Deploys WASM build to CDN

### Manual WASM Deployment
```bash
trunk build --release
# Deploy dist/ folder to any static host
```

## 🎮 Gameplay Tips

1. **Movement is survival**: Stay mobile, don't get cornered
2. **Accuracy upgrade** makes movement more responsive (collect purple circles!)
3. **Shield is tactical**: Use when overwhelmed (only lasts 4s)
4. **Damage stacks**: Collect gold squares to become powerful
5. **Combo multiplier**: Kill quickly to build score
6. **Wave weapon**: Great for keeping distance early game
7. **Trail weapon**: Better for high-damage melting when powered up

## 🐛 Known Issues

- Background image may take a moment to load on first run
- Some compiler warnings about unused fields (non-critical)

## 📝 Version History

See [CHANGELOG.md](CHANGELOG.md) for detailed version history.

**Current Version**: v2.7.1
- Fixed emoji rendering issues
- Enhanced sprite visibility with HDR glow
- Background parallax working
- All game systems stable

## 🤝 Contributing

This is a demo project, but feel free to fork and experiment!

## 📄 License

MIT License - See LICENSE file for details

## 🙏 Credits

- **Engine**: [Bevy Engine](https://bevyengine.org/)
- **Font**: Fira Sans
- **Background**: Space theme image

---

Built with 🦀 Rust and ❤️ for game development

**Play now**: `cargo run --release`
