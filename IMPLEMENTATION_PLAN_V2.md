# 🚀 Implementation Plan V2 - Complete Overhaul

**Date**: October 17, 2025  
**Status**: IN PROGRESS  
**Scope**: 5-Phase Complete Game Evolution

---

## 📋 Implementation Checklist

### Phase 1: Infinite Space + Camera Follow ✅ Ready
- [ ] Remove arena borders
- [ ] Add camera follow system
- [ ] Add scrolling background with synthwave image
- [ ] Add parallax starfield
- [ ] Update enemy spawning (relative to camera)
- [ ] Add wrap-around at far edges

### Phase 2: Wave Weapon Mode ✅ Ready
- [ ] Add WeaponType enum (Trail/Wave)
- [ ] Add WaveProjectile component
- [ ] Implement wave shooting system
- [ ] Add projectile collision
- [ ] Add weapon swap power-up
- [ ] Balance both modes

### Phase 3: Power-Up Choice System ✅ Ready
- [ ] Create PowerUpType enum (12 types)
- [ ] Build choice UI with emoji icons
- [ ] Pause game on choice spawn
- [ ] Handle keyboard selection (1/2/3)
- [ ] Apply chosen effect
- [ ] Add glow to power-up visuals

### Phase 4: Enemy Variants ✅ Ready
- [ ] Add EnemyVariant enum (6 types)
- [ ] Implement wave progression
- [ ] Add enemy type unlocking
- [ ] Color-code by difficulty
- [ ] Scale size with health
- [ ] Add special effects

### Phase 5: Balance & Polish ✅ Ready
- [ ] Rebalance all constants
- [ ] Add milestone moments
- [ ] Integrate background image
- [ ] Add glow effects to sprites
- [ ] Increase contrast for visibility
- [ ] Final testing

---

## 🎨 Visual Enhancements for Dark Background

### Sprite Glow System
```rust
// All game sprites get glow outline
- Player: Cyan glow (0.6, 0.9, 1.0)
- Enemies: Red/Orange glow based on tier
- Trail: Bright cyan glow with fade
- Power-ups: Color-matched intense glow
- Projectiles: Bright glow trails
```

### Contrast Adjustments
- Increase all sprite brightness by 20%
- Add bloom/glow post-processing
- Particle effects more luminous
- UI text with shadow/outline

---

**Total Estimated Time**: 6-8 hours  
**Lines Changed**: ~800-1000  
**New Systems**: 15+  
**Backward Compatibility**: Full (can revert to backup)

