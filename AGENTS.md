# AGENTS.md – Pattern Forge Onboarding & Operating Guide
_Last updated: 2025-06-21_

---

## 1. Mission Statement 🚀
Pattern Forge is a **Bevy 0.16** sandbox for rapid prototyping of game-play, AI, and tooling experiments.  
Our north star is *“maximum modularity & observability”*: every feature lives in its own plugin, can be toggled without side-effects, and is easy to inspect at runtime.

---

## 2. Quick Start Checklist ✅
1. **Rust toolchain**: `rustup default stable` (nightly only if ticket says so).  
2. **Run Dev Build**:  
   ```bash
   cargo run            # debug build, hot-reload on
Run Release (inspector off):

bash
Copy
Edit
cargo run --release
Open World Inspector (debug only): press <kbd>F1</kbd>.

Branch Workflow:

bash
Copy
Edit
git checkout -b <ticket-id>-short-desc
3. Repo Layout 🗂️ (high-level)
Path	Purpose
src/	Game code (all Rust).
src/core/	Global resources, events, diagnostics (CorePlugin).
src/player/, enemy/, ui/, editor/	Feature plugins (one folder per feature).
assets/	Art, audio, config files (RON/TOML/JSON).
docs/	Long-form design notes & experiment logs.
scripts/	Dev helpers (dev.sh, CI smoke tests, etc.).
AGENTS.md	You are here – read before coding.

Detailed design rationale: docs/Designing a Modular Bevy Base Layer.md.

4. Architectural Principles 🧩
Plugin-Per-Feature

Each folder has components.rs, systems.rs, plugin.rs, mod.rs.

Add/remove the feature by adding/removing its plugin in main.rs.

App State Machine (src/states.rs)

MainMenu, Playing, EditorMode, TestMode.

Gate systems with OnEnter, OnExit, or in_state().

Event Bus

Use Bevy #[derive(Event)] structs for cross-system messages (e.g. DamageEvent).

Never call another feature’s systems directly. Communicate via events.

Config Resources & Hot-Reload

Store tunables in assets/config/*.ron → load into GameConfig resource at startup.

AssetPlugin { watch_for_changes: true } is ON in debug; edit files and watch values change live.

Observability

bevy_inspector_egui for live entity/resource tweaking (debug builds only).

FrameTimeDiagnosticsPlugin + LogDiagnosticsPlugin always on in debug.

5. Coding Standards ✍️
Topic	Rule
Formatting	cargo fmt before each commit.
Lint	cargo clippy --no-deps must be warning-free for code you touch.
Commits	<emoji> <type>(<scope>): <subject> (#ticket-id)<br/>Example: ✨ feat(player): add WASD movement (#P4-2)
Tests	If ticket defines unit tests, place them in the same module (#[cfg(test)]).
Docs	Public items need rust-docs; each plugin gets a brief module-level comment.

6. Ticket Lifecycle 🗒️
Read ticket carefully – scope creep is the enemy.

Update/consult this file if a ticket changes conventions.

Open PR against main; link ticket & mention reviewers.

CI passes (cargo check, Clippy) ⇒ request review.

Merge Rules: at least one senior review + green CI.

7. Common Dev Commands 🔧
Goal	Command
Run with auto-rebuild	cargo watch -x run
List system schedule	RUST_LOG=bevy_mod_debugdump=info cargo run (requires bevy_mod_debugdump)
Clean build cache	cargo clean
Re-gen lockfile	cargo generate-lockfile (don’t hand-edit Cargo.lock)

8. Asset Conventions 🎨
Place textures in assets/textures/, audio in assets/audio/.

Naming: lowercase, hyphen-separated (goblin-idle.png).

Keep PSD/RAW sources out of repo – link via cloud storage if needed.

Use bevy_asset_loader collections when you need state-gated loading.

9. Experiment Notebook 📓
We log prototype results in docs/experiments/YYYY-MM-DD-title.md.
When you run a local experiment, create a quick note: context, change, result, next ideas.

10. Asking for Help 🤝
If requirements are unclear:

Comment on the ticket – clarity beats guessing.

Tag @maintainers in GitHub or drop a note in #pattern-forge-dev.

11. Non-Negotiables 🚫
Do not commit secrets or proprietary art.

Never merge with failing CI.

Avoid global singletons outside Bevy resources.

Keep main.rs minimal – all logic belongs in plugins.

Welcome to the Forge – experiment boldly, but leave the codebase cleaner than you found it. 🔥

markdown
Copy
Edit

---

### How to Roll This Out

1. Save the file directly as `AGENTS.md` in the repo root.  
2. Link to it from your ticket templates (`.github/ISSUE_TEMPLATE/...`) so every PR references it.  
3. Update the “Last updated” line whenever conventions change.

Feel free to tweak wording or add project-specific Slack/Discord channels, but this draft covers:
- **Context** (mission, architecture)
- **Standards** (branching, commits, linting)
- **How-to-run** instructions
- **Core conventions** your tickets already rely on.

Hand it to any junior dev or AI agent—they’ll have the map they need.