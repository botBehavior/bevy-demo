# Threadweaver MVP Specification

## Vision
Threadweaver is a minimalist arena survival experience built in Bevy for the web. The MVP spotlights the high-tension loop of weaving an energy thread to slice enemies while chasing a personal best score.

## Core Pillars
1. **Expressive Movement** – The player glides continuously, leaving a fading trail that can be shaped into lethal patterns.
2. **Reactive Combat** – Enemies charge from the arena edges and can be destroyed by intersecting the thread.
3. **Score Chasing** – Local combo multipliers and run summaries encourage replay and mastery.

## Player Controls
- Mouse or touch drag to steer the avatar.
- No button inputs; momentum is constant to maintain flow.

## Gameplay Loop
1. Move to reposition and curve the trail.
2. Weave overlapping paths to prepare slices.
3. Slice rushing enemies with the lingering thread.
4. Survive as long as possible to push local high scores.

## Systems Overview
### Thread System
- Maintain a queue of recent positions (2–3 seconds of history).
- Render as a glowing polyline with alpha falloff.
- Check collisions between trail segments and enemy hitboxes.

### Enemy System
- Spawn at screen edges with direction toward the player.
- Increase spawn frequency and speed over time.
- Destroyed enemies trigger particles, audio, and scoring events.

### Combo & Score
- Track timestamps of kills to maintain a combo window.
- Apply multipliers to score when kills remain within the window.
- Persist local best score using IndexedDB.

### Health & Fail State
- Single-hit defeat for MVP.
- End-of-run summary: duration, enemies defeated, best score comparison.

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
