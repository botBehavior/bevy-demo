# Threadweaver MVP Testing Guide

## Quick Start

### Test Native Build
```bash
cargo run
```

**Expected Behavior:**
- Window opens with 960x720 arena
- Player (light blue circle) starts at center
- Mouse cursor locks and becomes hidden
- Player smoothly follows mouse movement
- Cyan trail segments spawn behind player
- Red enemies spawn from arena edges and chase player
- HUD displays: Score, Health (5/5), Combo, Damage, Shield status, Game status

### Test Web Build
```bash
trunk serve
```

Then open http://localhost:8080 in your browser.

**Expected Behavior:**
- Canvas fills viewport (max 960x720)
- Same gameplay as native
- Cursor lock works on canvas click
- Performance: stable 60 FPS on mid-range hardware

## Core Gameplay Test Scenarios

### 1. Basic Movement
- ✅ Player should smoothly follow cursor/mouse
- ✅ Player should stay within arena bounds
- ✅ Trail segments should spawn every ~28ms
- ✅ Trail should fade out over ~2.6 seconds

### 2. Enemy Interactions
- ✅ Enemies spawn from arena edges every ~1.2s initially
- ✅ Enemies move toward player
- ✅ Enemy spawn rate increases over time
- ✅ Enemy speed scales with score

### 3. Trail Combat
- ✅ Enemies lose health when touching trail
- ✅ Enemies turn darker/disappear when defeated
- ✅ Score increases on enemy kill
- ✅ Combo multiplier builds with consecutive kills

### 4. Player Damage
- ✅ Direct enemy collision reduces health by 1
- ✅ Health displayed correctly in HUD
- ✅ Game ends when health reaches 0
- ✅ "Status: Down! Press SPACE to respawn" message appears

### 5. Power-Up System
**Heart (Red, 22px)**
- ✅ Spawns randomly from defeated enemies (~35% chance)
- ✅ Heals 1 HP when collected
- ✅ Cannot exceed max health
- ✅ Disappears after 12 seconds if not collected

**Shield (Blue, 24px)**
- ✅ Spawns randomly from defeated enemies
- ✅ Grants 10 seconds of invulnerability
- ✅ HUD shows remaining shield time
- ✅ Player takes no damage during shield duration
- ✅ Enemies still despawn on collision

**Damage Core (Orange, 20px)**
- ✅ Spawns randomly from defeated enemies
- ✅ Permanently increases damage for current run
- ✅ HUD shows damage multiplier (e.g., "Damage x1.5")
- ✅ Stacks with multiple pickups

### 6. Combo System
- ✅ Combo multiplier increases with rapid kills
- ✅ 1.2s window to maintain combo
- ✅ Combo x1.0, x1.5, x2.0, etc.
- ✅ Bonus score scales with multiplier
- ✅ HUD shows "Combo x2.0 (3)" format

### 7. Pause & Restart
- ✅ Press ESC to pause (unlocks cursor)
- ✅ Press ESC again to resume
- ✅ Press SPACE after death to restart
- ✅ Restart clears all enemies, trails, power-ups
- ✅ Health resets to 5/5
- ✅ Score resets, Best Score persists

### 8. UI/HUD
- ✅ Score: current and best displayed
- ✅ Health: X/5 format
- ✅ Combo: multiplier and streak count
- ✅ Buffs: damage multiplier and shield timer
- ✅ Status: Running/Paused/Down messages
- ✅ All text updates in real-time

## Performance Benchmarks

### Native Build
- **Target**: 60 FPS stable
- **Scenario**: 25+ enemies, 90+ trail segments
- **Hardware**: Intel Iris-class GPU or equivalent

### Web Build (WASM)
- **Target**: 60 FPS on Chrome/Firefox
- **Scenario**: Same as native
- **Note**: May drop to 45-55 FPS on lower-end devices (still playable)

## Known Issues (Not Bugs)

These are expected limitations of the MVP:
1. No audio feedback
2. No visual effects beyond basic sprites
3. No touch controls (desktop/mouse only)
4. No persistent high score storage (resets on page reload)
5. Single enemy type (basic charger)

## Regression Testing Checklist

Before any new features, verify:
- [ ] Native build compiles without errors
- [ ] WASM build compiles without errors
- [ ] Game launches and runs smoothly
- [ ] No console errors in browser
- [ ] All 8 test scenarios pass
- [ ] HUD updates correctly
- [ ] Power-ups work as designed
- [ ] Restart fully clears game state

## Reporting Issues

When reporting bugs, include:
1. **Platform**: Native (OS) or Web (Browser + version)
2. **Steps to reproduce**
3. **Expected vs actual behavior**
4. **Screenshots/video** if visual issue
5. **Console output** for crashes

## Development Testing Commands

```bash
# Format code
cargo fmt

# Check compilation (fast)
cargo check

# Check with clippy linting
cargo clippy --all-targets

# Build optimized native
cargo build --release

# Build for web
trunk build --release

# Test WASM target
cargo check --target wasm32-unknown-unknown
```

## Success Criteria

✅ **MVP is ready for wider testing when:**
- All 8 test scenarios pass
- No compilation errors or warnings
- 60 FPS performance on target hardware
- HUD displays all information correctly
- Power-ups function as designed
- Game loop is engaging for 60+ seconds

## Next Phase Testing

After MVP validation, test:
1. Touch input implementation
2. Audio feedback integration
3. Particle effects
4. IndexedDB persistence
5. Additional enemy types
6. Mobile device compatibility

---

**Last Updated**: October 17, 2025
**MVP Version**: 0.1.0
**Status**: ✅ Ready for Testing

