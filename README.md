# bevy-demo

🎯 Core Concept: “Threadweaver”

You are a luminous entity gliding through an endless black void.
Everywhere you move, you leave a trailing thread of energy.
Enemies rush at you from all directions — and anything that touches your thread is sliced apart.

Your goal: survive as long as possible, carve beautiful patterns, and build up combos through precise movement and risk.

🧩 Gameplay Loop
1. Move

Control with touch drag or mouse movement — you’re always gliding.

Your tail leaves a thread that persists for 2–3 seconds.

2. Slice

Enemies spawn from screen edges and rush toward you.

If an enemy touches your thread, it dies — satisfying pop + particle burst.

If it hits you, you lose a life (or the run ends).

3. Combo

Consecutive kills within short windows create a combo multiplier.

Threads glow brighter with higher combos (immediate visual payoff).

4. Level Up

Every few seconds of survival or after X kills, you pick one upgrade:

⚡ Thread Length ↑

💥 Thread Damage Radius ↑

🌀 Thread Persistence ↑

🔮 New Thread Type: Pulse, Explosive, Chain

5. Overload

If you keep moving too fast, your energy meter overheats.
You must slow down briefly or risk snapping your thread, resetting your combo — a light “risk management” mechanic.

🧠 Dopamine Architecture
Reward Tier	Event	Sensation	Design Hook
Instant (0–1s)	Thread slices enemy	Flash, pop, “schwip” audio	micro reward loop
Short (5–10s)	Combo multiplier / streak achieved	color + particle burst	“keep moving!” tension
Mid (1–2 min)	Evolution / new thread type	new color pattern + SFX shift	transformation dopamine
Long (session)	Personal best / unlocks	scoreboard glow, unlock palette	mastery loop
🎨 Visual Design (Prototype)

Super minimal:

Background: pure black.

Player: small glowing dot (white or blue).

Thread: fading neon trail, 2–3 px wide.

Enemies: colored triangles or squares.

Particles: short-lived sparks on kills.

Even in the prototype, this style will feel electric if timing, SFX, and easing are tuned right.

🧰 Technical Design (WASM)
Layer	Tool	Notes
Engine	Bevy 0.14 (Rust)	ECS, easy 2D sprite rendering
Physics	Simple AABB math	No physics engine needed
Rendering	bevy_winit + bevy_webgl2	compiles cleanly to WASM
Audio	WebAudio (web-sys)	slicing “swish” + pop sounds
Input	Mouse or touch position tracking	smooth interpolation each frame
Persistence	IndexedDB	best score, unlocked colors
FPS Target	60fps, fixed timestep	mobile-first fluidity
🧮 Core Systems Breakdown
1. Thread System

Stores last N positions (VecDeque).

Each frame, draw a polyline (neon glow, fade alpha over time).

Collision test: any enemy intersecting segment dies.

2. Enemy System

Enemies spawn at screen edges with homing behavior.

Speed increases over time.

Spawn rate accelerates logarithmically.

3. Combo System

Tracks time between kills.

Decay timer resets on each kill.

Multiplier affects particle density + sound pitch.

4. Upgrade System

Randomly present 2–3 upgrade cards every 30s or milestone.

Each modifies thread or energy parameters.

5. Overheat System

Bar fills as you move continuously.

Cooling = standing still (risk vs. reward).

Visual indicator: glow flicker, crackle.

🧱 MVP Scope (1-week prototype)
Day	Milestone
1	Bevy-WASM setup → render player dot + trail
2	Enemy spawns + simple homing movement
3	Trail collision detection + kill effect
4	Combo system + streak feedback
5	Overheat mechanic + basic HUD
6	Sound & polish (WebAudio pop/swish)
7	Upload to GitHub Pages (COOP/COEP headers) for testing
🪄 Optional polish (phase two)

Screen shake when combo hits 10x.

“Slow motion burst” when narrowly dodging.

Color palette unlocks after each 3-minute run.

Procedural “melodic” sound layer (frequency up with combo).

High-score web leaderboard (local or optional backend).

🚀 The dopamine thesis

Threadweaver trades projectiles for self-expression through movement.
It’s not about shooting — it’s about weaving chaos into order.
Fast, elegant, minimalist — and it can live entirely in a browser tab.