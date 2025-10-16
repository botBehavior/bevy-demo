# Threadweaver MVP Specification

## Vision
Threadweaver is a minimalist arena survival experience built in Bevy for the web. The MVP spotlights the high-tension loop of weaving an energy thread to slice enemies while chasing a personal best score.

## Core Pillars
1. **Expressive Movement** – The player glides continuously, leaving a fading trail that can be shaped into lethal patterns.
2. **Resilient Combat** – Trails deal damage over time, enemies fight back with health pools, and power-up drops add tactical choices mid-run.
3. **Score Chasing** – Local combo multipliers and run summaries encourage replay and mastery.

## Success Metrics
- **First Playtest Readiness:** Reach a playable web build that agents can launch locally within two minutes by the end of sprint day 3.
- **Performance Target:** Maintain a stable 60 FPS on mid-range laptops (Intel Iris-class GPUs) with at least 25 concurrent enemies.
- **Gameplay Validation:** Enable a full survival run (spawn, trail damage, fail state) with combo scoring that produces non-zero local scores in 90% of automated smoke-test runs.
- **Engagement Signal:** Capture anonymized session telemetry showing a median session length of at least 90 seconds during initial human testing cohort.

## Player Controls
- Mouse pointer lock drives the avatar; the cursor remains hidden while a run is active and is released on pause or defeat.
- Touch drag input is planned for parity but not yet implemented.
- Momentum is constant to maintain flow.

## Gameplay Loop
1. Move to reposition and curve the trail.
2. Weave overlapping paths to prepare slices.
3. Slice rushing enemies with the lingering thread.
4. Survive as long as possible to push local high scores.

## Systems Overview
### Thread System
- Maintain a queue of recent positions (≈2.6 seconds of history).
- Render as a glowing polyline with alpha falloff.
- Tick trail damage against enemy hitboxes using the player’s current damage multiplier.

### Enemy System
- Spawn at screen edges with direction toward the player.
- Increase spawn frequency and speed over time while scaling per-enemy health.
- On defeat, award score, roll for power-up drops, and trigger future VFX/SFX hooks.

### Combo & Score
- Track timestamps of kills to maintain a combo window.
- Apply multipliers to score when kills remain within the window.
- Persist local best score using IndexedDB (still pending implementation).

### Power-up & Buff System
- Defeated enemies can drop hearts (heal one HP), shields (10 seconds of invulnerability), or damage cores (per-run damage bonus).
- Power-ups expire if ignored and display simple sprite icons for now.
- The HUD surfaces current shield time remaining and accumulated damage multiplier.

### Health & Fail State
- Five-hit health pool with shield override: enemy contact removes one health unless the shield timer is active.
- End-of-run summary (duration, enemies defeated, best score comparison) remains a follow-up task.

## Technical Stack
- **Engine:** Bevy 0.14 targeting WASM.
- **Rendering:** `bevy_webgl2` with WASM-compatible plugins.
- **Input:** Pointer tracking mapped to in-game movement each frame.
- **Audio:** WebAudio-backed SFX for slices and hits.
- **Persistence:** IndexedDB via `bevy_web_storage` or custom bindings.

## Content Scope
- Minimalistic visual assets (player glow, thread, simple enemy shapes).
- Two enemy archetypes for pacing variety (baseline & faster charger).
- Color-coded feedback for combo milestones.

## Stretch Goals (post-MVP)
- Upgrade system that unlocks new thread archetypes.
- Overheat mechanic to limit sustained thread density.
- Screen shake, slow-motion dodges, palette unlock progression.
- Online leaderboard or social sharing of personal bests.
