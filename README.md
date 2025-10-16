# bevy-demo

🎯 Core Concept: “Threadweaver”

Threadweaver is a minimalist arena survival prototype built in Bevy. You are a luminous entity weaving radiant threads through a black void. Every movement leaves a lingering trail that harms enemies, so victory comes from deft positioning, pattern making, and managing the chaos closing in from every edge of the arena.

## Current Gameplay Slice
- **Constant motion:** The player slides toward the locked pointer target at a fixed speed while staying inside the 960×720 arena bounds.
- **Lingering trail weapon:** Trail segments persist for roughly 2.6 seconds and tick damage on enemies that pass over them instead of destroying on contact.
- **Enemy pressure:** Chargers spawn from the arena borders with scaling speed and health, forcing continual motion.
- **Health-driven survival:** The player begins each run with five hit points. Collisions remove one health unless a shield buff is active; defeat comes only when health reaches zero.
- **Power-up drops:** Defeated enemies can drop heart (heal), shield (10s invulnerability), or damage-core (permanent run-long damage bonus) pickups.
- **HUD & arena framing:** Border sprites outline the playfield. The HUD now reports score, best score, combo multiplier, health, damage bonus, shield timer, and run status to keep the player oriented.
- **Cursor lock & pause:** The pointer is hidden and locked to the canvas whenever a run is active. Press `Esc` to pause (releasing the cursor) or `Space` after a defeat to restart.

## Remaining Roadmap Highlights
- Add touch input alongside the mouse pointer-lock flow.
- Layer in particles, audio, and additional enemy archetypes for richer feedback.
- Persist best score and run summaries to IndexedDB.
- Implement end-of-run analytics and polish tasks from the design spec.

🚀 Threadweaver trades projectiles for self-expression through motion. It’s about weaving chaos into order.

📚 Additional Documentation for Agents
- [Threadweaver MVP Specification](docs/spec.md) – detailed product scope and system breakdown.
- [Collaboration Workflow](docs/workflow.md) – branching, review, and testing expectations for agent teams.
- [Automated Build Scaffold](docs/automation.md) – planned CI/CD steps to reach deployable builds quickly.

## Developer Quickstart

### Prerequisites
- Install Rust (1.79 or newer) with `rustup` (the repository includes `rust-toolchain.toml` to pin CI/CD installs).
- Add the `wasm32-unknown-unknown` target for web builds: `rustup target add wasm32-unknown-unknown`.
- Install [`trunk`](https://trunkrs.dev) for local WebAssembly builds: `cargo install --locked trunk`.

### Run the Native Prototype
```bash
cargo run
```

The binary launches the current combat slice: steer the avatar with the mouse (cursor lock enabled), weave trails to damage enemies, harvest power-ups, and survive as long as possible. Press `Esc` to pause/unlock the cursor and `Space` after a defeat to reset the run.

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

This spins up a hot-reloading dev server at `http://localhost:8080`, compiling the Bevy app to WebAssembly and mounting it on the `<canvas id="bevy-canvas">` element defined in `index.html`. Dev builds now run in Cargo's default debug mode so incremental compilation keeps iteration fast.

When you need a production-quality artifact, opt into the optimized pipeline manually:

```bash
trunk build --release
```

### Deploy to Vercel

The project ships with `vercel.json` and a scripted build (`scripts/vercel-build.sh`) so Vercel can provision Rust, compile the WASM bundle, and publish the static output under `dist/`.

1. Authenticate and link the repository (`vercel link`).
2. Trigger a build (`vercel --prod`). Vercel will execute the build script, producing the optimized WASM + JS glue code via `trunk build --release`.
3. Share the generated preview URL for remote playtesting.

Refer to `docs/status.md` for the latest deployment status snapshot.
