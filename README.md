# bevy-demo

🎯 **Threadweaver** - Infinite Space Survival

Threadweaver is a high-octane arena survival game built in Bevy. You are a luminous entity weaving deadly patterns through infinite space. Choose your weapon: weave lingering trails of energy, or unleash volleys of projectiles. Master momentum physics, build combos, and survive escalating waves of enemies in a stunning synthwave environment.

## 🎮 V2 Features (Current)

### Core Gameplay
- **Infinite Space**: Camera follows you through vast synthwave-themed space. No borders, pure freedom.
- **Dual Weapon Modes** (Press **Q** to toggle):
  - **Trail Mode**: Leave glowing energy trails that damage enemies on contact
  - **Wave Mode**: Fire 5-projectile spread bursts every 0.35s
- **Momentum Physics**: Smooth acceleration/deceleration with satisfying knockback
- **Power Fantasy Progression**: Start weak (4 HP), scale fast with power-ups and combos

### Combat System
- **Combo Multiplier**: Chain kills within 1.0s for increasing score bonuses
- **Screen Shake**: Intensity scales with combo streak for maximum juice
- **Knockback**: Enemies fly back 250 units when hit, you bounce 200 units when damaged
- **Hit Freeze**: Brief 0.04s pause on kills for impact sensation
- **Particle Explosions**: 20-particle radial bursts on every kill

### Visual Design
- **Synthwave Aesthetic**: Dark grid background with glowing neon sprites
- **High Contrast**: All gameplay elements glow brightly for perfect visibility
- **Smooth Animations**: 60 FPS stable with physics-based movement

### Power-Ups (15% drop rate)
- **Heart** (Red): Restore 1 HP
- **Shield** (Cyan): 4 seconds of invincibility
- **Damage** (Gold): +50% damage boost (stackable)

## 🕹️ Controls

| Input | Action |
|-------|--------|
| **Mouse Movement** | Aim direction (cursor locked in-game) |
| **Q** | Toggle between Trail/Wave weapon modes |
| **ESC** | Pause/Unpause (releases cursor) |
| **SPACE** | Restart after game over |

## 📊 Balance (V2)

| Stat | Value | Notes |
|------|-------|-------|
| Player Health | 4 | Die fast, learn fast |
| Player Speed | 950 | Responsive movement |
| Trail Damage | 3 | Feel powerful early |
| Enemy Start Speed | 180 | Manageable early game |
| Shield Duration | 4s | Tactical use, not invincible |
| Combo Window | 1.0s | Skill-based timing |

## 🚀 Quick Start

### Prerequisites
- **Rust** 1.79+ (repository includes `rust-toolchain.toml`)
- **WASM target**: `rustup target add wasm32-unknown-unknown`
- **Trunk**: `cargo install --locked trunk` (for web builds)

### Run Native (Recommended)
```bash
cargo run --release
```

Best performance, full features, immediate feedback.

### Run Web (For Sharing)
```bash
trunk serve --release
```

Visit `http://localhost:8080`. Click canvas to lock cursor and start playing!

### Build for Production
```bash
# Native binary
cargo build --release

# Web build
trunk build --release
```

## 🎯 Gameplay Tips

1. **Try Both Weapons**: Trail for defense, Wave for offense
2. **Build Combos**: Chain kills quickly for massive score bonuses
3. **Use Shield Tactically**: Save for emergency escapes (only 4s!)
4. **Master Momentum**: Smooth turns, predict enemy paths
5. **Power-Ups Are Rare**: Chase them when they spawn (15% chance)

## 📚 Documentation

- [V2 Complete Guide](docs/V2_COMPLETE.md) – Full feature list and changes
- [Juicy Game Complete](JUICY_GAME_COMPLETE.md) – Physics and particles implementation
- [Quick Reference](QUICK_REFERENCE.md) – Fast lookup for all features
- [MVP Specification](docs/spec.md) – Original design document
- [Workflow](docs/workflow.md) – Development guidelines

## 🛠️ Technical Details

### Stack
- **Engine**: Bevy 0.14
- **Physics**: Avian2D (momentum-based movement)
- **Particles**: Custom sprite-based system
- **Platform**: Native (Linux/Mac/Windows) + WebAssembly

### Performance
- **Target**: 60 FPS locked
- **Particles**: 100-150 concurrent, no lag
- **Enemies**: 50+ on-screen, stable
- **Physics**: Real-time steering behaviors

### Architecture
- **ECS**: Entity-Component-System throughout
- **State Management**: Resource-based game state
- **Rendering**: Sprite-based 2D with glow effects

## 🎨 Visual Aesthetic

Threadweaver embraces a **synthwave/retrowave** visual style:
- Dark grid background (infinite tiling)
- Neon cyan player with glow
- Bright enemy sprites (red/pink/orange gradient)
- Luminous power-ups color-coded by type
- Particle effects with fade-out alpha
- High contrast for gameplay clarity

## 🐛 Known Issues

- None critical! Minor cosmetic issues:
  - Background tiling seams at extreme distances
  - Some unused constants (reserved for V3)
  - Wave projectiles may briefly persist at edges (1.5s max)

## 🚧 Roadmap (V3)

Planned but not yet implemented:

- **Power-Up Choice System**: Choose 1 of 3 options when power-ups spawn
- **Enemy Variants**: 6 types (Fast, Tank, Splitter, Teleporter, etc.)
- **Wave Progression**: Structured waves with calm periods
- **Milestone Moments**: Special events at 5x/10x/15x combo
- **Meta-Progression**: Unlocks between runs

## 🤝 Contributing

This is an experimental prototype. Feel free to:
- Report bugs via issues
- Suggest balance tweaks
- Share gameplay recordings
- Fork and experiment

## 📜 License

See repository for license details.

---

## 🎉 What Makes This Special

**Threadweaver** isn't just another twin-stick shooter. It's a carefully tuned dopamine machine where:
- Every kill delivers 6 simultaneous feedback channels
- Movement feels smooth and weighty (not floaty or rigid)
- Strategic depth comes from weapon choice and positioning
- Power fantasy is real (start weak, become god-like)
- Visual design pops without overwhelming
- Physics make every impact feel meaningful

**It's not about reflexes. It's about flow state.** 🌊

---

**Current Version**: 2.0 - Infinite Space Edition  
**Status**: ✅ Playable, Polished, Performant  
**Fun Factor**: 🎮🎮🎮🎮🎮

**Ready to weave?** `cargo run --release` 🚀
