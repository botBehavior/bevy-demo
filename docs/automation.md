# Automated Build Scaffold

These notes outline the initial automation plan so agents can stand up CI/CD quickly and reliably.

## Continuous Integration Pipeline
1. **Install Toolchain**
   - Use the `actions-rs/toolchain` action to install Rust stable with the `wasm32-unknown-unknown` target.
   - Cache Cargo builds using `actions/cache` keyed by `Cargo.lock` and the target triple.
2. **Lint & Format**
   - Run `cargo fmt -- --check` to enforce formatting.
   - Run `cargo clippy --all-targets -- -D warnings` to ensure lint cleanliness.
3. **Test**
   - Execute `cargo test --all-targets` for native verification.
   - Build for WASM using `cargo build --release --target wasm32-unknown-unknown` to catch web regressions.
4. **Artifacts**
   - Upload the generated WASM bundle and assets as workflow artifacts for manual testing if available.

## Continuous Deployment Outline
- After CI success on `main`, trigger a deployment job that:
  1. Runs `wasm-bindgen` (via Trunk or wasm-bindgen-cli) to produce web-ready assets.
  2. Publishes the `dist/` directory to the `gh-pages` branch with `peaceiris/actions-gh-pages`.
  3. Invalidates cached assets by bumping a build timestamp in metadata.

## Local Automation Scripts
- Provide a `Makefile` or `justfile` with tasks:
  - `setup`: installs toolchain, wasm target, and Trunk.
  - `check`: runs fmt, clippy, and tests.
  - `wasm`: builds and serves the WASM target for local testing.
- Future agents should extend scripts rather than creating ad-hoc commands.

## Monitoring & Alerts
- Configure GitHub branch protection requiring CI success before merge.
- Integrate build notifications with the shared communication channel (e.g., Slack webhook).
- Document incident response steps in `docs/troubleshooting.md` once the pipeline is live.

## Next Steps
- Draft the GitHub Actions workflow (`.github/workflows/ci.yml`).
- Add deployment secrets for GitHub Pages (if needed).
- Validate the pipeline end-to-end with a dry run before inviting external testers.
