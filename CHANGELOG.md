# Changelog

All notable changes to this project are documented here.

## [v2.7.1] - 2025-10-17 - HOTFIX: Sprite Rendering

### Fixed
- **Critical**: Fixed emoji rendering showing as question marks (?)
  - Root cause: FiraSans-Bold.ttf doesn't support emoji characters
  - Solution: Reverted to SpriteBundle with enhanced HDR glow effects
- **Critical**: Fixed missing background image
  - Root cause: z-depth -1000 was beyond camera render range
  - Solution: Changed to z=-100 with proper alpha blending

### Changed
- All entities now use SpriteBundle with super bright colors (>2.0 RGB for HDR)
- Player: Bright cyan square (32px)
- Enemies: Bright red rectangles (40x36px)
- Power-ups: Distinct color + shape combinations for instant recognition
  - Heart: Red circle
  - Shield: Cyan tall rectangle
  - Damage: Gold square
  - Accuracy: Purple circle
- Background tiles now 5x5 grid at z=-100 for visibility

### Technical
- Improved performance: SpriteBundle lighter than Text2dBundle
- Removed font rendering overhead
- Platform compatibility: Works everywhere without emoji font issues

---

## [v2.7] - 2025-10-17 - Emoji Sprites (Reverted)

### Attempted
- Tried implementing emoji sprites with Text2dBundle
- Discovered font compatibility issues

### Learned
- Standard fonts don't include emoji glyphs
- Cross-platform emoji rendering requires specialized fonts (Noto Color Emoji, etc.)
- Text2dBundle has rendering overhead vs SpriteBundle

---

## [v2.6] - 2025-10-17 - Critical UX Fixes

### Fixed
- **Movement**: Accuracy power-up no longer causes jittery physics
  - Redesigned as multiplier (1.0-1.4x) instead of direct lerp addition
  - Added hard caps on acceleration/deceleration
  - Increased base movement responsiveness
- **Camera**: Increased smoothing from 0.08 to 0.30 for better follow
- **Wave Weapon**: Complete redesign for "ocean wave" effect
  - 6 particles (3 per side) instead of 2
  - Larger size (28-48px)
  - Stronger curve (400+ curve force)
  - Grows instead of shrinks as it travels
  - 2-second lifetime
- **Background**: Fixed infinite space feel
  - Changed clamping from ARENA_BOUNDS to ARENA_SIZE (5000.0)
  - Player can now explore much larger area

### Technical
- Player acceleration: 0.12 → 0.2
- Player deceleration: 0.25 → 0.4
- Camera smoothing: 0.08 → 0.30
- Wave particle lifetime: 1.5s → 2.0s
- Wave spawn distance: 18px → 35-71px

---

## [v2.5] - 2025-10-17 - Power Fantasy & Visual Polish

### Added
- **Accuracy Power-Up**: New upgrade for faster, snappier movement
  - Purple target icon
  - Incrementally improves player responsiveness
  - Stacking mechanic for progression
- **Infinite Background**: 4K space image with dynamic tiling
  - Parallax effect
  - Seamless infinite scrolling
- **Wave Weapon**: Alternative playstyle option
  - Toggle with Tab key
  - Curved projectiles that arc outward
  - Different damage/range profile

### Changed
- **Power-Up Rarity**: Drop chance 35% → 15% (makes them special)
- **Shield Duration**: 10s → 4s (tactical, not invincible)
- **Player Health**: 5 → 4 (faster deaths, more tension)
- **Enemy Speed**: 220 → 180 (less overwhelming)
- **Enemy Spawn Interval**: 1.2s → 2.0s (more breathing room)
- **Trail Damage**: 1 → 3 (power fantasy from start)
- **All Sprites**: Enhanced brightness and glow for dark background

### Balanced
- Heart power-up weight: 0.35
- Shield power-up weight: 0.25
- Damage power-up weight: 0.25
- Accuracy power-up weight: 0.15

---

## [v2.0] - 2025-10-17 - Game Feel Overhaul

### Added
- **Screen Shake**: Dynamic camera shake based on combo
- **Knockback**: Enemies pushed back when hit
- **Hit Freeze**: Brief time pause on kills for impact
- **Particle System**: Death explosions with colored particles
- **Player Momentum**: Smooth acceleration/deceleration
- **Enemy AI**: Steering behaviors with player prediction
- **Infinite Space**: Large arena (5000x5000) with camera follow
- **Combo System**: Tight 1.0s window for multiplier building

### Game Feel Constants
- Screen shake decay: 3.0
- Enemy knockback: 250.0
- Player knockback: 200.0
- Hit freeze: 0.04s
- Player acceleration: 0.12
- Player deceleration: 0.25
- Enemy turn speed: 0.18
- Camera smoothing: 0.08 (later increased to 0.30)

### Technical
- Integrated Avian2D physics engine
- Custom particle system (sprite-based)
- Camera follow with lerp smoothing
- Dynamic enemy spawning relative to camera
- Trail segment pooling system

---

## [v1.0] - 2025-10-16 - Initial Release

### Features
- Basic player movement with mouse control
- Enemy spawning and AI
- Trail weapon mechanic
- Health and collision system
- Score tracking
- Power-up system (Heart, Shield, Damage)
- Arena boundaries
- HUD display
- Cursor locking (Esc to release)

### Technical
- Bevy 0.14.2
- WASM support with Trunk
- Vercel deployment configuration
- Cross-platform (Windows, Mac, Linux, Web)

### Core Gameplay
- Player: 900 speed, 5 health
- Enemies: 220 base speed, incremental difficulty
- Trail: 1 damage, 2.6s lifetime
- Power-ups: 35% drop chance
- Arena: 1024x768

---

## Version Format

- **Major.Minor.Patch** (e.g., v2.7.1)
- **Major**: Significant gameplay changes
- **Minor**: New features, mechanics
- **Patch**: Bug fixes, balance tweaks

## Links

- **Repository**: (Add your repo URL)
- **Play Online**: (Add your Vercel URL)
- **Issues**: (Add issues URL)

