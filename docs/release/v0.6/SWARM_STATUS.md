v0.6 Alpha Release Swarm — Status
Branch: tranche/6 (from origin/develop @ 5b1bad5)
Source of truth: docs/release/v0.6/release-swarm.md

(a) Happening now
------------------
orchestrator (lead)  Sonnet   spawning frontend/backend/qa, building wave-1 task list
frontend              Sonnet   spawning now, wave 1 assigned
backend               Sonnet   spawning now, wave 1 assigned
qa                    Sonnet   spawning now, wave 1 assigned

(b) Happened
------------
- tranche/6 cut from origin/develop @ 5b1bad5, docs committed (43f8d46), pushed.
- Launch-readiness review fixed 7 blocker-class defects in release-swarm.md
  (stack mismatch, wrong ownership paths, merge authority, version-bump
  ownership, scope sizing, observer status, doctrine contradiction).
- Wired-integration ceremony waiver recorded as operator override in
  docs/release/v0.6/risks-and-open-questions.md.
- Swarm launched.

(c) On deck (wave 1 — 5 tasks per teammate)
--------------------------------------------
backend:
  1. Version bump 0.5.99 -> 0.6.0 (package.json, tauri.conf.json, Cargo.toml,
     buildVersionTriple.test.ts:44-47 anchor). No dependency, do first.
  2. Skill-point allocation persistence: Tauri command + rules_core hookup
     (currently in-memory only, lost on close).
  3. Level-up HP + choices persistence: Tauri command (LevelUpDialog.onAccept
     is an empty closure today) -- coordinate command name with frontend.
  4. Multiclass BAB/save stacking calc audit + fix, TDD (failing test first).
  5. Equipment AC bonus / carry capacity audit vs PCGen corpus (equipment
     per-item stats currently always default()).

frontend:
  1. Wire DetailsPanel (already written, never rendered) into Bio/Details tab.
  2. Build Feat picker (net-new; model on ItemPickerModal pattern), wire into
     Feats tab.
  3. Build Money panel (net-new; no money/wealth component exists today).
  4. Wire SkillAllocationDialog to backend's persistence command -- blocked on
     backend task 2's command name, coordinate via SendMessage.
  5. Wire LevelUpDialog.onAccept to backend's persistence command -- blocked
     on backend task 3's command name, coordinate via SendMessage.

qa:
  1. Run existing PCGen smoke test (tests/pcgen_runner_smoke.rs) end-to-end,
     record baseline pass/fail.
  2. Build gap list: alpha-bar calc surfaces (Sec 1.4) vs current tests/
     catalogue coverage.
  3. Write failing multiclass BAB/save stacking test ahead of backend task 4
     (or adopt backend's once delivered) -- coordinate via SendMessage.
  4. Draft SWARM_REPORT.md skeleton with four-check audit section stubbed.
  5. Ongoing: file PCGen-divergence defects as frontend/backend land work.

blocked-by notes:
  frontend#4 <- backend#2 (command name)
  frontend#5 <- backend#3 (command name)
  qa#3 coordinates with backend#4 (shared test, not a hard block)
