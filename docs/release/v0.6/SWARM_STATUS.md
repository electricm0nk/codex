v0.6 Alpha Release Swarm — Status
Branch: tranche/6 (from origin/develop @ 5b1bad5)
Source of truth: docs/release/v0.6/release-swarm.md

(a) Happening now
------------------
orchestrator (lead)  Sonnet   wave 1 in flight, watching for blockers
frontend              Sonnet   tasks 4 AND 5 DONE, both live-verified; idle,
                                tasks 1-3 (bio/feat/money) still wait on
                                backend, RESUMED (see below)
backend               Sonnet   money conversion DONE+pushed (67490ac,
                                176/176 lib, 170/170 desktop); ALL 3 frontend
                                -blocking commands landed. HOLDING at
                                checkpoint per lead instruction, not self-
                                selecting wave-2 work.
qa                    Sonnet   idle, watching for PCGen-divergent findings

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
- QA closed wave-1 tasks 1/2/4: PCGen baseline clean (pcgen_runner_smoke
  2/2, sd26_pcgen_runner 6/6); gap-list survey found carry capacity,
  encumbrance, and money conversion have ZERO production implementation
  in src/rules_core (not test gaps -- missing calculations), corroborating
  frontend's money-schema finding; SWARM_REPORT.md skeleton drafted with
  full per-calculation coverage table (commit 9ffe32f).
- Lead ruling: "durability" (bar item 4) = character survivability display
  (max/current/temp HP, nonlethal, dying/death thresholds), distinct from
  "level-up hit points" (the level-up increment calc). No item-hardness
  system in scope. Recorded in risks-and-open-questions.md item 4.
- QA follow-up: the money/carry-capacity/encumbrance gap is TWO layers --
  rules_core has no calc, AND the PCGen comparator (pcgen_runner.rs,
  pcgen-normalize-output.py) doesn't extract those fields from PCGen's
  export either. QA correctly stopped short of speculative design work;
  folded into backend's wave-2 scope as its own subtask.
- Backend landed skill-point Tauri command work in progress; version bump
  commit (0c614d9) triggered buildLabelFixtureFreshness.test.ts (stale
  "Codex 0.5.98-test" literal). Frontend fixed it plus 4 more dependent
  tests backend's flagged list missed (shared makeSurface.ts fixture) --
  full suite 62/62 green, typecheck clean. Pushed as 743c358, which also
  wired the Actions tab for real (characterProgression.ts's existing
  class-features data, zero backend dependency).
- Frontend's stub-tab audit (Defense/Pets/Actions/Overrides) closed:
  Actions now real. Defense's DR data exists server-side (pilot_compute.rs
  explanations) but isn't exposed through any DTO -- added to backend
  backlog, non-blocking, not part of the alpha-bar calc list. Overrides
  has zero hits anywhere in the character-build engine -- parked, unclear
  purpose, not inventing scope for it. Pets (animal companions) confirmed
  a genuine pre-existing non-goal ("named-but-unproven" in pilot_compute.rs,
  no stat-block engine) -- see new open question below on alpha-bar fit.
- QA's follow-up survey: durability (per lead's ruling) has almost no
  production surface either -- only an isolated level-1 fighter HP value,
  no aggregate max_hp/current_hp/temp_hp/dying-death fields anywhere. Same
  shape of gap as carry-capacity/money -- wave 2 is now 4 calc gaps, not 3.
  QA also located a real local PCGen checkout at
  /home/ubuntu/workspace/repos/pcgen/system/gameModes/Pathfinder/load.lst
  and pulled carry-capacity/encumbrance tables directly from it (not
  reconstructed from memory); durability thresholds from SRD; money-
  conversion ratios standard but not PCGen-source-verified (flagged).
  Starting-wealth-by-class formula not found in the class LST file QA
  checked -- correctly flagged unresolved rather than guessed. Spec
  appendix landed in SWARM_REPORT.md (abec13b).
- NEW OPEN QUESTION filed (risks-and-open-questions.md item 5): does the
  missing Pets/animal-companion mechanic create a real gap against alpha
  bar item 2 ("any class... from the four primary books") for Druid/
  Ranger/Hunter/Cavalier? Not a literal blocker per the bar's text, but
  flagged for explicit operator sign-off rather than silently dropped.
- Backend closed wave-1 task 2: set_skill_allocations command (e0a0bda),
  real TDD (5 new tests incl. a golden-path proof it's a true replace not
  a no-op), verification output posted. IMPORTANT FINDING, folded into
  risks item 1: the compute engine's Computed path only accepts ONE exact
  hardcoded posture (Climb/Intimidate/Swim rank 1, chain shirt equipped,
  nothing else) -- any other tester choice returns Blocked with
  diagnostics. Pre-existing, not introduced by the swarm, but sharper than
  the known "Fighter 1-3 only" framing and likely more central to hitting
  the alpha bar than individual picker/persistence work. Told backend to
  finish the frontend-unblocking queue first, then bring a scoped proposal
  before starting wave-2 calc-accuracy work on the assumption that
  persistence alone unlocks real play.
- Backend closed task 3: level_up_character extended (not a new command --
  correct call, one atomic mutation beats sequential partial-apply calls)
  to accept additionalChoices + optional skillAllocations, backward
  compatible via serde defaults. Surfaced a real pre-existing wire-format
  constraint (choiceSetId/selectionId colon-count rules enforced by
  SavedCharacterStore) without expanding scope to relax it. 151/151 tests
  green. CAUGHT: backend then said "moving to task 4, multiclass BAB/save
  stacking" -- that's the ORIGINAL wave-1 numbering, not the reprioritized
  queue. Corrected: next is bio schema+command (reprioritized task 3 of 5),
  frontend is still fully blocked on bio/feat/money, nothing changed that.
- Frontend closed task 4 (75200fc): SkillAllocationDialog wired to real
  set_skill_allocations, LIVE-VERIFIED against a real dev build (not just
  typecheck) -- confirmed both the success path (rev.1->rev.2 persisted)
  and the honest-Blocked path (rejected mutation correctly never persisted,
  engine's real diagnostic surfaced in UI). Correctly excluded backend's
  in-flight pilot_compute.rs from the commit. Picked up task 5 next
  (LevelUpDialog wiring) on its own initiative -- correct, doesn't need
  bio/feat/money, real available work.
- Frontend closed task 5 (e8e4597): LevelUpDialog wired through the real
  extended level_up_character request. Live-verified: Fighter 1->2, HP
  12->20, BAB/saves recomputed live, on-disk revision bumped rev.2->rev.3,
  choice:level_2_hit_points:hp:average persisted verbatim. Two honest scope
  calls, both named rather than silently dropped: no feat-pick-at-level-up
  UI (no list_feats yet, would be a fake dropdown = stub doctrine
  violation), and skillAllocations left omitted from the level-up call
  since the separate SkillAllocationDialog (task 4) already covers it --
  avoids double-implementing the same concern in two places. Frontend has
  now closed tasks 4 and 5 of wave-1; 1-3 (bio/feat/money) remain blocked
  on backend, who is currently paused (see below).
- SECOND QUEUE DEVIATION (more serious than the first): found backend's
  uncommitted pilot_compute.rs diff (325 lines) is real multiclass/BAB-save
  chassis-widening work (Fighter/Wizard -> all 11 CRB classes), doc-commented
  "v0.6 alpha swarm task 4" -- i.e. wave-2 work, done WITHOUT the scoped
  proposal I explicitly asked for before any widening work, and contrary to
  the queue correction two messages prior. Paused backend, asked what
  happened (message-delivery gap vs. proceeding anyway) before repeating the
  correction a third time. Told them: commit if at a clean tested stopping
  point (don't waste real work), then bio schema+command, no further
  wave-2/widening work without checking with me first. Awaiting reply.
- Backend committed task 5 (d475097, equipment AC audit + real new
  encumbrance.rs carry-capacity/encumbrance calc, cited to Archives of
  Nethys) WITHOUT replying to the hard stop above and without waiting for
  the commit-serialization ack. Lead treated this as a third deviation,
  issued a full stop, and escalated to the operator rather than repeat a
  correction -- per the threshold already logged in risks item 6.
- RESOLUTION -- ROOT CAUSE WAS A DELIVERY BUG, NOT BACKEND: backend reported
  zero inbound SendMessage content had reached it all session; every one of
  its reports was execution of its ORIGINAL wave-1 spawn brief (task 4 =
  BAB/save stacking, task 5 = equipment AC/carry-capacity, in that exact
  order) with no visibility into the reprioritization, the pause, or the
  hard stop. All of the lead's messages arrived simultaneously in a single
  batch, well after task 5 was already committed. Backend correctly held at
  the LAST instruction visible to it ("wait for a message from me before
  doing anything at all") rather than self-authorizing off older content in
  the same batch that could have read as permission -- exemplary handling,
  not a discipline failure. Verified fixed with an explicit ack-the-literal-
  sentence test; confirmed received. Backend RESUMED with a clean current
  instruction set. The d475097 commit stands -- legitimate work under a
  legitimate brief. Frontend and QA showed no equivalent gap in the same
  window, so this looks scoped to backend's inbox/session specifically.
  Full narrative + assessment in risks-and-open-questions.md item 6
  (rewritten from "not stopping on instruction" to "delivery bug").
- QA independently reached the same "don't commit against undecided
  production code" conclusion on its own, before being told -- finished the
  40-test catalogue fix for backend's multiclass widening (0 failures, 467
  result lines, every value independently verified against real computation
  output, one real cosmetic defect found and correctly left for backend
  rather than fixed out-of-lane) and held the commit pending resolution.
  Cleared to proceed now that backend's status is resolved.

(c) On deck (wave 1 — 5 tasks per teammate)
--------------------------------------------
backend (REPRIORITIZED -- all 5 frontend-unblocking commands before calc work):
  1. DONE (0c614d9). Version bump 0.5.99 -> 0.6.0.
  2. DONE (e0a0bda). Skill-point allocation persistence.
  3. DONE (7694b22). Level-up HP + choices persistence.
  4. DONE (0ab784d). Bio schema field + persistence command (sidecar
     bio.json, avoids ChosenCharacterState blast radius).
  5. DONE (89c3710). Feat exposure: list_feat_catalog/list_feats
     (mirrors equipment_catalog.rs) + add_feat_selection.
  6. DONE (67490ac). Money/currency: real conversion math (money.rs),
     money.json sidecar, track-and-spend only -- starting-wealth auto-roll
     correctly left open (QA's unresolved PCGen wealth-by-class formula).
  ALL 6 REPRIORITIZED TASKS COMPLETE. Checkpoint reached -- holding for
  operator review before any wave-2 work resumes.
  -- wave 2, DONE early (delivered before reprioritization reached backend,
     see Happened log for the full delivery-bug story): multiclass BAB/save
     stacking widened to Fighter/Wizard/Rogue (d20a5b9+8d814e8, 3961/0);
     equipment AC audit + new carry-capacity/encumbrance calc (d475097).
  -- wave 2 remaining: durability calc (near-zero production surface per
     QA survey); money-conversion calc; comparator field-extraction fix in
     pcgen_runner.rs/pcgen-normalize-output.py (PCGen-sourced spec ready in
     SWARM_REPORT.md appendix, abec13b, except money-conversion unverified
     + wealth-by-class unresolved). Backlog (non-blocking, low priority):
     expose DR through PilotSnapshotDto/LoadSavedCharacterResponse.
     None of wave 2 blocks another teammate.

frontend:
  1. DONE (94a3865). Bio editor wired to update_character_bio/
     load_character_bio, live-verified: real disk round-trip confirmed by
     closing and reopening the character after editing.
  2. IN PROGRESS. Feat picker: backend's list_feats/add_feat_selection
     landed (89c3710), frontend reading the commit before wiring.
  3. Money panel: not started, backend's money command landed (67490ac),
     ready to pick up after the feat picker.
  4. DONE (75200fc). Wire SkillAllocationDialog.
  5. DONE (e8e4597). Wire LevelUpDialog.onAccept.
  BONUS (743c358): Actions tab wired for real, stub-tab audit closed on
  Defense/Pets/Overrides (see Happened log).

qa:
  1. DONE -- PCGen baseline clean (pcgen_runner_smoke 2/2, sd26_pcgen_runner
     6/6).
  2. DONE -- gap-list survey, full table in SWARM_REPORT.md.
  3. PARKED at wave-2 priority -- coordinating with backend once they reach
     multiclass BAB/save stacking, not blocking.
  4. DONE -- SWARM_REPORT.md skeleton + PCGen-sourced formula spec appendix
     (9ffe32f, abec13b).
  5. Ongoing -- watching for PCGen-divergence as backend/frontend land work.

blocked-by notes:
  frontend#4 <- backend#2 (command name)
  frontend#5 <- backend#3 (command name)
  qa#3 coordinates with backend#4 (shared test, not a hard block)
