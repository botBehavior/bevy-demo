# bevy-demo

🎯 Core Concept: “Threadweaver”

You are a luminous entity gliding through an endless black void.
Everywhere you move, you leave a trailing thread of energy.
Enemies rush at you from all directions — and anything that touches your thread is sliced apart.

Your goal: survive as long as possible, carve beautiful patterns, and build up combos through precise movement and risk.

🧩 Gameplay Loop (MVP)
1. Move → weave → slice → survive

Control with touch drag or mouse movement — you’re always gliding.

Your tail leaves a thread that persists for 2–3 seconds, letting you weave patterns.

Enemies spawn from screen edges and rush toward you.

If an enemy touches your thread, it dies — satisfying pop + particle burst.

If it hits you, you lose a life (or the run ends).

Consecutive kills within short windows create a combo multiplier that feeds into your local score.

The MVP run ends when you’re hit; the goal is to survive longer and chase a higher personal best stored locally.

🧠 Dopamine Architecture
Reward Tier	Event	Sensation	Design Hook
Instant (0–1s)	Thread slices enemy	Flash, pop, “schwip” audio	micro reward loop
Short (5–10s)	Combo multiplier / streak achieved	color + particle burst	“keep moving!” tension
Mid (1–2 min)	Stretch: evolution / new thread type	new color pattern + SFX shift	transformation dopamine
Long (session)	Personal best (local) / future palette glow	mastery loop
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

3. Combo & Score System

Tracks time between kills.

Decay timer resets on each kill.

Multiplier affects particle density + sound pitch and drives the local score tracker.

🧱 MVP Scope (1-week prototype)
Day	Milestone
1	Bevy-WASM setup → render player dot + trail
2	Enemy spawns + simple homing movement
3	Trail collision detection + kill effect
4	Combo system + streak feedback
5	Score presentation + basic HUD (combo meter, best run stored locally)
6	Sound & polish (WebAudio pop/swish)
7	Upload to GitHub Pages (COOP/COEP headers) for testing
🪄 Stretch goals

- Upgrade draft moments (thread length, damage radius, persistence, new thread archetypes).
- Overheat/energy management layer to reward controlled pacing.
- Screen shake when combo hits 10x.
- “Slow motion burst” when narrowly dodging.
- Color palette unlocks after each 3-minute run.
- Procedural “melodic” sound layer (frequency up with combo).
- High-score web leaderboard (local or optional backend).

🚀 The dopamine thesis

Threadweaver trades projectiles for self-expression through movement.
It’s not about shooting — it’s about weaving chaos into order.
Fast, elegant, minimalist — and it can live entirely in a browser tab.

📚 Additional Documentation for Agents

- [Threadweaver MVP Specification](docs/spec.md) – detailed product scope and system breakdown.
- [Collaboration Workflow](docs/workflow.md) – branching, review, and testing expectations for agent teams.
- [Automated Build Scaffold](docs/automation.md) – planned CI/CD steps to reach deployable builds quickly.

- Upgrade draft moments (thread length, damage radius, persistence, new thread archetypes).
- Overheat/energy management layer to reward controlled pacing.
- Screen shake when combo hits 10x.
- “Slow motion burst” when narrowly dodging.
- Color palette unlocks after each 3-minute run.
- Procedural “melodic” sound layer (frequency up with combo).
- High-score web leaderboard (local or optional backend).

## Developer Quickstart

### Prerequisites
- Install Rust (1.79 or newer) with `rustup` (the repository includes `rust-toolchain.toml` to pin CI/CD installs).
- Add the `wasm32-unknown-unknown` target for web builds: `rustup target add wasm32-unknown-unknown`.
- Install [`trunk`](https://trunkrs.dev) for local WebAssembly builds: `cargo install --locked trunk`.

### Run the Native Prototype
```bash
cargo run
```

The binary launches a playable slice of the MVP loop: steer the glowing avatar with your mouse, weave trails, slice enemies, and chase a local high score. Press `Space` after a crash to restart a run instantly.

### Basic Quality Checks
```bash
cargo fmt
cargo check
```

These commands align with the workflow expectations and should pass before opening a pull request.

### Build the Web Client Locally
```bash
trunk serve
```

This spins up a hot-reloading dev server at `http://localhost:8080`, compiling the Bevy app to WebAssembly and mounting it on the `<canvas id="bevy-canvas">` element defined in `index.html`.

### Deploy to Vercel

The project ships with `vercel.json` and a scripted build (`scripts/vercel-build.sh`) so Vercel can provision Rust, compile the WASM bundle, and publish the static output under `dist/`.

1. Authenticate and link the repository (`vercel link`).
2. Trigger a build (`vercel --prod`). Vercel will execute the build script, producing the optimized WASM + JS glue code via `trunk build --release`.
3. Share the generated preview URL for remote playtesting.

Refer to `docs/status.md` for the latest deployment status snapshot.
Threadweaver trades projectiles for self-expression through movement.
It’s not about shooting — it’s about weaving chaos into order.
Fast, elegant, minimalist — and it can live entirely in a browser tab.

📚 Additional Documentation for Agents

- [Threadweaver MVP Specification](docs/spec.md) – detailed product scope and system breakdown.
- [Collaboration Workflow](docs/workflow.md) – branching, review, and testing expectations for agent teams.
- [Automated Build Scaffold](docs/automation.md) – planned CI/CD steps to reach deployable builds quickly.
