v0.6 Alpha Release Swarm — Status
Branch: tranche/6 (from origin/develop @ 5b1bad5)
Source of truth: docs/release/v0.6/release-swarm.md
Update model: (a)-(c) below are hand-written by the lead after every real
event (task completion, blocker, correction) -- this is the narrative
record. The "Auto Heartbeat" block at the very end of this file is a
separate, mechanical, cron-driven refresh (every 5 min, via
~/workspace/swarm-status-heartbeat.py) that proves liveness and flags
staleness even if the lead goes quiet; it never touches anything above it,
and the lead never hand-edits it.

Agent Status convention (operator directive, 2026-07-24): every write to
this file includes a per-agent status line using one of: working (doing
X), idle (queue clear, nothing assigned), waiting-on-<agent> (blocked on
another agent's in-flight work), paused (quota/outage, with reset time
if known). This replaces the looser free-text "Happening now" prose below
where the two overlap -- (a) still carries the narrative detail, this
block is the at-a-glance state.

**SECURITY CONCERN RAISED THEN RESOLVED, 2026-07-24 (lead).** A teammate
message arrived claiming `teammate_id="operator"`, directing the lead to
author `SWARM_TASKS.md` and citing an uncommitted `§8.4.1` section of
`release-swarm.md` as authority. The lead correctly treated this as
suspicious given the process (an unrecognized identity on the teammate
channel, backed by an uncommitted edit to a trusted doc) and reverted the
addition without complying, then flagged it plainly to the operator
rather than deciding unilaterally. **Resolved, not an attack**: the
operator confirmed directly (through the real user channel) that this
was legitimate — they're working with a separate collaborator agent on
the `/swarm/dashboard.html` tooling, and that agent authored both the
`§8.4.1` section and the "operator" teammate message on the operator's
behalf. The caution was the right call given the information available
at the time (an unattributed identity plus a tampered-looking doc edit
is a real red-flag pattern), and surfacing rather than silently
complying or silently refusing was the correct way to resolve genuine
ambiguity about who's asking. Net effect: `SWARM_TASKS.md` ownership is
now confirmed as a real, standing lead responsibility (see below); the
reverted `§8.4.1` section may need re-adding by the operator's
collaborator agent since the lead's revert discarded it (asked the
operator whether to restore it or leave it to them).

**`SWARM_TASKS.md` rebuilt and reconciled (lead, `ff705d93`).** The prior
seed content didn't match this swarm's real work (fabricated-looking
"Epic"-numbered rows) — rather than stamp real commit hashes onto false
task descriptions (which would itself be dishonest reporting regardless
of the request's legitimacy), rebuilt the table from scratch against the
swarm's real tracking record (`risks-and-open-questions.md` items 1-27,
this file's own Happened log, `git log`). ~40 rows, real commit hashes,
honest `done`/`in progress`/`queued`/`blocked` statuses including the
genuinely-blocked architecture items. Lead authoring commitment adopted
going forward: update this file on every meaningful status flip,
alongside the existing Agent Status convention.

- Frontend's comprehensive smoke test complete. Live-drove all 3 working
  classes (Fighter/Human, Fighter/Dwarf, Wizard/Elf, Rogue/Half-Elf) end
  to end -- creation, level-up (including the item-23 feat-pick path),
  every populated tab, Add Weapon/Add Spell/Add Feat, skill allocation,
  HP damage/heal. **One real bug found and fixed (e50d7762)**:
  ItemPickerModal.tsx used a bare `entry.key` as its React key; the real
  feat corpus has a genuine duplicate ("Combat Expertise" appears twice,
  same key, differing internal effect formula), which confused React's
  reconciler across re-filters -- searching showed phantom/stale rows
  while the count text stayed correct. Fixed with a compound key
  (`${entry.key}::${index}`), live-verified. The underlying grant flow
  was never actually broken (Toughness persisted and showed correctly
  even while the picker rendering glitched). **One real, unresolved
  finding, not fixed, needs backend investigation**: the Spells tab
  renders from `corpusDerived`, not `spellsSelected`, and doesn't
  reliably reflect what's actually persisted for non-Human Wizards --
  a brand-new Elf Wizard showed "No corpus-reachable spells" despite 3
  real, disk-confirmed spells surviving a full reload; a different,
  older Elf Wizard showed 1-of-3. Not a clean "never works" pattern --
  inconsistent. May be the same root cause as the already-tracked item C
  /item 18 (Wizard non-Human spell math absence) surfacing through
  `corpusDerived`, or a separate, narrower issue -- frontend correctly
  didn't guess at the backend mechanism (outside their ownership) and
  flagged with exact repro steps instead. Everything else: clean pass,
  no other correctness issues. 69/69 suite green throughout.

- **SIGNIFICANT FINDING (QA, 2026-07-24)**: root-caused the Spells tab
  inconsistency, and it's bigger than the Spells tab. Traced to
  `corpus_fixture_bundle()` (`apps/desktop/src-tauri/src/
  corpus_fixtures.rs`) — the desktop app's corpus-aware layer
  (`compute_pilot_with_corpus`) is bundled with a deliberately tiny,
  hardcoded 2-spell/2-equipment-record fixture (the doc comment itself
  says "not a general corpus provider, exhaustive corpus coverage is out
  of scope"). Any spell/item a user actually picks that isn't one of
  those exact 2 bundled records per category silently drops out of
  `corpus_derived` (school_coverage, equipped_items grounding, and now
  shape (c)'s `equipment_effects` too) with zero diagnostic -- the
  underlying data (`spells_selected`/`equipment_selections`) persists
  correctly the whole time, confirmed by reading the saved envelope
  directly. **Not the same root cause as item 18** (which is a
  race-gated chassis-explanation path, unrelated) and **not
  race-specific at all** -- a Human Wizard picking the same spells would
  show the identical gap. Empirically reproduced and reverted cleanly
  (touched only `pf1_adapter.rs`, confirmed via `git status` and a clean
  `cargo build`, never went near backend's in-progress files). **Real
  scope implication for shape (c) specifically**: its ACP/AC value only
  looks complete today because the fixed-loadout's Chain Shirt/Longsword
  happen to be exactly the 2 bundled equipment records -- any other gear
  would silently show nothing, the same gap. Two fix shapes offered, not
  QA's call: bundle more real spells/equipment (bounded, mechanical) vs.
  surface an honest "not in demo corpus" indicator instead of silent
  omission (small UX fix). Queued for backend once the attack-bonus
  slice lands.

- **Defense-tab feat-add render-staleness fixed (frontend, `7360fe4a`).**
  New shared `refreshDurability()` helper called after a successful feat
  grant on both paths (`handleAddFeat`, `handleLevelUpFeatPick`) -- falls
  back to `null` on failure, same honest treatment as the initial load,
  never stale or fabricated. Live-verified both paths on a real character
  with the Defense tab already open: HP moved live (22/22 -> 25/25 via
  the Feats-tab picker, then 31 -> mid-transition via the level-up feat
  pick), no reselect needed. Also re-confirmed the earlier ItemPickerModal
  compound-key fix is holding up under continued use. 69/69 suite green,
  typecheck clean, no new test file (pure data-refresh wiring, no new
  pure logic). Frontend's queue is clear again.

- **"Outside demo corpus" indicator landed (frontend, `5406e335`).** New
  `UnresolvedNotice` component rendered in both SpellsTab and GearTab
  whenever backend's new `unresolvedSpellIds`/`unresolvedEquipmentItemIds`
  is non-empty -- minimal by design, just count + raw ids, no fabricated
  detail. Live-verified against the exact character from frontend's
  original smoke-test finding ("Smoke Wizard Elf"): Spells tab now reads
  "3 spells not shown (outside demo corpus)" instead of the misleading
  "No corpus-reachable spells selected yet," confirming backend's
  root-cause call was right all along -- never a Wizard/race issue.
  **Real gap flagged, not assumed covered**: shape (c)'s `equipmentEffects`
  field (AC/ACP) is also on the wire now but isn't wired into the frontend
  TS type at all yet -- frontend correctly didn't assume it was out of
  scope for this task and flagged it explicitly rather than silently
  leaving it uncovered. 69/69 suite green, typecheck clean.

## Agent Status (2026-07-24, ~13:50 ET)
| Agent | Status | Detail |
|---|---|---|
- **"Outside demo corpus" indicator independently verified, clean (QA,
  2026-07-24).** Confirmed both `SpellsTab` and `GearTab` are genuinely,
  independently wired (not one path reused), the minimal-detail claim
  holds exactly (literally count + raw joined ids, nothing else), and
  backend's field-population diff replaces the old silent `continue` with
  a real push in both resolve loops. Reproduced RED themselves (reverted
  the push, watched the exact `[]` vs `["Magic Missile"]` mismatch,
  restored). One thing noted, correctly not flagged as a bug: the list is
  deliberately not deduplicated (a spell picked Known+Prepared shows up
  twice) -- consistent with the "never hide information" discipline, not
  a defect. Ran targeted tests only (not a full workspace regression),
  explicitly to stay out of backend's concurrent edit -- good judgment.

- **Shape (c)'s equipmentEffects wired into the frontend (frontend,
  `d8528ce2`).** New "Equipment Effects" stat-tile row on the Defense
  tab, above the existing DR line -- same honest-absence discipline as
  the rest of the tab: `armorClassDelta`/`armorCheckPenaltyTotal` always
  render (a real 0 isn't treated as absent); `maxDexCap`/
  `spellFailureChance`/`attackBonusDelta` only render when genuinely
  present, with particular care taken to preserve `attackBonusDelta`'s
  documented `Some(0)`-vs-absent distinction (exactly one weapon with no
  enhancement is a real "+0"; zero or 2+ weapons stays honestly hidden).
  Live-verified against a real Chain-Shirt-equipped Fighter (AC+4/ACP-2/
  MaxDex4/SpellFail20%/AttackBonus+0, all matching real PF1 Chain Shirt
  stats) and cross-checked identical values show up for a Wizard too --
  correctly identified as the known pre-existing fixed-loadout
  simplification, not something newly introduced. 69/69 suite green,
  typecheck clean.

- **REAL BUG FOUND (QA, 2026-07-24) — `EquipmentEffectsDto`'s three
  optional fields never actually hide on absence, they render garbled
  "+null"/"null%" instead.** `maxDexCap`/`spellFailureChance`/
  `attackBonusDelta` lack `#[serde(skip_serializing_if =
  "Option::is_none")]` (unlike the precedented `damage_reduction` field
  in the same file, which has it), so a Rust `None` serializes as
  `"attackBonusDelta":null` -- key present, literal `null` -- not an
  omitted key. The frontend's `!== undefined` hide-checks are dead code
  for this case (`null !== undefined` is `true`), so the hide-branch
  never fires; confirmed the actual broken render strings directly:
  `fmt(null)` -> `"+null"`, `` `${null}%` `` -> `"null%"`. **Confirmed
  three independent ways**, not just reasoned about: a throwaway Rust
  serialization test (reverted after, `character_hub.rs` confirmed
  clean again), tracing `loadSavedCharacterDetail.ts`'s zero-transform
  raw `invoke()` call, and running the actual JS comparison/template
  logic in Node. **Why the earlier live-verification missed it**: the
  one character tested (Chain Shirt + exactly one weapon) happens to
  resolve `Some(...)` for all three fields, so the None/hide path was
  never exercised -- any character with zero or 2+ weapons (a fresh
  character, a Wizard, a dual-wielder -- the common case) would show
  "Attack Bonus: +null" instead of nothing. **Fix is small, mechanical,
  and precedented**: add the same `skip_serializing_if` attribute
  `damage_reduction` already carries. Not fixed -- backend's file,
  currently mid-feat-widening; QA correctly didn't interrupt, flagged
  for whoever picks it up next. Typecheck clean, 69/69 still pass (no
  existing test exercises the None path, exactly how it slipped
  through).

## Agent Status (2026-07-24, ~14:18 ET)
| Agent | Status | Detail |
|---|---|---|
| backend | working | feat-effects widening landed and verified (f38e9f33, 254/254 lib + 203/203 desktop); already starting the null-serialization bug fix |
| frontend | idle | queue clear |
| qa | working | dispatched to verify the feat-effects widening |
| qa | idle | verified the outside-demo-corpus indicator clean (both tabs, RED reproduction); standing by for backend's or frontend's next landing |

(a) Happening now (refreshed 2026-07-24, ~06:40 ET, resumed after operator pause)
------------------------------------------
orchestrator (lead)  Sonnet   RESUMED after an operator-directed pause
                                (swarm was paused mid-cycle over a
                                stuck-in-a-loop concern; root cause found
                                and fixed -- a CronCreate job was still
                                firing the 20-min check-in prompt even
                                after the lead stopped issuing new
                                ScheduleWakeup calls; deleted). Resuming
                                with ScheduleWakeup only this time (single
                                mechanism, explicit stop between cycles),
                                not CronCreate, to avoid the same failure
                                mode recurring. Operator also provided:
                                the starting-wealth-by-class table
                                (item 7, cited to d20pfsrd.com) and
                                explicit go-aheads to un-defer items 17
                                (feat-effects engine) and 1 (the
                                architecture-wall math) onto the
                                actionable backlog.
frontend               Sonnet  re-engaging now; last known state was a
                                comprehensive live-UI smoke test,
                                interrupted by a quota limit before the
                                pause (resets 9:10am ET) -- may still be
                                unavailable, will check.
backend                Sonnet  re-engaging now with the 3-item queue the
                                operator just added (7, 17, 1, in that
                                order)
qa                     Sonnet  re-engaging now; will hold for something
                                real to verify once backend/frontend land
                                work, per standing practice

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
- Backend closed the last reprioritized task, money conversion (67490ac):
  real conversion math (money.rs, standard 1pp=10gp=100sp=1000cp, flagged
  not-PCGen-verified same as QA's spec note), money.json sidecar (mirrors
  bio pattern), track-and-spend only -- correctly left starting-wealth
  auto-roll open since QA flagged that PCGen formula unresolved rather
  than invent one. 176/176 lib tests, 170/170 desktop suite. This closed
  ALL 3 of frontend's blocking commands -- checkpoint reached, both
  teammates told to hold rather than self-select wave-2 scope. Lead
  reported a full checkpoint to the operator (tasks landed, remaining
  alpha-bar gap, the still-open posture-narrowness and Pets questions).
- Operator asked about SWARM_STATUS.md update automation; lead built a
  mechanical cron heartbeat (~/workspace/swarm-status-heartbeat.py, every
  5 min, see file header) as a liveness/staleness guarantee independent of
  the lead's own diligence, then resumed the swarm. During the pause,
  both teammates finished real work:
  * Backend: fixed the cosmetic "Fighter"-hardcoded-in-explanation-text
    defect QA found, via a proper class_summary_label(input) helper
    (dynamic, not a blind strip -- an existing test asserted the literal
    "Fighter" string for real Fighter input, which a strip would have
    broken). Committed d1905ed, full suite clean (176/176, 3961/3961,
    170/170).
  * Frontend: closed all 3 remaining wave-1 tasks. Bio editor (94a3865,
    live-verified with a real close-and-reopen disk round-trip). Feat
    picker (febf4d8, reused ItemPickerModal pattern, live-verified against
    the real 185-entry catalog, "Showing 185 of 185", added Cleave and
    confirmed on-disk persistence) -- honestly flagged that the tab can't
    show a character's *existing* full feat list since selected_feats
    isn't exposed via load_saved_character (new backend backlog item,
    same shape as the Defense/DR gap). Money panel (59d5bc0, live-verified
    both the success path -- added 150gp, confirmed PP15 display and
    on-disk totalCopper -- and the honest-failure path -- tried to
    overspend, got the real backend error, balance unchanged) -- correctly
    left equipment-purchase auto-deduct unbuilt rather than risk a
    non-atomic partial-apply bug, flagged as a real bounded follow-on.
  All 5 of frontend's original wave-1 tasks are now done: bio (94a3865),
  feat picker (febf4d8), money panel (59d5bc0), skill persistence
  (75200fc), level-up persistence (e8e4597).
- Backend proposed the posture-narrowness widening (4 exactness gates
  found, not 3 -- including a previously undocumented Human-Wizard
  spellbook gate). AC slice greenlit initially, then DROPPED
  mid-implementation when backend found baseline_armor_class lives in the
  corpus-free headless compute layer and can't cheaply reach the already-
  correct per-item AC math, which needs corpus access -- a ~347-call-site
  architecture question, not a wiring job. Backend caught this before
  writing the change and stopped to ask. Flagged for the operator: the
  same headless/corpus-aware split likely blocks attack-bonus and
  skill-posture widening too -- possibly one architecture fix unlocks all
  three pillars rather than three separate slices. Full writeup in
  risks-and-open-questions.md item 1. Backend redirected to items 2-4
  (durability, money-conversion PCGen verification, comparator
  field-extraction), none of which share this problem.
- QA adopted money-conversion + encumbrance into the test catalogue (12
  independently-authored tests, cross-checked the PCGen carrying-capacity
  table a second time from a different angle than the original spec pass,
  caught a real fixture-authoring trap in their own draft -- corpus KEY
  tokens don't take a "(Base)" suffix for general items -- and documented
  it for future fixture writers). Refreshed gap-list survey: money/carry-
  capacity/encumbrance no longer gaps; durability is now the sole
  zero-production-surface calculation; BAB/save breadth still 3 of 11
  classes. Caught and fixed a stale "AC greenlit" line in its own
  SWARM_REPORT.md attestation after seeing the drop -- good self-check on
  the document that matters most for closure.
- MAJOR FINDING: frontend's multiclass audit (asked to verify alpha bar
  item 3 is reachable through the UI, not just the engine) found character
  creation is single-class-only by design (multiclassing is a level-up-
  time action, matches backend's docs) -- structurally correct. But
  LIVE-testing (not just reading code) surfaced a real, silent,
  launch-blocking bug: the backend persisted a Fighter-2-into-Rogue-1
  multiclass level-up perfectly (verified on disk: two real class_level
  entries, correct recomputed BAB/saves/HP), but the frontend rendered it
  as a garbled single pseudo-class with HP wrong by 17 points. Root cause:
  characterProgression.ts's parseHeldClasses() split the wire-format
  classSummary on '/' ; character_hub.rs actually joins on ',' -- verified
  directly against the Rust source. The wrong assumption silently
  corrupted HP, skill-point totals, caster level, the Progression rail,
  and weapon proficiency for ANY multiclass character while single-class
  characters (no separator needed) looked completely fine -- exactly why
  it went unnoticed until someone actually drove a real multiclass
  level-up end-to-end rather than trusting that the code paths existed.
  Fixed and live-verified (d03bc89): same flow now shows "Fighter 2 /
  Rogue 1", correct HP 27/27, correct Progression-rail entry. Confirms
  alpha bar item 3 is genuinely reachable -- this was a real bug, not a
  missing affordance. Frontend now attempting a full 6-level multiclass
  walkthrough to proactively surface anything else like this.
- Backend closed item 2, durability (0aeed25): compute_max_hp +
  classify_durability, hit-die size verified against the primary
  cr_classes.lst corpus source directly rather than trusting
  class_tables.rs's existing citation. load_character_durability/
  adjust_character_hp commands, hp.json sidecar. Scoped to single-class
  Fighter/Wizard/Rogue -- multiclass HP honestly deferred: CharacterClassLevel
  stores cumulative totals per class, not the order individual levels were
  taken in, so which level gets the maximized first-level die is genuinely
  ambiguous from that data shape, not guessed at. Temp HP and the
  favored-class-bonus HP choice also deferred per QA's spec flag. Not
  frontend-blocking (wasn't on frontend's original ask), so no UI wiring
  yet -- command shapes documented for whenever a Health tab gets built.
  189/189 lib, 3973/3973 full suite. Moving to item 3 (money-conversion
  PCGen-source verification).
- UNATTRIBUTED FILE: docs/release/v0.6/SWARM_TASKS.md appeared untracked
  in the shared checkout, not created by the lead or any known teammate.
  Content (Epic 1/2/3/6/7 labels, tasks like "campaign manager + Drive
  persistence," "Wizard single-class completion") doesn't match anything
  in this swarm's actual work, and its claim that the observer's cron
  reads it as an "operator-pinned schema" contradicts the verified
  behavior of observer.py (reads SWARM_STATUS.md + mailbox/task JSON only).
  Flagged to the operator, not trusted, not edited, not deleted. Backend
  initially assumed it was the lead's -- corrected.
- MILESTONE: frontend completed a full 6-level multiclass live walkthrough
  (no code changes needed -- pure verification of the d03bc89 fix), taking
  a character from Fighter2/Rogue1 (level 3) through 4 more real level-ups
  to character level 6 (ending Fighter4/Rogue2). Every step: real backend
  mutation (revision_id incremented each time, rev.8 by the end), real
  server-side recompute, hand-checked PF1 math matching exactly (e.g.
  BAB+5 at the end = Fighter4's full +4 plus Rogue2's 3/4-progression
  floor(2*0.75)=+1), zero errors, zero display glitches, Feats tab still
  correct with two classes held. This is the first genuinely end-to-end
  live proof that alpha bar item 3 (advance 6 levels, multiclass required)
  actually works today, not just that the underlying pieces are wired.
  One transient hiccup (caught backend's durability.rs mid-edit, same
  class of false start as earlier) -- frontend handled it by waiting via
  a background poll instead of raising another alarm, learned from the
  earlier pattern. Next: same rigor applied to spell-slot selection at
  spell-gaining levels (bar item 3's other named requirement).
- Backend closed item 3, money-conversion PCGen verification: searched the
  local PCGen checkout directly (.lst files, .java source, output-sheet
  templates) for a currency-denomination table -- found none anywhere.
  Nothing contradicts the 1pp=10gp=100sp=1000cp ratio, but nothing
  affirmatively pins it to a PCGen source either (unlike carry-capacity's
  load.lst). Conclusion, now backed by a direct second search rather than
  left on QA's original flag alone: this is universal D20/PF1 arithmetic
  PCGen likely tracks as a single gold-piece-equivalent total, never
  exposed as denomination data. No ratio change needed -- doc-only update
  to money.rs recording the verification.
- COMMIT-HYGIENE NOTE: 6cbee9c ended up containing both backend's item-3
  doc update and this file's milestone entry under a docs-only commit
  message -- lead error, not backend's: `git commit` with no pathspec
  commits everything currently staged, and backend had already staged
  money.rs when the lead ran a bare `git commit` right after `git add`ing
  only SWARM_STATUS.md. Content is correct and wanted either way; not
  fixed via force-push (too disruptive with three live writers on this
  branch for a purely cosmetic issue). Lead now checks `git diff --cached
  --stat` immediately before every commit, not just before `git add`.
- MAJOR FINDING: frontend's spell-slot walkthrough hit the Wizard
  spellbook gate before ever reaching the spell picker. Unlike the other
  posture gates (narrow but reachable via one exact combo), this one is
  STRUCTURALLY UNREACHABLE -- compose_character_input never seeds the
  required school-specialization choices for any class, and
  CreateCharacterForm has no arcane-school field at all, so no UI action
  can ever satisfy the gate. Verified live two ways (fresh creation,
  multiclass level-up) -- both correctly Blocked, neither persisted.
  Directly blocks bar item 3's "select spells" for the class most
  associated with spellcasting. Read code first, verified before
  reporting, correctly did not improvise a fix (real cross-team scope
  decision: new UI field + new backend param, vs. a hardcoded default).
  Lead decision: hardcode the default specialization server-side,
  mirroring Fighter's existing hardcoded-loadout precedent -- sufficient
  for the bar (item 3 requires spell selection, not school choice), small
  and bounded. A real arcane-school selector goes to backlog. Backend
  paused item 4 to take this first since it unblocks a literal bar
  requirement. Full writeup in risks-and-open-questions.md item 1.
- Backend closed item 4, comparator field-extraction (b726a36): 5 new
  PCGen dimensions parsed from base-xml.ftl (encumbrance thresholds/total,
  durability.max_hp, best-effort money.total_copper from free-text
  MISC.FUNDS). No Rust struct changes needed -- pure serde deserialization,
  confirmed the safe case rather than another blast-radius trap. Genuine,
  confirmed (not overlooked) gap: PCGen's own export template hardcodes
  current HP as an empty tag, structurally absent, can't be extracted
  without a PCGen template change (out of repo scope). Verified two ways:
  synthetic fixture AND the real end-to-end PCGen pipeline (8/8,
  including the genuine non-mocked Gradle run). This closes wave-2 items
  2-4 in full (durability 0aeed25, money-verification 6cbee9c, comparator
  b726a36), none of which hit the AC slice's architecture problem.
- Backend confirmed and started the Wizard fix (crossed-message timing
  only, no discipline issue -- correctly held rather than self-select
  when uncertain, exactly the right instinct). Frontend's caster survey
  (Cleric/Druid/Bard/Sorcerer all need whole subsystems, not a seeded
  choice) relayed to backend to avoid duplicate investigation.
- Wizard spellbook GATE fix landed (3484b5d) -- compose_character_input
  seeds Evocation/opposed-Necromancy/opposed-Transmutation choices,
  gated Wizard-only (mirrors the existing Fighter/Human conditional-seed
  pattern), 2 new tests (Wizard-only confirmed via a Fighter negative
  control; a real recorded+prepared spell now reaches Computed via
  direct Rust construction of CharacterInput).
  Confirmed independently: no other class has an equivalent bespoke gate
  to generalize from -- none of Cleric/Druid/Bard/Sorcerer/etc are even
  chassis-supported yet (only Fighter/Wizard/Rogue), so none can hit a
  gate like this regardless.
  CORRECTION (frontend, same day): the "Wizard is now the second class
  that reaches Computed" claim below was PREMATURE -- struck. Frontend's
  live testing found the compute-layer fix is necessary but not
  sufficient: (1) create_character only ever persists a proven-Computed
  build, so a fresh Wizard (empty spellbook) can never be saved in the
  first place -- bootstrap deadlock; (2) the Add Spell picker can't
  bootstrap it either, since unmet_wizard_spellbook_conditions needs one
  spell recorded as BOTH Known AND Prepared, but add_spell_selection only
  sets one mode per call and each call is independently reject-if-not-
  Computed -- neither a Known-only nor Prepared-only call can ever build
  toward the other; backend's own passing test only reaches Computed by
  constructing CharacterInput directly in Rust, bypassing the command
  layer entirely, which proves the compute logic but not UI-reachability;
  (3) SEPARATELY, apply_level_up (the multiclass-dip path) never got the
  fix at all -- only compose_character_input (creation) was touched, so
  multiclassing Wizard onto an existing character still hits the OLD
  diagnostic. Net: there is currently no UI path, creation or level-up,
  that produces a live saved Wizard character. Backend fixing the
  level-up-path gap (small, clear parity fix) now; the bootstrap
  deadlock -- the real bottleneck -- needs a scoped proposal before code,
  same diligence as the posture-narrowness work.
  Also surfaced (not part of this commit, correctly left for qa): item
  4's landed comparator fix exposed a stale assertion in
  tests/sd26_pilot_case_verification.rs (expected 1 mismatch, now sees 6
  -- the known CG-03 mismatch plus 5 real MissingFromCodex entries, since
  Codex's own SelectedParityDimensions::from_receipt was never wired to
  populate the new encumbrance/durability dimensions). Real upside in the
  failure output: PCGen's actual golden-fixture values match backend's
  encumbrance.rs/durability.rs formulas exactly -- genuine external
  validation, not just internal self-consistency. Backend greenlit to do
  the from_receipt wiring as a small, bounded follow-on directly
  completing item 4's comparator work; QA to update the test expectation
  once it lands, same sequencing as the earlier BAB/save catalogue work.
- Backend split the from_receipt wiring into 3 problems before writing
  code: durability (small, landed alongside), encumbrance (initially
  looked blocked by the same headless/corpus-aware wall as the AC slice
  -- independent 2nd confirmation the boundary is real), money (confirmed
  structural exclusion, not deferred -- money.json's balance is never
  derived from CharacterInput/PilotReceipt, nothing for any
  receipt-projection function to expose). RESOLVED for encumbrance:
  backend found contract::PilotReceipt is already corpus-aware and
  already carries a resolved EncumbranceComputation, so an additive
  SelectedParityDimensions::from_pilot_receipt (2298780) sidesteps the
  wall entirely for parity-testing purposes, without touching the harder
  Computed/Blocked gating problem (still a future epic for AC/attack-
  bonus/skills). from_receipt itself provably untouched, zero blast
  radius on QA's 3 existing callers. Corrected in risks doc.
  Money's exclusion is a real backlog item now (needs its own
  money.json-reading comparison path if ever wanted), not folded into
  the encumbrance/AC architecture deferral.
- Redirected backend back to the two still-open Wizard gaps (level-up
  path fix, then the bootstrap-deadlock scoped proposal) after it went
  to the from_receipt wiring first -- normal reprioritization, not a
  repeat of the earlier discipline pattern; backend was holding for
  direction, not self-selecting past an explicit stop.
- Backend closed the level-up-path gap (1aabdf8): apply_level_up now
  seeds the canonical choices in the new-class-entry branch only,
  verified with a dedicated test that it fires exactly once per
  character and doesn't re-seed on a later Wizard level-up. Both
  fixable Wizard gaps now closed; the bootstrap deadlock remains.
- QA independently switched sd26_pilot_case_verification.rs to
  from_pilot_receipt (900beee) rather than accept the 5 new dimensions
  as permanent gaps -- built a real corpus fixture (reusing the GE-06
  posture), ran the actual corpus-aware compute path, and got 13 of 14
  dimensions genuinely matching PCGen's real export values (max_hp=12,
  light/medium/heavy=100/200/300, total carried=29 -- exact matches, not
  presence checks). Independently confirmed backend's "zero blast
  radius" claim on from_receipt rather than trusting it. The one
  pre-existing combat.baseline_melee_attack_bonus mismatch (CG-03,
  already known) is untouched.
- Backend's bootstrap-deadlock proposal: independently re-derived the
  root cause before reading the lead's framing (verification, not
  agreement), confirmed it's purely a first-spell bootstrap problem --
  add_spell_selection itself needs no redesign, works fine for every
  spell after the first. Proposed a new atomic command,
  record_and_prepare_spell_selection, pushing both Known and Prepared
  entries in one mutate_saved_character_at_root call -- same additive-
  command pattern as the rest of the codebase. Greenlit over the
  alternative (widening add_spell_selection) since it matches this
  swarm's established add-don't-modify pattern. Wizard-scoped, general
  shape not built speculatively. Backend building it now; frontend to be
  looped in on the signature once it lands -- the UI needs a decision on
  how "the first spell" is presented before the picker flow is testable.
- record_and_prepare_spell_selection landed and pushed (6e12437). Real
  and correct fix for its target problem (level-up parity + the
  add-spell-selection second-call bootstrap). Backend proactively looped
  in frontend (command signature) and QA (Wizard spellbook test surface
  impact) without waiting to be told. Frontend testing the real
  spell-picker flow next -- the original ask from before this whole
  investigation chain started.
- Backend closed item 8, feats-tab full-list exposure (1509124):
  chosen.selected_feats now exposed through LoadSavedCharacterResponse,
  found and fixed all 3 build sites for the shared struct literal.
  Small, bounded, no surprises as expected. Proactively messaged
  frontend with the field shape.
- QA scoped a Wizard PCGen fixture task and correctly stopped before
  sinking effort into it: no real PCGen-loadable Wizard .pcg fixture
  exists anywhere accessible (the one that exists is wrong-ruleset,
  GAMEMODE:3e). The Fighter case took multiple GE-05/SD-26 cycles to
  get a genuinely correct one -- hand-authoring a new Wizard fixture
  carries real risk of producing something that doesn't load in PCGen,
  multi-cycle-shaped work. Deferred to backlog (risks item 11). Going
  with the lighter substitute instead: cross-checking Wizard's spell-
  slot/spells-known/save-DC formulas against PCGen's source LST data
  directly, same technique as the carrying-capacity/encumbrance work --
  real external validation without the fixture-authoring risk.
- Frontend found one more small gap while wiring the spell picker:
  load_saved_character doesn't expose spells_selected either (same
  shape as items 6/8), so it can't detect "is this the first spell" and
  routes all Wizard spell adds through record_and_prepare_spell_selection
  regardless -- a sound, documented-pattern-consistent workaround, not
  blocking. Backend correctly flagged it rather than acting on it
  unprompted; logged as risks item 9a, not prioritized.
- Backend closed the real Wizard spell-save-DC gap QA found (3b39731):
  10 + spell_level + Intelligence modifier, mirroring the identical
  pattern already used for Paladin/Ranger/Sorcerer/Bard. 3 new tests,
  195/195 lib green. Coordinated cleanly with QA on the single test
  break it caused (a stale negative-control allowlist) -- held the
  commit until QA's fix landed, same sequencing discipline as before.
- THIRD CORRECTION to a premature "resolved" claim (frontend, live-
  tested again): record_and_prepare_spell_selection is real and correct
  for what it does, but insufficient alone -- it requires an already-
  saved Wizard-holding character, and nothing in the command surface
  could produce that state (class-acquisition is itself gated on spell
  state; spell-acquisition needs an already-saved Wizard). A distinct,
  one-level-deeper chicken-and-egg from the one already fixed. Struck
  the earlier "MILESTONE"/"second class reaches Computed" framing again,
  logged the real gap in risks item 10, sent backend a scoping question
  with a plausible (not mandated) direction: seed one canonical starter
  spell atomically at class-acquisition, mirroring the school-choice
  pattern. Frontend's routing commit (d55a919) still correct and
  necessary regardless of what's found next.
- Backend's investigation confirmed the deadlock precisely (traced to
  SD-24 Criterion 7.5's now-false "starts empty" assumption, invalidated
  by SD-21 E6b's later spellbook gate) and proposed the exact seeding
  fix, verified against real budget math (1 of 3 cantrips consumed,
  comfortably under budget at every supported level) and confirmed not
  to touch the separate level 1-3 ceiling. Correctly declined to fold in
  the unrelated source_class_id validation gap (pre-existing across the
  whole spell-selection family, fixing it inconsistently on one function
  would be worse than leaving both alone). Greenlit without reservation.
- Starter-spell bootstrap fix landed and pushed (b2a5eb6): both seed
  points (compose_character_input, apply_level_up's new-class-entry
  branch) now seed one canonical starter spell. Existing deadlock-
  reproduction tests rewritten (not deleted) into standing regression
  guards; 3 new tests prove the actual fix. 187/187 desktop suite.
  Backend correctly did NOT declare this resolved itself this time --
  explicitly asked for frontend's live verification given the pattern.
  NOT marked resolved in risks doc either, pending that report.
- MILESTONE, GENUINELY CONFIRMED THIS TIME: frontend live-tested the
  starter-spell fix end-to-end. Created a fresh Human Wizard -- creation
  form reported "computed and saved" for the first time in this
  investigation. Real sheet loaded (HP 8/8, real Progression rail).
  Used the real Add Spell picker twice (Magic Missile, then Alarm),
  both genuinely persisted (rev.1->rev.2->rev.3), looked correct at
  "2/2" against the 1st-level budget (1 base + 1 specialist + 0
  Int-mod). CORRECTED BELOW: that "2/2" was never actually server-
  enforced -- see the real bug entry further down. This closes the
  bootstrap/class-acquisition layers of the Wizard investigation:
  class acquisition, then first-spell bootstrap, now confirmed with
  real UI-driven persistence, not just compute-layer proof. Wizard is
  the second genuinely playable class alongside Fighter. Marked
  RESOLVED in risks doc item 10, for real this time.
  One new minor finding along the way: post-mutation corpus_derived
  render staleness (Spells tab briefly shows empty after a successful
  add, corrects on reload). Backend investigated and ruled their side
  out with confidence (mutation path is fully synchronous, no caching,
  response provably fresh) -- narrowed to a frontend render/state-
  timing issue, passed back to frontend. Not urgent, data always
  correct on reload.
- Frontend also landed the feats-tab full-list wiring (aa611ce) against
  backend's selectedFeats field (1509124) -- live-verified, Aldric's
  Feats tab correctly showed all 5 feats (3 fixed-loadout + Cleave +
  newly-added Toughness), revision bumped on the new add. Risks item 8
  marked RESOLVED.
- REAL BUG FOUND (frontend, while chasing the cosmetic render-
  staleness issue): Wizard spell-slot budget enforcement never applies
  to any real spell. parse_wizard_spellbook_spell_id expects the
  bespoke seed-spell format ("evocation.0.light") but real corpus
  spell keys ("Magic Missile", "Alarm") have no dots, so the parser
  returns None for every real spell and the consumption sum silently
  drops them all -- the over-budget check can mathematically never
  fire against real spells. Verified live: added a 3rd 1st-level spell
  past the 2-slot budget, expected Blocked, got silent acceptance and
  persistence instead (rev.3->rev.4 on disk). This RETROACTIVELY
  CORRECTS the earlier "2/2, honestly enforced" milestone claim above
  -- it was UI-side arithmetic, never server-verified. Directly
  relevant to alpha bar item 4. Frontend correctly did not propose a
  fix (real ambiguity in repair shape, not their file). Backend
  investigating now; fix must include a live-verified over-budget test
  case, not just a parser unit test, per lead instruction.
- MILESTONE, GENUINELY FINAL THIS TIME: spell-slot budget fix landed
  (365b3a1a) and confirmed NOT the same architecture wall as AC/
  encumbrance -- SPELL_LIST (652 real CRB records) is compiled directly
  into the binary, already headless-accessible, unlike equipment.
  parse_wizard_spellbook_spell_id resolves real keys first, falls back
  to the synthetic convention for zero blast radius on QA's fixtures;
  WIZARD_STARTER_SPELL_ID upgraded from placeholder to a real spell
  ("Light") now that real resolution exists. Genuine RED->GREEN: backend
  disabled the fix, watched the test fail with the exact reported
  symptom, restored it, confirmed green. Frontend live-re-verified with
  a fresh character (avoiding the prior save's stale pre-fix artifact):
  Magic Missile + Alarm accepted at 2/2 (disk-confirmed), Grease as a
  third spell honestly Blocked with a real diagnostic, revision
  unchanged, nothing persisted -- verified against disk specifically
  because the still-open render-staleness bug makes accepted-vs-blocked
  look identical on screen. This closes the ENTIRE Wizard investigation:
  class acquisition, first-spell bootstrap, and now slot-budget
  enforcement, all live-verified end-to-end. Only the cosmetic render-
  staleness bug remains open in this whole area (backend's to pick up,
  not urgent).
- OPERATOR DIRECTIVE, 2026-07-23: fully autonomous, operatorless
  operation, potentially for days. Resolved the 3 remaining "operator's
  call" items with the lead's own judgment (tranche-6 CI needs no new
  workflow; book-stub labels out of scope for this swarm; Pets/companion
  gap is a documented non-blocker, real future feature). Cron backstop
  set (~every 20 min, 7-day window) to keep dispatching work even if the
  turn-based flow ever goes quiet. Deep prioritized queues loaded into
  all 3 teammates with explicit "don't wait for check-in between tasks"
  instructions. Full reasoning in risks-and-open-questions.md.
- Backend confirmed Rogue reaches Computed through the real UI with
  ZERO gap (0bb37521) -- unlike Wizard, no hidden posture/spellbook
  gate exists for Rogue anywhere; compose_character_input uses the
  identical fixed loadout as every non-Fighter/non-Wizard class, and
  the one class-conditional sub-check only adds a requirement for
  Fighter, never blocks others. Confirmed empirically with 2 new tests
  through the real command path (creation + multiclass dip). No fix
  needed -- task 4's BAB/save widening was already sufficient. Third
  class (after Fighter, Wizard) confirmed reachable, for free. Moved
  autonomously to item 2 (DR exposure) without waiting for check-in.
- QA delivered a comprehensive coverage resurvey (81785b6f) against
  everything that's changed this session -- durability, encumbrance,
  money, Wizard spellbook, spell save DC, Rogue widening.
- Backend landed DR exposure (f7ce289d), spells_selected exposure
  (034d1b84, unblocks frontend's earlier routing workaround), then
  moved to investigating the money-purchase-coupling transaction shape
  before building it. QA folded a real de-risking finding on the
  Wizard .pcg fixture task into risks item 11 (real working syntax
  templates found for every section needed; one real unknown flagged,
  not assumed) -- correctly routed through the lead rather than
  committing to a file outside its own lane, even under autonomous mode.
- QA closed the Rogue reachability catalogue-coverage task (c800d568)
  with good judgment on a real complication: backend's 2 proof tests
  live in the separate codex-desktop crate (apps/desktop/src-tauri),
  which tests/** can't target -- same crate-boundary shape as the
  spell-slot-budget fix and mutation-op registry QA correctly left
  alone before. Instead of skipping coverage, QA read
  compose_character_input's source directly, confirmed its fixed
  loadout is byte-for-byte the existing GE-06 deterministic fixture
  shape, and wrote an independently-authored rules_core-layer
  equivalent proving the identical claim as a permanent tests/** entry
  -- complementary to backend's proof, not a duplicate, now under QA's
  ownership/sign-off same as the BAB/save and DC work. Moved to the
  systematic Fighter/Wizard/Rogue gap sweep next.
- Backend closed money-purchase coupling (29e67515): new atomic
  purchase_equipment command, resolves real catalog cost headlessly (no
  corpus wall -- checked before building), pre-checks affordability,
  adds equipment + deducts cost together, honestly Blocked (never free)
  if unaffordable or cost unknown. Re-confirmed the render-staleness
  elimination holds for the two newest mutation commands too (same
  synchronous, no-caching code path) -- nothing further diagnosable
  from backend's side without frontend-specific render/state detail.
- MAJOR FIND, priority bug: QA's systematic gap sweep found
  compute_selected_skill_modifiers applies the Climb/Intimidate/Swim
  class-skill bonus (+3) unconditionally regardless of actual class.
  Written originally for Fighter (whose real class-skill list genuinely
  includes all three); the Computed/Blocked gate was later widened to
  any dispatch-supported class, but the +3 itself was never made
  class-aware. Rogue is coincidentally correct (its real list also
  includes all three); Wizard's real list includes none of them.
  VERIFIED CONCRETELY, not reasoned abstractly: built the actual Wizard
  posture, got climb=5 (real value should be 2, no class-skill bonus) --
  a silently wrong number in the explanations list, no diagnostic fires.
  Narrower and more urgent than the already-known deferred "class-skill
  recognition is Fighter-only" finding (that one is in skill_allocation.rs,
  doesn't gate anything reaching Computed) -- this is the narrow
  deterministic-posture function that DOES gate Computed/Blocked, live
  and wrong right now. Two smaller findings from the same sweep, lower
  severity, logged but not actioned: level-up HP exposure is
  inconsistent per-class (Fighter has a dedicated explanation, Wizard/
  Rogue don't, though durability.rs already computes the same correct
  value for all three) -- reads as inconsistent exposure, not a missing
  calc; bonus spells from high ability score confirmed present and
  correctly wired, no gap. Backend redirected to the bug ahead of
  render-staleness (CORRECTION BELOW: this framing was wrong).
- Both quota outages recovered cleanly. All 3 teammates resumed after
  the ~11pm ET reset and picked up exactly where they'd left off.
- QA's continued sweep of the adjacent narrow-posture-gate functions
  (Fighter bonus-feat sub-check, Weapon/Armor Training level fallback)
  came back clean -- both correctly guarded/semantically right, not
  bugs, closing out that thread with certainty rather than leaving it
  assumed-fine.
- MAJOR REPORT (frontend, class-dropdown honesty audit, went deeper
  than asked): confirmed all 11 CRB classes appear in the create-
  character dropdown; 8 (Paladin/Ranger/Sorcerer/Bard/Barbarian/Monk/
  Cleric/Druid) were labeled "(Human only, partial)", implying Human
  produces a computed build. FALSE for all 8, verified via source
  (each has its own "does not compute a supported chassis" doc
  comment) AND live (fresh Human Barbarian -> real named Blocked
  diagnostic, never Saved) -- these never reach Computed for ANY race
  including Human, they just get nicer diagnostics. Fixed: new
  ClassSupportLevel variant `human-diagnostics-only`, distinct from
  the genuinely-working `partial-human-only`, honest dropdown/
  description/fallback text. No stub/silent-failure anywhere -- the
  Blocked path itself was always fine, just the labeling overclaimed.
  BONUS, opposite direction: Wizard and Rogue (labeled
  `partial-human-only`) turn out to be mislabeled too -- confirmed via
  git archaeology (3484b5d never had a Human condition) AND live
  (fresh Elf Wizard 1 and fresh Elf Rogue 1 both reached Computed/
  Saved, disk-confirmed with race_id=race:elf). Reclassified both to
  `full`, same as Fighter. Real, free additional alpha-bar progress on
  item 2 ("any class or race") -- the "Human only" framing for two of
  our three working classes was a stale, never-verified assumption.
  Pushed 34635157, 62/62 suite green.
- CORRECTION (frontend, re-tested live against the current build):
  the render-staleness bug is NOT fixed -- the lead's earlier "already
  eliminated on backend's side" framing was wrong (backend's own
  elimination of possible causes was real, but "eliminated as a cause"
  got conflated with "the bug is gone," and nobody had re-tested live
  since). Root cause identified precisely: character_hub.rs's
  CreateCharacterResponse enum has #[serde(tag = "kind")] with no
  rename_all, so corpus_derived stays snake_case on the wire while
  frontend expects camelCase. Real trap flagged: a bare
  rename_all="camelCase" would ALSO lowercase the "Saved"/"Blocked"
  tag values, breaking every outcome.kind==='Saved' check across the
  frontend -- needs a per-field rename or rename_all_fields instead.
  Reopened, queued for backend next with this guidance attached so
  they don't walk into the trap.
- MILESTONE: class-skill-modifier bug FIXED (93a0636d), verified
  against the real PCGen corpus (cr_abilities_class.lst) before
  writing code, not memory -- confirmed Fighter/Rogue genuinely have
  Climb/Intimidate/Swim as class skills, Wizard has none. Now only
  applies the +3 when the character has a class that actually grants
  it; multiclass correctly gets it via PF1's real union rule (tested
  explicitly, not assumed). One real test break found and coordinated
  with QA before pushing. Reading the full render-staleness context
  before touching that fix, per the trap flagged earlier -- correct
  instinct given the "obvious" fix (bare rename_all) would break tag
  matching across the frontend.
- Frontend explained an idle-with-no-report gap when checked: purchase_
  equipment wiring is code-complete (typecheck clean, 62/62), held
  uncommitted pending live verification per standing practice, and hit
  the same transient shared-checkout build break as before (backend's
  skill-modifier commit landing mid-edit) -- correctly waited it out via
  a background poll rather than assume a regression, resumed once clear.
- REAL PARITY GAP found (QA, following up on the Elf Wizard/Rogue
  confirmation): traced PCGen's actual Elf race data through the real
  reference chain (not just the top-level citation-only line) and
  confirmed +2 Intelligence is the CRB-standard racial default, not the
  "alternate variant" Codex's own comment claims -- explain_elf_race_seam
  only grounds DEX/CON, missing INT entirely. Matters specifically
  because Elf Wizard (Intelligence-cast) is now confirmed reachable --
  a live parity break for exactly the combination just proven to work,
  not theoretical. Small, well-scoped fix expected (mirrors the existing
  DEX/CON pattern). Queued for backend after render-staleness. QA moving
  to check other grounded races for the same "comment says out-of-scope,
  real PCGen data says default" pattern.
- CROSS-AGENT GUI COLLISION fixed (lead, f6fe0df2): driver.sh shared one
  DISPLAY_NUM=99, one state file, one log file, and cmd_stop's process-kill
  matched every agent's codex process globally -- any agent's launch/stop
  could hijack or kill another agent's session (this is what frontend
  flagged and QA independently hit as "black webview"/dying process/
  vanishing state file). Fixed: RUN_DESKTOP_AGENT=<name> namespaces
  DISPLAY_NUM (frontend=96/backend=97/qa=98/default=99) plus state/log
  files, and cmd_stop's kill is scoped to the caller's own DISPLAY via
  /proc/<pid>/environ. Also fixed a latent bug where cmd_stop never
  sourced its own state file (dead TAURI_PID/XVFB_PID branch). QA stood
  down its own stray instance cleanly before the fix landed -- no harm.
- Doc-hygiene pass (lead): SWARM_STATUS's wave-1/wave-2 on-deck table had
  gone stale (didn't reflect durability/money-conversion/comparator/DR/
  money-purchase/spells_selected all landing) -- replaced with an accurate
  consolidated snapshot (026b9cd0). risks-and-open-questions.md items 6,
  9, 9a corrected from stale "backlog, non-blocking" to RESOLVED to match
  what actually shipped.
- Backend's scoping-only pass (no code) on whether attack-bonus/skill
  widening hits the same headless/corpus-aware wall as AC: YES for
  attack-bonus (enhancement bonuses need corpus-resolved EquipmentRecord,
  same as AC) and YES for armor-check-penalty (ACCHECK: token same story),
  but NO for class-skill-list recognition -- that's a labor-volume
  hand-authoring problem (GROUNDED_FIGHTER_CLASS_SKILLS precedent), not an
  architecture wall. Folded into risks item 1 (81103838). Since the
  class-skill-list gap is confirmed tractable, dispatched backend to
  ground Wizard's and Rogue's real class-skill lists against the PCGen
  corpus and widen skill_allocation.rs's recognition beyond Fighter-only
  -- in progress.
- QA's second SWARM_REPORT.md consolidation checkpoint (58da213d): 4-race
  ability-modifier gap fully closed end to end, the create-character
  submission bug, the 3-instance Fighter multiclass/race lookup-gap sweep,
  items 6/9/9a resolved. Bar-distance assessment unchanged in shape:
  multiclass breadth (3/11), class-chassis breadth (8/11 missing engines),
  posture narrowness, feat effects remain the real distance, all
  architecture-level.
- QA ran an INTERIM four-check wired-integration audit early (lead's
  request, since remaining bar-distance looks architecture-bounded rather
  than "more bugs to find") against the full combined diff
  (origin/develop...origin/tranche/6, 116 files, ~10.6k insertions):
  clean, zero real violations (c3b5fba8). Lead independently re-extracted
  and re-ran all four checks without reading QA's extraction first, per
  the doctrine's executed-by-QA/re-verified-by-lead requirement --
  confirmed identical result (f3676470). This is an interim checkpoint;
  the closure-time audit still runs separately against the final diff.
- Frontend's GUI sanity pass, first real live use of the fixed driver.sh
  (RUN_DESKTOP_AGENT=frontend, DISPLAY :96): clean first-try launch, no
  collisions. Fresh Dwarf Rogue ("Borin Sanity") -- deliberately a
  different race+class combo than the earlier capstone -- covering
  creation with correct Dwarf adjustments, money Add, money Spend (a
  genuinely different code path from purchase_equipment, first time
  tested), money Spend correctly Blocked on overspend with balance
  unchanged on disk, feat add, level-up to Rogue 2 with feat/money
  surviving intact. No bugs found. Also chased down and explained a
  stale Load-Character list-row observation as a session-scoped list
  cache (clears on app restart), not a real data bug. Dispatched next:
  wire the still-stub Defense tab to the real damage_reduction field
  backend exposed (f7ce289d) but nothing on the frontend side renders
  yet -- scoped narrowly to what's actually computed today, no
  speculative AC/save rendering.
- Dispatched QA next: independent tests/** catalogue coverage for the
  three recently-landed DTO-exposure fixes (DR, purchase_equipment
  atomicity, spells_selected) that currently only have backend's own
  inline tests -- same shape as the earlier BAB/save and Rogue
  reachability catalogue work.
- THIRD QUOTA OUTAGE (all 3 teammates, ~01:40-04:10 ET): resets 4:10am ET,
  same pattern as the two earlier ones. QA's finished-but-uncommitted
  Wizard/Rogue skill-allocation test verified and landed by the lead
  (d35521ec) to avoid losing it during the outage; QA's separate DR test
  (39fef863) had already landed cleanly. Logged and all 3 re-engaged with
  their paused tasks once past reset -- clean recovery again.
- Backend's systematic Fighter-only-grounding sweep (post-reset) came back
  clean, one near-miss (explain_rogue_level1_chassis) traced and correctly
  ruled out (no downstream consumer, cosmetic-only). Adopted as a real
  signal the swarm is down to architecture-level gaps -- logged as risks
  item 21.
- Backend consolidated the three architecture-level gaps (items 1/17/18)
  into a single scoping doc for the operator, docs/release/v0.6/
  future-epic-scoping.md (c30f9b04), with a pointer from the top of
  risks-and-open-questions.md. Side-by-side table (what's blocked/why/fix
  shape/size), explicit answer to "which gaps would bridging the headless/
  corpus-aware wall actually unlock" (attack-bonus + skill-ACP: yes;
  feat-effects + Wizard-non-Human spell math: no, independent problems),
  and a plain recommendation (B/C have no architecture prerequisite and
  can start anytime; A is the expensive, consequential piece worth its own
  design pass). Good, disciplined commit hygiene noted: checked
  `git diff --cached --stat` was empty before staging, confirmed only its
  own 2 files landed, left frontend's and QA's concurrent in-progress
  edits untouched.
- QA's fresh-eyes review of the lead's stewardship commit (d35521ec) found
  a real, independent gap: the pre-existing
  sd20_skill_allocation_class_skill.rs multiclass test used the bare
  string "wizard", not skill_allocation.rs's actual "class:wizard" id, so
  it never really exercised Wizard recognition in a multiclass union --
  correctly still a valid Fighter-plus-unrecognized-class test, just not
  the Wizard proof it looked like. Writing a proper fixture-driven
  multiclass test with the real class ids to close the gap; in progress.
- QA's verification-completeness sweep (self-directed, scanning the full
  develop..HEAD commit history for anything not yet independently
  checked): 5 clean confirmations landed today, each reading the actual
  code/diff rather than trusting the commit message -- the Defense-tab DR
  wiring (risks item 6), the durability status thresholds against real
  PF1 rules (risks item 4), all 11 classes' support-level labeling (risks
  item 15, not just the two already touched), the corpus_derived
  wire-serialization fix (risks item 10, including QA reproducing RED
  themselves by removing the fix and watching it fail before restoring
  it), and the 185-record feat catalog exposure (per-category counts
  re-grepped from the data files themselves: 50+110+8+17=185, plus
  confirming append-safety from the source side, not just the earlier
  empirical test). Continuing down the unverified-commit list
  (Bio/LevelUp persistence commands next).
- QA's sweep continued: independently verified `7694b227` (level-up
  hit-die/feat/skill persistence) -- traced the "exactly one colon"
  validator to its real source in `local_store.rs` and confirmed via
  `git log -S` it's genuinely pre-existing (dated 3 days before this
  swarm started, not invented to justify the fix); read the atomicity
  test directly and confirmed it reloads from disk (not just in-memory
  state) to prove the class-level bump, both choice entries, and all
  three skill allocations landed together in one mutation. 6th clean
  independent verification today. Moving to Bio persistence next.
- QA's sweep continued: independently verified `0ab784df` (bio field
  persistence) -- confirmed the "requires existing character" claim
  traces to a real `SavedCharacterStore::load` check (not the naive
  `root.exists()` the commit deliberately avoided), read the overwrite
  test directly (second save's value alone survives reload, no merge),
  confirmed the disclosed default-when-absent asymmetry in source, and
  correctly cross-attributed the stale-test side-fix to the earlier
  Rogue BAB/save widening rather than treating it as unexplained. 7th
  clean independent verification today (Feats+Bio+LevelUp persistence
  all now independently confirmed). Continuing down the unverified list.
- QA's sweep continued: independently verified `d03bc89d` (multiclass
  classSummary comma-separator fix) -- confirmed the fix itself is
  correct (ran `parseHeldClasses` directly, matches the commit's
  live-verified outcome), but found a real, concrete gap:
  characterProgression.ts has zero test coverage anywhere, including for
  the exact silent-multiclass-corruption bug just fixed there -- nothing
  guards against it recurring. Logged as risks item 22, routed to
  frontend once free (QA correctly didn't pause its own sweep or touch
  frontend's file). Then verified `e0a0bda4` (set_skill_allocations,
  genuine wholesale-replace confirmed via a reversed-order golden-path
  test) and `67490acb`'s Tauri-layer money persistence (rejects negative
  balance before any write, no duplicated/drifted conversion logic --
  calls the same pure function QA's own earlier test already covers).
  9 clean independent verifications today plus the 1 real gap found.
  QA has now covered essentially every backend Tauri persistence command;
  what's left unchecked is the 8 frontend UI-wiring commits themselves
  (does the UI call the right command with the right shape) -- harder to
  verify without live-driving the app. Greenlit to continue into that
  with RUN_DESKTOP_AGENT=qa now that driver.sh is fixed.
- Backend's consolidated full-workspace health check, run fresh rather
  than trusting the many per-commit green reports individually: `codex`
  crate full workspace suite 4239/1 (the 1 failure a wall-clock
  performance assertion squeezed by running 4 heavy checks concurrently,
  confirmed a contention flake by re-running alone: 25/25, 1.13s vs the
  2.34s threshold breach), 0 warnings; release build clean, 0 warnings,
  1m21s; desktop Tauri crate 197/197, 1 pre-existing unrelated warning;
  frontend typecheck clean, 63/63 tests green including against
  frontend's own in-progress edits. Flagged the flaky test honestly
  rather than silently omitting or silently calling it clean -- worth
  noting in the eventual attestation as a documented, re-verified
  non-issue. No code changes, pure verification.
- Items 22-26 (the last wave) all resolved today: characterProgression.ts
  test coverage, LevelUpDialog's feat-pick gap, a new PCGen BAB parity
  dimension, the systemic frontend-test-coverage pattern across 5
  modules, and the Load-list stale-row refresh -- every one independently
  re-verified by QA after landing, no exceptions.
- Lead wrote up the bounded-backlog-exhaustion assessment promised in
  open question 3: all 26 risk items resolved or correctly deferred, two
  clean backend self-scans in a row, a clean consolidated health check,
  QA's completeness sweep essentially exhaustive. Not claiming the alpha
  bar is met -- class/multiclass breadth and the three architecture gaps
  remain real distance -- but the bounded, same-session backlog is
  genuinely exhausted, not abandoned early.
- QA's final comprehensive SWARM_REPORT.md consolidation (34bf1ffa): a
  new "Current-state summary" section distinct from the four incremental
  checkpoints -- one coherent read of where things stand, the defects
  table brought fully current (added the wire-serialization bug as
  defect 10), the bar-distance assessment restated plainly, and a "what's
  genuinely left" section matching the lead's own assessment. Still
  explicitly not signing an attestation.
- Frontend running a comprehensive live-UI smoke test across the whole
  app (all 3 working classes, multiple races, several levels, every
  populated tab) as a genuine end-to-end sanity check broader than any
  single-feature verification pass today -- in progress.

(c) Consolidated status (refreshed 2026-07-24, ~06:15 ET — supersedes the
prior refresh, itself already superseded by everything since)
--------------------------------------------------------------------------
- Fighter, Wizard, and Rogue all reach `Computed`/`Saved` end-to-end
  through the shipped UI, for ANY race (not Human-only), confirmed through
  a full 6-level multiclass walkthrough and multiple independent level-up
  paths.
- Money, equipment purchase (atomically coupled), feats (full list +
  selection), skill allocation, bio, level-up, DR (both initial-load and
  recompute DTOs), spells_selected, and durability/HP (a real tab, live
  disk-confirmed both directions including the Dying/clamped-max
  transitions) are all real, persisted, and exposed/rendered.
- The multiclass `classSummary` parser and its silent-corruption bug both
  now have real regression coverage (item 22); a new
  `combat.base_attack_bonus` PCGen parity dimension is wired end-to-end
  and confirmed against real PCGen output (item 24).
- QA completed an exhaustive verification-completeness sweep today: 16
  distinct commits/areas independently re-verified clean by reading the
  actual code (not commit messages) — Defense tab, durability thresholds
  against real PF1 rules, all 11 classes' support-level labeling,
  wire-serialization with a genuine RED reproduction, feat catalog counts,
  level-up/bio/skill-allocation/money persistence, SkillAllocationDialog,
  Bio editor, Feat picker+Feats tab, Wizard spell routing, Actions tab
  (Money panel's static half also clean; live-UI leg inconclusive on an
  unrelated window-geometry quirk, not a code bug). Real findings from the
  sweep: item 23 (LevelUpDialog never collects a feat pick, stale
  excusing comment — frontend actively closing this now) and item 25 (a
  confirmed systemic pattern: several frontend persistence-wiring modules
  — classSummary parser now fixed, skillsModel, Bio editor, likely others
  — shipped with sound logic but zero dedicated test coverage; not
  re-flagged per-instance once the pattern was clear).
- Backend ran two self-directed scans today beyond the originally-assigned
  work, both real: the DTO-consumption sweep (2 gaps, both now closed)
  and the PCGen parity-comparator scan (1 real dimension added, a clean
  second-pass negative result closing that thread). Currently running a
  consolidated full-workspace health check as closure-readiness prep.
- Known, correctly-deferred future-epic gaps (not attempted this wave,
  flagged for operator review): AC/attack-bonus/skill-posture widening
  (risks item 1), the feat-effects engine's total absence (risks item 17),
  Wizard non-Human spell-math completeness (risks item 18).
- Operator-only open item: starting-wealth-by-class (risks item 7,
  content-provenance/licensing question, not an engineering call).
- driver.sh's cross-agent GUI collision fixed and pushed (f6fe0df2) --
  concurrent agents now namespace DISPLAY_NUM/state/log files via
  RUN_DESKTOP_AGENT=<name>, and cmd_stop's process-kill is DISPLAY-scoped
  instead of global. Unblocks frontend/QA's live GUI verification.
- risks-and-open-questions.md items 6, 9, 9a corrected from stale
  "backlog, non-blocking" to RESOLVED to match what's actually landed.

## Agent Status (2026-07-24, ~15:25 ET)
| Agent | Status | Detail |
|---|---|---|
| backend | idle | closure-readiness pass done: genuinely clean (4271/4271 root-crate tests, 204/204 desktop, 0-hit formal 4-check doctrine audit against the full develop...tranche/6 diff); lead independently re-ran the doctrine checks and the full workspace suite to confirm |
| frontend | idle | closure-readiness pass done and pushed (c014ffec) — one real pre-existing gap found and fixed (WeaponsTab's dead Print button, risks item 30); lead-verified |
| qa | idle | standing by; nothing currently needs independent verification |

**Checkpoint (2026-07-24, ~15:25 ET): bounded swarm-actionable backlog is exhausted.** All three teammates idle with clean queues, not blocked on each other or infrastructure. Everything remaining in SWARM_TASKS.md is blocked/deferred on an operator decision (items 1, 18, 27) or is multi-cycle future-epic scope (class breadth, 12-class starting wealth). Written up plainly as a real checkpoint finding in SWARM_REPORT.md (risks item 31) rather than manufacturing busywork.

**Operator decisions received (2026-07-24, ~15:35 ET):** all three gated items resolved directly by the operator following the checkpoint. Item 27: widen the posture gate to accept any equipment (real engineering scope, backend to scope/sequence first). Item 18: widen Wizard non-Human spell-math now, backend's smaller-than-feared scoping read notwithstanding still needs full empirical verification. Item 1's multi-weapon case: add a real equipment-attachment schema field, sequenced together with item 27 since both need the same underlying data. New wave of real, bounded (item 18) and larger-scope (items 1/27) backend work dispatched.

## Agent Status (2026-07-24, ~16:05 ET)
| Agent | Status | Detail |
|---|---|---|
| backend | working | item 18 fully closed (7c2d2ad2, lead + QA both independently verified); now on the items 1+27 scoping/sequencing plan (equipment-attachment schema + posture-gate widening), report pending |
| frontend | idle | queue clear; will get real work once the items 1/27 scoping plan surfaces UI-facing follow-ups |
| qa | idle | independently re-verified item 18 with a full-function trace, clean, surfaced a sharper finding (the pre-fix bug silently skipped ceiling enforcement entirely, not just the DC display); standing by |

(d) Lead-side verification note (2026-07-24, ~14:20 ET)
------------------------------------------
Resumed after context compaction. Found backend's null-serialization fix
complete but uncommitted in the working tree (character_hub.rs +
risks-and-open-questions.md + 2 regenerated parity-report artifacts).
Independently re-ran the new wire-serialization test and the full desktop
suite (204/204, matching backend's own report) before committing/pushing
as 874df6db. Also independently confirmed
`buildLabelFixtureFreshness.test.ts` (risks item 2's frontend follow-up)
passes with zero commits touching the 3 named fixture files -- it was
never actually broken by the version bump in practice, so that item is
now fully resolved, not just "in flight."

No open blocked-by dependencies between teammates at this checkpoint.

<!-- AUTO-HEARTBEAT-START -->
## Auto Heartbeat (mechanical, cron-driven every 5 min -- lead does not hand-edit this section)
Last mechanical check: 2026-07-24T16:20:01
Latest commit on tranche/6: 28ff0c5e chore(v0.6): auto-heartbeat refresh [skip lead-review] (5 minutes ago) (2026-07-24T16:15:02-04:00)
Last inbox activity (mtime proxy): backend=2026-07-24T15:23:08 frontend=2026-07-24T15:02:02 qa=2026-07-24T15:48:54 lead=2026-07-24T15:52:13
<!-- AUTO-HEARTBEAT-END -->
