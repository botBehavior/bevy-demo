# Build & Deployment Status – Threadweaver MVP

_Last updated: 2025-10-16_

## Deployment Readiness
- ✅ **WebAssembly build path configured** via Trunk + `index.html`; run `trunk build --release` to emit the static `dist/` bundle.
- ✅ **Vercel automation scripted** (`scripts/vercel-build.sh`) to install Rust, add the WASM target, and invoke Trunk in release mode.
- 🚧 **Production URL** pending first Vercel deployment run; trigger `vercel --prod` once repository is connected.

## Gameplay Slice Health
- ✅ Native build (`cargo run`) exercises the move → weave → slice → survive loop with restart + scoring.
- ✅ Web build uses the same code path with canvas mounting and WASM panic hooks for better diagnostics.
- 🔄 Upcoming polish: audio cues, responsive HUD layout, particle effects, and balancing per [docs/spec.md](spec.md).

## Success Metrics Checkpoint
- **30 FPS on mid-tier laptops** – native build sustains >60 FPS; validate on Web deployment during testing.
- **First-play survival >45 seconds median** – collect telemetry from remote testers via manual survey in next milestone.
- **Combo >3 achieved in 80% of runs** – local instrumentation ready; add analytics hooks post Vercel launch.

## Next Steps
1. Connect repository to Vercel and trigger the first production deployment for remote playtests.
2. Capture gameplay feedback and performance metrics from remote testers.
3. Prioritize polish tasks (moment-to-moment feedback, mobile touch tuning, audio pass) ahead of public reveal.
