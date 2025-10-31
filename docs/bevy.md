# Bevy Web/WASM AAA UI – Agent Standard (v0.1)

A practical, prescriptive reference for agents building **browser-first, cross‑platform** games with **Bevy (Rust)**, with an emphasis on **UI**, **WASM/WebGPU**, and **production conventions**. This is not a tutorial—treat it as a living engineering standard.

---

## 0) Scope & Targets

* **Engine**: Bevy ≥ 0.17 (Rust stable).
* **Web**: `wasm32-unknown-unknown` target, **WebGPU** browsers (Chrome 113+, Edge 113+, Safari TP+/iOS 16.4+, Firefox 141+), progressive enhancement where feasible.
* **Desktop**: Windows/macOS/Linux via `winit` (default Bevy windowing).
* **Mobile**: iOS/Android as stretch goals; web build must remain first‑class.

**Primary goals**

1. Ship a high‑fidelity 2D/3D game **in the browser** using WebAssembly + WebGPU.
2. Provide a **robust in‑engine UI** that is game‑pad and touch friendly, themeable, and accessible.
3. Maintain a single codebase with platform shims and feature flags.

---

## 1) Architecture Précis (Bevy ECS)

* **ECS**: Entities (IDs with no data), Components (data), Systems (functions running on component queries).
* **Schedules**: `Startup` → `Update` → `PostUpdate` (plus fixed/update sub‑schedules). Use explicit **System Sets** for order and determinism.
* **Plugins**: Group systems/resources. Each feature area in this document maps to its own plugin with a public `Plugin` type.
* **Assets**: `AssetServer` + handles; all asset paths resolve through Bevy’s virtual FS. For web, prefer packed archives or CDN with CORS enabled.

**Mandatory layout**

```
crates/
  core/            # ECS types, resources, events (no platform deps)
  render/          # materials, meshes, pipelines (wgpu-facing)
  ui/              # UI widgets, theming, input focus, a11y hooks
  gameplay/        # states, scenes, rules, data-driven content
  platform/        # features: web/native/mobile; fs/net shims
  launcher/        # bin: main.rs (feature-selects platform)
assets/            # packed or hashed assets
```

---

## 2) Web/WASM Build Pipeline (Required)

**Tooling baseline**

* Rust stable, target `wasm32-unknown-unknown`
* `wasm-bindgen` (via Bevy’s web support and bundlers)
* **Either** `trunk` (preferred) **or** `wasm-server-runner` for local dev
* `wasm-opt` (Binaryen) in CI for size/perf
* Static hosting with correct headers (COOP/COEP if enabling threads later)

**Make targets**

* `make web-dev` → runs local dev server (hot‑reload when feasible).
* `make web-release` → `--release`, `wasm-bindgen`, `wasm-opt -O`, emits `/dist`.
* `make native` → standard `cargo run --release`.

**Index + loader**

* Provide `index.html` that loads the generated `*.js` glue and inserts a `<canvas>` Bevy binds to.
* Add a full‑screen toggle and **graceful fallback** message if WebGPU unsupported.

**Binary size controls**

* Disable debug symbols in release; prefer LZ4/ Brotli compression at edge (CDN).
* Feature‑gate heavy systems; use `panic=abort` on web.

---

## 3) Browser Platform Constraints & Policies

* **Threads**: Default **off** on web; enable only when you control headers (COOP/COEP) and target browsers support **Wasm threads + SharedArrayBuffer**. All systems must remain **single‑thread tolerant**.
* **Timers**: Rely on Bevy’s time; do not presume high‑res timers.
* **File IO**: No direct OS FS. Use `AssetServer` (HTTP fetch), IndexedDB for saves (via a persistence crate), or server APIs.
* **Networking**: WebSockets or WebRTC (Matchbox) only. UDP not available.
* **Audio**: Use Bevy’s audio backends that target WebAudio; avoid blocking calls.
* **Clipboard/IME**: Route via browser events; provide a **virtual keyboard** path on touch.

---

## 4) Rendering on Web (WebGPU via wgpu)

* Target **WebGPU**. Avoid WebGL‑only features.
* Respect browser VRAM/heap limits—**texture formats**: prefer BC/ASTC where supported, provide fallback to compressed RGBA; atlas UI textures to minimize bind changes.
* **Frame budget**: design for 60 FPS; hold **~4–6 ms** for main game logic, **~6–8 ms** for render on mid‑tier laptops.
* **LOD** + **instancing** mandatory for 3D; minimize draw calls; batch UI where possible.
* Post‑processing must be toggleable; default to medium settings on web.

---

## 5) Bevy UI Standard (Game‑grade)

### 5.1 Goals

* **Retained‑mode** UI using `bevy_ui` for in‑game HUD, menus, and overlays.
* **Themeable** (colors, typography, spacing) with runtime switching.
* **Accessible**: Screen‑reader metadata for critical flows; focus order predictable; cursor/hover states; **virtual keyboard** for touch; scalable text.

### 5.2 Layout & Units

* Use `NodeBundle` with **Flexbox** and **CSS Grid** layout styles.
* Prefer **percent** for responsive sizing; `px` only for icons/border widths.
* Establish an 8‑pt spacing scale and `em` based typography scale.

### 5.3 Widgets

* Required: Button, Toggle, Slider, Progress, TextInput, Dropdown, Modal, Tooltip, Toast, Tabs, List, Tree (for debug tools), and Controller‑navigable Menu.
* All widgets expose **Signals/Events** (clicked, changed, focused, submitted) and read/write **Data Models** in ECS resources (unidirectional data flow).

### 5.4 Input & Focus

* Centralize focus with a `UiFocus` resource + system set.
* Navigation maps: **Keyboard** (Arrows/WASD), **Gamepad** (D‑pad/LS, A/B/X/Y), **Pointer** (mouse/touch).
* Ensure hover and active cursors; provide **controller glyphs** in hints.

### 5.5 Theming

* Provide a `Theme` resource (colors, radii, fonts, shadows, paddings) applied via systems that write to style components.
* Support **dark/light** and **high‑contrast** themes; live‑switchable.

### 5.6 Accessibility (a11y)

* Add semantic roles to nodes; ordered focus traversal; readable labels; captions for sound‑only feedback; scalable UI (min 125% fine).
* Provide **screen‑reader** hooks where available and safe on web.

### 5.7 Third‑party UI integrations (when to use)

* **bevy_ui** (default): Best ECS integration, modern layouts, long‑term bet.
* **bevy_egui**: Excellent for **debug tools** / editors; immediate‑mode; rich widgets; web‑ready; keep it sandboxed to dev/overlay layers.
* **Kayak UI / bevy_immediate**: Consider for experimental declarative/IM flows; not mandatory.

---

## 6) Cross‑Platform UX Rules

* **Single code path** for game logic; platform crates only wrap IO and platform peculiarities.
* **Fonts**: bundle explicit font families (no system font assumptions).
* **Input**: Support mouse+kb, touch, and **gamepad** everywhere.
* **Windows**: Enforce min window size; dynamic DPI scaling.

---

## 7) Networking & Persistence (Web‑safe)

* **Transport**: WebSockets for client↔server; optional **WebRTC** for p2p rollback (Matchbox + GGRS).
* **Saves**: JSON/ron + compression, persisted to IndexedDB; sync via API if authenticated.
* **Replay/Telemetry**: Event log stream (ring buffer) with opt‑in upload.

---

## 8) Performance Budget & Profiling

* **Budgets (Web mid‑tier)**:

  * Logic ≤ 6 ms; Render ≤ 8 ms; UI ≤ 2 ms; Net ≤ 1 ms.
* **Assets**: ≤ 25 MB initial; lazy load heavy scenes; texture atlases; mesh LOD.
* **WASM**: Optimize with `wasm-opt -O`, strip panics, compile in release only.
* **Profiling**: Bevy diagnostics + browser Performance panel; instrument systems with spans; capture GPU frame times (WebGPU timestamp queries when supported).

---

## 9) Testing Matrix (release gate)

* **Browsers**: Chrome stable, Edge stable, Firefox ≥ 141, Safari ≥ 17.
* **Devices**: Win11 mid‑laptop iGPU; MacBook M‑series; iPad/iPhone Safari; Android Chrome; at least one low‑end Chromebook.
* **Input**: mouse+kb, touch, Xbox/PS controllers.
* **Net**: loss/latency simulations (±100ms, 1–3% loss) for p2p and client/server.

---

## 10) CI/CD & Hosting

* CI steps: build native + web; run unit/integration tests; `wasm-opt`; emit `/dist`.
* Artifact includes integrity hashes; deploy to static hosting / CDN.
* **Headers**: `Cross-Origin-Opener-Policy: same-origin`, `Cross-Origin-Embedder-Policy: require-corp` (if threading), correct MIME for `.wasm`.

---

## 11) Minimal Code Conventions

* Each crate exposes a `Plugin` that registers all systems/resources.
* System functions are pure, side‑effects via events/resources only.
* No `unsafe` without ADR.
* All `bevy_ui` widgets live under `ui/widgets/*` with stories/examples.

---

## 12) Templates & Snippets

### 12.1 `Cargo.toml` (workspace excerpt)

```toml
[workspace]
resolver = "2"
members = [
  "crates/core",
  "crates/render",
  "crates/ui",
  "crates/gameplay",
  "crates/platform",
  "crates/launcher",
]

[profile.release]
opt-level = "z"         # size‑first for web
lto = true
codegen-units = 1
strip = true
panic = "abort"
```

### 12.2 Launcher `main.rs` (platform select)

```rust
#[cfg(target_arch = "wasm32")]
fn main() { web::run(); }
#[cfg(not(target_arch = "wasm32"))]
fn main() { native::run(); }
```

### 12.3 Web run (outline)

```rust
pub fn run() {
    use bevy::prelude::*;
    App::new()
        .add_plugins((DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window { canvas: Some("#bevy".into()), ..default() }),
            ..default()
        }),))
        .add_plugins((UiPlugin, GameplayPlugin))
        .run();
}
```

### 12.4 UI Theme Resource (sketch)

```rust
#[derive(Resource, Clone)]
pub struct Theme { /* colors, fonts, radii, spacing */ }
```

---

## 13) Decision Records (keep updated)

* **UI stack**: default `bevy_ui`; `bevy_egui` for debug tools only.
* **WebGPU only**: no WebGL code paths.
* **Threads**: off by default on web; future ADR when enabling.

---

## 14) Onboarding Checklist (Agent)

* Install toolchain (Rust stable, wasm target, trunk/runner, wasm‑opt).
* Build native + web examples.
* Review UI widgets + theme stories.
* Run perf scenes; confirm budgets on the testing matrix.

---

## 15) Glossary

* **ECS** (Entity–Component–System): core architecture of Bevy.
* **WASM**: WebAssembly binary format executed in browsers.
* **WebGPU**: modern browser GPU API; Bevy uses it via `wgpu`.

---

### Appendix A – Makefile Targets (example)

```
web-dev:
	rustup target add wasm32-unknown-unknown
	trunk serve --release --open

web-release:
	rustup target add wasm32-unknown-unknown
	trunk build --release
	wasm-opt -O -o dist/app.opt.wasm dist/app.wasm
```

### Appendix B – Accessibility Baseline

* All interactive widgets focusable; visible focus ring.
* Color contrast ≥ WCAG AA.
* Controller and touch parity for all screens.

### Appendix C – Failure Modes to Watch

* WebGPU not available → show compatibility dialog + link to native build.
* IndexedDB quota exceeded → fall back to in‑memory saves + warning.
* Asset CORS blocked → fail fast with friendly diagnostic overlay.
