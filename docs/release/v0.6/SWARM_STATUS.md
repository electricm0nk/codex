v0.6 Alpha Release Swarm — Status
Branch: tranche/6 (from origin/develop @ 5b1bad5)
Source of truth: docs/release/v0.6/release-swarm.md

(a) Happening now
------------------
orchestrator (lead)  Sonnet   wave 1 in flight, watching for blockers
frontend              Sonnet   committing TABS cleanup fix; auditing 4 remaining
                                stub tabs (Defense/Pets/Actions/Overrides) for
                                already-wireable-without-backend work; otherwise
                                idle on backend's 5 command deliverables
backend               Sonnet   REPRIORITIZED: version bump, then skill-persist,
                                level-up-persist, bio command, feat exposure,
                                money command (in that order) -- all 5 unblock
                                frontend; BAB/save stacking + equipment AC
                                pushed to back of wave 1
qa                    Sonnet   wave 1: PCGen smoke baseline, coverage gap list

(b) Happened
------------
- tranche/6 cut from origin/develop @ 5b1bad5, docs committed (43f8d46), pushed.
- Launch-readiness review fixed 7 blocker-class defects in release-swarm.md
  (stack mismatch, wrong ownership paths, merge authority, version-bump
  ownership, scope sizing, observer status, doctrine contradiction).
- Wired-integration ceremony waiver recorded as operator override in
  docs/release/v0.6/risks-and-open-questions.md.
- Swarm launched (frontend/backend/qa spawned).
- Routing correction: lead must be reached via SendMessage "main", not
  "orchestrator" (wrong name in initial teammate briefs) -- corrected with
  all three teammates.
- Frontend investigation corrected two wave-1 task assumptions:
  * DetailsPanel already renders unconditionally in CharacterSheet.tsx (not
    an unwired-component gap). Real gap: bio fields are session-local
    useState, zero backend persistence, no bio field in ChosenCharacterState
    or character_input.rs. Tab-switch "Details"/"Bio" entries are dead
    duplicates resolving to a generic coming-soon placeholder -- removing
    them is itself a real fix for the no-stub-surface bar item.
  * A 185-record CRB feat catalog already exists server-side
    (src/rules_core/rules_tables/crb/feats.rs) with zero Tauri exposure --
    feat-picker task reframed as "expose + consume", not build-from-scratch.
  * Money/currency has no schema field anywhere in the engine -- only
    per-item cost_gp pricing exists. Confirmed biggest lift of the three.
  Frontend sent backend a consolidated ask (bio schema+command, feat
  list+add-selection command, money field+command) directly.
- BLOCKER FILED: frontend reported all 5 wave-1 tasks gate on backend work
  that doesn't exist yet (verified against schema/corpus before escalating --
  correctly refused to build throwaway UI shells against nonexistent
  commands, per no-stub doctrine). Landed one real fix in the meantime:
  removed dead Details/Bio TABS entries (typecheck clean, commit pending).
  Lead response: reprioritized backend's queue to land all 5 frontend-
  blocking commands before calc-accuracy work (BAB/save, equipment AC),
  and assigned frontend a non-blocking audit of the 4 remaining stub tabs
  (Defense/Pets/Actions/Overrides) for anything already wireable today.

(c) On deck (wave 1 — 5 tasks per teammate)
--------------------------------------------
backend (REPRIORITIZED -- all 5 frontend-unblocking commands before calc work):
  1. Version bump 0.5.99 -> 0.6.0 (package.json, tauri.conf.json, Cargo.toml,
     buildVersionTriple.test.ts:44-47 anchor). No dependency, do first.
  2. Skill-point allocation persistence: Tauri command + rules_core hookup.
  3. Level-up HP + choices persistence: Tauri command.
  4. Bio schema field + persistence command (frontend ask).
  5. Feat exposure: list_feats + add_feat_selection against existing
     185-record CRB catalog, rules_tables/crb/feats.rs (frontend ask).
  6. Money/currency schema field + command -- no existing schema slot,
     biggest lift (frontend ask).
  -- pushed to wave 2: multiclass BAB/save stacking (TDD), equipment AC /
     carry capacity audit vs PCGen corpus (neither blocks another teammate).

frontend (revised after investigation, see Happened log):
  1. Remove dead Details/Bio tab-switch entries (real coming-soon fix); wire
     bio fields to backend persistence once backend's bio command lands.
  2. Feat picker: expose+consume existing 185-record CRB catalog via
     ItemPickerModal pattern, wire once backend's list_feats/add_feat lands.
  3. Money panel: build shell now; wire once backend's money schema+command
     lands (biggest lift of the three, don't block other work on it).
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
