# Collaboration Workflow for Agents

This repository is designed for a network of collaborating agents to ship the Threadweaver MVP swiftly and safely. Follow the guidelines below to maintain velocity while preserving quality.

## Branching Strategy
- Create feature branches from `main` with the format `feature/<short-topic>` or `chore/<task>`.
- Keep branches focused on a single logical change set.
- Rebase onto `main` before opening a pull request to avoid merge conflicts.

## Commit Discipline
- Write descriptive commit messages summarizing the change.
- Include references to the files or systems touched in the commit body when necessary.
- Avoid committing generated artifacts; rely on build steps to reproduce outputs.

## Pull Request Checklist
1. Ensure the README, spec, or relevant docs are updated when behavior changes.
2. Add automated tests or lightweight validation as features stabilize.
3. Provide a summary of changes and verification steps in the PR description.
4. Request review from another agent before merging.

## Code Review Expectations
- Reviewers validate functionality, readability, and alignment with the MVP spec.
- Leave actionable comments; suggest concrete improvements.
- Confirm all acceptance criteria are satisfied before approving.

## Testing & Quality Gates
- Run `cargo fmt`, `cargo clippy`, and `cargo test` locally or via CI prior to PR submission.
- For WASM builds, execute `cargo build --target wasm32-unknown-unknown` to ensure web compatibility.
- Document any skipped checks with justification in the PR.

## Release & Deployment
- Use GitHub Actions (to be configured) for continuous integration.
- Tag release candidates with `rc-<date>` after automated checks pass.
- Publish successful builds to GitHub Pages for human playtesting.

## Task Coordination
- Track tasks in the project board (to be created) with clear owners and due dates.
- Break large efforts into sub-tasks to enable parallelism across agents.
- Maintain a daily async stand-up note capturing progress, blockers, and next steps.

## Knowledge Sharing
- Update documentation in the `docs/` folder when systems evolve.
- Capture debugging steps or known issues in `docs/troubleshooting.md` (create if absent).
- Prefer templates and checklists to reduce cognitive load for future agents.
