---
title: SD-19 Core Rules Spell/Equipment Reachability — Loop Progress
mirrors: /home/ubuntu/workspace/SD-19-core-rules-spell-equipment-reachability-scope-draft.md
created: 2026-07-16T1631
snapshot_as_of: "origin/tranche/3 @ 509b7be (unchanged since cycle-2026-07-17T2400's Human interaction-row judgment-call landing -- see that cycle's own detailed entry below. Re-confirmed live at cycle-2026-07-17T0218, T0221, T0223, T0226, T0229, and cycle-2026-07-17T0232 (terminal-state re-confirmation, NO-OP): direct grep of all 34 support_state_matrix.rs rows independently corroborates 33/34 Supported/ProductVisible with interaction.non_human_any_class.progression_pressure still permanently Unverified/Observed, byte-for-byte matching the 2400 cycle's own closing summary -- no drift, no doc/matrix disagreement. No loop-eligible work exists; this is the loop's expected terminal state per its own 'How the loop will end' section.)"
---

# SD-19 progress doc

This is SD-19's own progress doc, per
`SD-19-core-rules-spell-equipment-reachability-loop-instruction.md` (the
loop-instruction file is self-declared self-sufficient and does not share
mechanics with SD-18's progress doc; this supersedes the older
`scope-draft.md` §4.4 convention which proposed appending to the shared
SD-18 doc — the loop-instruction file is the newer, canonical source for
cycle mechanics per its own §"Required reading").

## Status summary

Both pre-loop slices have **landed** on `tranche/3`:

- Foundation slice (scope doc §1.0): commit `73da1de` — `rules_tables`
  module, `RuleSetId`, CRB `class_tables`/`spell_list`/`equipment_tables`
  (bootstrap coverage: BAB/saves derived from `pilot_compute.rs`'s existing
  formulas for every class's supported level range; one representative
  spell per school and one item per equipment category, copied verbatim
  from the real PCGen corpus). `cargo test --locked --test
  sd19_table_store_foundation` green (4/4).
- Capability slice (scope doc §1.1): commit `569ca55` —
  `pilot_compute_corpus.rs` (`compute_pilot_with_corpus`,
  `CorpusPilotReceipt`, `CorpusDerivedSection`, `TableCellRef`,
  `SchoolCoverage`, `ResolvedEquipment`, `DerivedEquipmentStats`),
  `spell_resolver.rs`, `equipment_resolver.rs`,
  `CharacterInput.spells_selected`, `MatrixSubjectType::School` /
  `::Equipment`, 13 real-corpus fixtures. `cargo test --locked --test
  sd19_seam_shapes_correctness` green (6/6). Operator-confirmed
  (2026-07-16) the seam resolves generically (school/category read off
  the resolved corpus record) rather than dispatching through
  per-school/per-category files, so it is genuinely exercisable
  end-to-end now, not only after a loop cycle lands — see
  `technical-design.md`'s 2026-07-16 review note.

`cargo test --locked` 3378/3378 green (3372 pre-existing + 4 + 2 new,
zero SD-18 regressions); `cargo clippy --locked --tests -- -D warnings`
clean throughout.

The seam and resolvers now exist and are green, so Step 1 eligibility
check #3 no longer blocks §2.4/§2.5 criteria. All 15 acceptance criteria
(1 seam-shapes-correctness — closed by the capability slice itself — + 9
spell schools + 4 equipment categories + 1 foundation — closed by the
foundation slice itself) are ready to re-derive eligibility: `2/15`
closed (the two pre-loop gates), `13/15` open and now eligible for
per-criterion cycles.

Cycle-2026-07-16T2100 landed the first per-criterion cycle: Abjuration
(§2.4) is now Partial/Computed, commit `bc21e7c`. `3/15` criteria
touched (2 pre-loop gates + 1 school), `12/15` still open.

Cycle-2026-07-16T2153 landed the second per-criterion cycle: Conjuration
(§2.4) is now Partial/Computed, commit `cd66045`. `4/15` criteria
touched (2 pre-loop gates + 2 schools), `11/15` still open.

Cycle-2026-07-16T2200 landed the third per-criterion cycle: Divination
(§2.4) is now Partial/Computed, commit `3d1b79b`. `5/15` criteria
touched (2 pre-loop gates + 3 schools), `10/15` still open.

Cycle-2026-07-16T2203 landed the fourth per-criterion cycle: Enchantment
(§2.4) is now Partial/Computed, commit `dede499`. `6/15` criteria
touched (2 pre-loop gates + 4 schools), `9/15` still open.

Cycle-2026-07-16T2209 landed the fifth per-criterion cycle: Evocation
(§2.4) is now Partial/Computed, commit `98bfe11`. `7/15` criteria
touched (2 pre-loop gates + 5 schools), `8/15` still open.

Cycle-2026-07-16T2215 landed the sixth per-criterion cycle: Illusion
(§2.4) is now Partial/Computed, commit `87a39a8`. `8/15` criteria
touched (2 pre-loop gates + 6 schools), `7/15` still open.

Cycle-2026-07-16T2221 landed the seventh per-criterion cycle: Necromancy
(§2.4) is now Partial/Computed, commit `27982fa`. `9/15` criteria
touched (2 pre-loop gates + 7 schools), `6/15` still open.

Cycle-2026-07-16T2227 landed the eighth per-criterion cycle: Transmutation
(§2.4) is now Partial/Computed, commit `078977d`. `10/15` criteria
touched (2 pre-loop gates + 8 schools), `5/15` still open. This is the
last §2.4 school before Universal (5 spells) — the §2.4 sweep is one
cycle from closed.

Cycle-2026-07-16T2233 landed the ninth per-criterion cycle: Universal
(§2.4) is now Partial/Computed, commit `268c987`. `11/15` criteria
touched (2 pre-loop gates + 9 schools), `4/15` still open. The full §2.4
spell-school sweep (9/9 schools) is now closed. The next frontier is §2.5
equipment categories (`arms_armor`, `general`, `magic_items`,
`equipmods`), starting a new test/row shape (equipment resolver +
`MatrixSubjectType::Equipment`, not `School`).

Cycle-2026-07-16T1841 landed the tenth per-criterion cycle, and the first
§2.5 cycle: arms_armor (§2.5) is now Partial/Computed, commit `e08607e`.
`12/15` criteria touched (2 pre-loop gates + 9 schools + 1 equipment
category), `3/15` still open. Representative 3-item sample (Longsword,
Banded Mail, Armor Spikes); only Longsword grounds through the foundation
slice's bootstrap table cell. The next frontier is `general` (§2.5).

Cycle-2026-07-16T2340 landed the eleventh per-criterion cycle: general
(§2.5) is now Partial/Computed, commit `eaaa6b7`. `13/15` criteria
touched (2 pre-loop gates + 9 schools + 2 equipment categories), `2/15`
still open. Representative 3-item sample (Backpack, Torch, Waterskin);
only Backpack grounds through the foundation slice's bootstrap table
cell. The next frontier is `magic_items` (§2.5).

Cycle-2026-07-16T1858 landed the twelfth per-criterion cycle: magic_items
(§2.5) is now Partial/Computed, commit `1689b16`. `14/15` criteria
touched (2 pre-loop gates + 9 schools + 3 equipment categories), `1/15`
still open (`equipmods`, the last §2.5 category and the last open SD-19
criterion). Representative 3-item sample (Amulet of Natural Armor +1,
Belt of Giant Strength +2, Ring of Protection +1) — deliberately drawn
from non-`.COPY=` records; none grounds through the foundation slice's
bootstrap table cell (`EQUIPMENT_TABLES`' single magic_items entry,
`"Potion of Aid"`, was not in the sample by design). This cycle
discovered (but per its file-touch scope did not fix) a category-wide
parser limitation: a large share of `cr_equip_magic_items.lst` (scrolls,
wands, potions) use PCGen's `.COPY=` naming, and `equipment.rs`'s by-name
record merge (intended for `cr_equip_general.lst`'s genuine same-name
continuation rows) instead collapses every distinct `.COPY=` item sharing
a base word ("Wand", "Potion", ...) into one merged `EquipmentRecord`
carrying many distinct `KEY:` tokens; `equipment_id_resolve`'s
`equipment_key_token` helper returns only the first such token, so only
the alphabetically-first item under each merged name resolves today
(discovered when `Wand of Magic Missile` failed to resolve while `Potion
of Aid` — alphabetically first among potions — happened to succeed). This
is a parser-level defect in `src/pcgen_import/lst_parser/equipment.rs`
(SD-17's lane), not a resolver-normalization edge case this cycle's
file-touch scope could fix; see the informational Open Blockers note
below.

Cycle-2026-07-16T2359 landed the thirteenth per-criterion cycle, and the
last: equipmods (§2.5) is now Partial/Computed, commit `c46c9b6`. `15/15`
criteria touched (2 pre-loop gates + 9 schools + 4 equipment categories),
`0/15` still open. Representative 3-item sample (Masterwork (Weapon),
Brace, Disarm; cr_equipmods.lst) — none grounds through the foundation
slice's bootstrap table cell (`EQUIPMENT_TABLES`' single equipmods entry,
`"Material ~ Cloth"`, was not in the sample by design). This cycle
checked `cr_equipmods.lst` for the `.COPY=`-style merge collision
discovered in the magic_items cycle per that cycle's own handoff note,
and found the collision is not limited to `.COPY=` rows here: two plain
(non-`.COPY=`) records share the name `"Cloth"` (`KEY:Material ~ Cloth`
and `KEY:Artisan's Tools (Cloth)`), which is exactly why the foundation
slice's own bootstrap fixture ("Material ~ Cloth") happens to resolve —
it is the first-inserted token under the merged name. A name-frequency
scan across the whole file confirmed this cycle's three sample names
are each unique, avoiding the collision. This closes the full §2.5
equipment-category sweep (4/4) and **all 15 SD-19 acceptance criteria's
per-cycle-eligible work** (2 pre-loop gates + 9 §2.4 schools + 4 §2.5
equipment categories). Every row is `Partial/Computed`; the remaining
work per criterion is the operator-driven UI-surfacing gate to
`Supported/Product-visible` (not loop-driven, per the loop instruction's
own definition) plus the structural gaps named in `decisions.md` §1.3
(slot math, spellbook engine, spell save DCs, broad equipment-effect
computation) that no SD-19 cycle can close. No further per-criterion
cycle is eligible under this loop's own eligibility rule until the
operator authorizes a new tranche-level scope (e.g. the SD-17-lane
`.COPY=`/by-name-merge parser fix flagged across the last two cycles, or
UI-surfacing work).

## §2.4 Spell schools (9)

- Abjuration — Partial/Computed | commit bc21e7c | card t_6ed01b2d | cycle-2026-07-16T2100. All 73 real-corpus Abjuration spells (cr_spells.lst) resolve via `spell_id_resolve` and reach `corpus_derived.school_coverage[Abjuration]` through `compute_pilot_with_corpus`; row is not yet Supported/Product-visible (operator UI surfacing still required; slot math/spellbook posture/DCs stay permanently out of scope per decisions.md §1.3).
- Conjuration — Partial/Computed | commit cd66045 | card t_b771eaf6 | cycle-2026-07-16T2153. All 116 real-corpus Conjuration spells (cr_spells.lst) resolve via `spell_id_resolve` and reach `corpus_derived.school_coverage[Conjuration]` through `compute_pilot_with_corpus`; row is not yet Supported/Product-visible (operator UI surfacing still required; slot math/spellbook posture/DCs stay permanently out of scope per decisions.md §1.3).
- Divination — Partial/Computed | commit 3d1b79b | card t_cfba1278 | cycle-2026-07-16T2200. All 50 real-corpus Divination spells (cr_spells.lst) resolve via `spell_id_resolve` and reach `corpus_derived.school_coverage[Divination]` through `compute_pilot_with_corpus`; row is not yet Supported/Product-visible (operator UI surfacing still required; slot math/spellbook posture/DCs stay permanently out of scope per decisions.md §1.3).
- Enchantment — Partial/Computed | commit dede499 | card t_a6ad2615 | cycle-2026-07-16T2203. All 60 real-corpus Enchantment spells (cr_spells.lst) resolve via `spell_id_resolve` and reach `corpus_derived.school_coverage[Enchantment]` through `compute_pilot_with_corpus`; row is not yet Supported/Product-visible (operator UI surfacing still required; slot math/spellbook posture/DCs stay permanently out of scope per decisions.md §1.3).
- Evocation — Partial/Computed | commit 98bfe11 | card t_effdd6c2 | cycle-2026-07-16T2209. All 87 real-corpus Evocation spells (cr_spells.lst) resolve via `spell_id_resolve` and reach `corpus_derived.school_coverage[Evocation]` through `compute_pilot_with_corpus`; row is not yet Supported/Product-visible (operator UI surfacing still required; slot math/spellbook posture/DCs stay permanently out of scope per decisions.md §1.3).
- Illusion — Partial/Computed | commit 87a39a8 | card t_c7b87479 | cycle-2026-07-16T2215. All 47 real-corpus Illusion spells (cr_spells.lst) resolve via `spell_id_resolve` and reach `corpus_derived.school_coverage[Illusion]` through `compute_pilot_with_corpus`; row is not yet Supported/Product-visible (operator UI surfacing still required; slot math/spellbook posture/DCs stay permanently out of scope per decisions.md §1.3).
- Necromancy — Partial/Computed | commit 27982fa | card t_4fe73701 | cycle-2026-07-16T2221. All 62 real-corpus Necromancy spells (cr_spells.lst) resolve via `spell_id_resolve` and reach `corpus_derived.school_coverage[Necromancy]` through `compute_pilot_with_corpus`; row is not yet Supported/Product-visible (operator UI surfacing still required; slot math/spellbook posture/DCs stay permanently out of scope per decisions.md §1.3).
- Transmutation — Partial/Computed | commit 078977d | card t_dd9c1ae7 | cycle-2026-07-16T2227. All 152 real-corpus Transmutation spells (cr_spells.lst) resolve via `spell_id_resolve` and reach `corpus_derived.school_coverage[Transmutation]` through `compute_pilot_with_corpus`; row is not yet Supported/Product-visible (operator UI surfacing still required; slot math/spellbook posture/DCs stay permanently out of scope per decisions.md §1.3).
- Universal — Partial/Computed | commit 268c987 | card t_12cca058 | cycle-2026-07-16T2233. All 5 real-corpus Universal spells (cr_spells.lst) resolve via `spell_id_resolve` and reach `corpus_derived.school_coverage[Universal]` through `compute_pilot_with_corpus`; row is not yet Supported/Product-visible (operator UI surfacing still required; slot math/spellbook posture/DCs stay permanently out of scope per decisions.md §1.3). §2.4 sweep (9/9 schools) is now closed.

## §2.5 Equipment categories (4)

- arms_armor — Partial/Computed | commit e08607e | card t_47eca99f | cycle-2026-07-16T1841. Representative sample (Longsword, Banded Mail, Armor Spikes; cr_equip_arms_armor.lst) resolves via `equipment_id_resolve` and reaches `corpus_derived.equipped_items` through `compute_pilot_with_corpus`; only Longsword grounds through the foundation slice's bootstrap table cell (the other two sample items' `table_cell` stays `None`); row is not yet Supported/Product-visible (operator UI surfacing still required; `derived_stats` population is a documented capability-slice non-goal, not this cycle's job).
- general — Partial/Computed | commit eaaa6b7 | card t_a9a39797 | cycle-2026-07-16T2340. Representative sample (Backpack, Torch, Waterskin; cr_equip_general.lst) resolves via `equipment_id_resolve` and reaches `corpus_derived.equipped_items` through `compute_pilot_with_corpus`; only Backpack grounds through the foundation slice's bootstrap table cell (the other two sample items' `table_cell` stays `None`); row is not yet Supported/Product-visible (operator UI surfacing still required; `derived_stats` population is a documented capability-slice non-goal, not this cycle's job).
- magic_items — Partial/Computed | commit 1689b16 | card t_b165e9ce | cycle-2026-07-16T1858. Representative sample (Amulet of Natural Armor +1, Belt of Giant Strength +2, Ring of Protection +1; cr_equip_magic_items.lst) resolves via `equipment_id_resolve` and reaches `corpus_derived.equipped_items` through `compute_pilot_with_corpus`; none grounds through the foundation slice's bootstrap table cell (deliberately drawn from non-`.COPY=` records — see Open Blockers for the category-wide `.COPY=` parser-merge limitation discovered this cycle); row is not yet Supported/Product-visible (operator UI surfacing still required; `derived_stats` population is a documented capability-slice non-goal, not this cycle's job).
- equipmods — Partial/Computed | commit c46c9b6 | card t_bcdb8ecf | cycle-2026-07-16T2359. Representative sample (Masterwork (Weapon), Brace, Disarm; cr_equipmods.lst) resolves via `equipment_id_resolve` and reaches `corpus_derived.equipped_items` through `compute_pilot_with_corpus`; none grounds through the foundation slice's bootstrap table cell (deliberately drawn from names verified unique across the file — see the status summary's note on the `"Cloth"` by-name merge collision this cycle checked for and avoided); row is not yet Supported/Product-visible (operator UI surfacing still required; `derived_stats` population is a documented capability-slice non-goal, not this cycle's job). Closes the full §2.5 sweep (4/4) and all 15 SD-19 acceptance criteria's per-cycle-eligible work.

## Open blockers

### cycle-2026-07-16T1947 | no-op re-derivation — full 15/15 frontier still exhausted, no new operator directive (not a hard stop, not a defect)

**Condition:** Ninth consecutive live re-derivation reaching the same
conclusion as cycle-2026-07-16T1944/T1941/T1936/T1940/T1933/T1935/T1928/T1923.
Re-derived live rather than trusting any prior summary:

- `git fetch origin tranche/3` + `git log origin/tranche/3 --oneline -8`:
  HEAD still `c46c9b6`, matching `snapshot_as_of`. `git status --porcelain`
  0 lines; `git worktree list --porcelain` shows only the primary
  worktree; current branch `tranche/3`.
- In-flight check: `ps -eo pid,ppid,etime,stat,cmd | grep claude` showed
  one `claude -p` process (pid 3093341) running the identical SD-19 loop
  prompt. Traced this shell's own ancestry (`ps -o pid,ppid,cmd -p $$`)
  directly to pid 3093341 → ppid 2604107 (`sd19-loop-supervisor.sh`),
  confirming that process is this session's own top-level process, not a
  second competing claim — no Hard-stop-#3 collision.
- Code-level re-verification: `grep -n "MatrixSubjectType::School(\|
  MatrixSubjectType::Equipment(" src/rules_core/support_state_matrix.rs`
  confirms all 9 `School(...)` rows and all 4 `Equipment(...)` rows
  present. `ls tests/sd19_*.rs` shows all 15 per-criterion/slice test
  files present, matching this doc's own §2.4/§2.5 tracking exactly — no
  drift. Directly inspected `support_state`/`evidence_tier` immediately
  following each of the 13 `School(`/`Equipment(` row constructors: all 13
  still read `support_state: SupportState::Partial, evidence_tier:
  EvidenceTier::Computed` — none has silently advanced to `ProductVisible`.
- Read the two required SD-18 investigation-cycle sections
  (`cycle-2026-07-15T0300` §3.4, `cycle-2026-07-15T0400` §3.5, both in
  `programs/codex/requirements/SD-18-core-rules-breadth/artifacts/SD-18-core-rules-breadth-progress.md`,
  found at lines 7721 and 7861) as read-only reference; both describe the
  pre-capability-slice structural blocker (no corpus-aware compute path,
  no spell-content selection mechanism), fully superseded by the SD-19
  foundation + capability slices already landed. No write made to that
  file.

### cycle-2026-07-16T1944 | no-op re-derivation — full 15/15 frontier still exhausted, no new operator directive (not a hard stop, not a defect)

**Condition:** Eighth consecutive live re-derivation reaching the same
conclusion as cycle-2026-07-16T1941/T1936/T1940/T1933/T1935/T1928/T1923.
Re-derived live rather than trusting any prior summary:

- `git fetch origin tranche/3` + `git log origin/tranche/3 --oneline -8`:
  HEAD still `c46c9b6`, matching `snapshot_as_of`. `git status --porcelain`
  0 lines; `git worktree list --porcelain` shows only the primary
  worktree; current branch `tranche/3`.
- In-flight check: `ps -eo pid,ppid,etime,stat,cmd | grep claude` showed
  one `claude -p` process (pid 3091983) running the identical SD-19 loop
  prompt. Traced this shell's own ancestry (`ps -o pid,ppid,cmd -p $$` →
  parent → grandparent) directly to pid 3091983 → ppid 2604107
  (`sd19-loop-supervisor.sh`), confirming that process is this session's
  own top-level process, not a second competing claim — no Hard-stop-#3
  collision.
- Code-level re-verification: `grep -n "MatrixSubjectType::School(\|
  MatrixSubjectType::Equipment(" src/rules_core/support_state_matrix.rs`
  confirms all 9 `School(...)` rows and all 4 `Equipment(...)` rows
  present. `ls tests/sd19_*.rs` shows all 15 per-criterion/slice test
  files present, matching this doc's own §2.4/§2.5 tracking exactly — no
  drift. Directly inspected `evidence_tier`/`support_state` immediately
  following each of the 13 `School(`/`Equipment(` row constructors: all 13
  still read `support_state: SupportState::Partial, evidence_tier:
  EvidenceTier::Computed` — none has silently advanced to `ProductVisible`
  (the one `EvidenceTier::ProductVisible` occurrence in the file, at line
  6661, belongs to the unrelated pre-existing SD-18 §3.3
  `MatrixSubjectType::Interaction` row, not an SD-19 row).
- Read the two required SD-18 investigation-cycle sections
  (`cycle-2026-07-15T0300` §3.4, `cycle-2026-07-15T0400` §3.5, both in
  `programs/codex/requirements/SD-18-core-rules-breadth/artifacts/SD-18-core-rules-breadth-progress.md`,
  found at lines 2797 and 2817) as read-only reference; both describe the
  pre-capability-slice structural blocker (no corpus-aware compute path,
  no spell-content selection mechanism), fully superseded by the SD-19
  foundation + capability slices already landed. No write made to that
  file.

**Why not self-heal / no commit this cycle:** Identical reasoning to the
prior seven no-op entries: no next granular work-unit exists under Step
1/Step 2 for any of the 15 criteria. Every §2.4 school already landed
100% of its corpus spells; every §2.5 category already landed its
representative sample per the scope doc's own bounded acceptance
criteria. Inventing new loop-routed work (widening equipment samples,
fixing the SD-17-lane `.COPY=`/by-name-merge parser defect, or
UI-surfacing) would itself be the forbidden tranche-level decision.

**Resolution required:** Unchanged — operator decides the next
tranche-level scope. Not a blocker for SD-19's own loop-routed scope; all
15 criteria remain at `Partial/Computed`.

### cycle-2026-07-16T1941 | no-op re-derivation — full 15/15 frontier still exhausted, no new operator directive (not a hard stop, not a defect)

**Condition:** Seventh consecutive live re-derivation reaching the same
conclusion as cycle-2026-07-16T1936/T1940/T1933/T1935/T1928/T1923. Re-derived
live rather than trusting any prior summary:

- `git fetch origin tranche/3` + `git log origin/tranche/3 --oneline -5`:
  HEAD still `c46c9b6`, matching `snapshot_as_of`. `git status --porcelain`
  0 lines; `git worktree list --porcelain` shows only the primary
  worktree.
- In-flight check: `ps -eo pid,ppid,etime,stat,cmd | grep claude` showed
  one `claude -p` process (pid 3090902) running the identical SD-19 loop
  prompt. Traced this shell's own `$PPID` chain (`ps -o pid,ppid,cmd -p
  $$` → `$PPID` → grandparent) directly to pid 3090902 → ppid 2604107
  (`sd19-loop-supervisor.sh`), confirming that process is this session's
  own top-level process, not a second competing claim — no Hard-stop-#3
  collision.
- Code-level re-verification: `grep -n "MatrixSubjectType::School(\|
  MatrixSubjectType::Equipment(" src/rules_core/support_state_matrix.rs`
  confirms all 9 `School(...)` rows and all 4 `Equipment(...)` rows
  present. `ls tests/sd19_*.rs` shows all 15 per-criterion/slice test
  files present, matching this doc's own §2.4/§2.5 tracking exactly — no
  drift. Additionally spot-checked `evidence_tier` on every §2.4/§2.5 row
  directly (`grep -n evidence_tier` around each `School(`/`Equipment(`
  block): all 13 rows still read `EvidenceTier::Computed` with
  `next_required_uplift: "operator UI surfacing to promote evidence_tier
  to ..."` — none has silently drifted to `ProductVisible`, confirming no
  row is closer to `Supported/Product-visible` than the last cycle found.
- Read the two required SD-18 investigation-cycle sections
  (`cycle-2026-07-15T0300` §3.4, `cycle-2026-07-15T0400` §3.5, both in
  `programs/codex/requirements/SD-18-core-rules-breadth/artifacts/SD-18-core-rules-breadth-progress.md`,
  found at lines 2797 and 2817) as read-only reference; both describe the
  pre-capability-slice structural blocker (no corpus-aware compute path,
  no spell-content selection mechanism), fully superseded by the SD-19
  foundation + capability slices already landed. No write made to that
  file.

**Why not self-heal / no commit this cycle:** Identical reasoning to the
prior six no-op entries: no next granular work-unit exists under Step
1/Step 2 for any of the 15 criteria. Every §2.4 school already landed
100% of its corpus spells; every §2.5 category already landed its
representative sample per the scope doc's own bounded acceptance
criteria. Inventing new loop-routed work (widening equipment samples,
fixing the SD-17-lane `.COPY=`/by-name-merge parser defect, or
UI-surfacing) would itself be the forbidden tranche-level decision.

**Resolution required:** Unchanged — operator decides the next
tranche-level scope. Not a blocker for SD-19's own loop-routed scope; all
15 criteria remain at `Partial/Computed`.

### cycle-2026-07-16T1940 | no-op re-derivation — full 15/15 frontier still exhausted, no new operator directive (not a hard stop, not a defect)

**Condition:** Sixth consecutive live re-derivation reaching the same
conclusion as cycle-2026-07-16T1933/T1935/T1928/T1923 and the
cycle-2026-07-16T2359 closing note. Re-derived live rather than trusting
any prior summary:

- `git fetch origin tranche/3` + `git log origin/tranche/3 --oneline -10`:
  HEAD still `c46c9b6`, matching `snapshot_as_of`. `git status --porcelain`
  0 lines; `git worktree list --porcelain` shows only the primary
  worktree.
- In-flight check: `ps -eo pid,etime,stat,cmd | grep claude` showed one
  `claude -p` process (pid 3089050) running the identical SD-19 loop
  prompt — this session's own top-level process, not a second competing
  process. No Hard-stop-#3 collision.
- Code-level re-verification: `grep -n "MatrixSubjectType::School(\|MatrixSubjectType::Equipment("
  src/rules_core/support_state_matrix.rs` confirms all 9 `School(...)`
  rows and all 4 `Equipment(...)` rows present. `ls tests/ | grep ^sd19_`
  shows all 15 per-criterion/slice test files present, matching this
  doc's own §2.4/§2.5 tracking exactly — no drift.
- Read the two required SD-18 investigation-cycle sections
  (`cycle-2026-07-15T0300` §3.4 at line 7721, `cycle-2026-07-15T0400`
  §3.5 at line 7927 of
  `programs/codex/requirements/SD-18-core-rules-breadth/artifacts/SD-18-core-rules-breadth-progress.md`)
  as read-only reference; both remain consistent with, not contradictory
  to, the conclusion that the capability slice's seam already closed the
  structural gap they identified. No write made to that file.

**Why not self-heal / no commit this cycle:** Identical reasoning to the
prior four no-op entries: no next granular work-unit exists under Step
1/Step 2 for any of the 15 criteria. Every §2.4 school already landed
100% of its corpus spells; every §2.5 category already landed its
representative sample per the scope doc's own bounded acceptance
criteria. Inventing new loop-routed work (widening equipment samples,
fixing the SD-17-lane `.COPY=`/by-name-merge parser defect, or
UI-surfacing) would itself be the forbidden tranche-level decision.

**Resolution required:** Unchanged — operator decides the next
tranche-level scope. Not a blocker for SD-19's own loop-routed scope; all
15 criteria remain at `Partial/Computed`.

### cycle-2026-07-16T1933 | no-op re-derivation — full 15/15 frontier still exhausted, no new operator directive (not a hard stop, not a defect)

**Condition:** Fifth consecutive live re-derivation reaching the same
conclusion as cycle-2026-07-16T2359's closing note and the
cycle-2026-07-16T1923/T1928/T1935 re-checks. Re-derived live rather than
trusting any prior summary:

- `git fetch origin tranche/3` + `git log origin/tranche/3 --oneline -5`:
  HEAD still `c46c9b6`, matching `snapshot_as_of`. `git status --porcelain`
  0 lines; `git worktree list --porcelain` shows only the primary
  worktree.
- In-flight check: `ps -eo pid,etime,stat,cmd | grep claude` showed one
  `claude -p` process (pid 3088206) running the identical SD-19 loop
  prompt. Traced this shell's own ancestry (`ps -o pid,ppid,cmd -p $$` up
  through `$PPID`) and confirmed pid 3088206 is this very session's
  top-level process (ppid 2604107), not a second competing process. No
  Hard-stop-#3 collision.
- Code-level re-verification: `grep -c "MatrixSubjectType::School\|MatrixSubjectType::Equipment"
  src/rules_core/support_state_matrix.rs` confirms all 9 `School(...)`
  rows and all 4 `Equipment(...)` rows present. `ls tests/ | grep ^sd19_`
  shows all 15 per-criterion/slice test files present (9 school + 4
  equipment + seam-shapes-correctness + table-store-foundation), matching
  this doc's own §2.4/§2.5 tracking exactly — no drift.
- Read the two required SD-18 investigation-cycle sections
  (`cycle-2026-07-15T0300` §3.4, `cycle-2026-07-15T0400` §3.5, at
  `programs/codex/requirements/SD-18-core-rules-breadth/artifacts/SD-18-core-rules-breadth-progress.md:7721`
  and `:7927`) as read-only reference; both remain consistent with, not
  contradictory to, the conclusion that the capability slice's seam
  already closed the structural gap they identified. No write made to
  that file.

**Why not self-heal / no commit this cycle:** Identical reasoning to the
cycle-2026-07-16T1935/T1928/T1923 entries: no next granular work-unit
exists under Step 1/Step 2 for any of the 15 criteria. Every §2.4 school
already landed 100% of its corpus spells; every §2.5 category already
landed its representative sample per the scope doc's own bounded
acceptance criteria. Inventing new loop-routed work (widening equipment
samples, fixing the SD-17-lane `.COPY=`/by-name-merge parser defect, or
UI-surfacing) would itself be the forbidden tranche-level decision.

**Resolution required:** Unchanged — operator decides the next
tranche-level scope. Not a blocker for SD-19's own loop-routed scope; all
15 criteria remain at `Partial/Computed`. Future cycles should continue
the cheap live-state check (git HEAD + in-flight + matrix/test-file grep)
before standing down again, rather than re-deriving the full history each
time.

### cycle-2026-07-16T1935 | no-op re-derivation — full 15/15 frontier still exhausted, no new operator directive (not a hard stop, not a defect)

**Condition:** Fourth consecutive live re-derivation reaching the same
conclusion as cycle-2026-07-16T2359's closing note, cycle-2026-07-16T1923's
re-check, and cycle-2026-07-16T1928's re-check. Re-derived live rather than
trusting any prior summary:

- `git fetch origin tranche/3` + `git log origin/tranche/3 --oneline -8`:
  HEAD still `c46c9b6`, matching `snapshot_as_of`. `git status --porcelain`
  0 lines; `git worktree list --porcelain` shows only the primary
  worktree.
- In-flight check: `ps -eo pid,ppid,etime,stat,cmd | grep claude` showed
  one `claude -p` process (pid 3087520, ppid 2604107 — the standing
  supervisor `sd19-loop-supervisor.sh`) running the identical SD-19 loop
  prompt. Traced this shell's own ancestry (`ps -o pid,ppid,cmd -p $$`)
  and confirmed pid 3087520 is this very session's direct parent, not a
  second competing process. No Hard-stop-#3 collision.
- Code-level re-verification: `grep -n
  "MatrixSubjectType::School(\|MatrixSubjectType::Equipment("
  src/rules_core/support_state_matrix.rs` shows all 9 `School(...)` rows
  and all 4 `Equipment(...)` rows present. `ls tests/ | grep sd19` shows
  all 15 per-criterion/slice test files present, matching this doc's own
  §2.4/§2.5 tracking exactly — no drift.
- Read the two required SD-18 investigation-cycle sections
  (`cycle-2026-07-15T0300` §3.4, `cycle-2026-07-15T0400` §3.5, archived at
  `programs/codex/requirements/SD-18-core-rules-breadth/artifacts/SD-18-core-rules-breadth-progress.md`)
  as read-only reference; both remain consistent with, not contradictory
  to, the conclusion that the capability slice's seam already closed the
  structural gap they identified. No write made to that file.

**Why not self-heal / no commit this cycle:** Identical reasoning to the
cycle-2026-07-16T1928 and cycle-2026-07-16T1923 entries: no next granular
work-unit exists under Step 1/Step 2 for any of the 15 criteria. Every
§2.4 school already landed 100% of its corpus spells; every §2.5 category
already landed its representative sample per the scope doc's own bounded
acceptance criteria. Inventing new loop-routed work (widening equipment
samples, fixing the SD-17-lane `.COPY=`/by-name-merge parser defect, or
UI-surfacing) would itself be the forbidden tranche-level decision.

**Resolution required:** Unchanged — operator decides the next
tranche-level scope. Not a blocker for SD-19's own loop-routed scope; all
15 criteria remain at `Partial/Computed`. Future cycles should continue
the cheap live-state check (git HEAD + in-flight + matrix/test-file grep)
before standing down again, rather than re-deriving the full history each
time.

### cycle-2026-07-16T1928 | no-op re-derivation — full 15/15 frontier still exhausted, no new operator directive (not a hard stop, not a defect)

**Condition:** Third consecutive live re-derivation reaching the same
conclusion as cycle-2026-07-16T2359's closing note and
cycle-2026-07-16T1923's re-check. Re-derived live rather than trusting
either prior summary:

- `git fetch origin tranche/3` + `git log origin/tranche/3 --oneline -8`:
  HEAD still `c46c9b6`, matching `snapshot_as_of`. `git status --porcelain`
  0 lines; `git worktree list --porcelain` shows only the primary
  worktree.
- In-flight check: `ps -eo pid,ppid,etime,stat,cmd | grep claude` showed
  one `claude -p` process (pid 3086498, ppid 2604107 — the standing
  supervisor `sd19-loop-supervisor.sh`) running the identical SD-19 loop
  prompt. Traced this shell's own ancestry (`ps -o pid,ppid,cmd -p $$`)
  and confirmed pid 3086498 is this very session's direct parent, not a
  second competing process. No Hard-stop-#3 collision.
- Code-level re-verification: `grep -n
  "MatrixSubjectType::School(\|MatrixSubjectType::Equipment("
  src/rules_core/support_state_matrix.rs` shows all 9 `School(...)` rows
  and all 4 `Equipment(...)` rows present. `ls tests/ | grep sd19` shows
  all 15 per-criterion/slice test files present, matching this doc's own
  §2.4/§2.5 tracking exactly — no drift.
- Read `decisions.md` §9 (source-book subdirectory pattern) and the two
  required SD-18 investigation-cycle sections
  (`cycle-2026-07-15T0300` §3.4, `cycle-2026-07-15T0400` §3.5, archived at
  `programs/codex/requirements/SD-18-core-rules-breadth/artifacts/SD-18-core-rules-breadth-progress.md`)
  as read-only reference; both remain consistent with, not contradictory
  to, the conclusion that the capability slice's seam already closed the
  structural gap they identified. No write made to that file.

**Why not self-heal / no commit this cycle:** Identical reasoning to the
cycle-2026-07-16T1923 entry below: no next granular work-unit exists
under Step 1/Step 2 for any of the 15 criteria. Every §2.4 school already
landed 100% of its corpus spells; every §2.5 category already landed its
representative sample per the scope doc's own bounded acceptance
criteria. Inventing new loop-routed work (widening equipment samples,
fixing the SD-17-lane `.COPY=`/by-name-merge parser defect, or
UI-surfacing) would itself be the forbidden tranche-level decision.

**Resolution required:** Unchanged — operator decides the next
tranche-level scope. Not a blocker for SD-19's own loop-routed scope; all
15 criteria remain at `Partial/Computed`. Future cycles should continue
the cheap live-state check (git HEAD + in-flight + matrix/test-file grep)
before standing down again, rather than re-deriving the full history each
time.

### cycle-2026-07-16T1923 | no-op re-derivation — full 15/15 frontier confirmed exhausted, no new operator directive (not a hard stop, not a defect)

**Condition:** This cycle re-derived eligibility live per Step 1, exactly
as the prior cycle's own handoff instructed, rather than trusting the
prior cycle's summary as current. Live checks performed:

- `git fetch origin tranche/3` + `git log origin/tranche/3 --oneline -8`:
  HEAD is `c46c9b6` (equipmods), matching the progress doc's
  `snapshot_as_of`. `git status --porcelain` is 0 lines; `git worktree
  list` shows only the primary worktree.
- In-flight check: `ps -eo pid,etime,stat,cmd | grep claude` shows this
  cycle's own `claude -p` process (pid 3085123) as a direct child of the
  standing supervisor (`sd19-loop-supervisor.sh`, pid 2604107) — i.e.
  this session *is* the supervisor's currently-live cycle, not a second
  competing process. No other `claude -p` process is running the SD-19
  loop prompt. No Hard-stop-#3 collision.
- Code-level re-verification (independent of the progress doc's prose):
  `grep -n "MatrixSubjectType::School(\|MatrixSubjectType::Equipment("
  src/rules_core/support_state_matrix.rs` shows all 9 `School(...)` rows
  (Abjuration, Conjuration, Divination, Enchantment, Evocation, Illusion,
  Necromancy, Transmutation, Universal) and all 4 `Equipment(...)` rows
  (ArmsArmor, General, MagicItems, Equipmods) present. `ls tests/ | grep
  sd19` shows all 13 per-criterion test files
  (`sd19_school_<school>.rs` x9, `sd19_equipment_<category>.rs` x4) plus
  the two pre-loop-slice tests. This matches the progress doc's own
  §2.4/§2.5 tracking sections exactly — no drift, no discrepancy.
- Read the two required SD-18 investigation-cycle sections as read-only
  reference (`cycle-2026-07-15T0300` §3.4 and `cycle-2026-07-15T0400`
  §3.5 in `SD-18-core-rules-breadth-progress.md`, now archived at
  `programs/codex/requirements/SD-18-core-rules-breadth/artifacts/`):
  both confirm the historical structural gap (`pilot_compute.rs` had no
  corpus-aware compute path) that SD-19's capability slice exists to
  close — consistent with, not contradicting, this cycle's conclusion
  that the seam now exists and every §2.4/§2.5 criterion has already had
  its per-cycle-eligible work landed through it.

**Why not self-heal / no commit this cycle:** There is no next granular
work-unit left to pick under Step 1/Step 2. Every §2.4 school already
landed its full defined scope (100% of the school's corpus spells, per
the scope doc's acceptance criterion). Every §2.5 category already
landed its full defined scope (a representative sample, per the scope
doc's acceptance criterion — the bar is explicitly "a representative
sample," not exhaustive coverage). Widening equipment samples toward
more-exhaustive coverage, fixing the SD-17-lane `.COPY=`/by-name-merge
parser defect, or doing operator UI-surfacing work are all named next
steps in the prior cycle's own summary — none is a per-cycle-eligible
loop-routed criterion under this instruction file's own rules; inventing
one from inside a cycle would itself be the forbidden tranche-level
decision the instruction file repeatedly warns against.

**Resolution required:** Operator decides the next tranche-level scope
(UI-surfacing pass, or the SD-17-lane parser fix). Not a blocker for
SD-19's own loop-routed scope — all 15 criteria remain at
`Partial/Computed`, exactly where the prior cycle left them. No action
needed from a future cycle beyond re-confirming this conclusion still
holds (git HEAD unchanged, no new operator directive) before standing
down again.

### cycle-2026-07-16T2359 | informational, non-blocking — `.COPY=`-independent by-name merge collision confirmed present in `cr_equipmods.lst` too (routed to SD-17's lane, not a hard stop)

**Condition:** Not a blocker for this cycle — a commit landed (`c46c9b6`),
so this entry is informational only, extending the prior cycle's
(cycle-2026-07-16T1858) `.COPY=` finding with a broader confirmation:
`equipment.rs`'s `open_record` by-name merge collision is not limited to
PCGen's `.COPY=` naming convention. While selecting this cycle's
representative sample, a name-frequency scan across the whole
`cr_equipmods.lst` file found two plain (non-`.COPY=`) records sharing
the exact name `"Cloth"`: `Cloth` (`KEY:Material ~ Cloth`, line 10) and
`Cloth` (`KEY:Artisan's Tools (Cloth)`, line 78). These merge into a
single `EquipmentRecord` under `open_record`'s `(kind, name)` key exactly
as the `.COPY=` case does; `equipment_key_token`'s `.find()` returns only
the first `KEY:` token inserted (`Material ~ Cloth`, since line 10
precedes line 78), so a request for `Material ~ Cloth` still resolves
correctly (which is why the foundation slice's own bootstrap fixture,
keyed `"Material ~ Cloth"`, has worked all along), but a request for
`Artisan's Tools (Cloth)` would not resolve to its own identity via
`equipment_id_resolve`'s primary (verbatim-KEY) lookup path (which only
ever sees the merged record's first token), and could fall through to
the normalized-name path and return the merged record under its
first-token identity instead of `None` — a distinct failure shape from
the `.COPY=` case (silent misresolution risk vs. simple non-resolution).

**Why not self-heal / not fixed this cycle:** Same reasoning as the
magic_items cycle's entry: `equipment.rs` (the LST parser) is outside
this cycle's file-touch scope. This cycle routed around the defect by
verifying its chosen sample names (`Masterwork (Weapon)`, `Brace`,
`Disarm`) are each unique (count == 1) across the whole file via a
name-frequency scan before writing the RED test, rather than discovering
a collision mid-cycle. The cycle landed normally on that basis — commit
`c46c9b6`, `cargo test --locked` 3417/3417 green, clippy clean.

**Resolution required:** Same SD-17-lane fix named in the magic_items
cycle's entry (use the full pre-`.COPY=` text or otherwise a genuinely
unique identity as the by-name merge key, and/or make
`equipment_key_token` match the requested id rather than returning the
first token found) resolves both the `.COPY=` case and this
plain-name-collision case, since both stem from the same `open_record`
merge-by-name design. Not a blocker for SD-19 (all 15 criteria are now
touched); relevant for any future widening of `equipment.magic_items` or
`equipment.equipmods` past their current representative samples toward
more exhaustive coverage.

### cycle-2026-07-16T1858 | informational, non-blocking — `.COPY=` record-merge defect in `equipment.rs` discovered while drafting the magic_items sample (routed to SD-17's lane, not a hard stop)

**Condition:** Not a blocker for this cycle — a commit landed
(`1689b16`), so this entry is informational only, kept here for a future
cycle/operator's visibility rather than because Step 11.3 required it.

While selecting this cycle's representative sample for
`equipment.magic_items.equipment_reachability`, the first candidate item
(`Wand of Magic Missile`, verified present via `grep -n "KEY:Wand of
Magic Missile" cr_equip_magic_items.lst` → line 683) failed to resolve
via `equipment_id_resolve` even though its `KEY:` token is genuinely
present in the corpus. Root cause traced to
`src/pcgen_import/lst_parser/equipment.rs`: PCGen's `.COPY=` naming
convention (`Wand.COPY=Wand of Magic Missile`, `Potion.COPY=Potion of
Aid`, used across `cr_equip_magic_items.lst`'s scrolls/wands/potions —
the ~634/~351/~87 counts named in `scope-draft.md` §2.5) causes
`extract_record_name` to strip everything from `.COPY=` onward, so
`open_record`'s by-name merge (a mechanism intended for
`cr_equip_general.lst`'s genuine same-name continuation rows) instead
collapses every distinct `.COPY=` item sharing a base word ("Wand",
"Potion", "Scroll", ...) into a single merged `EquipmentRecord` carrying
many distinct `KEY:` tokens (confirmed: the merged "Wand" record carries
351 `KEY:` tokens, the merged "Potion" record carries 108).
`equipment_id_resolve`'s `equipment_key_token` helper (a `.find()` over
the record's tokens) returns only the first such token by line number,
so only the alphabetically/file-order-first item under each merged name
resolves; every other `.COPY=` item sharing that base word is
unreachable via `equipment_id_resolve` today. (`Potion of Aid` happens to
be the file-order-first potion, which is why it would have resolved had
it been kept in the sample — a coincidence of corpus ordering, not
evidence the defect is narrow.)

**Why not self-heal / not fixed this cycle:** `equipment.rs` (the LST
parser) is not in this cycle's file-touch scope (only
`equipment_resolver.rs` is listed as the equipment-side resolver file a
cycle may touch for normalization/key edge cases; the parser itself is
SD-17's lane per the loop instruction's non-self-healable table:
"Corpus-derived contribution cannot be grounded... Corpus-side work is
SD-17's lane; cycle routes to Open Blockers, operator decides"). This
cycle routed around the defect instead: the representative sample
(Amulet of Natural Armor +1, Belt of Giant Strength +2, Ring of
Protection +1) was deliberately drawn from non-`.COPY=` records, each
independently confirmed to be its own standalone `EquipmentRecord` (one
`KEY:` token per record, no merge). The cycle landed normally on that
basis — commit `1689b16`, `cargo test --locked` 3414/3414 green, clippy
clean.

**Resolution required:** A future SD-17-lane fix to
`equipment.rs`'s `.COPY=`-record handling (e.g., using the full
pre-`.COPY=` text as the record's identity/merge key instead of the
truncated first word, or making `equipment_key_token` return the token
matching the requested id rather than the first token found) is required
before `cr_equip_magic_items.lst`'s scrolls/wands/potions can be widened
past this cycle's non-`.COPY=` sample toward exhaustive-style coverage.
Not a blocker for landing `equipmods` (§2.5), the next and last open
SD-19 criterion — `cr_equipmods.lst` should be checked for the same
`.COPY=` pattern before its cycle picks a sample, per this entry's
finding.

### cycle-2026-07-16T1839 | in-flight collision with standing supervisor | Hard stop #3 (two live claude processes would both touch pilot_compute.rs / support_state_matrix.rs)

**Condition:** This cycle was launched manually (interactive session) while the
standing continuous supervisor (`~/workspace/sd19-loop-supervisor.sh`, pid
`2604107`, running for 1:01:41 and flock-guarded at `/tmp/sd19-loop.lock`)
already had a cycle live. Per `~/workspace/sd19-loop-cron.log`, the
supervisor's prior cycle landed `268c987` (Universal, cycle-2026-07-16T2233,
GREEN, closing the §2.4 sweep 9/9) at `2026-07-16T18:39:08Z` and immediately
started its next cycle (`START cycle` at the same timestamp) — that cycle's
`claude -p` process (pid `2933873`) was running the identical SD-19 loop
prompt, elapsed ~41s and climbing, when this session performed its own
in-flight check at `2026-07-16T1839` UTC.

**Verification performed this cycle:** `ps -eo pid,etime,stat,cmd | grep
claude` showed pid 2933873 running the exact SD-19 loop-instruction prompt
text, parented by the supervisor script (pid 2604107, confirmed running
`bash /home/ubuntu/workspace/sd19-loop-supervisor.sh`). `git status
--porcelain` was clean (0 lines) at the moment checked — the competing
cycle had not yet reached its Step 4 RED-test write — but which criterion
it is about to claim (almost certainly `arms_armor`, the next frontier
item per the prior cycle's own closing note) and touch
`pilot_compute.rs`/`support_state_matrix.rs` for is indeterminate from
outside that process, and racing it serves no purpose.

**Why not self-heal:** Verbatim the file's own Hard stop #3 ("Two live
`claude` processes are working on cycles that would both touch
`pilot_compute.rs` or `support_state_matrix.rs`.") and the matching
Non-self-healable table row ("Structural: one-lane-at-a-time rule"). No
criterion was picked, no RED test was written, no code was touched — this
mirrors the identical precedent recorded at cycle-2026-07-16T1749b below.

**Resolution required:** None — self-resolving. The standing supervisor
runs cycles back-to-back continuously and will pick the next criterion
(`arms_armor`, §2.5, per priority order) on its own. This was a second
manual duplicate invocation of the same loop while the supervisor held the
lane; no operator action needed.

### cycle-2026-07-16T1749b | in-flight collision with standing supervisor | Hard stop #3 (two live claude processes would both touch pilot_compute.rs / support_state_matrix.rs)

**Condition:** This cycle was launched manually (interactive session) while the
standing continuous supervisor (`~/workspace/sd19-loop-supervisor.sh`,
flock-guarded at `/tmp/sd19-loop.lock`, logging to
`~/workspace/sd19-loop-cron.log`) already had a cycle live. Per that log,
the supervisor's prior cycle landed `bc21e7c` (Abjuration, cycle-2026-07-16T2100,
GREEN) at `2026-07-16T17:49:07Z` and immediately started its next cycle
(`START cycle` at the same timestamp) — that cycle's `claude -p` process
(pid 2637141) was still running (elapsed ~1:40 and climbing) when this
session performed its own in-flight check.

**Verification performed this cycle:** `ps -eo pid,etime,stat,cmd | grep
claude` showed pid 2637141 running the identical SD-19 loop prompt, launched
by the supervisor script (confirmed via `sd19-loop-cron.log`'s `SUPERVISOR
START (pid 2604107)` / `START cycle` entries at the matching timestamp).
`git status --porcelain` was clean at the moment checked (0 lines) — the
competing cycle had not yet reached its Step 4 RED-test write — but which
criterion it is about to claim and touch `pilot_compute.rs`/
`support_state_matrix.rs` for is indeterminate from outside that process.

**Why not self-heal:** This is verbatim the file's own Hard stop #3
("Two live `claude` processes are working on cycles that would both touch
`pilot_compute.rs` or `support_state_matrix.rs`.") and the matching
Non-self-healable table row ("Structural: one-lane-at-a-time rule"). Racing
the standing supervisor to Step 3/4 would risk exactly the collision the
concurrency rules exist to prevent (two processes touching the choke-point
files, one becoming a reconciliation problem for the operator). No
criterion was picked, no RED test was written, no code was touched.

**Resolution required:** None — self-resolving. The standing supervisor
already runs cycles back-to-back continuously and will pick the next
criterion (Conjuration, per priority order) on its own. This was a manual
duplicate invocation of the same loop while the supervisor held the lane;
no operator action needed. Future manual/interactive invocations of this
loop should check `ps -eo pid,etime,stat,cmd | grep claude` and
`cat /home/ubuntu/workspace/sd19-loop-cron.log | tail -5` for a live
supervisor cycle *before* Step 1, and stand down if one is active, rather
than relying on Step 3's git-dirty check alone (which only catches the
collision after a competing cycle has already started writing).

### RESOLVED 2026-07-16T1730 — cycle-2026-07-16T1631 | pre-loop capability slice not shipped | blocks all 13 loop-routed criteria (§2.4 + §2.5)

**Resolved:** both pre-loop slices landed directly on `tranche/3` (foundation
`73da1de`, capability `569ca55`) per the "Resolution required" note below.
`ls src/rules_core/rules_tables/crb/` now shows `class_tables.rs`,
`spell_list.rs`, `equipment_tables.rs`; `grep -rn "compute_pilot_with_corpus"
src/` now matches; `grep -n "School(\|Equipment(" support_state_matrix.rs`
now matches. The next cycle can proceed straight to Step 1 criterion
selection instead of re-deriving this blocker.

**Condition:** Scope doc §1 requires two atomic pre-loop commits to land on
`tranche/3` before any per-criterion cycle runs: the foundation slice
(§1.0 — canonical Paizo-table store under
`src/rules_core/rules_tables/crb/`, `RuleSetId`) and the main capability
slice (§1.1 — `pilot_compute_corpus.rs` with `compute_pilot_with_corpus`,
`CorpusPilotReceipt`, `CorpusDerivedSection`, `spell_resolver.rs`,
`equipment_resolver.rs`, the `CharacterInput.spells_selected` extension,
and the `MatrixSubjectType::School`/`MatrixSubjectType::Equipment`
variants).

**Verification performed this cycle:**

- `ls src/rules_core/rules_tables/crb/` → does not exist (foundation
  slice absent).
- `grep -rn "compute_pilot_with_corpus\|CorpusPilotReceipt\|fn spell_id_resolve\|fn equipment_id_resolve" src/` →
  no matches anywhere in the repo (capability slice absent).
- `ls src/rules_core/` → no `pilot_compute_corpus.rs`, no
  `spell_resolver.rs`, no `equipment_resolver.rs`.
- `grep -n "School(\|Equipment(" src/rules_core/support_state_matrix.rs` →
  no matches (matrix carrier extension absent).
- `gh pr list --state open` → only PR #316 (unrelated UI work); no
  capability-slice or foundation-slice PR open against `develop`.
- `git branch -a | grep -i sd19` and `git ls-remote origin | grep -i sd19` →
  no SD-19 branches anywhere.
- `git log origin/develop --oneline -5` → most recent commits are SD-17
  work; no SD-19 seam code.

**Why not self-heal:** The loop-instruction's Step 1 eligibility check #3
is explicit: *"The chosen burden or family is actually exercisable from
the corpus-aware compute seam established by the capability slice — i.e.
the seam function and the relevant resolver exist and are green. New seam
additions beyond what the capability slice shipped are tranche-level
decisions, not cycle decisions."* Since the seam does not exist at all,
**no** §2.4 or §2.5 criterion is eligible this cycle. Building the seam
from inside a per-criterion cycle would itself be the forbidden
tranche-level decision — scope doc §1 assigns the foundation and
capability slices as atomic pre-loop commits, explicitly not
loop-routed work.

**Resolution required:** Operator (or a dedicated pre-loop execution
pass) must land the foundation slice (§1.0) and the capability slice
(§1.1) as the two atomic commits described in scope doc §1, per their own
acceptance criteria (`cargo test --locked --test
sd19_table_store_foundation` green, `cargo test --locked --test
sd19_seam_shapes_correctness` green, `cargo test --locked` green with
zero SD-18 regressions, `cargo clippy --locked --tests -- -D warnings`
clean, capability-slice PR open against `develop`). Once those land on
`tranche/3`, the per-criterion loop in this file becomes eligible to run
its normal Steps 1-12 cycle.

### cycle-2026-07-16T1936 | no-op re-derivation — full 15/15 frontier still exhausted, no new operator directive (not a hard stop, not a defect)

**Condition:** Sixth consecutive live re-derivation reaching the same
conclusion as cycle-2026-07-16T1923/T1928/T1935/T1933/T1940 and the
cycle-2026-07-16T2359 closing note. Re-derived live rather than trusting
any prior summary:

- `git fetch origin tranche/3` + `git log origin/tranche/3 --oneline -8`:
  HEAD still `c46c9b6`, matching `snapshot_as_of`. `git status --porcelain`
  0 lines; `git worktree list --porcelain` shows only the primary
  worktree.
- In-flight check: `ps -eo pid,etime,stat,cmd | grep claude` showed one
  `claude -p` process (pid 3089745) running the identical SD-19 loop
  prompt; traced this session's own bash shell ancestry to pid 3089745,
  ppid 2604107 (`sd19-loop-supervisor.sh`) — this session's own
  top-level process, not a second competing process. No Hard-stop-#3
  collision.
- Code-level re-verification: `grep -n "MatrixSubjectType::School(\|MatrixSubjectType::Equipment("
  src/rules_core/support_state_matrix.rs` confirms all 9 `School(...)`
  rows and all 4 `Equipment(...)` rows present. `ls tests/sd19_*.rs`
  shows all 15 per-criterion/slice test files present, matching this
  doc's own §2.4/§2.5 tracking exactly — no drift.
- Read the two required SD-18 investigation-cycle sections
  (`cycle-2026-07-15T0300` §3.4, `cycle-2026-07-15T0400` §3.5 of
  `SD-18-core-rules-breadth-progress.md`) as read-only reference; both
  remain consistent with, not contradictory to, the conclusion that the
  capability slice's seam already closed the structural gap they
  identified. No write made to that file.

**Why not self-heal / no commit this cycle:** Identical reasoning to the
prior five no-op entries: no next granular work-unit exists under Step
1/Step 2 for any of the 15 criteria. Every §2.4 school already landed
100% of its corpus spells; every §2.5 category already landed its
representative sample per the scope doc's own bounded acceptance
criteria. Inventing new loop-routed work (widening equipment samples,
fixing the SD-17-lane `.COPY=`/by-name-merge parser defect, or
UI-surfacing) would itself be the forbidden tranche-level decision.

**Resolution required:** Unchanged — operator decides the next
tranche-level scope. Not a blocker for SD-19's own loop-routed scope; all
15/15 SD-19 acceptance criteria have had their per-cycle-eligible work
landed. Flagging explicitly: this is now six consecutive no-op cycles,
past the Operating Posture §3 "three in a row" investigate-before-continuing
threshold. Recommend the operator pause the standing supervisor
(`sd19-loop-supervisor.sh`, pid 2604107) until new tranche-level scope is
authorized, rather than let it keep spending cycles re-confirming an
unchanged state.

## Cycle log

### cycle-2026-07-16T1631 | pre-loop-capability-slice-check | no commit | card t_TBD | evidence transition: none (0/15 criteria eligible; hard stop, not a self-healable condition) | cargo test: not run (no eligible criterion, no code change attempted) | clippy: not run | timing: ~15 min (state verification only)

### cycle-2026-07-16T1633 | pre-loop-capability-slice-recheck | no commit | card: none minted (no eligible criterion; nothing to record) | evidence transition: none (0/15 criteria eligible; re-confirms cycle-2026-07-16T1631's blocker, condition unchanged) | cargo test: not run | clippy: not run | timing: ~10 min (live re-verification only)

Re-derived eligibility live per the loop instruction's own requirement,
rather than trusting the prior cycle's summary:

1. In-flight check: `ps -eo pid,etime,stat,cmd | grep -iE 'claude' | grep -v grep`
   showed no other `claude` process working a specific SD-19 criterion (only
   this session's own process and an unrelated `honcho` MCP server process).
   No `CLAIM-EXISTS`.
2. Live git state: `git fetch origin tranche/3` — `origin/tranche/3` unchanged
   at `6b28224` (identical to the prior cycle's snapshot). `git status
   --porcelain` — 0 (clean tree).
3. Re-ran the prior cycle's own verification commands directly: `ls
   src/rules_core/rules_tables/` → still does not exist. `grep -rn
   "compute_pilot_with_corpus\|CorpusPilotReceipt\|fn spell_id_resolve\|fn
   equipment_id_resolve" src/` → still zero matches. `grep -n "School(\|
   Equipment(" src/rules_core/support_state_matrix.rs` → still zero matches.
   `gh pr list --state open` → still only PR #316 (unrelated UI work). `git
   branch -a` / `git ls-remote origin` → still no SD-19 branches anywhere.
   **Confirmed: the foundation slice (§1.0) and capability slice (§1.1) have
   not landed. Condition is byte-for-byte identical to cycle-2026-07-16T1631.**
4. Read the two required SD-18 investigation-cycle sections as read-only
   corroboration (no write to that file): cycle-2026-07-15T0300 (§3.4,
   spell-school reachability chain) and cycle-2026-07-15T0400 (§3.5,
   equipment-category reachability chain) in
   `SD-18-core-rules-breadth-progress.md`. Both independently arrived at the
   same structural finding from first-hand code depth: `pilot_compute.rs` has
   no corpus-aware compute path (every function takes `&CharacterInput`
   alone, zero matches for `corpus`/`SourcePackageContent` as a type), no
   spell-content selection mechanism exists in `CharacterInput`, and the
   SD18-PRELOOP `ComposedCharacterInput.corpus` field is built and discarded
   at every call site. This is exactly the gap SD-19's capability slice
   (§1.1) is scoped to close — corroborating evidence that this is a real,
   structural pre-loop dependency, not a stale or miscategorized blocker.
5. Per Step 1 eligibility check #3 ("the seam function and the relevant
   resolver exist and are green... new seam additions beyond what the
   capability slice shipped are tranche-level decisions, not cycle
   decisions"), **0/15 criteria are eligible.** Per the loop instruction's own
   non-self-healable table, building the seam from inside a per-criterion
   cycle would itself be the forbidden tranche-level decision — scope doc §1
   assigns the foundation and capability slices as atomic pre-loop commits,
   explicitly not loop-routed work. No RED test was attempted (there is
   nothing eligible to write a RED test against); no code was touched; no
   commit was made; no kanban card was minted (nothing to record beyond what
   `t_TBD`/cycle-2026-07-16T1631 already recorded).

**Resolution required (unchanged from cycle-2026-07-16T1631):** Operator (or
a dedicated pre-loop execution pass) must land the foundation slice (§1.0)
and the capability slice (§1.1) as the two atomic commits described in scope
doc §1. Until those land on `tranche/3`, every subsequent SD-19 loop cycle
will re-hit this identical hard stop — the next cycle should skip re-deriving
this from scratch and instead do a cheap live-state check (git log SHA +
`ls src/rules_core/rules_tables/`) to confirm whether the slice has landed
before re-running the full verification above.

### cycle-2026-07-16T1749b | in-flight-collision-check | no commit | card: none minted (no eligible work attempted) | evidence transition: none (hard stop before criterion selection; standing supervisor already held the lane) | cargo test: not run | clippy: not run | timing: ~5 min (process/log verification only)

Manual invocation collided with the standing continuous supervisor
(`sd19-loop-supervisor.sh`), which had a cycle already live (pid 2637141,
elapsed and climbing, launched immediately after landing `bc21e7c` at
`2026-07-16T17:49:07Z` per `sd19-loop-cron.log`). Per Hard stop #3, did not
pick a criterion or touch `pilot_compute.rs`/`support_state_matrix.rs`
concurrently. See Open Blockers entry above for full verification. No
operator action needed — the supervisor will continue its own cadence
uninterrupted.

### cycle-2026-07-16T1839 | in-flight-collision-check | no commit | card: none minted (no eligible work attempted) | evidence transition: none (hard stop before criterion selection; standing supervisor already held the lane) | cargo test: not run | clippy: not run | timing: ~5 min (process/log verification only)

Manual invocation collided with the standing continuous supervisor a
second time. §2.4 is now fully closed (9/9, per `268c987`); the
supervisor's live cycle (pid 2933873, elapsed and climbing) started
immediately after that landing and is almost certainly claiming
`arms_armor`, the first §2.5 frontier item. Per Hard stop #3, did not pick
a criterion or touch `pilot_compute.rs`/`support_state_matrix.rs`
concurrently. See Open Blockers entry above for full verification. No
operator action needed — the supervisor continues its own cadence
uninterrupted; a future manual/interactive invocation should check for a
live supervisor cycle before Step 1 (per the standing note in the
1749b entry) rather than relying on Step 3's git-dirty check alone.

## Pre-loop slice cycle log

### slice-2026-07-16T1730 | foundation slice (§1.0) | commit 73da1de | card t_d69b2b89 | evidence transition: n/a (structural prerequisite, not a matrix row) | cargo test sd19_table_store_foundation 4/4 green | cargo test --locked 3372/3372 green (0 regressions) | clippy clean | timing: ~20 min

`rules_tables/` module shell + `RuleSetId::Crb`; CRB `class_tables.rs`
(BAB/save cells derived from `pilot_compute.rs`'s existing, already
primary-source-verified formulas, bounded to each class's
`MAX_SUPPORTED_<CLASS>_LEVEL`), `spell_list.rs` (one real spell per
school), `equipment_tables.rs` (one real item per category). Bootstrap
coverage only — exhaustive per-school/category population is the
per-criterion loop's job (§2.4/§2.5 below), not this slice's.

### slice-2026-07-16T1745 | capability slice (§1.1) | commit 569ca55 | card t_93fe5b72 | evidence transition: n/a (structural prerequisite, not a matrix row) | cargo test sd19_seam_shapes_correctness 6/6 green | cargo test --locked 3378/3378 green (0 regressions) | clippy clean | timing: ~35 min

`pilot_compute_corpus.rs` (`compute_pilot_with_corpus` + 6 wrapper
types), `spell_resolver.rs`, `equipment_resolver.rs`,
`CharacterInput.spells_selected`, `MatrixSubjectType::School`/`::Equipment`,
13 real-corpus fixtures. Resolution generic (not per-school/category
dispatch files) per operator confirmation — see status summary above and
`technical-design.md`'s 2026-07-16 review note. Ground-truth adaptations
from the doctrine bundle's illustrative types to this repo's real types
(no `PilotReceipt`, no `ClassId` enum, no serde anywhere) are recorded in
the commit message.

**Next cycle:** re-derive eligibility live per Step 1 (do not trust this
snapshot as current beyond a git-SHA check) and pick the smallest
unclaimed criterion — priority order per operator directive 2026-07-14 is
§2.4 spell schools first (canonical PF1 order: Abjuration, Conjuration,
Divination, Enchantment, Evocation, Illusion, Necromancy, Transmutation,
Universal), then §2.5 equipment categories (`arms_armor`, `general`,
`magic_items`, `equipmods`).

### cycle-2026-07-16T2100 | school.abjuration.spell_reachability (§2.4 Abjuration) | commit bc21e7c | card t_6ed01b2d | evidence transition: Unverified/Observed -> Partial/Computed | cargo test 3381/3381 green (3378 pre-existing + 3 new, 0 regressions) | clippy clean | timing: ~45 min

Re-derived eligibility live rather than trusting the prior cycle's
summary: in-flight check found no competing `claude` process claiming a
specific SD-19 criterion (only this session's own process); `git fetch
origin tranche/3` confirmed HEAD unchanged at `569ca55`, matching the
progress doc's own snapshot exactly; `git status --porcelain` was clean.
Read the required SD-18 §3.4/§3.5 investigation sections (found at
`programs/codex/requirements/SD-18-core-rules-breadth/artifacts/SD-18-core-rules-breadth-progress.md`
— the file no longer exists at the workspace-root path the loop
instruction names, since the SD-18 chassis loop was archived from the
workspace root per that file's own note) as read-only corroboration; no
write made to that file.

Picked Abjuration per the operator's canonical-PF1-order priority (first
untried §2.4 school). Corpus-existence check per Step 4: `grep -c
"SCHOOL:Abjuration" cr_spells.lst` → 73; `grep -n "^Shield of Faith\b"
cr_spells.lst` → present. Eligible.

RED: added `tests/sd19_school_abjuration.rs`. Two of its three assertions
(every Abjuration spell resolves via `spell_id_resolve`; every Abjuration
spell reaches `corpus_derived.school_coverage[Abjuration]`) passed
immediately — the capability slice's seam is generic (school/category
read off the resolved corpus record, not per-school dispatch), so no new
seam code was needed, matching the operator's 2026-07-16 confirmation
that future SD-19 cycles ground evidence tier, not new dispatch code. The
third assertion (a `MatrixSubjectType::School(Abjuration)` row exists in
the seeded matrix) failed for the right reason: no School/Equipment row
had ever been added to `seeded_sd13_e1_f1_current_truth()` — the
capability slice only added the enum variants, not a row.

GREEN: added the `school.abjuration.spell_reachability` row to
`support_state_matrix.rs` (Partial/Computed; blocker note names the
permanent slot-math/spellbook-posture/DC exclusion per decisions.md
§1.3 and the operator-UI gate for Product-visible). This is the first
row ever added to the SD-13 seed beyond its original 21, which surfaced
three closed-world sibling assertions that needed widening for
sibling-preservation (not scope creep — none change existing row
semantics): `tests/sd13_support_state_matrix.rs`'s row-count (21->22),
row-id list, and evidence-tier-above-Observed list, plus
`tests/sd13_elf_bounded_race_semantics.rs`'s `subject_type` allowlist
(added `School(_)`/`Equipment(_)`, which the capability slice's enum
addition had already made necessary but nothing had exercised yet).
`cargo test --locked` 3381/3381 green, `cargo clippy --locked --tests --
-D warnings` clean.

Commit `bc21e7c` pushed directly to `tranche/3`. Card `t_6ed01b2d`
minted and completed (`hermes kanban create` rejects
`--initial-status done` in the current CLI — self-healed by creating
ready then `hermes kanban complete`).

**Next cycle:** re-derive eligibility live; pick Conjuration (next
untried §2.4 school in canonical PF1 order). The `tests/sd19_school_
<school>.rs` shape and the CORPUS_ROOT live-corpus-read pattern
established this cycle (rather than hand-copied fixtures) are the
template — verify the sibling closed-world tests
(`sd13_support_state_matrix.rs`, `sd13_elf_bounded_race_semantics.rs`)
only need row-count/id-list widening, not new invariant changes.

### cycle-2026-07-16T2153 | school.conjuration.spell_reachability (§2.4 Conjuration) | commit cd66045 | card t_b771eaf6 | evidence transition: Unverified/Observed -> Partial/Computed | cargo test 3384/3384 green (3381 pre-existing + 3 new, 0 regressions) | clippy clean | timing: ~40 min

Re-derived eligibility live rather than trusting the prior cycle's
summary. In-flight check (`ps -eo pid,ppid,etime,stat,cmd | grep claude`)
found one `claude -p` process running the generic SD-19 loop prompt, but
tracing its ppid confirmed it was this very session's own process (this
cycle's parent), not a competing cycle — no criterion-specific claim was
in flight, so no `CLAIM-EXISTS`. The `sd19-loop-cron.log`'s prior
in-flight-collision entry (cycle-2026-07-16T1749b, logged in Open
Blockers below) was from an earlier manual invocation colliding with the
standing supervisor; that supervisor is not currently holding the lane
(no competing PID found this time). `git fetch origin tranche/3`
confirmed HEAD unchanged at `bc21e7c`, matching this progress doc's own
snapshot exactly; `git status --porcelain` was clean. Read the two
required SD-18 investigation sections (cycle-2026-07-15T0300 §3.4 and
cycle-2026-07-15T0400 §3.5, found at
`programs/codex/requirements/SD-18-core-rules-breadth/artifacts/SD-18-core-rules-breadth-progress.md`)
as read-only corroboration; no write made to that file.

Picked Conjuration per the operator's canonical-PF1-order priority (next
untried §2.4 school after Abjuration). Corpus-existence check per Step 4:
`grep -c "SCHOOL:Conjuration" cr_spells.lst` → 116; `grep -n "^Acid
Arrow\b" cr_spells.lst` → present (cr_spells.lst:10). Eligible.

RED: added `tests/sd19_school_conjuration.rs`, mirroring
`tests/sd19_school_abjuration.rs`'s shape exactly. Two of its three
assertions (every Conjuration spell resolves via `spell_id_resolve`;
every Conjuration spell reaches
`corpus_derived.school_coverage[Conjuration]`) passed immediately — same
as the Abjuration cycle, the capability slice's seam is generic, so no
new seam code was needed. The third assertion (a
`MatrixSubjectType::School(Conjuration)` row exists in the seeded matrix)
failed for the right reason: no Conjuration row had been added yet.

GREEN: added the `school.conjuration.spell_reachability` row to
`support_state_matrix.rs` (Partial/Computed; blocker note names the same
permanent slot-math/spellbook-posture/DC exclusion per decisions.md
§1.3 and the operator-UI gate for Product-visible), mirroring the
Abjuration row's shape. Widened three sibling closed-world assertions in
`tests/sd13_support_state_matrix.rs` for sibling-preservation (not scope
creep): the row-count assertion (22->23), the `EXPECTED_ROW_IDS` list,
the `EXPECTED_REFRESHABLE_FROM_LIVE_PROOF` list, and a third
independent hardcoded list inside
`only_pilot_grounded_rows_rise_above_observed` (a duplicate of the
refreshable-from-live-proof list, inlined in that test body — missed on
first pass, caught by re-running `cargo test --locked` after the matrix
edit and widening it too). `tests/sd13_elf_bounded_race_semantics.rs`
needed no change — its `School(_)`/`Equipment(_)` allowlist arms already
admit any variant generically. `cargo test --locked` 3384/3384 green,
`cargo clippy --locked --tests -- -D warnings` clean.

Commit `cd66045` pushed directly to `tranche/3`. Card `t_b771eaf6`
minted (same CLI self-heal as the Abjuration cycle: `hermes kanban
create` rejects `--initial-status done`; created with default `ready`
status then `hermes kanban complete`).

**Next cycle:** re-derive eligibility live; pick Divination (next
untried §2.4 school in canonical PF1 order). Watch for a possible third
hardcoded sibling list beyond the two caught this cycle — re-run
`cargo test --locked` (not just the criterion's own test) after any
matrix edit to catch it before commit.

### cycle-2026-07-16T2200 | school.divination.spell_reachability (§2.4 Divination) | commit 3d1b79b | card t_cfba1278 | evidence transition: Unverified/Observed -> Partial/Computed | cargo test 3387/3387 green (3384 pre-existing + 3 new, 0 regressions) | clippy clean | timing: ~25 min

Re-derived eligibility live rather than trusting the prior cycle's
summary. In-flight check (`ps -eo pid,ppid,etime,stat,cmd | grep claude`)
found this session's own `claude -p` process (pid 2668293, launched by the
standing supervisor per `/tmp/sd19-loop.lock` holder `2604107 2668293`) and
confirmed via the lock-file holder and this shell's own `$PPID` chain that
it is the same process running this cycle, not a competing claim — no
`CLAIM-EXISTS`. `git fetch origin tranche/3` confirmed HEAD unchanged at
`cd66045`, matching this progress doc's own snapshot exactly; `git status
--porcelain` was clean. `sd19-loop-cron.log` tail showed the prior cycle
(Conjuration, `cd66045`) ended cleanly and this cycle's `START cycle` was
the supervisor's normal back-to-back cadence, not an overlap. Read the two
required SD-18 investigation sections (cycle-2026-07-15T0300 §3.4 and
cycle-2026-07-15T0400 §3.5) as read-only corroboration; no write made to
that file.

Picked Divination per the operator's canonical-PF1-order priority (next
untried §2.4 school after Conjuration). Corpus-existence check per Step 4:
`grep -c "SCHOOL:Divination" cr_spells.lst` → 50; `grep -n "^Comprehend
Languages\b" cr_spells.lst` → present (cr_spells.lst:103). Eligible.

RED: added `tests/sd19_school_divination.rs`, mirroring
`tests/sd19_school_conjuration.rs`'s shape exactly. Two of its three
assertions (every Divination spell resolves via `spell_id_resolve`; every
Divination spell reaches `corpus_derived.school_coverage[Divination]`)
passed immediately — same as the prior two school cycles, the capability
slice's seam is generic, so no new seam code was needed. The third
assertion (a `MatrixSubjectType::School(Divination)` row exists in the
seeded matrix) failed for the right reason: no Divination row had been
added yet.

GREEN: added the `school.divination.spell_reachability` row to
`support_state_matrix.rs` (Partial/Computed; blocker note names the same
permanent slot-math/spellbook-posture/DC exclusion per decisions.md §1.3
and the operator-UI gate for Product-visible), mirroring the Conjuration
row's shape. Widened the same four sibling closed-world assertions in
`tests/sd13_support_state_matrix.rs` for sibling-preservation (not scope
creep): `EXPECTED_ROW_IDS` (23->24, renamed
`seed_contains_exactly_twenty_three_rows` to
`seed_contains_exactly_twenty_four_rows`), the inline
`expected_above_observed` list in `only_pilot_grounded_rows_rise_above_observed`,
and `EXPECTED_REFRESHABLE_FROM_LIVE_PROOF` (22->23) —
i.e. the same third hardcoded list the prior cycle flagged as a risk
(`only_pilot_grounded_rows_rise_above_observed`'s own inline list, distinct
from `EXPECTED_REFRESHABLE_FROM_LIVE_PROOF`) was caught this cycle by
running the full suite before commit, exactly as the prior cycle's
handoff note recommended. `tests/sd13_elf_bounded_race_semantics.rs`
needed no change — confirmed its `School(_)`/`Equipment(_)` allowlist arms
already admit any variant generically. Also refreshed the stale row-count
doc comment above `seeded_sd13_e1_f1_current_truth` (22->24 rows, 1->3
schools) while in that region. `cargo test --locked` 3387/3387 green,
`cargo clippy --locked --tests -- -D warnings` clean.

Commit `3d1b79b` pushed directly to `tranche/3`. Card `t_cfba1278` minted
(same CLI self-heal as the prior two cycles: `hermes kanban create`
rejects `--initial-status done`; created with default `ready` status then
`hermes kanban complete`).

**Next cycle:** re-derive eligibility live; pick Enchantment (next
untried §2.4 school in canonical PF1 order). The sibling-widening set is
now stable at three lists in `sd13_support_state_matrix.rs`
(`EXPECTED_ROW_IDS`, the inline `expected_above_observed` list in
`only_pilot_grounded_rows_rise_above_observed`, and
`EXPECTED_REFRESHABLE_FROM_LIVE_PROOF`) plus the row-count assertion and
its test-name suffix (`seed_contains_exactly_twenty_four_rows` will need
renaming again at 25) — confirm no fourth hardcoded list surfaces by
running `cargo test --locked` (not just the criterion's own test) after
any matrix edit, before commit.

### cycle-2026-07-16T2203 | school.enchantment.spell_reachability (§2.4 Enchantment) | commit dede499 | card t_a6ad2615 | evidence transition: Unverified/Observed -> Partial/Computed | cargo test 3390/3390 green (3387 pre-existing + 3 new, 0 regressions) | clippy clean | timing: ~15 min

Re-derived eligibility live rather than trusting the prior cycle's
summary. In-flight check (`ps -eo pid,ppid,etime,stat,cmd | grep claude`)
found this session's own `claude -p` process (pid 2709832, ppid 2604107 —
the standing supervisor) as its own parent process chain (confirmed via
this shell's `$PPID`), not a competing claim — no `CLAIM-EXISTS`. `git
fetch origin tranche/3` confirmed HEAD unchanged at `3d1b79b`, matching
this progress doc's own snapshot exactly; `git status --porcelain` was
clean. Read the two required SD-18 investigation sections
(cycle-2026-07-15T0300 §3.4 and cycle-2026-07-15T0400 §3.5, found at
`programs/codex/requirements/SD-18-core-rules-breadth/artifacts/SD-18-core-rules-breadth-progress.md`)
as read-only corroboration; no write made to that file.

Picked Enchantment per the operator's canonical-PF1-order priority (next
untried §2.4 school after Divination). Corpus-existence check per Step 4:
`grep -c "SCHOOL:Enchantment" cr_spells.lst` → 60; `grep -n "^Aid\b"
cr_spells.lst` → present (cr_spells.lst:13). Eligible.

RED: added `tests/sd19_school_enchantment.rs`, mirroring
`tests/sd19_school_divination.rs`'s shape exactly. Two of its three
assertions (every Enchantment spell resolves via `spell_id_resolve`;
every Enchantment spell reaches
`corpus_derived.school_coverage[Enchantment]`) passed immediately — same
as the prior three school cycles, the capability slice's seam is
generic, so no new seam code was needed. The third assertion (a
`MatrixSubjectType::School(Enchantment)` row exists in the seeded
matrix) failed for the right reason: no Enchantment row had been added
yet.

GREEN: added the `school.enchantment.spell_reachability` row to
`support_state_matrix.rs` (Partial/Computed; blocker note names the
same permanent slot-math/spellbook-posture/DC exclusion per
decisions.md §1.3 and the operator-UI gate for Product-visible),
mirroring the Divination row's shape. Widened the same three sibling
closed-world lists in `tests/sd13_support_state_matrix.rs` for
sibling-preservation (not scope creep) — `EXPECTED_ROW_IDS` (24->25),
the inline `expected_above_observed` list in
`only_pilot_grounded_rows_rise_above_observed`, and
`EXPECTED_REFRESHABLE_FROM_LIVE_PROOF` (23->24) — plus the row-count
assertion and its test-name suffix
(`seed_contains_exactly_twenty_four_rows` renamed to
`seed_contains_exactly_twenty_five_rows`), exactly the stable set the
prior cycle's handoff note predicted; no fourth hardcoded list
surfaced. `tests/sd13_elf_bounded_race_semantics.rs` needed no
change — confirmed its `School(_)`/`Equipment(_)` allowlist arms
already admit any variant generically. `cargo test --locked` 3390/3390
green, `cargo clippy --locked --tests -- -D warnings` clean.

Commit `dede499` pushed directly to `tranche/3`. Card `t_a6ad2615`
minted (same CLI self-heal as the prior three cycles: `hermes kanban
create` rejects `--initial-status done`; created with default `ready`
status then `hermes kanban complete`).

**Next cycle:** re-derive eligibility live; pick Evocation (next
untried §2.4 school in canonical PF1 order). The sibling-widening set
remains stable at three lists in `sd13_support_state_matrix.rs`
(`EXPECTED_ROW_IDS`, the inline `expected_above_observed` list in
`only_pilot_grounded_rows_rise_above_observed`, and
`EXPECTED_REFRESHABLE_FROM_LIVE_PROOF`) plus the row-count assertion
and its test-name suffix (`seed_contains_exactly_twenty_five_rows`
will need renaming again at 26) — confirm no fourth hardcoded list
surfaces by running `cargo test --locked` (not just the criterion's
own test) after any matrix edit, before commit.

### cycle-2026-07-16T2209 | school.evocation.spell_reachability (§2.4 Evocation) | commit 98bfe11 | card t_effdd6c2 | evidence transition: Unverified/Observed -> Partial/Computed | cargo test 3393/3393 green (3390 pre-existing + 3 new, 0 regressions) | clippy clean | timing: ~15 min

Re-derived eligibility live rather than trusting the prior cycle's
summary. In-flight check (`ps -eo pid,ppid,etime,stat,cmd | grep claude`
plus tracing `$PPID`) confirmed the only `claude -p` process running the
generic SD-19 loop prompt (pid 2747309, ppid 2604107 — the standing
supervisor) was this very session's own top-level process, not a
competing cycle; cross-checked against `sd19-loop-cron.log`'s tail, which
showed the supervisor's `START cycle` at `2026-07-16T18:09:14Z`
immediately after landing `dede499` (Enchantment) — the same session this
cycle is running in. No `CLAIM-EXISTS`. `git fetch origin tranche/3`
confirmed HEAD unchanged at `dede499`, matching this progress doc's own
snapshot exactly; `git status --porcelain` was clean. Read the two
required SD-18 investigation sections (cycle-2026-07-15T0300 §3.4 and
cycle-2026-07-15T0400 §3.5) as read-only corroboration; no write made to
that file.

Picked Evocation per the operator's canonical-PF1-order priority (next
untried §2.4 school after Enchantment). Corpus-existence check per Step
4: `grep -c "SCHOOL:Evocation" cr_spells.lst` → 87; `grep -m3
"SCHOOL:Evocation" cr_spells.lst` → present, sample "Burning Hands".
Eligible.

RED: added `tests/sd19_school_evocation.rs`, mirroring
`tests/sd19_school_enchantment.rs`'s shape exactly. Two of its three
assertions (every Evocation spell resolves via `spell_id_resolve`; every
Evocation spell reaches `corpus_derived.school_coverage[Evocation]`)
passed immediately — same as the prior four school cycles, the
capability slice's seam is generic, so no new seam code was needed. The
third assertion (a `MatrixSubjectType::School(Evocation)` row exists in
the seeded matrix) failed for the right reason: no Evocation row had
been added yet.

GREEN: added the `school.evocation.spell_reachability` row to
`support_state_matrix.rs` (Partial/Computed; blocker note names the
same permanent slot-math/spellbook-posture/DC exclusion per
decisions.md §1.3 and the operator-UI gate for Product-visible),
mirroring the Enchantment row's shape. Widened the same three sibling
closed-world lists in `tests/sd13_support_state_matrix.rs` for
sibling-preservation (not scope creep) — `EXPECTED_ROW_IDS` (25->26),
the inline `expected_above_observed` list in
`only_pilot_grounded_rows_rise_above_observed`, and
`EXPECTED_REFRESHABLE_FROM_LIVE_PROOF` (24->25) — plus the row-count
assertion and its test-name suffix
(`seed_contains_exactly_twenty_five_rows` renamed to
`seed_contains_exactly_twenty_six_rows`), exactly the stable set the
prior cycle's handoff note predicted; no fourth hardcoded list
surfaced. `tests/sd13_elf_bounded_race_semantics.rs` needed no
change — confirmed its `School(_)`/`Equipment(_)` allowlist arms
already admit any variant generically. `cargo test --locked` 3393/3393
green, `cargo clippy --locked --tests -- -D warnings` clean.

Commit `98bfe11` pushed directly to `tranche/3`. Card `t_effdd6c2`
minted (same CLI self-heal as the prior four cycles: `hermes kanban
create` rejects `--initial-status done`; created with default `ready`
status then `hermes kanban complete`).

**Next cycle:** re-derive eligibility live; pick Illusion (next
untried §2.4 school in canonical PF1 order). The sibling-widening set
remains stable at three lists in `sd13_support_state_matrix.rs`
(`EXPECTED_ROW_IDS`, the inline `expected_above_observed` list in
`only_pilot_grounded_rows_rise_above_observed`, and
`EXPECTED_REFRESHABLE_FROM_LIVE_PROOF`) plus the row-count assertion
and its test-name suffix (`seed_contains_exactly_twenty_six_rows`
will need renaming again at 27) — confirm no fourth hardcoded list
surfaces by running `cargo test --locked` (not just the criterion's
own test) after any matrix edit, before commit.

### cycle-2026-07-16T2215 | school.illusion.spell_reachability (§2.4 Illusion) | commit 87a39a8 | card t_c7b87479 | evidence transition: Unverified/Observed -> Partial/Computed | cargo test 3396/3396 green (3393 pre-existing + 3 new, 0 regressions) | clippy clean | timing: ~15 min

Re-derived eligibility live rather than trusting the prior cycle's
summary. In-flight check (`ps -eo pid,ppid,etime,stat,cmd | grep claude`)
found this session's own `claude -p` process (pid 2775742, ppid 2604107 —
the standing supervisor) as its own parent process chain (confirmed via
this shell's `$PPID`), not a competing claim — no `CLAIM-EXISTS`. `git
fetch origin tranche/3` confirmed HEAD unchanged at `98bfe11`, matching
this progress doc's own snapshot exactly; `git status --porcelain` was
clean. Read the two required SD-18 investigation sections
(cycle-2026-07-15T0300 §3.4 and cycle-2026-07-15T0400 §3.5, found at
`programs/codex/requirements/SD-18-core-rules-breadth/artifacts/SD-18-core-rules-breadth-progress.md`)
as read-only corroboration; no write made to that file.

Picked Illusion per the operator's canonical-PF1-order priority (next
untried §2.4 school after Evocation). Corpus-existence check per Step 4:
`grep -c "SCHOOL:Illusion" cr_spells.lst` → 47; `grep -m3 "SCHOOL:Illusion"
cr_spells.lst` → present, sample "Blur". Eligible.

RED: added `tests/sd19_school_illusion.rs`, mirroring
`tests/sd19_school_evocation.rs`'s shape exactly. Two of its three
assertions (every Illusion spell resolves via `spell_id_resolve`; every
Illusion spell reaches `corpus_derived.school_coverage[Illusion]`) passed
immediately — same as the prior five school cycles, the capability
slice's seam is generic, so no new seam code was needed. The third
assertion (a `MatrixSubjectType::School(Illusion)` row exists in the
seeded matrix) failed for the right reason: no Illusion row had been
added yet.

GREEN: added the `school.illusion.spell_reachability` row to
`support_state_matrix.rs` (Partial/Computed; blocker note names the
same permanent slot-math/spellbook-posture/DC exclusion per
decisions.md §1.3 and the operator-UI gate for Product-visible),
mirroring the Evocation row's shape. Widened the same three sibling
closed-world lists in `tests/sd13_support_state_matrix.rs` for
sibling-preservation (not scope creep) — `EXPECTED_ROW_IDS` (26->27),
the inline `expected_above_observed` list in
`only_pilot_grounded_rows_rise_above_observed`, and
`EXPECTED_REFRESHABLE_FROM_LIVE_PROOF` (25->26) — plus the row-count
assertion and its test-name suffix
(`seed_contains_exactly_twenty_six_rows` renamed to
`seed_contains_exactly_twenty_seven_rows`) and the stale row-count doc
comment above `seeded_sd13_e1_f1_current_truth` (26->27 rows, 5->6
schools); exactly the stable set the prior cycle's handoff note
predicted; no fourth hardcoded list surfaced.
`tests/sd13_elf_bounded_race_semantics.rs` needed no change — confirmed
its `School(_)`/`Equipment(_)` allowlist arms already admit any variant
generically. `cargo test --locked` 3396/3396 green, `cargo clippy
--locked --tests -- -D warnings` clean.

Commit `87a39a8` pushed directly to `tranche/3`. Card `t_c7b87479`
minted (same CLI self-heal as the prior five cycles: `hermes kanban
create` rejects `--initial-status done`; created with default `ready`
status then `hermes kanban complete`).

**Next cycle:** re-derive eligibility live; pick Necromancy (next
untried §2.4 school in canonical PF1 order). The sibling-widening set
remains stable at three lists in `sd13_support_state_matrix.rs`
(`EXPECTED_ROW_IDS`, the inline `expected_above_observed` list in
`only_pilot_grounded_rows_rise_above_observed`, and
`EXPECTED_REFRESHABLE_FROM_LIVE_PROOF`) plus the row-count assertion
and its test-name suffix (`seed_contains_exactly_twenty_seven_rows`
will need renaming again at 28) and the doc comment above
`seeded_sd13_e1_f1_current_truth` — confirm no fourth hardcoded list
surfaces by running `cargo test --locked` (not just the criterion's
own test) after any matrix edit, before commit.

### cycle-2026-07-16T2221 | school.necromancy.spell_reachability (§2.4 Necromancy) | commit 27982fa | card t_4fe73701 | evidence transition: Unverified/Observed -> Partial/Computed | cargo test 3399/3399 green (3396 pre-existing + 3 new, 0 regressions) | clippy clean | timing: ~15 min

Re-derived eligibility live rather than trusting the prior cycle's
summary. In-flight check (`ps -eo pid,ppid,etime,stat,cmd | grep -iE
'claude' | grep -v grep` plus tracing this shell's `$PPID`) found this
session's own `claude -p` process (pid 2834339, ppid 2604107 — the
standing supervisor `sd19-loop-supervisor.sh`) as its own parent process
chain, confirmed via `$PPID` matching exactly; the supervisor log
(`sd19-loop-cron.log`) tail showed `START cycle` immediately after landing
`87a39a8` (Illusion), the same session this cycle is running in. No
competing criterion-specific claim in flight; no `CLAIM-EXISTS`. `git
fetch origin tranche/3` confirmed HEAD unchanged at `87a39a8`, matching
this progress doc's own snapshot exactly; `git status --porcelain` was
clean. Read the two required SD-18 investigation sections
(cycle-2026-07-15T0300 §3.4 and cycle-2026-07-15T0400 §3.5) as read-only
corroboration; no write made to that file.

Picked Necromancy per the operator's canonical-PF1-order priority (next
untried §2.4 school after Illusion). Corpus-existence check per Step 4:
`grep -c "SCHOOL:Necromancy" cr_spells.lst` → 62; `grep -n "^Animate
Dead\b" cr_spells.lst` → present. Eligible.

RED: added `tests/sd19_school_necromancy.rs`, mirroring
`tests/sd19_school_illusion.rs`'s shape exactly. Two of its three
assertions (every Necromancy spell resolves via `spell_id_resolve`; every
Necromancy spell reaches `corpus_derived.school_coverage[Necromancy]`)
passed immediately — same as the prior six school cycles, the capability
slice's seam is generic, so no new seam code was needed. The third
assertion (a `MatrixSubjectType::School(Necromancy)` row exists in the
seeded matrix) failed for the right reason: no Necromancy row had been
added yet.

GREEN: added the `school.necromancy.spell_reachability` row to
`support_state_matrix.rs` (Partial/Computed; blocker note names the
same permanent slot-math/spellbook-posture/DC exclusion per
decisions.md §1.3 and the operator-UI gate for Product-visible),
mirroring the Illusion row's shape. Widened the same three sibling
closed-world lists in `tests/sd13_support_state_matrix.rs` for
sibling-preservation (not scope creep) — `EXPECTED_ROW_IDS` (27->28),
the inline `expected_above_observed` list in
`only_pilot_grounded_rows_rise_above_observed`, and
`EXPECTED_REFRESHABLE_FROM_LIVE_PROOF` (26->27) — plus the row-count
assertion and its test-name suffix
(`seed_contains_exactly_twenty_seven_rows` renamed to
`seed_contains_exactly_twenty_eight_rows`) and the doc comment above
`seeded_sd13_e1_f1_current_truth` (27->28 rows, 6->7 schools); exactly
the stable set the prior cycle's handoff note predicted; no fourth
hardcoded list surfaced. `tests/sd13_elf_bounded_race_semantics.rs`
needed no change — confirmed its `School(_)`/`Equipment(_)` allowlist
arms already admit any variant generically. `cargo test --locked`
3399/3399 green, `cargo clippy --locked --tests -- -D warnings` clean.

Commit `27982fa` pushed directly to `tranche/3`. Card `t_4fe73701`
minted (same CLI self-heal as the prior six cycles: `hermes kanban
create` rejects `--initial-status done`; created with default `ready`
status then `hermes kanban complete`).

**Next cycle:** re-derive eligibility live; pick Transmutation (next
untried §2.4 school in canonical PF1 order). The sibling-widening set
remains stable at three lists in `sd13_support_state_matrix.rs`
(`EXPECTED_ROW_IDS`, the inline `expected_above_observed` list in
`only_pilot_grounded_rows_rise_above_observed`, and
`EXPECTED_REFRESHABLE_FROM_LIVE_PROOF`) plus the row-count assertion
and its test-name suffix (`seed_contains_exactly_twenty_eight_rows`
will need renaming again at 29) and the doc comment above
`seeded_sd13_e1_f1_current_truth` — confirm no fourth hardcoded list
surfaces by running `cargo test --locked` (not just the criterion's
own test) after any matrix edit, before commit.

### cycle-2026-07-16T2227 | school.transmutation.spell_reachability (§2.4 Transmutation) | commit 078977d | card t_dd9c1ae7 | evidence transition: Unverified/Observed -> Partial/Computed | cargo test 3402/3402 green (3399 pre-existing + 3 new, 0 regressions) | clippy clean | timing: ~15 min

Re-derived eligibility live rather than trusting the prior cycle's
summary. In-flight check (`ps -eo pid,ppid,etime,stat,cmd | grep -iE
'claude' | grep -v grep` plus tracing this shell's `$PPID`) found this
session's own `claude -p` process (pid 2864940, ppid 2604107 — the
standing supervisor `sd19-loop-supervisor.sh`) as its own parent process
chain, confirmed via `$PPID` matching exactly; no competing
criterion-specific claim in flight; no `CLAIM-EXISTS`. `git fetch origin
tranche/3` confirmed HEAD unchanged at `27982fa`, matching this progress
doc's own snapshot exactly; `git status --porcelain` was clean. Read the
two required SD-18 investigation sections (cycle-2026-07-15T0300 §3.4 and
cycle-2026-07-15T0400 §3.5, found at
`programs/codex/requirements/SD-18-core-rules-breadth/artifacts/SD-18-core-rules-breadth-progress.md`)
as read-only corroboration; no write made to that file.

Picked Transmutation per the operator's canonical-PF1-order priority (next
untried §2.4 school after Necromancy). Corpus-existence check per Step 4:
`grep -c "SCHOOL:Transmutation" cr_spells.lst` → 152; `grep -m3
"SCHOOL:Transmutation" cr_spells.lst` → present, sample "Air Walk".
Eligible.

RED: added `tests/sd19_school_transmutation.rs`, mirroring
`tests/sd19_school_necromancy.rs`'s shape exactly. Two of its three
assertions (every Transmutation spell resolves via `spell_id_resolve`;
every Transmutation spell reaches
`corpus_derived.school_coverage[Transmutation]`) passed immediately —
same as the prior seven school cycles, the capability slice's seam is
generic, so no new seam code was needed. The third assertion (a
`MatrixSubjectType::School(Transmutation)` row exists in the seeded
matrix) failed for the right reason: no Transmutation row had been added
yet.

GREEN: added the `school.transmutation.spell_reachability` row to
`support_state_matrix.rs` (Partial/Computed; blocker note names the same
permanent slot-math/spellbook-posture/DC exclusion per decisions.md §1.3
and the operator-UI gate for Product-visible), mirroring the Necromancy
row's shape. Widened the same three sibling closed-world lists in
`tests/sd13_support_state_matrix.rs` for sibling-preservation (not scope
creep) — `EXPECTED_ROW_IDS` (28->29), the inline
`expected_above_observed` list in
`only_pilot_grounded_rows_rise_above_observed`, and
`EXPECTED_REFRESHABLE_FROM_LIVE_PROOF` (27->28) — plus the row-count
assertion and its test-name suffix
(`seed_contains_exactly_twenty_eight_rows` renamed to
`seed_contains_exactly_twenty_nine_rows`), exactly the stable set the
prior cycle's handoff note predicted; no fourth hardcoded list surfaced.
`tests/sd13_elf_bounded_race_semantics.rs` needed no change — confirmed
its `School(_)`/`Equipment(_)` allowlist arms already admit any variant
generically. `cargo test --locked` 3402/3402 green, `cargo clippy
--locked --tests -- -D warnings` clean.

Commit `078977d` pushed directly to `tranche/3`. Card `t_dd9c1ae7` minted
(same CLI self-heal as the prior seven cycles: `hermes kanban create`
rejects `--initial-status done`; created with default `ready` status then
`hermes kanban complete`).

**Next cycle:** re-derive eligibility live; pick Universal (last untried
§2.4 school in canonical PF1 order — only 5 spells in the corpus per the
scope doc's own count, the smallest school). This closes the full §2.4
sweep (9/9 schools) once landed. After Universal, the next frontier is
§2.5 equipment categories (`arms_armor`, `general`, `magic_items`,
`equipmods`), starting a new test/row shape (equipment resolver +
`MatrixSubjectType::Equipment`, not `School`) rather than the school
template used for all 8 cycles so far. The sibling-widening set remains
stable at three lists in `sd13_support_state_matrix.rs`
(`EXPECTED_ROW_IDS`, the inline `expected_above_observed` list in
`only_pilot_grounded_rows_rise_above_observed`, and
`EXPECTED_REFRESHABLE_FROM_LIVE_PROOF`) plus the row-count assertion and
its test-name suffix (`seed_contains_exactly_twenty_nine_rows` will need
renaming again at 30) and the doc comment above
`seeded_sd13_e1_f1_current_truth` — confirm no fourth hardcoded list
surfaces by running `cargo test --locked` (not just the criterion's own
test) after any matrix edit, before commit.

### cycle-2026-07-16T2233 | school.universal.spell_reachability (§2.4 Universal) | commit 268c987 | card t_12cca058 | evidence transition: Unverified/Observed -> Partial/Computed | cargo test 3405/3405 green (3402 pre-existing + 3 new, 0 regressions) | clippy clean | timing: ~15 min

Re-derived eligibility live rather than trusting the prior cycle's
summary. In-flight check (`ps -eo pid,ppid,etime,stat,cmd | grep -iE
'claude' | grep -v grep` plus tracing this shell's own `$PPID` chain)
found this session's own `claude -p` process (pid 2899465, ppid
2604107 — the standing supervisor `sd19-loop-supervisor.sh`) as its own
parent process, confirmed via exact `$PPID` match; no competing
criterion-specific claim in flight; no `CLAIM-EXISTS`. `git fetch origin
tranche/3` confirmed HEAD unchanged at `078977d`, matching this progress
doc's own snapshot exactly; `git status --porcelain` was clean.
`sd19-loop-cron.log` tail showed the prior cycle (Transmutation,
`078977d`) ended cleanly and this cycle's `START cycle` was the
supervisor's normal back-to-back cadence, not an overlap. Read the two
required SD-18 investigation sections (cycle-2026-07-15T0300 §3.4 and
cycle-2026-07-15T0400 §3.5, found at
`programs/codex/requirements/SD-18-core-rules-breadth/artifacts/SD-18-core-rules-breadth-progress.md`)
as read-only corroboration; no write made to that file.

Picked Universal per the operator's canonical-PF1-order priority (last
untried §2.4 school after Transmutation). Corpus-existence check per Step
4: `grep -c "SCHOOL:Universal" cr_spells.lst` → 5; `grep -n
"SCHOOL:Universal" cr_spells.lst` → present, sample "Wish", "Arcane Mark",
"Limited Wish", "Permanency", "Prestidigitation". Eligible.

RED: added `tests/sd19_school_universal.rs`, mirroring
`tests/sd19_school_transmutation.rs`'s shape exactly. Two of its three
assertions (every Universal spell resolves via `spell_id_resolve`; every
Universal spell reaches `corpus_derived.school_coverage[Universal]`)
passed immediately — same as the prior eight school cycles, the
capability slice's seam is generic, so no new seam code was needed. The
third assertion (a `MatrixSubjectType::School(Universal)` row exists in
the seeded matrix) failed for the right reason: no Universal row had been
added yet.

GREEN: added the `school.universal.spell_reachability` row to
`support_state_matrix.rs` (Partial/Computed; blocker note names the same
permanent slot-math/spellbook-posture/DC exclusion per decisions.md §1.3
and the operator-UI gate for Product-visible), mirroring the
Transmutation row's shape. Widened the same three sibling closed-world
lists in `tests/sd13_support_state_matrix.rs` for sibling-preservation
(not scope creep) — `EXPECTED_ROW_IDS` (29->30), the inline
`expected_above_observed` list in
`only_pilot_grounded_rows_rise_above_observed`, and
`EXPECTED_REFRESHABLE_FROM_LIVE_PROOF` (28->29) — plus the row-count
assertion and its test-name suffix
(`seed_contains_exactly_twenty_nine_rows` renamed to
`seed_contains_exactly_thirty_rows`), exactly the stable set the prior
cycle's handoff note predicted; no fourth hardcoded list surfaced.
`tests/sd13_elf_bounded_race_semantics.rs` needed no change — confirmed
its `School(_)`/`Equipment(_)` allowlist arms already admit any variant
generically. `cargo test --locked` 3405/3405 green, `cargo clippy
--locked --tests -- -D warnings` clean.

Commit `268c987` pushed directly to `tranche/3`. Card `t_12cca058` minted
(same CLI self-heal as the prior eight cycles: `hermes kanban create`
rejects `--initial-status done`; created with default `ready` status then
`hermes kanban complete`).

**Next cycle:** re-derive eligibility live. §2.4 spell schools is now
9/9 landed — the full spell-school sweep is closed. The next frontier is
§2.5 equipment categories, priority order per operator directive
2026-07-14: `arms_armor`, `general`, `magic_items`, `equipmods` (corpus-
natural order). This starts a new test/row shape: the equipment resolver
(`equipment_id_resolve`) and `MatrixSubjectType::Equipment(EquipmentCategory)`
rather than the `School`/`spell_id_resolve` template used for all 9
cycles so far — the next cycle should read `equipment_resolver.rs` and
the `EquipmentCategory` enum shape (in `rules_tables/crb/equipment_tables.rs`
or `support_state_matrix.rs`) before writing its RED test, since there is
no existing per-category sibling test to mirror yet. Corpus-existence
check for `arms_armor`: `cr_equip_arms_armor.lst`. Watch for a possible
fourth hardcoded sibling list in `sd13_support_state_matrix.rs` (none has
surfaced in 9 cycles, but the row-count test-name suffix
(`seed_contains_exactly_thirty_rows`) will need renaming again at 31) —
confirm by running `cargo test --locked` (not just the criterion's own
test) after any matrix edit, before commit.

### cycle-2026-07-16T1841 | equipment.arms_armor.equipment_reachability (§2.5 arms_armor) | commit e08607e | card t_47eca99f | evidence transition: Unverified/Observed -> Partial/Computed | cargo test 3408/3408 green (3405 pre-existing + 3 new, 0 regressions) | clippy clean | timing: ~35 min

Re-derived eligibility live. In-flight check (`ps -eo pid,etime,stat,cmd |
grep -iE 'claude' | grep -v grep`) found exactly one `claude -p` process
running this exact SD-19 loop prompt (this session itself, matching the
standing supervisor's normal back-to-back cadence — no competing process).
`git fetch origin tranche/3` confirmed HEAD unchanged at `268c987`,
matching this progress doc's own snapshot; `git status --porcelain` was
clean. Read the two required SD-18 investigation sections
(cycle-2026-07-15T0300 §3.4 and cycle-2026-07-15T0400 §3.5, found at
`programs/codex/requirements/SD-18-core-rules-breadth/artifacts/SD-18-core-rules-breadth-progress.md`)
as read-only corroboration; no write made to that file.

Picked `arms_armor` per the operator's corpus-natural equipment-category
priority (first §2.5 item after the full §2.4 sweep closed). This is the
first §2.5 cycle — no existing per-category sibling test to mirror, so
read `equipment_resolver.rs`, `pilot_compute_corpus.rs`'s `equipped_items`
shape, and `rules_tables::crb::equipment_tables::EquipmentCategory` before
writing the RED test. Chose a 3-item representative sample (per
`scope-draft.md` §2.5's "representative sample" bar, distinct from §2.4's
exhaustive-per-school bar): Longsword, Banded Mail, Armor Spikes.
Corpus-existence check per Step 4: `grep -n "KEY:Longsword (Base)"`,
`grep -n "KEY:Banded Mail (Base)"`, `grep -n "KEY:Armor Spikes"` against
`cr_equip_arms_armor.lst` — all three present (lines 165, 46, 137). Eligible.

RED: added `tests/sd19_equipment_arms_armor.rs`. Two of its three
assertions (every sample item resolves via `equipment_id_resolve`; every
sample item reaches `corpus_derived.equipped_items` with
`equipment_record_name`/`equipment_record_key` populated) passed
immediately — the capability slice's `equipment_id_resolve` and
`compute_pilot_with_corpus` are already generic (proven in the capability
slice's own `sd19_seam_shapes_correctness.rs`), so no new seam code was
needed, mirroring the §2.4 cycles' finding. The third assertion (a
`MatrixSubjectType::Equipment(ArmsArmor)` row exists in the seeded matrix)
failed for the right reason: no arms_armor row had been added yet.

GREEN: added the `equipment.arms_armor.equipment_reachability` row to
`support_state_matrix.rs` (Partial/Computed; blocker note names (a) the
same operator-UI gate for Product-visible, (b) that only the Longsword
sample item grounds through the foundation slice's single bootstrap table
cell (`EQUIPMENT_TABLES` has one arms_armor entry, keyed `"Longsword
(Base)"`) while Banded Mail/Armor Spikes' `table_cell` stays `None`, and
(c) that `derived_stats` (armor_bonus/attack_bonus/max_dex/spell_failure)
stays fully unpopulated — the seam module's own doc comment already names
this a documented "bounded-baseline non-goal" of the capability slice, not
a per-cycle widening job). Widened the same three sibling closed-world
lists in `tests/sd13_support_state_matrix.rs` for sibling-preservation —
`EXPECTED_ROW_IDS` (30->31), the inline `expected_above_observed` list in
`only_pilot_grounded_rows_rise_above_observed`, and
`EXPECTED_REFRESHABLE_FROM_LIVE_PROOF` (29->30) — plus the row-count
assertion and its test-name suffix (`seed_contains_exactly_thirty_rows`
renamed to `seed_contains_exactly_thirty_one_rows`), exactly the stable
set the prior cycle's handoff note predicted; no fourth hardcoded list
surfaced. `cargo test --locked` 3408/3408 green, `cargo clippy --locked
--tests -- -D warnings` clean.

Commit `e08607e` pushed directly to `tranche/3`. Card `t_47eca99f` minted
(same CLI self-heal as the prior nine cycles: `hermes kanban create`
rejects `--initial-status done`; created with `--initial-status running`
which the CLI itself normalized to `ready`, then `hermes kanban complete`).

**Next cycle:** re-derive eligibility live. `arms_armor` (§2.5) is now
Partial/Computed, 1/4 equipment categories touched. The next frontier is
`general` (§2.5), per the operator's corpus-natural priority order
(`arms_armor`, `general`, `magic_items`, `equipmods`). The
`sd19_equipment_arms_armor.rs` test file is now the sibling template to
mirror for `general` (same three-assertion shape: corpus-existence KEY:
grep, resolve+equipped_items membership, matrix-row check) — pick a fresh
3-item representative sample from `cr_equip_general.lst`, verify each
sample item's `KEY:` token via live grep before writing the test, and note
that the foundation slice's `EQUIPMENT_TABLES` bootstrap entry for
`General` is keyed `"Backpack"` (so only a sample item matching that key
will carry a non-`None` `table_cell`, same asymmetry pattern as this
cycle's Longsword-only grounding). Watch for a possible fourth hardcoded
sibling list in `sd13_support_state_matrix.rs` (none has surfaced in 10
cycles, but the row-count test-name suffix
(`seed_contains_exactly_thirty_one_rows`) will need renaming again at 32)
— confirm by running `cargo test --locked` after any matrix edit, before
commit.

### cycle-2026-07-16T2340 | equipment.general.equipment_reachability (§2.5 general) | commit eaaa6b7 | card t_a9a39797 | evidence transition: Unverified/Observed -> Partial/Computed | cargo test 3411/3411 green (3408 pre-existing + 3 new, 0 regressions) | clippy clean | timing: ~20 min

Re-derived eligibility live rather than trusting the prior cycle's
summary. In-flight check (`ps -eo pid,ppid,etime,stat,cmd | grep -iE
'claude' | grep -v grep` plus tracing this shell's own `$PPID`) found
this session's own `claude -p` process (pid 2974589, ppid 2604107 — the
standing supervisor `sd19-loop-supervisor.sh`) as its own parent process
chain, confirmed via exact `$PPID` match; no competing criterion-specific
claim in flight; no `CLAIM-EXISTS`. `git fetch origin tranche/3`
confirmed HEAD unchanged at `e08607e`, matching this progress doc's own
snapshot exactly; `git status --porcelain` was clean. Read the two
required SD-18 investigation sections (cycle-2026-07-15T0300 §3.4 and
cycle-2026-07-15T0400 §3.5, found at
`programs/codex/requirements/SD-18-core-rules-breadth/artifacts/SD-18-core-rules-breadth-progress.md`)
as read-only corroboration; no write made to that file. Confirmed both
sections describe the pre-capability-slice structural blocker (no
corpus-aware compute seam existed at investigation time) — fully
superseded by the SD-19 foundation + capability slices that already
landed; not a live blocker for this cycle.

Picked `general` per the operator's corpus-natural equipment-category
priority (second §2.5 item, following `arms_armor`). Read
`equipment_resolver.rs`'s normalization rule and the arms_armor sibling
test/row before writing this cycle's test, per the prior cycle's
handoff note. Chose a 3-item representative sample from
`cr_equip_general.lst`: Backpack, Torch, Waterskin. Corpus-existence
check per Step 4: `grep -n "KEY:Backpack"`, `grep -n "KEY:Torch"`, `grep
-n "KEY:Waterskin"` against `cr_equip_general.lst` — all three present
(lines 107, 180, 183). Eligible.

RED: added `tests/sd19_equipment_general.rs`, mirroring
`tests/sd19_equipment_arms_armor.rs`'s shape exactly. Two of its three
assertions (every sample item resolves via `equipment_id_resolve`; every
sample item reaches `corpus_derived.equipped_items` with
`equipment_record_name`/`equipment_record_key` populated) passed
immediately — the capability slice's `equipment_id_resolve` and
`compute_pilot_with_corpus` are already generic, so no new seam code was
needed, mirroring the arms_armor cycle's finding. The third assertion (a
`MatrixSubjectType::Equipment(General)` row exists in the seeded matrix)
failed for the right reason: no `general` row had been added yet.

GREEN: added the `equipment.general.equipment_reachability` row to
`support_state_matrix.rs` (Partial/Computed; blocker note names (a) the
same operator-UI gate for Product-visible, (b) that only the Backpack
sample item grounds through the foundation slice's single bootstrap
table cell (`EQUIPMENT_TABLES` has one general entry, keyed
`"Backpack"`) while Torch/Waterskin's `table_cell` stays `None`, and (c)
that `derived_stats` (armor_bonus/attack_bonus/max_dex/spell_failure)
stays fully unpopulated — the same documented "bounded-baseline
non-goal" as every prior §2.5 cycle, not a per-cycle widening job).
Widened the same three sibling closed-world lists in
`tests/sd13_support_state_matrix.rs` for sibling-preservation —
`EXPECTED_ROW_IDS` (31->32), the inline `expected_above_observed` list
in `only_pilot_grounded_rows_rise_above_observed`, and
`EXPECTED_REFRESHABLE_FROM_LIVE_PROOF` (30->31) — plus the row-count
assertion and its test-name suffix
(`seed_contains_exactly_thirty_one_rows` renamed to
`seed_contains_exactly_thirty_two_rows`), exactly the stable set the
prior cycle's handoff note predicted; no fourth hardcoded list surfaced.
`tests/sd13_elf_bounded_race_semantics.rs` needed no change — confirmed
its `School(_)`/`Equipment(_)` allowlist arms already admit any variant
generically. `cargo test --locked` 3411/3411 green, `cargo clippy
--locked --tests -- -D warnings` clean.

Commit `eaaa6b7` pushed directly to `tranche/3`. Card `t_a9a39797` minted
(same CLI self-heal as the prior ten cycles: `hermes kanban create`
rejects `--initial-status done`; created with `--initial-status running`
which the CLI itself normalized to `ready`, then `hermes kanban
complete`).

**Next cycle:** re-derive eligibility live. `general` (§2.5) is now
Partial/Computed, 2/4 equipment categories touched. The next frontier is
`magic_items` (§2.5), per the operator's corpus-natural priority order
(`arms_armor`, `general`, `magic_items`, `equipmods`). The
`sd19_equipment_general.rs` test file is now the sibling template to
mirror for `magic_items` (same three-assertion shape: corpus-existence
KEY: grep, resolve+equipped_items membership, matrix-row check) — pick a
fresh 3-item representative sample from `cr_equip_magic_items.lst`,
verify each sample item's `KEY:` token via live grep before writing the
test, and note that the foundation slice's `EQUIPMENT_TABLES` bootstrap
entry for `MagicItems` is keyed `"Potion of Aid"` (so only a sample item
matching that key will carry a non-`None` `table_cell`, same asymmetry
pattern as the prior two equipment cycles). Watch for a possible fourth
hardcoded sibling list in `sd13_support_state_matrix.rs` (none has
surfaced in 11 cycles, but the row-count test-name suffix
(`seed_contains_exactly_thirty_two_rows`) will need renaming again at 33)
— confirm by running `cargo test --locked` after any matrix edit, before
commit.

### cycle-2026-07-16T1858 | equipment.magic_items.equipment_reachability (§2.5 magic_items) | commit 1689b16 | card t_b165e9ce | evidence transition: Unverified/Observed -> Partial/Computed | cargo test 3414/3414 green (3411 pre-existing + 3 new, 0 regressions) | clippy clean | timing: ~25 min

Re-derived eligibility live rather than trusting the prior cycle's
summary. In-flight check (`ps -eo pid,ppid,etime,stat,cmd | grep -iE
'claude' | grep -v grep` plus tracing this shell's own `$PPID`) found this
session's own `claude -p` process (pid 3007273, ppid 2604107 — the
standing supervisor `sd19-loop-supervisor.sh`) as its own parent process
chain, confirmed via exact `$PPID` match; no competing criterion-specific
claim in flight; no `CLAIM-EXISTS`. `git fetch origin tranche/3`
confirmed HEAD unchanged at `eaaa6b7`, matching this progress doc's own
snapshot exactly; `git status --porcelain` was clean. Read the two
required SD-18 investigation sections (cycle-2026-07-15T0300 §3.4 and
cycle-2026-07-15T0400 §3.5, as read-only reference) per the invocation's
own instruction; no write made to that file.

Picked `magic_items` per the operator's corpus-natural equipment-category
priority (third §2.5 item, following `arms_armor` and `general`). Read
`equipment_resolver.rs`'s normalization rule and the `general` sibling
test/row before writing this cycle's test, per the prior cycle's handoff
note. Initial 3-item representative sample drafted per the prior cycle's
suggestion — `Potion of Aid` (to ground via the foundation slice's
bootstrap table cell), `Wand of Magic Missile`, `Ring of Protection +1` —
all three confirmed present via `grep -n "KEY:<token>"
cr_equip_magic_items.lst` before the test was written (lines 491, 683,
396). Eligible per the corpus-existence check.

RED (first pass): `equipment_id_resolve` returned `None` for `Wand of
Magic Missile` even though its `KEY:` token is genuinely present in the
corpus. Investigated with a scratch example binary (`examples/check_wand.rs`,
deleted after use — not part of this commit) rather than guessing:
traced the failure to `equipment.rs`'s `open_record`, which merges
records by `(kind, name)`, combined with `extract_record_name` stripping
`.COPY=` suffixes — so every `.COPY=` item sharing a base word ("Wand",
"Potion") collapses into one merged `EquipmentRecord` (confirmed: 351
`KEY:` tokens under `name == "Wand"`, 108 under `name == "Potion"`), and
`equipment_key_token`'s `.find()` surfaces only the first by line number.
`Potion of Aid` happened to be file-order-first among potions (which is
why it alone would have resolved), `Wand of Magic Missile` was not
file-order-first among wands (`Wand of Acid Splash` was). This is a
parser-level defect in `equipment.rs`, not in this cycle's file-touch
scope (`equipment_resolver.rs`, not the parser) — logged as an
informational, non-blocking Open Blockers entry (this cycle still landed
a commit) routed to SD-17's lane.

RED (corrected sample): replaced `Potion of Aid` and `Wand of Magic
Missile` with two non-`.COPY=` records — `Amulet of Natural Armor +1`
(line 20) and `Belt of Giant Strength +2` (line 38) — each independently
confirmed via the scratch example to be its own standalone
`EquipmentRecord` (exactly one `KEY:` token, no merge), alongside `Ring of
Protection +1` (already confirmed standalone). Added
`tests/sd19_equipment_magic_items.rs`, mirroring
`tests/sd19_equipment_general.rs`'s three-assertion shape. Two of its
three assertions (every sample item resolves via `equipment_id_resolve`;
every sample item reaches `corpus_derived.equipped_items` with
`equipment_record_name`/`equipment_record_key` populated) passed
immediately with the corrected sample — same as every prior §2.4/§2.5
cycle, the capability slice's seam is generic, so no new seam code was
needed. The third assertion (a `MatrixSubjectType::Equipment(MagicItems)`
row exists in the seeded matrix) failed for the right reason: no
`magic_items` row had been added yet.

GREEN: added the `equipment.magic_items.equipment_reachability` row to
`support_state_matrix.rs` (Partial/Computed; blocker note names (a) the
same operator-UI gate for Product-visible, (b) that none of the three
sample items grounds through the foundation slice's bootstrap table cell
today (`EQUIPMENT_TABLES`' single magic_items entry is keyed `"Potion of
Aid"`, deliberately not in the corrected sample), (c) that `derived_stats`
stays fully unpopulated — the same documented "bounded-baseline
non-goal" as every prior §2.5 cycle, and (d) the category-wide `.COPY=`
parser-merge limitation discovered this cycle. Widened the same three
sibling closed-world lists in `tests/sd13_support_state_matrix.rs` for
sibling-preservation — `EXPECTED_ROW_IDS` (32->33), the inline
`expected_above_observed` list in
`only_pilot_grounded_rows_rise_above_observed`, and
`EXPECTED_REFRESHABLE_FROM_LIVE_PROOF` (31->32) — plus the row-count
assertion and its test-name suffix
(`seed_contains_exactly_thirty_two_rows` renamed to
`seed_contains_exactly_thirty_three_rows`), exactly the stable set the
prior cycle's handoff note predicted; no fourth hardcoded list surfaced.
`cargo test --locked` 3414/3414 green, `cargo clippy --locked --tests --
-D warnings` clean.

Commit `1689b16` pushed directly to `tranche/3`. Card `t_b165e9ce` minted
(same CLI self-heal as the prior eleven cycles: `hermes kanban create`
rejects `--initial-status done`; created with `--initial-status running`
which the CLI itself normalized to `ready`, then `hermes kanban
complete`).

**Next cycle:** re-derive eligibility live. `magic_items` (§2.5) is now
Partial/Computed, 3/4 equipment categories touched, `14/15` SD-19
criteria touched overall. The next and last frontier is `equipmods`
(§2.5) — `cr_equipmods.lst`, per the operator's corpus-natural priority
order (`arms_armor`, `general`, `magic_items`, `equipmods`). Landing
`equipmods` closes the full §2.5 sweep and all 15 SD-19 acceptance
criteria's per-cycle-eligible work (both pre-loop gates + 9 schools + 4
equipment categories). Per this cycle's Open Blockers note: check
`cr_equipmods.lst` for the same `.COPY=` naming pattern before drafting
its representative sample, and prefer non-`.COPY=` (or verified-standalone)
records if it is present, rather than discovering the same collision
mid-cycle. The `tests/sd19_equipment_magic_items.rs` file is the sibling
template to mirror (three-assertion shape: corpus-existence KEY: grep,
resolve+equipped_items membership, matrix-row check). Watch for a
possible fourth hardcoded sibling list in `sd13_support_state_matrix.rs`
(none has surfaced in 12 cycles, but the row-count test-name suffix
(`seed_contains_exactly_thirty_three_rows`) will need renaming again at
34) — confirm by running `cargo test --locked` after any matrix edit,
before commit.

### cycle-2026-07-16T2359 | equipment.equipmods.equipment_reachability (§2.5 equipmods) | commit c46c9b6 | card t_bcdb8ecf | evidence transition: Unverified/Observed -> Partial/Computed | cargo test 3417/3417 green (3411 pre-existing + 3 new + 3 sibling-widened, 0 regressions) | clippy clean | timing: ~40 min

Re-derived eligibility live rather than trusting the prior cycle's
summary. In-flight check (`ps -eo pid,ppid,etime,stat,cmd | grep -iE
'claude' | grep -v grep` plus tracing this shell's own `$PPID` chain via
a small Python ancestry-walk since `pstree` was unavailable) found one
`claude -p` process (pid 3051536, ppid 2604107 — the standing supervisor
`sd19-loop-supervisor.sh`) running the identical SD-19 loop prompt; this
shell's own `$PPID` (3051536) matched that pid exactly, confirming it is
this very session's own top-level process, not a competing claim — no
`CLAIM-EXISTS`. `git fetch origin tranche/3` confirmed HEAD unchanged at
`1689b16`, matching this progress doc's own snapshot exactly; `git status
--porcelain` was clean; `git worktree list --porcelain` showed only the
single primary worktree. Read the two required SD-18 investigation
sections (found at
`programs/codex/requirements/SD-18-core-rules-breadth/artifacts/SD-18-core-rules-breadth-progress.md`,
since the file no longer exists at the workspace-root path the loop
instruction names) as read-only corroboration — both describe the
pre-capability-slice structural blocker, fully superseded by the SD-19
foundation + capability slices already landed; not a live blocker for
this cycle. No write made to that file.

Picked `equipmods` per the operator's corpus-natural equipment-category
priority (fourth and last §2.5 item, following `arms_armor`, `general`,
and `magic_items`). Read `equipment_resolver.rs`'s normalization rule and
the `magic_items` sibling test/row before writing this cycle's test, per
the prior cycle's handoff note — including its Open Blockers instruction
to check `cr_equipmods.lst` for the same `.COPY=` naming pattern before
drafting a sample.

Corpus-existence and defect-avoidance check per Step 4: `grep -c
"\.COPY="` against `cr_equipmods.lst` → 335 of 347 total `KEY:`-bearing
records use `.COPY=`, confirming the prior cycle's warning was
warranted. Went further than a `.COPY=`-only check: wrote a name-frequency
scan (Python, ad hoc, not committed) over the *entire* file's name field
(column 0, `.COPY=` suffix stripped when present) and found the
`open_record` by-name merge collision is not limited to `.COPY=` rows —
two plain (non-`.COPY=`) records share the name `"Cloth"` (`KEY:Material
~ Cloth` at line 10, `KEY:Artisan's Tools (Cloth)` at line 78), which
also merge. Selected three sample items independently confirmed unique
(count == 1 across the whole file) via the same scan: `Masterwork
(Weapon)` (`KEY:Special Quality ~ Masterwork ~ Weapon`, line 18), `Brace`
(`KEY:Special Quality ~ Brace`, line 38), `Disarm` (`KEY:Special Quality
~ Disarm`, line 39) — each independently re-verified present via `grep
-n "KEY:<token>" cr_equipmods.lst` before the test was written. Eligible.

RED: added `tests/sd19_equipment_equipmods.rs`, mirroring
`tests/sd19_equipment_magic_items.rs`'s three-assertion shape. Two of its
three assertions (every sample item resolves via `equipment_id_resolve`;
every sample item reaches `corpus_derived.equipped_items` with
`equipment_record_name`/`equipment_record_key` populated) passed
immediately — same as every prior §2.4/§2.5 cycle, the capability
slice's seam is generic, so no new seam code was needed; the sample's
pre-verified uniqueness meant no `.COPY=`/by-name-merge surprise
occurred this time (unlike the magic_items cycle's first-pass RED
failure). The third assertion (a
`MatrixSubjectType::Equipment(Equipmods)` row exists in the seeded
matrix) failed for the right reason: no `equipmods` row had been added
yet.

GREEN: added the `equipment.equipmods.equipment_reachability` row to
`support_state_matrix.rs` (Partial/Computed; blocker note names (a) the
same operator-UI gate for Product-visible, (b) that none of the three
sample items grounds through the foundation slice's bootstrap table cell
(`EQUIPMENT_TABLES`' single equipmods entry is keyed `"Material ~
Cloth"`, deliberately not in this sample), (c) that `derived_stats` stays
fully unpopulated — the same documented "bounded-baseline non-goal" as
every prior §2.5 cycle, and (d) the broader (not `.COPY=`-limited)
by-name-merge collision this cycle confirmed present in this file too).
Widened the same three sibling closed-world lists in
`tests/sd13_support_state_matrix.rs` for sibling-preservation —
`EXPECTED_ROW_IDS` (33->34), the inline `expected_above_observed` list in
`only_pilot_grounded_rows_rise_above_observed`, and
`EXPECTED_REFRESHABLE_FROM_LIVE_PROOF` (32->33) — plus the row-count
assertion and its test-name suffix
(`seed_contains_exactly_thirty_three_rows` renamed to
`seed_contains_exactly_thirty_four_rows`) and the doc comment above
`seeded_sd13_e1_f1_current_truth` (33->34 rows, 3->4 equipment
categories); exactly the stable set the prior cycles' handoff notes
predicted; no fourth hardcoded list surfaced. `cargo test --locked`
3417/3417 green, `cargo clippy --locked --tests -- -D warnings` clean.

Commit `c46c9b6` pushed directly to `tranche/3`. Card `t_bcdb8ecf` minted
(`hermes kanban create --initial-status ready` is no longer a valid
choice — the CLI now only accepts `blocked`/`running`; created with
`--initial-status running`, which the CLI normalized to `ready`, then
`hermes kanban complete`, same self-heal pattern as all prior cycles).

**This closes the full §2.5 equipment-category sweep (4/4:
arms_armor, general, magic_items, equipmods) and all 15 SD-19
acceptance criteria's per-cycle-eligible work** (2 pre-loop gates + 9
§2.4 schools + 4 §2.5 equipment categories). Every row now sits at
`Partial/Computed`. Per the loop instruction's own definition of
`supported/Product-visible`, no further per-criterion cycle is eligible
until the operator either (a) authorizes UI-surfacing work to promote
rows' evidence_tier, or (b) authorizes a new tranche-level scope such as
the SD-17-lane `equipment.rs` by-name/`.COPY=`-merge fix flagged across
the last two cycles' Open Blockers entries. **Next cycle instruction:**
re-derive eligibility live per Step 1 rather than trusting this summary
as current; if all 15 criteria are still `Partial/Computed` with no new
operator directive, the correct action is a NO-OP cycle that records the
live re-confirmation in this doc rather than inventing new loop-routed
work — building UI-surfacing or the parser fix from inside a per-cycle
loop would itself be a forbidden tranche-level decision, mirroring the
reasoning in the very first blocked cycles (cycle-2026-07-16T1631/1633)
before the pre-loop slices landed.

### cycle-2026-07-16T1923 | no-op-full-frontier-recheck | no commit | card: none minted (no eligible work attempted) | evidence transition: none (15/15 criteria already touched; re-confirms cycle-2026-07-16T2359's closing conclusion, condition unchanged) | cargo test: not run (no eligible criterion, no code change attempted) | clippy: not run | timing: ~10 min (live re-verification only)

Re-derived eligibility live rather than trusting the prior cycle's own
summary: `git fetch origin tranche/3` confirms HEAD still `c46c9b6` (no
drift since the equipmods cycle); in-flight check confirms this cycle's
own `claude -p` process (pid 3085123) is the standing supervisor's
currently-live child, not a second competing process (no Hard-stop-#3
collision); direct greps of `support_state_matrix.rs` and `tests/`
independently confirm all 9 `MatrixSubjectType::School` rows, all 4
`MatrixSubjectType::Equipment` rows, and all 13 per-criterion test files
are present and match the progress doc's own tracking sections exactly.
Read the two required SD-18 §3.4/§3.5 investigation-cycle sections
(archived at
`programs/codex/requirements/SD-18-core-rules-breadth/artifacts/SD-18-core-rules-breadth-progress.md`)
as read-only reference; both corroborate, without contradicting, the
historical structural gap SD-19's capability slice closed. No next
granular work-unit exists under Step 1/Step 2 for any of the 15
criteria — every §2.4 school already landed 100% of its corpus spells;
every §2.5 category already landed its representative sample, per the
scope doc's own bounded acceptance criteria. Logged a matching Open
Blockers entry (informational, not a hard stop) documenting the full
re-derivation. No code, commit, or kanban card this cycle since nothing
was eligible; the standing supervisor will re-run this same live check
on its next back-to-back cycle until the operator authorizes new
tranche-level scope (UI-surfacing work or the SD-17-lane `.COPY=`/
by-name-merge parser fix).

### cycle-2026-07-16T1928 | no-op-full-frontier-recheck | no commit | card: none minted (no eligible work attempted) | evidence transition: none (15/15 criteria already touched; re-confirms cycle-2026-07-16T2359's and cycle-2026-07-16T1923's closing conclusion, condition unchanged) | cargo test: not run (no eligible criterion, no code change attempted) | clippy: not run | timing: ~10 min (live re-verification only)

Re-derived eligibility live rather than trusting either prior summary:
`git fetch origin tranche/3` confirms HEAD still `c46c9b6`; in-flight
check traced this shell's own `$PPID` chain (`ps -o pid,ppid,cmd -p $$`)
directly to pid 3086498 (ppid 2604107, the standing supervisor), so the
one running `claude -p` process found is this session's own top-level
process, not a competing claim — no Hard-stop-#3 collision. Direct greps
of `support_state_matrix.rs` and `tests/` independently confirm all 9
`MatrixSubjectType::School` rows, all 4 `MatrixSubjectType::Equipment`
rows, and all 15 test files (13 per-criterion + 2 pre-loop-slice) are
present and match this doc's own tracking sections exactly. Also read
`decisions.md` §9 (source-book subdirectory pattern) and the two
required SD-18 §3.4/§3.5 sections as read-only corroboration; neither
changes this cycle's conclusion. No next granular work-unit exists under
Step 1/Step 2 for any of the 15 criteria. Logged a matching Open
Blockers entry (informational, not a hard stop). No code, commit, or
kanban card this cycle since nothing was eligible; the standing
supervisor will re-run this same live check on its next back-to-back
cycle until the operator authorizes new tranche-level scope
(UI-surfacing work or the SD-17-lane `.COPY=`/by-name-merge parser fix).

### cycle-2026-07-16T1935 | no-op-full-frontier-recheck | no commit | card: none minted (no eligible work attempted) | evidence transition: none (15/15 criteria already touched; re-confirms cycle-2026-07-16T2359's, cycle-2026-07-16T1923's, and cycle-2026-07-16T1928's closing conclusion, condition unchanged) | cargo test: not run (no eligible criterion, no code change attempted) | clippy: not run | timing: ~10 min (live re-verification only)

### cycle-2026-07-16T1933 | no-op-full-frontier-recheck | no commit | card: none minted (no eligible work attempted) | evidence transition: none (15/15 criteria already touched; re-confirms cycle-2026-07-16T2359's, cycle-2026-07-16T1923's, cycle-2026-07-16T1928's, and cycle-2026-07-16T1935's closing conclusion, condition unchanged) | cargo test: not run (no eligible criterion, no code change attempted) | clippy: not run | timing: ~8 min (live re-verification only)

Re-derived eligibility live rather than trusting any prior summary: `git
fetch origin tranche/3` confirms HEAD still `c46c9b6`; in-flight check
traced this shell's own `$PPID` chain (`ps -o pid,ppid,cmd -p $$`)
directly to pid 3087520 (ppid 2604107, the standing supervisor), so the
one running `claude -p` process found is this session's own top-level
process, not a competing claim — no Hard-stop-#3 collision. Direct greps
of `support_state_matrix.rs` and `tests/` independently confirm all 9
`MatrixSubjectType::School` rows, all 4 `MatrixSubjectType::Equipment`
rows, and all 15 test files (13 per-criterion + 2 pre-loop-slice) are
present and match this doc's own tracking sections exactly. Read the two
required SD-18 §3.4/§3.5 investigation-cycle sections as read-only
reference; neither changes this cycle's conclusion. No next granular
work-unit exists under Step 1/Step 2 for any of the 15 criteria. Logged a
matching Open Blockers entry (informational, not a hard stop). No code,
commit, or kanban card this cycle since nothing was eligible; the standing
supervisor will re-run this same live check on its next back-to-back
cycle until the operator authorizes new tranche-level scope (UI-surfacing
work or the SD-17-lane `.COPY=`/by-name-merge parser fix).

### cycle-2026-07-16T1936 | no-op-full-frontier-recheck | no commit | card: none minted (no eligible work attempted) | evidence transition: none (15/15 criteria already touched; re-confirms cycle-2026-07-16T2359's, T1923's, T1928's, T1935's, T1933's, and T1940's closing conclusion, condition unchanged) | cargo test: not run (no eligible criterion, no code change attempted) | clippy: not run | timing: ~3 min (live re-verification only)

Re-derived eligibility live rather than trusting any prior summary:
`git fetch origin tranche/3` confirms HEAD still `c46c9b6`, matching
`snapshot_as_of`; `git status --porcelain` 0 lines; `git worktree list
--porcelain` shows only the primary worktree. In-flight check
(`ps -eo pid,etime,stat,cmd | grep claude`) found exactly one `claude -p`
process running the SD-19 loop prompt (pid 3089745); traced this
session's own bash shell ancestry (`ps -o pid,ppid,cmd`) directly to pid
3089745 -> ppid 2604107 (`sd19-loop-supervisor.sh`), confirming that
process is this session's own top-level process, not a second competing
claim — no Hard-stop-#3 collision. Direct greps of
`support_state_matrix.rs` confirm all 9 `MatrixSubjectType::School(...)`
rows and all 4 `MatrixSubjectType::Equipment(...)` rows present; `ls
tests/sd19_*.rs` confirms all 15 per-criterion/slice test files present
— no drift from this doc's own §2.4/§2.5 tracking. Read the two required
SD-18 §3.4/§3.5 investigation-cycle sections as read-only reference;
neither changes this cycle's conclusion. No next granular work-unit
exists under Step 1/Step 2 for any of the 15 criteria — every §2.4
school already landed 100% of its corpus spells; every §2.5 category
already landed its representative sample. Logged a matching Open
Blockers entry below. No code, commit, or kanban card this cycle since
nothing was eligible.

**Operator note (escalating beyond prior no-op entries):** this is the
sixth consecutive no-op cycle re-confirming the identical conclusion
(T1923, T1928, T1935, T1933, T1940, T1936), well past the "three in a
row" threshold the loop instruction's own Operating Posture §3 flags as
worth investigating. The standing supervisor will keep re-deriving and
re-confirming this same exhausted-frontier state on every future
back-to-back cycle until the operator either (a) authorizes UI-surfacing
work to promote rows' evidence_tier toward `Product-visible`, or (b)
authorizes the SD-17-lane `equipment.rs` `.COPY=`/by-name-merge parser
fix as a new tranche-level scope, or (c) pauses the supervisor. Recorded
here so the next reader (operator or cycle) does not have to re-count.

### cycle-2026-07-16T1941 | no-op-full-frontier-recheck | no commit | card: none minted (no eligible work attempted) | evidence transition: none (15/15 criteria already touched; re-confirms cycle-2026-07-16T2359's, T1923's, T1928's, T1935's, T1933's, T1940's, and T1936's closing conclusion, condition unchanged) | cargo test: not run (no eligible criterion, no code change attempted) | clippy: not run | timing: ~9 min (live re-verification only)

Re-derived eligibility live rather than trusting any prior summary: `git
fetch origin tranche/3` confirms HEAD still `c46c9b6`, matching
`snapshot_as_of`; `git status --porcelain` 0 lines; `git worktree list
--porcelain` shows only the primary worktree. In-flight check
(`ps -eo pid,ppid,etime,stat,cmd | grep claude`) found exactly one
`claude -p` process running the SD-19 loop prompt (pid 3090902); traced
this session's own bash shell ancestry (`ps -o pid,ppid,cmd -p $$`)
directly to pid 3090902 → ppid 2604107 (`sd19-loop-supervisor.sh`),
confirming that process is this session's own top-level process, not a
second competing claim — no Hard-stop-#3 collision. Direct greps of
`support_state_matrix.rs` confirm all 9 `MatrixSubjectType::School(...)`
rows and all 4 `MatrixSubjectType::Equipment(...)` rows present, and this
cycle went one step further than every prior no-op cycle by also
re-checking each row's `evidence_tier` directly rather than only row
presence — all 13 still read `Computed`, none has silently advanced to
`ProductVisible`. `ls tests/sd19_*.rs` confirms all 15 per-criterion/slice
test files present — no drift from this doc's own §2.4/§2.5 tracking.
Read the two required SD-18 §3.4/§3.5 investigation-cycle sections
(located at lines 2797 and 2817 of the SD-18 progress doc) as read-only
reference; neither changes this cycle's conclusion. No next granular
work-unit exists under Step 1/Step 2 for any of the 15 criteria. Logged a
matching Open Blockers entry above. No code, commit, or kanban card this
cycle since nothing was eligible.

**Operator note:** this is now the SEVENTH consecutive no-op cycle
re-confirming the identical conclusion (T1923, T1928, T1935, T1933,
T1940, T1936, T1941). The frontier genuinely has no further loop-eligible
work: all 15 SD-19 acceptance criteria (2 pre-loop gates + 9 §2.4 schools
+ 4 §2.5 equipment categories) are landed at `Partial/Computed`, and
promotion to `Supported/Product-visible` requires operator-driven
UI-surfacing work per the loop instruction's own "What supported /
Product-visible actually means" section — not a per-cycle loop action.
The standing supervisor will keep re-confirming this same state every
cycle until the operator (a) authorizes UI-surfacing work, (b) authorizes
the SD-17-lane `equipment.rs` `.COPY=`/by-name-merge parser fix as new
tranche-level scope, or (c) pauses the supervisor.

### cycle-2026-07-16T1944 | no-op-full-frontier-recheck | no commit | card: none minted (no eligible work attempted) | evidence transition: none (15/15 criteria already touched; re-confirms cycle-2026-07-16T1941's, T2359's, T1923's, T1928's, T1935's, T1933's, T1940's, and T1936's closing conclusion, condition unchanged) | cargo test: not run (no eligible criterion, no code change attempted) | clippy: not run | timing: ~6 min (live re-verification only)

Re-derived eligibility live rather than trusting any prior summary: `git
fetch origin tranche/3` confirms HEAD still `c46c9b6`, matching
`snapshot_as_of`; `git status --porcelain` 0 lines; `git worktree list
--porcelain` shows only the primary worktree. In-flight check
(`ps -eo pid,ppid,etime,stat,cmd | grep claude`) found exactly one
`claude -p` process running the SD-19 loop prompt (pid 3091983); traced
this session's own bash shell ancestry (`ps -o pid,ppid,cmd -p $$`)
directly to pid 3091983 → ppid 2604107 (`sd19-loop-supervisor.sh`),
confirming that process is this session's own top-level process, not a
second competing claim — no Hard-stop-#3 collision. Direct greps of
`support_state_matrix.rs` confirm all 9 `MatrixSubjectType::School(...)`
rows and all 4 `MatrixSubjectType::Equipment(...)` rows present, and this
cycle re-checked each row's `evidence_tier`/`support_state` directly — all
13 still read `Partial`/`Computed`, none has silently advanced to
`ProductVisible`. `ls tests/sd19_*.rs` confirms all 15 per-criterion/slice
test files present — no drift from this doc's own §2.4/§2.5 tracking.
Read the two required SD-18 §3.4/§3.5 investigation-cycle sections
(located at lines 2797 and 2817 of the SD-18 progress doc) as read-only
reference; neither changes this cycle's conclusion. No next granular
work-unit exists under Step 1/Step 2 for any of the 15 criteria. Logged a
matching Open Blockers entry above. No code, commit, or kanban card this
cycle since nothing was eligible.

**Operator note:** this is now the EIGHTH consecutive no-op cycle
re-confirming the identical conclusion (T1923, T1928, T1935, T1933,
T1940, T1936, T1941, T1944). The frontier genuinely has no further
loop-eligible work: all 15 SD-19 acceptance criteria (2 pre-loop gates + 9
§2.4 schools + 4 §2.5 equipment categories) are landed at
`Partial/Computed`, and promotion to `Supported/Product-visible` requires
operator-driven UI-surfacing work per the loop instruction's own "What
supported / Product-visible actually means" section — not a per-cycle
loop action. The standing supervisor will keep re-confirming this same
state every cycle until the operator (a) authorizes UI-surfacing work, (b)
authorizes the SD-17-lane `equipment.rs` `.COPY=`/by-name-merge parser fix
as new tranche-level scope, or (c) pauses the supervisor.

### cycle-2026-07-16T1947 | no-op-full-frontier-recheck | no commit | card: none minted (no eligible work attempted) | evidence transition: none (15/15 criteria already touched; re-confirms cycle-2026-07-16T1944's, T1941's, T2359's, T1923's, T1928's, T1935's, T1933's, T1940's, and T1936's closing conclusion, condition unchanged) | cargo test: not run (no eligible criterion, no code change attempted) | clippy: not run | timing: ~10 min (live re-verification only)

Re-derived eligibility live rather than trusting any prior summary: `git
fetch origin tranche/3` confirms HEAD still `c46c9b6`, matching
`snapshot_as_of`; `git status --porcelain` 0 lines; `git worktree list
--porcelain` shows only the primary worktree; current branch `tranche/3`.
In-flight check (`ps -eo pid,ppid,etime,stat,cmd | grep -iE 'claude' |
grep -v grep`) found exactly one `claude -p` process running the SD-19
loop prompt (pid 3093341, ppid 2604107); traced this shell's own ancestry
(`ps -o pid,ppid,cmd -p $$`) directly to pid 3093341 → ppid 2604107
(`sd19-loop-supervisor.sh`), confirming that process is this session's
own top-level process, not a second competing claim — no Hard-stop-#3
collision. Direct greps of `support_state_matrix.rs` confirm all 9
`MatrixSubjectType::School(...)` rows and all 4
`MatrixSubjectType::Equipment(...)` rows present, and this cycle
re-checked each row's `support_state`/`evidence_tier` directly — all 13
still read `Partial`/`Computed`, none has silently advanced to
`ProductVisible`. `ls tests/sd19_*.rs` confirms all 15 per-criterion/slice
test files present — no drift from this doc's own §2.4/§2.5 tracking.
Read the two required SD-18 §3.4/§3.5 investigation-cycle sections
(found at lines 7721 and 7861 of
`programs/codex/requirements/SD-18-core-rules-breadth/artifacts/SD-18-core-rules-breadth-progress.md`)
as read-only reference; neither changes this cycle's conclusion. No next
granular work-unit exists under Step 1/Step 2 for any of the 15 criteria
— every §2.4 school already landed 100% of its corpus spells; every §2.5
category already landed its representative sample. Logged a matching
Open Blockers entry above. No code, commit, or kanban card this cycle
since nothing was eligible.

**Operator note:** this is now the NINTH consecutive no-op cycle
re-confirming the identical conclusion (T1923, T1928, T1935, T1933,
T1940, T1936, T1941, T1944, T1947). The frontier genuinely has no further
loop-eligible work: all 15 SD-19 acceptance criteria (2 pre-loop gates + 9
§2.4 schools + 4 §2.5 equipment categories) are landed at
`Partial/Computed`, and promotion to `Supported/Product-visible` requires
operator-driven UI-surfacing work per the loop instruction's own "What
supported / Product-visible actually means" section — not a per-cycle
loop action. The standing supervisor will keep re-confirming this same
state every cycle until the operator (a) authorizes UI-surfacing work, (b)
authorizes the SD-17-lane `equipment.rs` `.COPY=`/by-name-merge parser fix
as new tranche-level scope, or (c) pauses the supervisor.

## Post-loop-closure operator work

Both (a) and (b) above were authorized and completed on 2026-07-16,
after the supervisor was paused per the operator's confirmation:

### 2026-07-16T[post-closure] | SD-17-lane equipment.rs parser fix | PR #317 (branch `fix/sd17-equipment-copy-merge`, not `tranche/3` — general parser bug, out of SD-19's own scope) | not yet merged

Fixed `open_record`'s merge-by-name defect that collapsed distinct
`.COPY=` equipment variants (and coincidentally-same-named plain rows)
sharing a base template into one `EquipmentRecord`, silently discarding
every `KEY:` token but the first. Now merges by matching `KEY:` token
(falling back to name-only for the KEY-less case). See the PR for full
detail; not part of `tranche/3`'s own history since it's SD-17's lane,
not SD-19's.

### 2026-07-16T[post-closure] | UI-surfacing work | commits `98a23ee` / `626539c` / `d5f6531` on `tranche/3` | card t_90cfc527 | cargo test 3418/3418 green (root) + 67/67 (desktop backend) | npm typecheck clean, 40/40 test files, build succeeds | clippy clean | live-verified via run-desktop skill

Three commits, in dependency order:
1. `98a23ee` — fixed two bugs discovered while wiring this: (a)
   `spells_selected` was never written by the saved-character serializer
   (added to the struct and the *parser* by the capability slice, but
   never the writer), so it silently vanished on save/reload; (b) the
   fixture-line parser for `spell=` lines assumed exactly 3 colon-parts,
   breaking when `source_class_id` itself contains a colon (e.g.
   "class:demo", matching this codebase's universal "kind:name"
   convention) — fixed to parse from the edges instead of a flat split.
2. `626539c` — `apps/desktop/src-tauri/src/sd19_corpus.rs` (NEW): loads a
   small bundled corpus-fixture set (2 spells, 2 equipment items) as a
   Tauri resource; wired `compute_pilot_with_corpus` into
   `create_character`/`load_saved_character`; replaced the Character
   Sheet's "Spells"/"Gear" "coming soon" placeholders with real rendering
   of `corpus_derived` data. Added 2 hardcoded demo spell selections
   (Alarm/Abjuration, Blur/Illusion) to the fixed test loadout (only Human
   Fighter L1-3 reaches `Computed` today, so this is a
   reachability-demonstration sample, not a class-appropriateness claim).
   Also fixed two pre-existing breakages discovered along the way: the
   desktop app (a separate Cargo crate) didn't compile at all against
   `tranche/3` HEAD, independent of this work — SD-19's 15 earlier
   commits were never checked against `apps/desktop`.
3. `d5f6531` — promoted `school.abjuration.spell_reachability` and
   `school.illusion.spell_reachability` fully to `Supported`/`ProductVisible`
   (both schools' full corpus coverage was already proven by their
   automated tests; the UI just needed to satisfy condition 1, "the
   operator's UI surfaces it"). `equipment.arms_armor.equipment_reachability`
   promoted to `evidence_tier: ProductVisible` only — `support_state`
   stays `Partial`, since the row's own named sample (Longsword, Banded
   Mail, Armor Spikes) isn't fully UI-grounded (only Longsword + the
   newly-added Chain Shirt are shown live; Banded Mail/Armor Spikes remain
   automated-test-only). Updated 25 test files' closed-world "no row is
   Supported" invariant checks to allowlist these two rows — the first
   two rows ever legitimately promoted to `Supported` in this matrix's
   history.

Live verification: created a Human Fighter character in the actual
running desktop app (Tauri + Xvfb via the `run-desktop` skill), confirmed
the Spells tab renders "Abjuration — Alarm ✓ grounded" / "Illusion — Blur
✓ grounded" and the Gear tab renders "Longsword ✓ grounded" / "Chain
Shirt", both sourced from the real IPC response, not mock data.

**Remaining for full `Supported/Product-visible` closure on all 15
criteria:** the arms_armor row (widen the UI-visible sample or narrow its
named-sample claim), and the other 8 spell schools + 3 equipment
categories (all still `Partial/Computed`, not yet UI-surfaced at all).
Not attempted this round — the operator asked specifically for "the
UI-surfacing work" as a bounded follow-up to the loop's closure, not
exhaustive UI coverage of every row.

## Open blockers

### cycle-2026-07-16T[loop-resume] | BLOCKER: SD-17 parser-merge fix required before §2.5 full coverage can run

**Condition:** The loop instruction file was amended (operator directive
2026-07-16, "i want to make sure that we brought in ALL spells, ALL
armor, ALL weapons, ALL equipment, not just samples") to require full
per-item coverage for all 4 §2.5 equipment categories, symmetric with
§2.4's already-full spell coverage. Per the amended file's own §"What
this loop fires against" step 1: "Check the parser-merge defect status
first... If no [fix landed], the cycle writes to Open Blockers... and
exits FAIL. The loop does NOT spawn §2.5 cycles against a known-blocked
criterion."

**Verification performed this cycle:**

- `gh pr view 317 --json state,mergedAt` → `{"state":"OPEN","mergedAt":null}`.
  The SD-17 parser-merge fix (`open_record`'s merge-by-`KEY:` correction
  in `src/pcgen_import/lst_parser/equipment.rs`, discovered and fixed in
  the same session that did SD-19's UI-surfacing work) exists as a real,
  tested fix — but only on its own branch (`fix/sd17-equipment-copy-merge`,
  based on `develop`), not merged anywhere.
- `git log origin/tranche/3 --oneline -- src/pcgen_import/lst_parser/equipment.rs`
  → only `9adf6f2` (the original SD-17-B-5 parser commit, #296). No fix
  commit present on `tranche/3`.
- `git status --porcelain` on `tranche/3` → 0 (clean); in-flight check
  (`ps -eo pid,etime,stat,cmd | grep claude`) → only this session's own
  process, no competing cycle.

**Why not self-heal:** Per the amended file's own §"What this loop fires
against" and the pre-existing "corpus-side work is SD-17's lane" row in
the Self-healing posture table: the parser-merge defect is explicitly
SD-17-lane work, not SD-19's to fix from inside a loop cycle. The fix
already exists and is verified (`cargo test` green on the fix's own
branch per its PR description) — what's missing is purely an operator
merge decision, not more engineering.

**Resolution required:** Operator merges PR #317 (or otherwise lands its
fix onto `tranche/3`). Once `git log origin/tranche/3 -- \
src/pcgen_import/lst_parser/equipment.rs` shows the fix commit, the next
cycle proceeds to re-run all 4 §2.5 cycles at full per-item coverage per
the amended criterion. Until then, every §2.5-full-coverage cycle
attempt will re-hit this identical blocker — no further cycle should be
spawned against it without checking this first.

**RESOLVED 2026-07-16T[loop-resume+1]:** Operator directed "use the
tranche 3 branch" — cherry-picked the fix commit (`00f5802` on
`fix/sd17-equipment-copy-merge`) directly onto `tranche/3` rather than
merging PR #317 into `develop` first (PR #317 remains open, unmerged,
tracking the `develop`-side fix separately). Landed as `22eeed9` on
`tranche/3`. `cargo test --locked` 3422/3422 green (0 regressions);
`cargo clippy --locked --tests -- -D warnings` clean. `git log
origin/tranche/3 -- src/pcgen_import/lst_parser/equipment.rs` now shows
`22eeed9` at the tip. The blocker is cleared — §2.5 full-coverage cycles
are now eligible to proceed.

### cycle-log

cycle: 2026-07-16T[loop-resume]
criterion touched: none (pre-cycle blocker check per the amended loop file's own explicit step 1)
row_or_kind: n/a
commit: no commit: no eligible criterion — full-coverage §2.5 work is structurally blocked on an unmerged SD-17-lane fix (PR #317)
card: no card: no eligible work attempted
verify: cargo test not run; clippy not run (no code change attempted)
status: FAIL

## §2.5 full-coverage re-grounding (2026-07-16, post-blocker-clear)

Per the amended loop instruction's "Coverage gap closure" directive, all
4 §2.5 equipment categories re-ground at **full per-item coverage**
(every real corpus record, not a 3-item sample) with **full
`TableCellRef` grounding** (operator confirmed: expand the table store to
real entries for all ~2,977 objects, not reachability-only). Three
commits:

### cycle-2026-07-16T[full-coverage-1] | table store expansion | commit `5fef69c` | card t_5d17fcda | cargo test 3420/3420 green (CORPUS_ROOT set) | clippy clean

Expanded `rules_tables::crb::equipment_tables` from 4 bootstrap entries
to one real entry per corpus record across all 4 categories (arms_armor
310, general 453, magic_items 1556, equipmods 658 — **2977 total**),
split into `equipment_data/<category>.rs` files, generated
programmatically from the real PCGen corpus (not hand-authored).
`EquipmentTableEntry.cost_gp` widened `Option<u32>` → `Option<f64>` (real
costs are frequently fractional). `EQUIPMENT_TABLES` const → `equipment_tables()`
fn (OnceLock-cached, since Rust can't concatenate 4 differently-sized
const arrays at compile time).

### cycle-2026-07-16T[full-coverage-2] | equipment resolver exact-name-match fix | commit `de88434` | card t_5d17fcda | cargo test green | clippy clean

Discovered while proving full coverage: KEY-less records distinguished
only by parenthesized content (e.g. real corpus's "Improvised Weapon
(1d2)" vs "(1d3)" vs ... "(2d10)") collided under the normalized-name
fallback's parenthetical-stripping, always resolving to whichever
sibling the linear scan hit first. Fixed by adding an exact
(unnormalized) name-match step between the KEY match and the lossy
normalized fallback.

### cycle-2026-07-16T[full-coverage-3] | all 4 categories re-grounded at full coverage | commit `513d8a6` | card t_5d17fcda | cargo test green (CORPUS_ROOT set) | clippy clean

- **arms_armor: 310/310.** commit `5fef69c`+`513d8a6`. All real-corpus
  records resolve, reach `equipped_items`, ground through `TableCellRef`.
- **general: 453/453.**
- **magic_items: 1556/1556**, including every `.COPY=` variant the prior
  cycle deliberately avoided pending the parser fix.
- **equipmods: 344/658 raw records are independently addressable; all
  344 resolve at full coverage.** The other 314 raw records are
  PCGen-internal hidden legacy alias rows (no `KEY:` of their own,
  `VISIBLE:NO`, fallback name-identity colliding with a real modifier's
  own `KEY:` — e.g. `Material ~ Steel.COPY=STEEL` colliding with the real
  `Material ~ Steel` modifier). Confirmed for all 314 with zero
  exceptions (data-driven investigation, not a guess). These are not
  independently selectable equipment, so **344/344 is full coverage of
  every real item**, not a partial result. All 658 raw records are still
  transcribed verbatim into the table store.

**Matrix rows:** all 4 promoted from "3-item sample, Computed" to "full
category coverage, Computed" — `support_state` stays `Partial`,
`evidence_tier` stays `Computed`, **not** `Product-visible`. This walks
back arms_armor's prior partial `ProductVisible` promotion (from the
2026-07-16 desktop UI-surfacing work, commit `d5f6531`): per the amended
loop instruction's own §5 ("UI-surfacing gate is downstream... 
full-coverage Partial/Computed is the loop's exit condition"), the
desktop app surfaces only 2 of arms_armor's 310 items live, not the full
category, so the row's "every named sample is grounded" bar is not met
at the new full-category scope.

**§2.5 full-coverage sweep: closed (4/4).** All 15 SD-19 acceptance
criteria remain at their evidence ceiling absent further operator
direction: `Partial/Computed` (schools) or `Partial/Computed`
(equipment, now full-coverage) or `Supported/ProductVisible` (Abjuration,
Illusion — from the UI-surfacing work, unaffected by this cycle since
§2.4 was explicitly not re-run per the amended instruction's own §3).

**Remaining for full `Supported/Product-visible` closure:** operator UI
surfacing of each equipment category's *full* item set (not just the
2-item Longsword/Chain-Shirt demo sample already shown), and/or the
8 spell schools beyond Abjuration/Illusion. Not attempted this round —
scoped to exactly what the amended loop instruction asked for.

## §2.5 UI-surfacing closure (2026-07-16, post-full-coverage)

Operator, when asked to bound "equipment UI-surfacing for all 4
categories" between a small per-category demo vs. a full catalog
browser, explicitly chose **"Full catalog browser"** — a new, separate
UI view listing/searching all ~2,977 items across all 4 categories, not
a single character's Gear tab. Built and shipped in two commits:

### cycle-2026-07-16T[ui-surfacing-1] | equipment catalog browser | commit `c19b9be` | cargo test green (root 3420+/3420+, desktop backend 69/69) | npm test 40/40 | typecheck/build clean

- New Tauri command `list_equipment_catalog` (`apps/desktop/src-tauri/src/sd19_equipment_catalog.rs`)
  returning every entry from `rules_tables::crb::equipment_tables()` —
  proven at exactly 2977 total, 310/453/1556/658 per category via a
  backend unit test.
- New desktop screen `apps/desktop/src/equipmentCatalog/EquipmentCatalogScreen.tsx`:
  category filter chips (All + 4 categories with live counts), a
  name-substring search box, a 200-row render cap with an explicit
  "Showing first N of X matching items" message (no silent truncation).
- Wired into hub navigation: `LandingScreen.tsx` gained a "Browse
  Equipment Catalog" link; `CharacterHubPage.tsx` gained an
  `equipmentCatalog` mode.
- Boundary/runtime split follows the existing pattern:
  `boundary/loadEquipmentCatalog.ts` (raw invoke wrapper) →
  `equipmentCatalog/equipmentCatalogRuntime.ts` (adds a 5-item
  preview-mode fallback for browser-only dev via `hasTauriRuntime()`).

### cycle-2026-07-16T[ui-surfacing-2] | promote all 4 equipment rows to Supported/ProductVisible | commit `e9845f2` | cargo test green (root + desktop) | npm test/typecheck/build clean | live-verified via run-desktop

All 4 `equipment.*.equipment_reachability` rows promoted
`Partial/Computed` → `Supported/ProductVisible`: every real corpus item
is both grounded (proven by the full-coverage cycle above) and now
surfaced live in an operator-visible UI, satisfying the loop
instruction's own definition of Supported/Product-visible. This
supersedes the earlier full-coverage cycle's walk-back note (arms_armor
was demoted from a premature partial `ProductVisible` because the
desktop app only showed a 2-item demo sample at that time — the full
catalog browser now closes that gap for all 4 categories at once, not
just arms_armor).

Updated the 4 per-category proof tests
(`tests/sd19_equipment_{arms_armor,general,magic_items,equipmods}.rs`)
to assert `Supported`/`ProductVisible` instead of `Partial`/`Computed`.
Extended all 24 closed-world "no row is unexpectedly Supported" tests
plus the master `tests/sd13_support_state_matrix.rs` (which also carries
the canonical 34-row `EXPECTED_ROW_IDS` list and row/count assertions)
to allowlist the 4 new equipment row ids alongside the existing 2
school rows (Abjuration, Illusion).

**Live verification (run-desktop skill):** launched the Tauri app under
Xvfb, navigated Landing → "Browse Equipment Catalog", confirmed the
"All (2977)" tab and all 4 per-category tabs show the correct counts
(310/453/1556/658), confirmed name search narrows correctly (e.g.
"sword" → 26 matches with real names/costs), confirmed category
filtering isolates a single category's real corpus-derived rows (e.g.
"Magic Items (1556)" showing "Amulet of Mighty Fists +1" at 4000 gp).

**§2.5 equipment-category sweep: closed at full coverage AND full
UI-surfacing (4/4).** Combined with the pre-existing Abjuration/Illusion
school rows, 6 of 15 SD-19 acceptance criteria are now
Supported/Product-visible; the remaining 7 spell schools (Conjuration,
Divination, Enchantment, Evocation, Necromancy, Transmutation,
Universal) stay `Partial/Computed` — reachability-grounded from §2.4
but not yet UI-surfaced. Not attempted this round — scoped to exactly
what the operator asked for (equipment UI-surfacing).

## Full-matrix closure: Spell Catalog Browser build + row promotion (2026-07-16)

Per the loop instruction's `## Full-matrix closure` section (operator
directive 2026-07-16 mandate expansion: reach Supported/Product-visible
on every seeded matrix row except the one named non-Human interaction
exception), this cycle's priority-1 item was the **Spell Catalog
Browser** build, then promoting all 9 `school.*.spell_reachability`
rows — mirroring the Equipment Catalog Browser precedent exactly. Three
commits:

### cycle-2026-07-16T1930-1 | spell list full-coverage expansion | commit `3c731b6` | cargo test green (root, CORPUS_ROOT set) | clippy clean

Discovered while starting the browser build: unlike
`equipment_tables()` (expanded to full corpus coverage in the prior
loop-closure round), `rules_tables::crb::spell_list::SPELL_LIST` was
still at its original 10-entry bootstrap (one representative spell per
school) — the §2.4 "9/9 schools full coverage" claim was true for
*reachability* (grounded live against the corpus via `spell_id_resolve`
at test time) but not for this static table, which the browser needs to
list every spell without re-parsing the corpus at runtime. Expanded
`SPELL_LIST` to all 652 real `cr_spells.lst` records, generated
programmatically from the live corpus (not hand-authored). Verified the
per-school counts against a direct corpus parse before writing the
generator: Abjuration 73, Conjuration 116, Divination 50, Enchantment
60, Evocation 87, Illusion 47, Necromancy 62, Transmutation 152,
Universal 5 (652 total), matching the scope doc's own citation exactly.
Kept `SPELL_LIST` as a single `pub const` (not an `equipment_tables()`
style `OnceLock` fn) since 652 entries fit in one array literal with no
cross-file concatenation needed — `spell_resolver.rs` and
`tests/sd19_table_store_foundation.rs` kept working unchanged.

### cycle-2026-07-16T1930-2 | spell catalog browser | commit `d6e0e4c` | cargo test green (root, desktop backend 71/71) | npm test 40/40 | typecheck/build clean

- New Tauri command `list_spell_catalog` (`apps/desktop/src-tauri/src/sd19_spell_catalog.rs`)
  returning every entry from `rules_tables::crb::spell_list::SPELL_LIST`
  — proven at exactly 652 total, correct per-school breakdown, via a
  backend unit test.
- New desktop screen `apps/desktop/src/spellCatalog/SpellCatalogScreen.tsx`:
  school filter chips (All + 9 schools with live counts), a
  name-substring search box, a 200-row render cap with an explicit
  "Showing first N of X matching spells" message (no silent truncation).
- Wired into hub navigation: `LandingScreen.tsx` gained a "Browse Spell
  Catalog" link alongside the existing equipment one; `CharacterHubPage.tsx`
  gained a `spellCatalog` mode.
- Boundary/runtime split follows the existing pattern:
  `boundary/loadSpellCatalog.ts` (raw invoke wrapper) →
  `spellCatalog/spellCatalogRuntime.ts` (adds a 5-item preview-mode
  fallback for browser-only dev via `hasTauriRuntime()`).

**Live verification (run-desktop skill):** launched the Tauri app under
Xvfb, navigated Landing → "Browse Spell Catalog", confirmed the "All
(652)" tab and all 9 per-school tabs show the correct counts
(73/116/50/60/87/47/62/152/5), confirmed school filtering isolates a
single school's real corpus-derived spells (clicked "Illusion (47)" →
"47 matching spells", entries Blur/Color Spray/Disguise Self/
Displacement/... all tagged Illusion with real descriptions and
levels), confirmed name search narrows correctly (typed "fireball" →
"2 matching spells": Delayed Blast Fireball (Level 7) and Fireball
(Level 3), both real Evocation records with real descriptions).

### cycle-2026-07-16T1930-3 | promote all 9 spell school rows to Supported/ProductVisible | commit `12593cc` | cargo test green (root + desktop) | npm test/typecheck/build clean | live-verified via run-desktop

All 9 `school.*.spell_reachability` rows now `Supported`/`ProductVisible`
(the 7 remaining rows — Conjuration, Divination, Enchantment, Evocation,
Necromancy, Transmutation, Universal — promoted from `Partial/Computed`;
Abjuration and Illusion were already at this bar from the prior
loop-closure round). Every real corpus spell is both grounded (§2.4's
pre-existing full reachability work) and now surfaced live in an
operator-visible UI, satisfying the loop instruction's own definition of
Supported/Product-visible.

Updated the 7 per-school proof tests
(`tests/sd19_school_{conjuration,divination,enchantment,evocation,necromancy,transmutation,universal}.rs`)
to assert `Supported`/`ProductVisible` instead of `Partial`/`Computed`.
Extended the 24 closed-world "no row is unexpectedly Supported" tests
plus the master `tests/sd13_support_state_matrix.rs` to allowlist the 7
newly promoted row ids alongside the 2 existing school rows and 4
equipment rows (25 files total, matching the file-touch count of the
prior equipment-promotion cycle).

**Full-matrix closure status: 13 of 34 rows now Supported/ProductVisible**
(9 spell schools + 4 equipment categories). Per `## Full-matrix closure`'s
priority order, the next frontier is the **Class Progression Browser**
(12 `class.*` rows), then the **Race Trait Browser** (7 `race.*` rows),
then the **Human interaction-row judgment call** (1 row), leaving the
permanently-excluded non-Human interaction row untouched throughout.

## Full-matrix closure: Class Progression Browser build (2026-07-16, cycle 2026-07-16T2358)

Per `## Full-matrix closure`'s priority order, this cycle's target was the
**Class Progression Browser** build (Tauri command + DTOs + React screen +
nav wiring), mirroring the Equipment/Spell Catalog Browser precedent.

### cycle-2026-07-16T2358 | class progression catalog browser | commit `9313e30` | card t_d6d8935b | cargo test green (root + desktop backend 73/73) | npm test 40/40 | typecheck/build clean | live-verified via run-desktop

- New Tauri command `list_class_catalog`
  (`apps/desktop/src-tauri/src/sd19_class_catalog.rs`) returning every
  entry from `rules_tables::crb::class_tables::class_tables()` — proven
  at exactly 207 rows across all 11 CRB classes (20/20/20/15/20/12/20/20/
  20/20/20 for Barbarian/Bard/Cleric/Druid/Fighter/Monk/Paladin/Ranger/
  Rogue/Sorcerer/Wizard) via a backend unit test.
- New desktop screen `apps/desktop/src/classCatalog/ClassCatalogScreen.tsx`:
  class filter chips (All + 11 classes with live counts), a class-name
  search box, a 200-row render cap with an explicit "showing first N of
  X" message (no silent truncation).
- Wired into hub navigation: `LandingScreen.tsx` gained a "Browse Class
  Progression" link; `CharacterHubPage.tsx` gained a `classCatalog` mode.
- Boundary/runtime split follows the existing pattern:
  `boundary/loadClassCatalog.ts` (raw invoke wrapper) →
  `classCatalog/classCatalogRuntime.ts` (adds a preview-mode fallback for
  browser-only dev via `hasTauriRuntime()`).

**Live verification (run-desktop skill):** launched the Tauri app under
Xvfb, navigated Landing → "Browse Class Progression", confirmed the "All
(207)" tab and all 11 per-class tabs show the correct counts (Barbarian
20, Bard 20, Cleric 20, Druid 15, Fighter 20, Monk 12, Paladin 20, Ranger
20, Rogue 20, Sorcerer 20, Wizard 20), confirmed class filtering isolates
a single class's real corpus-derived rows (clicked "Wizard (20)" →
20 matching rows, half-BAB/good-Will progression e.g. level 9 BAB +4,
Fort +3/Ref +3/Will +6), confirmed name search narrows correctly (typed
"rogue" → 20 matching rows, three-quarter-BAB/good-Reflex progression
e.g. level 9 BAB +6, Fort +3/Ref +6/Will +3).

**Clippy note:** `cargo clippy --locked --tests -- -D warnings` inside
`apps/desktop/src-tauri` fails on 6 pre-existing, unrelated findings in
`src/update/transaction.rs` (a `large_enum_variant`, four
`std::io::Error::other` suggestions, one `derivable_impls`). Verified via
`git stash` that these are present on `origin/tranche/3 @ 12593cc` before
this cycle's changes — not introduced by this cycle. The root-workspace
`cargo clippy --locked --tests -- -D warnings` (run from the repo root)
is clean, and `sd19_class_catalog.rs` itself has zero clippy findings
when filtered out of the desktop-crate run. Not this cycle's file-touch
scope to fix; noted here for the record, not filed as a new Open Blocker
since it doesn't block this cycle's own work.

**Row promotion deferred — genuinely too large for this cycle.** Unlike
the equipment/spell precedent (25 closed-world "no unexpectedly Supported
row" test files, each a simple string-exclusion-list addition, plus a
handful of dedicated per-domain proof test files), the 12 `class.*` row
ids are referenced across **251 existing SD-13/SD-18 test files**
(`grep -rl` count) with hard-coded, individually-worded
`assert_eq!(row.support_state, SupportState::Partial)` /
`assert_eq!(row.evidence_tier, EvidenceTier::Computed)` sibling-
preservation checks narrating specific slice history (e.g. "Fighter rows
stay Partial/Computed" / "row must stay Partial after the barbarian
slice" / "paladin row must keep its later-accepted Partial posture").
Promoting all 12 rows to Supported/ProductVisible in the same commit as
the browser build would require locating and editing each of these
per-file, per-row assertions correctly — not a mechanical bulk
find/replace, since some of the same files' narration is specific to
*why* the row was Partial at that point in slice history and would need
rewording, not just a value swap. This is a materially different scale
than the "one cycle may promote every row in the browser's domain at
once" precedent the loop instruction describes (Step 2), which was sized
against the equipment/spell case. Per Step 2's own escape valve ("unless
it turns out to be genuinely too large for one cycle"), this cycle builds
and live-verifies the browser only; the 12-row promotion is left as a
dedicated follow-up cycle's work-unit. The browser itself is fully
functional and ready to serve as the UI-surfacing evidence once the
promotion cycle lands.

**Full-matrix closure status: still 13 of 34 rows Supported/ProductVisible**
(unchanged this cycle — browser build only). Per `## Full-matrix
closure`'s priority order, the next frontier is either (a) the class row
promotion follow-up (12 rows, ~251-file test surface), or (b) starting
the **Race Trait Browser** (7 `race.*` rows) ahead of the class
promotion, at the next cycle's discretion — both are legitimate next
steps; whichever cycle picks this up should re-check this doc's cycle log
for what, if anything, has landed since.

### cycle-log

cycle: 2026-07-16T2358
criterion touched: Class Progression Browser build (Full-matrix closure UI-surfacing precondition for 12 class.* rows)
row_or_kind: n/a (browser build; no matrix row transitions this cycle)
commit: 9313e30
card: t_d6d8935b
verify: cargo test root + desktop backend 73/73 green; npm test 40/40; typecheck/build clean; clippy clean (root workspace; pre-existing unrelated desktop-crate debt confirmed not introduced by this cycle)
status: GREEN

## Full-matrix closure: Race Trait Catalog Browser build + row promotion (2026-07-16, cycle 2026-07-16T2359-race)

Re-derived eligibility live per Step 1 rather than trusting the prior
cycle's summary: `git fetch origin tranche/3` + `git log --oneline -8`
confirmed HEAD at `9313e30` (Class Progression Browser build, no
promotion), `git status --porcelain` clean, working tree on `tranche/3`.
In-flight check (`ps -eo pid,ppid,etime,stat,cmd | grep claude`) showed
this session's own top-level process only — no competing cycle. Per
`## Full-matrix closure`'s priority order, item 4 (Class Progression
Browser, build done / promotion deferred as genuinely too large — see
the prior cycle's own note) and item 5 (Race Trait Browser, fully open)
were both legitimate next steps per the prior cycle's own handoff.
Chose the Race Trait Browser: a clean, well-scoped, three-times-proven
pattern (equipment/spell/class), lower collision risk than reopening
the still-large class-row-promotion problem under time pressure.

### cycle-2026-07-16T2359-race-1 | race trait catalog browser | commit `e0aec74` | cargo test root 3420/3420 green (2 new) | desktop backend 75/75 green | npm test 40/40 | typecheck/build clean

- New static table `src/rules_core/rules_tables/crb/race_tables.rs`:
  `RaceId` enum (7 variants) + `RaceTraitEntry` (race_id, trait_name,
  value, detail), 49 entries total transcribed verbatim from the
  already-grounded per-race explanation seams in `pilot_compute.rs`
  (`explain_human_pilot_race_seam` + `explain_human_trait_bundle`,
  `explain_dwarf_race_seam`, `explain_elf_race_seam`,
  `explain_gnome_race_seam`, `explain_half_elf_race_seam`,
  `explain_half_orc_race_seam`, `explain_halfling_race_seam`) — counted
  directly from each function's `explanations.push` call count before
  writing the table, not assumed from docstrings (which had drifted
  stale for Halfling: docstring said "six" dimensions, actual code
  grounds eight, including Fearless and Halfling Luck added by a later
  SD18 cycle). Per-race counts: Human 6, Dwarf 9, Elf 7, Gnome 8,
  Half-Elf 6, Half-Orc 5, Halfling 8 (49 total).
- New Tauri command `list_race_catalog`
  (`apps/desktop/src-tauri/src/sd19_race_catalog.rs`) returning every
  entry from `rules_tables::crb::race_tables::race_traits()` — proven
  at exactly 49 rows with the per-race breakdown above via a backend
  unit test.
- New desktop screen `apps/desktop/src/raceCatalog/RaceCatalogScreen.tsx`:
  race filter chips (All + 7 races with live counts, Half-Elf/Half-Orc
  labeled with the hyphen), a trait/race-name search box, a 200-row
  render cap with an explicit "showing first N of X" message (no
  silent truncation).
- Wired into hub navigation: `LandingScreen.tsx` gained a "Browse Race
  Traits" link; `CharacterHubPage.tsx` gained a `raceCatalog` mode.
- Boundary/runtime split follows the existing pattern:
  `boundary/loadRaceCatalog.ts` (raw invoke wrapper) →
  `raceCatalog/raceCatalogRuntime.ts` (adds a 5-item preview-mode
  fallback for browser-only dev via `hasTauriRuntime()`).

**Live verification (run-desktop skill):** launched the Tauri app under
Xvfb, navigated Landing → "Browse Race Traits", confirmed the "All (49)"
tab and all 7 per-race tabs show the correct counts (Human 6, Dwarf 9,
Elf 7, Gnome 8, Half-Elf 6, Half-Orc 5, Halfling 8), confirmed race
filtering isolates a single race's real trait rows (clicked "Dwarf (9)"
→ 9 matching rows: Ability Modifiers, Defensive Training, Greed, Hardy,
Senses, Size, Speed, Stability, Stonecunning, each with real
corpus-cited detail text), confirmed name search narrows correctly
(typed "keen" → 4 matching rows: Keen Senses for Elf, Gnome, Half-Elf,
and Halfling, each with the correct +2 Perception detail text).
Screenshots captured (landing link, full browser, Dwarf filter, "keen"
search).

### cycle-2026-07-16T2359-race-2 | promote all 7 race matrix rows to Supported/ProductVisible | commit `2b6748d` | card t_07db19b0 | cargo test root 3422/3422 green | desktop backend 75/75 green | npm test/typecheck/build clean

All 7 `race.*.*_semantics` rows promoted `Partial/Computed` →
`Supported/ProductVisible`: every named trait dimension in each row's
own `blocker_or_lossiness_note` was already grounded (SD-13/SD-18
compute-grounding, unchanged this cycle per Step 1 eligibility rule 4),
and is now surfaced live in the Race Trait Catalog browser, satisfying
the loop instruction's own definition of Supported/Product-visible.

**Test-surface scale (smaller than the deferred Class Progression
case, tractable in one cycle):** unlike the 12 class rows (referenced
across ~253 files, deferred by the prior cycle), the 7 race rows are
referenced across only **32 files** — 13 SD13 per-race
recognition/baseline tests (`sd13_*_race_semantics_recognition.rs`,
`sd13_race_*_bounded_semantics.rs`, `sd13_*_bounded_race_semantics.rs`,
plus the master `sd13_support_state_matrix.rs`) and 19 SD18 per-family
widening tests (one per named trait landed incrementally: 5 Dwarf, 3
Elf, 4 Gnome, 2 Half-Elf, 1 Half-Orc, 4 Halfling). Each SD18 file's own
`matrix_<race>_row_stays_partial_computed_and_grounding_ref_names_this_slice`
test asserted the row stayed Partial/Computed after that specific
slice — historically true for that slice alone, but superseded by this
cycle's separate, later promotion. Updated each file's assertions to
Supported/ProductVisible, added a note explaining the two-step history
(honest Partial→Partial widening at the time, later promoted by the
browser), and removed each file's now-obsolete
`!matches!(support_state, Supported | Lossy)` guard (replaced where the
guard was the test's sole assertion with a narrower "never Lossy"
guard, mirroring the Elf/Dwarf `sd13_*_bounded_race_semantics.rs`
files' own equivalent guard).

Also extended the 25 closed-world "no unexpected Supported row" tests
(24 sibling-preservation files plus the master
`tests/sd13_support_state_matrix.rs`, which also carries the canonical
34-row `EXPECTED_ROW_IDS` list) to allowlist the 7 new race row ids
alongside the existing 9 school + 4 equipment rows.

Fixed one additional test outside the `tests/` tree: the desktop app's
own `apps/desktop/src-tauri/src/sd13_support_state_matrix.rs` snapshot
bridge carries an independent `human_pilot_row_remains_partial_and_computed`
test that pinned the pre-promotion Human row state via its own
string-token projection (`"partial"`/`"computed"` rather than the enum
variants) — updated to `"supported"`/`"product-visible"` (verified the
exact token spelling via the file's own `evidence_tier_token` mapping
rather than guessing).

**Live verification:** same run-desktop session as the browser-build
cycle above (both landed in one sitting); the browser itself is the
UI-surfacing evidence for this promotion, per the loop instruction's
own definition (condition 1: live operator-reachable UI; condition 2:
every named trait already grounded, unchanged this cycle).

**Full-matrix closure status: 20 of 34 rows now Supported/ProductVisible**
(9 spell schools + 4 equipment categories + 7 races). Per `## Full-matrix
closure`'s priority order, the next frontier is either (a) the Class
Progression row promotion follow-up (12 rows, ~253-file test surface,
still deferred — genuinely larger than this cycle's 32-file race case),
or (b) the Human interaction-row judgment call (the last remaining
item, tackled last per the loop instruction's own priority ordering
since it needs a written decision, not a mechanical build) — both
legitimate next steps; whichever cycle picks this up should re-check
this doc's cycle log for what, if anything, has landed since. The
permanently-excluded non-Human interaction row remains untouched
throughout, as required.

### cycle-log

cycle: 2026-07-16T2359-race
criterion touched: Race Trait Browser build (2359-race-1) + promote all 7 race.* rows (2359-race-2)
row_or_kind: race:human | race:dwarf | race:elf | race:gnome | race:half-elf | race:half-orc | race:halfling
commit: e0aec74 (browser build), 2b6748d (row promotion)
card: t_07db19b0
verify: cargo test root 3422/3422 green; desktop backend 75/75 green; npm test 40/40; typecheck/build clean; clippy clean; live-verified via run-desktop
status: GREEN

## Full-matrix closure: Class Progression row promotion, Fighter subset (2026-07-16, cycle 2026-07-16T2011)

Re-derived eligibility live per Step 1: `git fetch origin tranche/3` + `git log
--oneline -5` confirmed HEAD at `2b6748d` (race-row promotion, no class-row
work), `git status --porcelain` clean, working tree on `tranche/3`. In-flight
check (`ps -eo pid,etime,stat,cmd | grep claude`) showed only this session's
own process and one unrelated bare interactive `claude` session with no
SD-19-criterion-naming prompt visible in its args — no competing claim.

Per `## Full-matrix closure`'s priority order, item 4 (Class Progression
Browser: build already done at `9313e30`; row promotion deferred by that same
cycle as genuinely too large — ~252 files reference the 12 class row ids,
versus the 32-file race case) was the next open item, since item 5 (Race
Trait Browser) closed last cycle. Rather than re-attempt all 12 rows in one
commit (already shown infeasible: `grep -rl` across the union of all 12 class
row ids hits 252 files with individually-worded, slice-history-narrating
assertions, not a mechanical bulk find/replace), this cycle applies Step 2's
own escape valve ("split by sub-step... if genuinely too large") one level
finer: split the 12-row class promotion **by class** instead of attempting
it as a single atomic work-unit. Chose Fighter first — it has two rows
(level_1_pilot, levels_2_10) but the smallest combined file footprint of any
class (28 files naming a Fighter row id directly; union with the 25
closed-world files brings the total to 39, close to the race cycle's own
32-file scope and confirmed tractable by direct `grep -rl` measurement before
starting, not assumed).

### cycle-2026-07-16T2011 | Fighter class-row promotion (class.fighter.level_1_pilot, class.fighter.levels_2_10) | commit `9024709` | card t_8b0af254 | cargo test root 3422/3422 green | desktop backend 75/75 green | npm test 40/40 | typecheck/build clean | clippy clean

Promoted both Fighter rows `Partial/Computed` -> `Supported/ProductVisible`.
Condition 2 (every named grounded milestone) was already satisfied — SD-13/
SD-18 compute grounding for Fighter levels 1-20 was complete and unchanged
this cycle. Condition 1 (live UI surface) was already satisfied too — the
Class Progression Catalog browser (`apps/desktop/src/classCatalog/
ClassCatalogScreen.tsx`, `list_class_catalog` Tauri command) shipped and was
live-verified by the prior cycle (`9313e30`), so this cycle needed no new
browser work, only the matrix-and-test promotion.

**File surface (39 files, matching the pre-cycle estimate exactly):**
- `src/rules_core/support_state_matrix.rs`: both Fighter rows' `support_state`/
  `evidence_tier` flipped to `Supported`/`ProductVisible`; `blocker_or_lossiness_note`
  extended (not rewritten) to name the Class Progression Catalog browser as
  the UI-surfacing evidence; `next_required_uplift` rewritten to name the
  still-out-of-scope compute burdens (Weapon Training damage-roll half,
  Bravery's fear/save-resolution engine, Armor/Weapon Mastery application)
  as future-SD-N scope rather than a live per-cycle target.
- 25 closed-world "no unexpected Supported row" files (24 sibling-preservation
  tests plus the master `tests/sd13_support_state_matrix.rs`): mechanically
  extended each file's exclusion-list anchor (`&& r.row_id != "race.halfling.bounded_semantics"`)
  with two new lines for the Fighter row ids, verified by scripted insertion
  + `git diff --stat` (each file showed exactly 2 insertions, 0 deletions,
  confirming no unintended edits).
- 6 cross-class sibling-preservation tests whose own loops asserted Fighter
  stays Partial after their own class's slice (Barbarian, Monk, Rogue,
  Paladin, the shared hybrid-chassis test, Ranger — Ranger's loop mixed
  Fighter with Rogue/Barbarian/Monk, so Fighter was split into its own loop
  rather than flipping the whole loop, keeping the still-Partial classes'
  assertions untouched).
- The master `tests/sd13_support_state_matrix.rs`: two direct Fighter-row
  tests (`fighter_level_1_row_is_partial_and_computed`,
  `fighter_levels_2_10_row_is_partial_and_computed_and_names_what_remains`)
  updated to assert `Supported`/`ProductVisible`, plus its own closed-world
  list (handled by the same scripted insertion as the other 24 files).
- 9 Fighter-specific proof tests (`sd13_fighter_level{2_level3,4,5,6,7,8,
  9_level10}_progression.rs`, `sd13_fighter_bravery.rs`) whose own historical
  assertions pinned the pre-promotion Partial/Computed state — including
  removing two now-false guard assertions
  (`assert_ne!(level_1.support_state, SupportState::Supported, ...)` in
  `sd13_fighter_level1_mandatory_milestone_classification.rs` and
  `assert_ne!(levels_2_10.support_state, SupportState::Supported)` in
  `sd13_fighter_level2_level3_progression.rs`) that would otherwise now fail
  honestly (the row IS Supported).
- `sd13_fighter_level1_hit_point_baseline.rs`: single direct assertion
  updated with an explanatory NOTE comment.
- 10 SD18 per-level Fighter widening tests (`sd18_fighter_level{11..20}_widening.rs`,
  `sd18_fighter_level11_armor_training3.rs`): single-pattern mechanical
  substitution (`fighter.support_state`/`fighter.evidence_tier` assertions),
  scripted and verified via `cargo test` after.

**One RED->GREEN catch, not anticipated by the pre-cycle file survey:** after
the mechanical edits, `cargo test --locked` surfaced one genuine failure —
`sd13_fighter_level1_mandatory_milestone_classification.rs`'s
`matrix_level_1_row_keeps_a_concrete_next_required_uplift` test asserted the
row's `next_required_uplift` contains the substring `"level-10"`, which the
rewritten uplift text (now naming out-of-scope future-SD-N burdens instead of
the now-closed level-10 progression widening) no longer does. Fixed by
updating the test's own assertion and adding an explanatory comment, rather
than forcing the substring back into the uplift text artificially. This is
the RED evidence for this UI-surfacing/promotion cycle (per Step 4's
UI-surfacing branch, there is no corpus-existence RED test in the
compute-grounding sense; the RED here was the genuine test failure caught by
running the full suite before committing).

**Full-matrix closure status: 21 of 34 rows now Supported/ProductVisible**
(9 spell schools + 4 equipment categories + 7 races + 2 Fighter class rows).
Per `## Full-matrix closure`'s priority order, the next frontier is the
remaining 10 class rows (Rogue, Barbarian, Bard, Cleric, Druid, Monk,
Paladin, Ranger, Sorcerer, Wizard) — recommend continuing the per-class (or
per-few-classes) split established by this cycle rather than reattempting
all 10 at once, since the file-footprint-per-class varies (Rogue: 31 files,
Monk: 18 files per the pre-cycle survey — smaller classes first is likely
the fastest path to closure) — or the Human interaction-row judgment call
(tackled last per the loop instruction's own priority ordering, since it
needs a written decision, not a mechanical build). The permanently-excluded
non-Human interaction row remains untouched throughout, as required.

### cycle-log

cycle: 2026-07-16T2011
criterion touched: Class Progression row promotion, Fighter subset (class.fighter.level_1_pilot, class.fighter.levels_2_10)
row_or_kind: class.fighter.level_1_pilot | class.fighter.levels_2_10
commit: 9024709
card: t_8b0af254
verify: cargo test root 3422/3422 green; desktop backend 75/75 green; npm test 40/40; typecheck/build clean; clippy clean (root workspace; desktop-crate pre-existing unrelated update/transaction.rs debt confirmed not introduced by this cycle, same finding as the prior cycle)
status: GREEN

## Full-matrix closure: Class Progression row promotion, Monk subset (2026-07-16, cycle 2026-07-16T2038)

Re-derived eligibility live per Step 1 rather than trusting the prior cycle's
summary: `git fetch origin tranche/3` + `git log origin/tranche/3 --oneline -8`
confirmed HEAD at `9024709` (Fighter class-row promotion, no further class-row
work), `git status --porcelain` clean, working tree on `tranche/3`. In-flight
check (`ps -eo pid,ppid,etime,stat,cmd | grep claude`) traced this shell's own
process ancestry (`ps -o pid,ppid,cmd -p $$` up through `$PPID`) directly to
pid 3591650 as this session's own top-level `claude -p` process (the exact
prompt text confirmed identical) — no second competing process running the
SD-19 loop prompt. Re-verified live against `support_state_matrix.rs` directly:
all 10 remaining class rows (Rogue, Barbarian, Bard, Cleric, Druid, Monk,
Paladin, Ranger, Sorcerer, Wizard) confirmed still `SupportState::Partial` by
grepping each row's block individually, not assumed from this doc's prose.

Per the prior cycle's own recommendation ("smaller classes first is likely the
fastest path to closure"), measured live file-footprint-per-class via
`grep -rl "class\.<name>\." tests/ apps/desktop/src-tauri/src/` for all 10
remaining classes before picking: Monk 18, Druid 20, Barbarian 26, Cleric 27,
Wizard 29, Rogue 31, Sorcerer 34, Bard 36, Paladin 39, Ranger 42. Monk is the
smallest, matching the prior cycle's own estimate exactly. Chose Monk.

### cycle-2026-07-16T2038 | Monk class-row promotion (class.monk.bounded_progression) | commit `2657c82` | card t_46470ace | cargo test root 335/335 test-binary suites green | desktop backend 75/75 green | npm test 40/40 | typecheck/build clean | clippy clean

Promoted the Monk row `Partial/Computed` -> `Supported/ProductVisible`.
Condition 2 (every named grounded milestone) was already satisfied — SD-13/
SD-18 compute grounding for Monk levels 1-12 was complete and unchanged this
cycle. Condition 1 (live UI surface) was already satisfied too — the Class
Progression Catalog browser (`apps/desktop/src/classCatalog/ClassCatalogScreen.tsx`,
`list_class_catalog` Tauri command) shipped and was live-verified by an
earlier cycle (`9313e30`), so this cycle needed no new browser work, only the
matrix-and-test promotion.

**File surface (39 files, 2 more than the pre-cycle 37-file estimate — see the
self-heal note below for why):**
- `src/rules_core/support_state_matrix.rs`: the Monk row's `support_state`/
  `evidence_tier` flipped to `Supported`/`ProductVisible`; `blocker_or_lossiness_note`
  extended (not rewritten) to name the Class Progression Catalog browser as
  the UI-surfacing evidence; `next_required_uplift` rewritten to name the
  still-out-of-scope compute burdens (the bonus feat's own per-feat execution
  engine, Wholeness of Body/Abundant Step's own execution, Monk level 13+) as
  future-SD-N scope rather than a live per-cycle target.
- 25 closed-world "no unexpected Supported row" files (24 sibling-preservation
  tests plus the master `tests/sd13_support_state_matrix.rs`): mechanically
  extended each file's exclusion-list anchor with one new line for the Monk
  row id.
- 3 cross-class sibling-preservation tests whose own loops asserted Monk
  stays Partial after their own class's slice (Paladin, Ranger, Rogue) —
  Monk was split out into its own Supported check alongside Fighter,
  mirroring the Fighter cycle's own split of Ranger's shared loop.
- 12 Monk-specific proof tests (`sd13_monk_level{1..10}_*progression*.rs`
  family, `sd13_monk_second_bonus_feat.rs`, `sd13_monk_bonus_feats_three_and_four.rs`,
  `sd18_monk_level11_diamond_body.rs`, `sd18_monk_level12_widening.rs`) whose
  own historical assertions pinned the pre-promotion Partial/Computed state,
  updated to Supported/ProductVisible with an explanatory comment. No
  `assert_ne!` guards needed removal this cycle (unlike Fighter's two) — none
  existed in the Monk-specific files.

**One self-heal catch, not anticipated by the pre-cycle file survey:** after
the mechanical edits, a full `cargo test --locked` run surfaced one genuine
failure — `sd13_fighter_bravery.rs`'s `matrix_preserves_fighter_level_1_and_other_accepted_rows`
test panicked ("the Bravery slice must not promote any row to Supported or
Lossy"). Root cause: the pre-cycle survey (`grep -rl "class\.monk\."`) had
deliberately excluded files with "fighter" in the name on the assumption they
only carry Fighter-specific direct assertions, but 7 of them
(`sd13_fighter_bravery.rs` plus `sd13_fighter_level{4,5,6,7,8,9_level10}_progression.rs`)
also carry their own copy of the generic closed-world "no unexpected Supported
row" check (confirmed by re-deriving the full file set via
`grep -rl 'r.row_id != "class.fighter.levels_2_10"'` rather than the narrower
`class\.monk\.` anchor — this returns the true 25-file closed-world set,
which the initial per-domain grep does not capture for a *newly*-promoted
row). Fixed by extending the exclusion list in all 7 files with the Monk row
id, then re-ran the full suite clean. This is the RED evidence for this
UI-surfacing/promotion cycle (per Step 4's UI-surfacing branch, there is no
corpus-existence RED test in the compute-grounding sense; the RED here was
the genuine test failure caught by running the full suite before committing).
**Handoff note for the next class-row promotion cycle:** measure the
closed-world file set via the generic exclusion-list anchor
(`grep -rl 'r.row_id != "class.fighter.levels_2_10"'` or equivalent), not a
per-domain name grep, to avoid missing files whose name doesn't mention the
newly-promoted class.

**Full-matrix closure status: 22 of 34 rows now Supported/ProductVisible**
(9 spell schools + 4 equipment categories + 7 races + Fighter (2 rows) + Monk
(1 row)). Per `## Full-matrix closure`'s priority order, the next frontier is
the remaining 9 class rows (Rogue, Barbarian, Bard, Cleric, Druid, Paladin,
Ranger, Sorcerer, Wizard) — per-class file-footprint survey measured this
cycle for sequencing (smallest first): Druid 20, Barbarian 26, Cleric 27,
Wizard 29, Rogue 31, Sorcerer 34, Bard 36, Paladin 39, Ranger 42 — or the
Human interaction-row judgment call (tackled last per the loop instruction's
own priority ordering, since it needs a written decision, not a mechanical
build). The permanently-excluded non-Human interaction row remains untouched
throughout, as required.

### cycle-log

cycle: 2026-07-16T2038
criterion touched: Class Progression row promotion, Monk subset (class.monk.bounded_progression)
row_or_kind: class.monk.bounded_progression
commit: 2657c82
card: t_46470ace
verify: cargo test root 335/335 test-binary suites green (0 failures); desktop backend 75/75 green; npm test 40/40; typecheck/build clean; clippy clean
status: GREEN

## Full-matrix closure: Class Progression row promotion, Druid subset (2026-07-16, cycle 2026-07-16T2103)

Re-derived eligibility live per Step 1 rather than trusting the prior cycle's
summary: `git fetch origin tranche/3` + `git log origin/tranche/3 --oneline -8`
confirmed HEAD at `2657c82` (Monk class-row promotion, no further class-row
work), `git status --porcelain` clean, working tree on `tranche/3`. In-flight
check (`ps -eo pid,etime,stat,cmd | grep -iE 'claude' | grep -v grep`) showed
this session's own `claude -p` process plus one unrelated bare interactive
`claude` session with no arguments naming any SD-19 criterion — no competing
claim. Also read the two required SD-18 investigation-cycle sections
(cycle-2026-07-15T0300 §3.4, cycle-2026-07-15T0400 §3.5 in
`SD-18-core-rules-breadth-progress.md`, found at
`programs/codex/requirements/SD-18-core-rules-breadth/artifacts/`) as
read-only reference — both are historical records of spell-school/
equipment-category reachability being structurally blocked at that time,
later resolved by SD-19's own foundation/capability slices; no action
required, background context only.

Re-verified live against `support_state_matrix.rs` directly: the row's
`support_state` field confirmed `SupportState::Partial` by direct read before
starting. Per the prior cycle's own recommendation ("smallest first"),
measured live file-footprint via `grep -rl "class\.druid\." tests/
apps/desktop/src-tauri/src/` (20 files) unioned with the 25-file closed-world
set (via `grep -rl 'r.row_id != "class.monk.bounded_progression"'`, 5-file
overlap, 40 total) — matching the prior cycle's own 20-file estimate for
Druid exactly. Druid is the smallest of the remaining 9 classes, so chosen
next per the established sequencing.

### cycle-2026-07-16T2103 | Druid class-row promotion (class.druid.progression_and_spell_burden) | commit `dc7b3b5` | card t_d68b306b | cargo test root 3422/3422 green | desktop backend 75/75 green | clippy clean

Promoted the Druid row `Partial/Computed` -> `Supported/ProductVisible`.
Condition 2 (every named grounded milestone) was already satisfied — SD-13/
SD-18 compute grounding for Druid levels 1-15 (Wild Empathy, Nature Sense,
the nature-bond choice recognition, base attack/save progression, Woodland
Stride, Trackless Step, Resist Nature's Lure, A Thousand Faces, Timeless
Body) was complete and unchanged this cycle. Condition 1 (live UI surface)
was already satisfied too — the Class Progression Catalog browser
(`apps/desktop/src/classCatalog/ClassCatalogScreen.tsx`, `list_class_catalog`
Tauri command) shipped and was live-verified by an earlier cycle (`9313e30`),
so this cycle needed no new browser work, only the matrix-and-test
promotion.

**File surface (41 files, matching the pre-cycle 40-file estimate plus 1 —
see below for why):**
- `src/rules_core/support_state_matrix.rs`: the Druid row's `support_state`/
  `evidence_tier` flipped to `Supported`/`ProductVisible`; `blocker_or_lossiness_note`
  extended (not rewritten) to name the Class Progression Catalog browser as
  the UI-surfacing evidence; `next_required_uplift` rewritten to name the
  still-out-of-scope compute burdens (the animal-companion execution burden,
  the Wild Shape execution burden, the prepared divine spell posture burden,
  and Druid level 16+) as future-SD-N scope rather than a live per-cycle
  target.
- 25 closed-world "no unexpected Supported row" files (24 sibling-preservation
  tests plus the master `tests/sd13_support_state_matrix.rs`): mechanically
  extended each file's exclusion-list anchor (inserted right after the
  existing `&& r.row_id != "class.monk.bounded_progression"` line, scripted
  across all 25 in one pass) with one new line for the Druid row id.
- 15 Druid-specific proof tests (`sd13_druid_base_attack_and_saves.rs`,
  `sd13_druid_level{1..10}_progression.rs`/`_spell_baseline.rs`,
  `sd18_druid_level{11..15}_widening.rs`) whose own historical assertions
  pinned the pre-promotion Partial/Computed state, updated to
  Supported/ProductVisible with an explanatory comment. `sd13_druid_level1_spell_baseline.rs`
  additionally carried its own direct `assert_ne!(druid.support_state,
  SupportState::Supported)` guard, removed as now-false (mirroring the
  Fighter cycle's own guard removals).
- 4 cross-class sibling-preservation tests whose own loops asserted Druid
  stays Partial after their own class's slice, each split the same way the
  Fighter/Monk cycles split those classes out: `sd13_monk_level1_chassis_baseline.rs`
  (Druid moved from the "Sorcerer, Bard, Cleric, Druid" Partial loop into
  the existing Fighter Supported loop, now "Fighter and Druid"),
  `sd13_paladin_level1_chassis_and_spell_burden_separation.rs` (Druid moved
  from the "Cleric, Druid" Partial loop, leaving Cleric alone, into the
  existing Monk Supported block, now a "Monk and Druid" loop),
  `sd13_ranger_level1_chassis_and_class_feature_separation.rs` (Druid moved
  from the "Bard, Cleric, Druid, Sorcerer" Partial loop into the existing
  "Fighter and Monk" Supported loop, now "Fighter, Monk, and Druid").
  `sd13_rogue_level1_chassis_baseline.rs` only referenced Druid in its
  closed-world exclusion list (no separate "stays Partial" loop naming
  Druid), so it needed no further change beyond the mechanical closed-world
  edit already covered above.

**One self-heal catch, not anticipated by the pre-cycle file survey:** after
removing Druid from the Paladin file's "Cleric, Druid" loop (leaving a
single-element `for id in ["class.cleric.progression_and_spell_burden"]`
loop), `cargo clippy --locked --tests -- -D warnings` failed with
`clippy::single_element_loop`. Fixed by rewriting the loop as a plain
non-loop assertion (`let cleric = matrix.row(...); assert_eq!(...)`) rather
than suppressing the lint or leaving a spurious loop — the extra file this
added over the 40-file pre-cycle estimate. Re-ran `cargo test --locked`
(3422/3422 green) and `cargo clippy --locked --tests -- -D warnings` (clean)
after the fix, confirming no other regressions. This is the RED evidence for
this UI-surfacing/promotion cycle (per Step 4's UI-surfacing branch, there is
no corpus-existence RED test in the compute-grounding sense; the RED here was
the clippy failure caught by running the full lint pass before committing).

**Live verification:** not repeated this cycle — the Class Progression
Catalog browser's live-verification evidence (screenshot proof of the full
class list, a class filter, and search against real corpus-derived data) was
already captured by the browser-build cycle (`9313e30`) and is unchanged;
per the loop instruction's own Step 5 guidance ("at least once per new
browser, not necessarily every cycle after that"), re-verifying per
individual row promotion is not required.

**Full-matrix closure status: 23 of 34 rows now Supported/ProductVisible**
(9 spell schools + 4 equipment categories + 7 races + Fighter (2 rows) +
Monk (1 row) + Druid (1 row)). Per `## Full-matrix closure`'s priority order,
the next frontier is the remaining 8 class rows (Rogue, Barbarian, Bard,
Cleric, Paladin, Ranger, Sorcerer, Wizard) — per-class file-footprint survey
from the prior cycle for sequencing (smallest first): Barbarian 26, Cleric
27, Wizard 29, Rogue 31, Sorcerer 34, Bard 36, Paladin 39, Ranger 42 (these
counts predate this cycle's edits and should be re-measured live by the next
cycle rather than assumed unchanged) — or the Human interaction-row judgment
call (tackled last per the loop instruction's own priority ordering, since it
needs a written decision, not a mechanical build). The permanently-excluded
non-Human interaction row remains untouched throughout, as required.

### cycle-log

cycle: 2026-07-16T2103
criterion touched: Class Progression row promotion, Druid subset (class.druid.progression_and_spell_burden)
row_or_kind: class.druid.progression_and_spell_burden
commit: dc7b3b5
card: t_d68b306b
verify: cargo test root 3422/3422 green; desktop backend 75/75 green; clippy clean (npm test/typecheck/build not re-run — matrix-and-test-only promotion cycle, no browser/frontend code touched, mirroring the Monk cycle's own scope)
status: GREEN

## Full-matrix closure: Class Progression row promotion, Barbarian subset (2026-07-16/17, cycle 2026-07-17T0044)

Re-derived eligibility live per Step 1 rather than trusting the prior cycle's
summary: `git fetch origin tranche/3` + `git log origin/tranche/3 --oneline -8`
confirmed HEAD at `dc7b3b5` (Druid class-row promotion, no further class-row
work), `git status --porcelain` clean, working tree on `tranche/3`. In-flight
check (`ps -eo pid,ppid,etime,stat,cmd | grep -iE 'claude' | grep -v grep`
plus tracing this shell's own ancestry via `ps -o pid,ppid,cmd -p $$` up
through `$PPID`) traced this session's own top-level `claude -p` process
(pid 3667637, exact prompt text confirmed identical, parent
`sd19-loop-supervisor.sh` pid 3387203) directly — no second competing
process running the SD-19 loop prompt. Re-verified live against
`support_state_matrix.rs` directly (`grep -n "row_id:"` for all `class.*`
rows): all 7 non-Fighter/Monk/Druid class rows (Rogue, Barbarian, Bard,
Cleric, Paladin, Ranger, Sorcerer, Wizard — 8 total minus 1 read error,
corrected to the 8 actually-remaining rows below) confirmed `SupportState::
Partial` by direct read before starting.

Per the prior cycle's own recommendation ("smallest first"), measured live
file-footprint via `grep -rl "class\.<name>\." tests/
apps/desktop/src-tauri/src/` for all 8 remaining classes: Barbarian 26,
Cleric 27, Wizard 29, Rogue 31, Sorcerer 34, Bard 36, Paladin 39, Ranger 42
— matching the prior cycle's own estimate exactly. Barbarian is the
smallest, so chosen next per the established sequencing.

### cycle-2026-07-17T0044 | Barbarian class-row promotion (class.barbarian.bounded_progression) | commit `59aeacc` | card t_cbbbb7d5 | cargo test root 3422/3422 green | desktop backend 75/75 green | clippy clean

Promoted the Barbarian row `Partial/Computed` -> `Supported/ProductVisible`.
Condition 2 (every named grounded milestone) was already satisfied — SD-13/
SD-18 compute grounding for Barbarian levels 1-20 (base attack/save
progression, fast movement, Rage's flat numeric surface across all three
magnitude tiers, Uncanny Dodge, Trap Sense across all six tiers, Improved
Uncanny Dodge, Damage Reduction across all five tiers, ten numbered Rage
Power choice-recognition slots, Indomitable Will, Tireless Rage) was
complete and unchanged this cycle. Condition 1 (live UI surface) was
already satisfied too — the Class Progression Catalog browser
(`apps/desktop/src/classCatalog/ClassCatalogScreen.tsx`, `list_class_catalog`
Tauri command) shipped and was live-verified by an earlier cycle (`9313e30`),
so this cycle needed no new browser work, only the matrix-and-test
promotion.

**File surface (46 files, matching the pre-cycle 45-file estimate — 1 matrix
carrier + 21 Barbarian-specific proof tests + 4 cross-class
sibling-preservation splits + 25 closed-world files, minus 5 files counted
in both the Barbarian-specific grep and the closed-world grep, i.e. no
extra self-heal file was needed this cycle):**
- `src/rules_core/support_state_matrix.rs`: the Barbarian row's
  `support_state`/`evidence_tier` flipped to `Supported`/`ProductVisible`;
  `blocker_or_lossiness_note` extended (not rewritten) to name the Class
  Progression Catalog browser as the UI-surfacing evidence; `next_required_uplift`
  rewritten to name the still-out-of-scope compute burdens (rage-state
  execution, the Rage Power choice-list feature's own effects, Improved
  Uncanny Dodge's flanking-resolution engine, the Damage Reduction
  application engine, the saving-throw-resolution engine Indomitable Will
  would need, and weapon familiarity) as future-SD-N scope rather than a
  live per-cycle target.
- 25 closed-world "no unexpected Supported row" files (24 sibling-preservation
  tests plus the master `tests/sd13_support_state_matrix.rs`): found this
  time via the generic exclusion-list anchor
  (`grep -rl 'r.row_id != "class.druid.progression_and_spell_burden"'`)
  per the prior cycle's own handoff note, rather than a per-domain
  `class\.barbarian\.` grep — confirmed the same 25-file set the per-domain
  grep would have missed files for. Mechanically extended each file's
  exclusion-list anchor with one new line for the Barbarian row id.
- 4 cross-class sibling-preservation tests whose own loops asserted
  Barbarian stays Partial after their own class's slice (Monk, Paladin,
  Ranger, Rogue) — Barbarian was moved into each file's existing Supported
  loop (mirroring the Fighter/Monk/Druid cycles' own splits), and the
  now-solo "stays Partial" checks were either removed (Monk, Paladin) or
  reduced from a 2-element loop to a plain non-loop assertion to avoid
  `clippy::single_element_loop` proactively (Ranger's Rogue-only remainder
  was written as a block from the start, not a loop, learning from the
  Druid cycle's own clippy self-heal rather than repeating it).
- 21 Barbarian-specific proof tests (`sd13_barbarian_level{1..10}_*`
  family, `sd13_barbarian_rage_power_slots.rs`,
  `sd18_barbarian_level{11..20}_*`) whose own historical assertions pinned
  the pre-promotion Partial/Computed state, updated to
  Supported/ProductVisible with an explanatory comment matching the
  Druid-cycle wording convention. No `assert_ne!` guards needed removal
  this cycle (unlike Fighter's two and Druid's one) — none existed in the
  Barbarian-specific files. The level-1 file's own stale prose comment
  ("Stays Partial/Computed. The slice is bounded; we are not claiming
  Supported.") was annotated with a NOTE rather than deleted, preserving
  the historical record of what the SD13-E3 slice originally proved.

**No self-heal catch this cycle** — unlike Fighter (2 assert_ne! guard
removals), Monk (7 additional closed-world files discovered via the
generic-anchor method), and Druid (1 clippy::single_element_loop fix), this
cycle's full `cargo test --locked` (3422/3422 green) and
`cargo clippy --locked --tests -- -D warnings` (clean) both passed on the
first run after the mechanical edits — applying the Monk cycle's own
handoff note (generic exclusion-list anchor, not per-domain grep) and the
Druid cycle's own handoff note (write cross-class splits as plain blocks,
not single-element loops, when moving a class out of a 2-element "stays
Partial" loop) up front avoided both prior cycles' respective catches. This
is the RED evidence for this UI-surfacing/promotion cycle (per Step 4's
UI-surfacing branch, there is no corpus-existence RED test in the
compute-grounding sense; the full-suite and full-lint runs before
committing serve as the RED/GREEN gate).

**Live verification:** not repeated this cycle — the Class Progression
Catalog browser's live-verification evidence (screenshot proof of the full
class list, a class filter, and search against real corpus-derived data)
was already captured by the browser-build cycle (`9313e30`) and is
unchanged; per the loop instruction's own Step 5 guidance ("at least once
per new browser, not necessarily every cycle after that"), re-verifying per
individual row promotion is not required.

**Full-matrix closure status: 24 of 34 rows now Supported/ProductVisible**
(9 spell schools + 4 equipment categories + 7 races + Fighter (2 rows) +
Monk (1 row) + Druid (1 row) + Barbarian (1 row)). Per `## Full-matrix
closure`'s priority order, the next frontier is the remaining 7 class rows
(Cleric, Wizard, Rogue, Sorcerer, Bard, Paladin, Ranger) — per-class
file-footprint survey from this cycle for sequencing (smallest first):
Cleric 27, Wizard 29, Rogue 31, Sorcerer 34, Bard 36, Paladin 39, Ranger 42
(these counts predate this cycle's edits and should be re-measured live by
the next cycle rather than assumed unchanged) — or the Human interaction-row
judgment call (tackled last per the loop instruction's own priority
ordering, since it needs a written decision, not a mechanical build). The
permanently-excluded non-Human interaction row remains untouched
throughout, as required.

### cycle-log

cycle: 2026-07-17T0044
criterion touched: Class Progression row promotion, Barbarian subset (class.barbarian.bounded_progression)
row_or_kind: class.barbarian.bounded_progression
commit: 59aeacc
card: t_cbbbb7d5
verify: cargo test root 3422/3422 green; desktop backend 75/75 green; clippy clean (npm test/typecheck/build not re-run — matrix-and-test-only promotion cycle, no browser/frontend code touched, mirroring the Monk/Druid cycles' own scope)
status: GREEN

## Full-matrix closure: Class Progression row promotion, Cleric subset (2026-07-17, cycle 2026-07-17T0130)

Re-derived eligibility live per Step 1 rather than trusting the prior cycle's
summary: `git fetch origin tranche/3` + `git log origin/tranche/3 --oneline -8`
confirmed HEAD at `59aeacc` (Barbarian class-row promotion, no further
class-row work), `git status --porcelain` clean, working tree on
`tranche/3`. In-flight check (`ps -eo pid,ppid,etime,stat,cmd | grep -iE
'claude' | grep -v grep` plus tracing this shell's own ancestry via `ps -o
pid,ppid,cmd -p $$` up through `$PPID`) traced this session's own top-level
`claude -p` process (pid 3717130, exact prompt text confirmed identical,
parent `sd19-loop-supervisor.sh` pid 3387203) directly — no second
competing process running the SD-19 loop prompt. Also read the two required
SD-18 investigation-cycle sections (cycle-2026-07-15T0300 §3.4,
cycle-2026-07-15T0400 §3.5 in
`programs/codex/requirements/SD-18-core-rules-breadth/artifacts/SD-18-core-rules-breadth-progress.md`,
the current on-disk location of that file, not the workspace-root path
named in SD-19's own loop instruction's stale cross-reference) as read-only
reference, read-only — no write to that file. Re-verified live against
`support_state_matrix.rs` directly (`grep -n "row_id:"` for all `class.*`
rows plus direct `support_state`/`evidence_tier` field reads): all 7
remaining class rows (Rogue, Wizard, Sorcerer, Bard, Cleric, Paladin,
Ranger) confirmed `SupportState::Partial` / `EvidenceTier::Computed` by
direct read before starting.

Per the prior cycle's own recommendation ("smallest first"), re-measured
live file-footprint via `grep -rl "class\.<name>\." tests/
apps/desktop/src-tauri/src/` for all 7 remaining classes: Cleric 27, Wizard
29, Rogue 31, Sorcerer 34, Bard 36, Paladin 39, Ranger 42 — matching the
prior cycle's own estimate exactly. Cleric is the smallest, so chosen next
per the established sequencing.

### cycle-2026-07-17T0130 | Cleric class-row promotion (class.cleric.progression_and_spell_burden) | commit `033b9e8` | card t_ee387f8f | cargo test root 3422/3422 green | desktop backend 75/75 green | clippy clean

Promoted the Cleric row `Partial/Computed` -> `Supported/ProductVisible`.
Condition 2 (every named grounded milestone) was already satisfied — SD-13/
SD-18 compute grounding for Cleric levels 1-20 (base attack/save
progression, Channel Energy's die count and uses-per-day across every
supported level, the domain choice seam, the flat domain spell slot count
across every supported level, the Good domain's Touch of Good sacred bonus
and uses-per-day across every supported level, and the Healing domain's
Rebuke Death uses-per-day) was complete and unchanged this cycle. Condition
1 (live UI surface) was already satisfied too — the Class Progression
Catalog browser (`apps/desktop/src/classCatalog/ClassCatalogScreen.tsx`,
`list_class_catalog` Tauri command) shipped and was live-verified by an
earlier cycle (`9313e30`), so this cycle needed no new browser work, only
the matrix-and-test promotion.

**File surface (47 files, matching the pre-cycle 47-file estimate — 1
matrix carrier + 21 Cleric-specific proof tests + 4 cross-class
sibling-preservation splits + 25 closed-world files, minus 4 files counted
in both the Cleric-specific grep and the closed-world grep, i.e.
`sd13_cleric_level1_spell_baseline.rs` is both Cleric's own baseline file
and a member of the closed-world set):**
- `src/rules_core/support_state_matrix.rs`: the Cleric row's
  `support_state`/`evidence_tier` flipped to `Supported`/`ProductVisible`;
  `blocker_or_lossiness_note` extended (not rewritten) to name the Class
  Progression Catalog browser as the UI-surfacing evidence; `next_required_uplift`
  rewritten to name the still-out-of-scope compute burdens (Rebuke Death's
  heal-amount dice-roll/hit-point-state engine, domain spell-list contents,
  and the prepared divine spell posture burden) as future-SD-N scope rather
  than a live per-cycle target.
- 25 closed-world "no unexpected Supported row" files (24
  sibling-preservation tests plus the master `tests/sd13_support_state_matrix.rs`):
  found via the generic exclusion-list anchor
  (`grep -rl 'r.row_id != "class.barbarian.bounded_progression"'`) per the
  prior cycle's own handoff note — confirmed the same 25-file set as the
  prior three cycles' method. Mechanically extended each file's
  exclusion-list anchor with one new line for the Cleric row id, via a
  bulk `perl -0pi` insertion for the 19 files with the uniform
  single-line-anchor shape, plus 6 files (Druid, Monk, Paladin, Ranger,
  Cleric's own file) edited individually because they also needed a
  cross-class loop split or carried the row's own baseline assertion.
- 4 cross-class sibling-preservation tests whose own loops asserted Cleric
  stays Partial after their own class's slice (Ranger, Monk, Druid,
  Paladin) — Cleric was moved into each file's existing Supported loop
  (mirroring the Fighter/Monk/Druid/Barbarian cycles' own splits). Ranger's
  and Monk's files had Cleric inside a 3-element {Bard, Cleric, Sorcerer}
  "stays Partial" loop, reduced to a 2-element {Bard, Sorcerer} loop.
  Paladin's file had Cleric as a standalone single-row assert (not a loop),
  removed and folded into its existing Supported loop. Druid's file also
  had Cleric inside a 3-element loop, reduced the same way, with a new
  standalone Cleric-promoted assert added (Druid's file predates the
  Monk/Barbarian promotions and never grew a shared "Supported" loop of its
  own for those rows, so Cleric's promotion was recorded as an independent
  block rather than appended to a nonexistent loop).
- 21 Cleric-specific proof tests (`sd13_cleric_level{1..10}_*` family,
  `sd13_cleric_base_attack_and_saves.rs`, `sd13_cleric_domain_powers.rs`,
  `sd18_cleric_level{11..20}_widening.rs`) whose own historical assertions
  pinned the pre-promotion Partial/Computed state, updated to
  Supported/ProductVisible with an explanatory comment matching the
  Barbarian-cycle wording convention. The level-1 file's own stale
  assertion name (`matrix_cleric_row_is_partial_computed_and_names_...`)
  was left unrenamed per the Barbarian cycle's own precedent (rename would
  ripple into other tests' function references; a NOTE comment preserves
  the historical record instead), mirroring how the Barbarian level-1
  file's stale prose comment was annotated rather than deleted.

**One self-heal catch this cycle** (unlike Barbarian's zero-catch run):
the first `next_required_uplift` rewrite for the Cleric row dropped the two
literal substrings the row's own baseline proof test asserts
(`matrix_cleric_row_is_partial_computed_and_names_grounded_and_remaining_burdens`
in `sd13_cleric_level1_spell_baseline.rs` requires
`next_required_uplift.contains("Cleric") && next_required_uplift.contains("domain power")`).
The first full-suite run caught this as a single test failure; fixed by
rewording the uplift text to open with "grounding Cleric's remaining domain
power" before recommitting. `cargo test --locked` (3422/3422 green) and
`cargo clippy --locked --tests -- -D warnings` (clean) both passed after
the fix. This is the RED evidence for this UI-surfacing/promotion cycle
(per Step 4's UI-surfacing branch, there is no corpus-existence RED test in
the compute-grounding sense; the full-suite and full-lint runs before
committing serve as the RED/GREEN gate). Handoff note for the next cycle:
when a class row's own baseline proof test asserts specific substrings in
`next_required_uplift` or `blocker_or_lossiness_note`, grep the row's own
`_level1_` or `_base_attack_and_saves` proof file for `.contains(` calls
against those two fields *before* rewriting them, not after.

**Live verification:** not repeated this cycle — the Class Progression
Catalog browser's live-verification evidence (screenshot proof of the full
class list, a class filter, and search against real corpus-derived data)
was already captured by the browser-build cycle (`9313e30`) and is
unchanged; per the loop instruction's own Step 5 guidance ("at least once
per new browser, not necessarily every cycle after that"), re-verifying per
individual row promotion is not required.

**Full-matrix closure status: 25 of 34 rows now Supported/ProductVisible**
(9 spell schools + 4 equipment categories + 7 races + Fighter (2 rows) +
Monk (1 row) + Druid (1 row) + Barbarian (1 row) + Cleric (1 row)). Per
`## Full-matrix closure`'s priority order, the next frontier is the
remaining 6 class rows (Wizard, Rogue, Sorcerer, Bard, Paladin, Ranger) —
per-class file-footprint survey from the prior cycle for sequencing
(smallest first, should be re-measured live by the next cycle rather than
assumed unchanged): Wizard 29, Rogue 31, Sorcerer 34, Bard 36, Paladin 39,
Ranger 42 — or the Human interaction-row judgment call (tackled last per
the loop instruction's own priority ordering, since it needs a written
decision, not a mechanical build). The permanently-excluded non-Human
interaction row remains untouched throughout, as required.

### cycle-log

cycle: 2026-07-17T0130
criterion touched: Class Progression row promotion, Cleric subset (class.cleric.progression_and_spell_burden)
row_or_kind: class.cleric.progression_and_spell_burden
commit: 033b9e8
card: t_ee387f8f
verify: cargo test root 3422/3422 green; desktop backend 75/75 green; clippy clean (npm test/typecheck/build not re-run — matrix-and-test-only promotion cycle, no browser/frontend code touched, mirroring the Monk/Druid/Barbarian cycles' own scope)
status: GREEN

## Full-matrix closure: Class Progression row promotion, Wizard subset (2026-07-17, cycle 2026-07-17T2130)

Re-derived eligibility live per Step 1 rather than trusting the prior cycle's
summary: `git fetch origin tranche/3` + `git log origin/tranche/3 --oneline -10`
confirmed HEAD at `033b9e8` (Cleric class-row promotion, no further class-row
work landed since), `git status --porcelain` clean, working tree on
`tranche/3`. In-flight check (`ps -eo pid,etime,stat,cmd | grep -iE 'claude' |
grep -v grep`) showed one other live `claude -p` process (pid 3774363)
running the identical generic SD-19 loop-cycle prompt with no specific
criterion named — this is this session's own top-level process (the harness
invocation running this very cycle), not a second competing process; no
process was found naming a specific SD-19 acceptance criterion, so no
criterion was excluded by the in-flight check. Re-verified live against
`support_state_matrix.rs` directly (`grep -n "row_id:"` for all `class.*`
rows plus direct `support_state`/`evidence_tier` field reads): all 6
remaining class rows (Wizard, Rogue, Sorcerer, Bard, Paladin, Ranger)
confirmed `SupportState::Partial` / `EvidenceTier::Computed` by direct read
before starting. Also read the two required SD-18 investigation-cycle
sections (cycle-2026-07-15T0300 §3.4, cycle-2026-07-15T0400 §3.5 in
`programs/codex/requirements/SD-18-core-rules-breadth/artifacts/SD-18-core-rules-breadth-progress.md`,
the current on-disk location, not the workspace-root path named in SD-19's
own loop instruction's stale cross-reference) as read-only reference — no
write to that file.

Per the prior cycle's own recommendation ("smallest first"), re-measured
live file-footprint via `grep -rl "class\.<name>\." tests/
apps/desktop/src-tauri/src/` for all 6 remaining classes: Wizard 29, Rogue
31, Sorcerer 34, Bard 36, Paladin 39, Ranger 42 — matching the prior cycle's
own estimate exactly. Wizard is the smallest, so chosen next per the
established sequencing.

### cycle-2026-07-17T2130 | Wizard class-row promotion (class.wizard.progression_and_spell_burden) | commit `2157af4` | card t_6e1eda15 | cargo test root 3422/3422 green | desktop backend 75/75 green | clippy clean

Promoted the Wizard row `Partial/Computed` -> `Supported/ProductVisible`.
Condition 2 (every named grounded milestone) was already satisfied — SD-13/
SD-18 compute grounding for Wizard levels 1-20 (base attack/save
progression, Scribe Scroll's bonus-feat grant, the school specialization
choice recognition, the specialist bonus-slot flat count across every
supported level, and Intense Spells' bonus-damage magnitude across every
supported level) was complete and unchanged this cycle. Condition 1 (live UI
surface) was already satisfied too — the Class Progression Catalog browser
(`apps/desktop/src/classCatalog/ClassCatalogScreen.tsx`, `list_class_catalog`
Tauri command) shipped and was live-verified by an earlier cycle (`9313e30`),
so this cycle needed no new browser work, only the matrix-and-test
promotion.

**File surface (46 files, matching the pre-cycle 46-file estimate — 1 matrix
carrier + 21 Wizard-specific proof tests + 7 cross-class negative-control
files + the full 25-file closed-world set, minus overlap):**
- `src/rules_core/support_state_matrix.rs`: the Wizard row's
  `support_state`/`evidence_tier` flipped to `Supported`/`ProductVisible`;
  `blocker_or_lossiness_note` extended (not rewritten) to name the Class
  Progression Catalog browser as the UI-surfacing evidence; `next_required_uplift`
  rewritten to name the still-out-of-scope compute burdens (school-power
  execution machinery, opposed-school preparation cost, the bonus-feat
  selection/execution engine, and the prepared spellbook/spell-slot posture
  burden) as future-SD-N scope rather than a live per-cycle target.
- 25 closed-world "no unexpected Supported row" files (24
  sibling-preservation tests plus the master `tests/sd13_support_state_matrix.rs`):
  found via the generic exclusion-list anchor
  (`grep -rl 'r.row_id != "class.cleric.progression_and_spell_burden"'`) per
  the prior cycle's own handoff note — confirmed the same 25-file set as the
  prior five cycles' method (Fighter, Monk, Druid, Barbarian, Cleric). Applied
  a bulk `perl -pi` insertion adding one new exclusion line for the Wizard
  row id immediately after each file's existing Cleric exclusion line — all
  25 files had the anchor exactly once, so no individual-file exceptions were
  needed this cycle (unlike the Cleric cycle's 6 hand-edited files).
- 7 cross-class negative-control files whose own standalone tests asserted
  Wizard stays Partial after their own class's slice (Bard, Cleric, Druid,
  Monk, Paladin, Ranger, Sorcerer level-1 files) — each edited individually
  to flip the pinned `wizard.support_state`/`wizard.evidence_tier` assertion
  to `Supported`/`ProductVisible` and reword the surrounding prose/assert
  messages, mirroring how the Cleric cycle's own cross-class splits were
  hand-edited. Unlike Cleric (which needed loop-splitting because multiple
  classes shared one loop), every Wizard reference in these 7 files was
  already a standalone single-row assert block, so no loop-splitting was
  required this cycle.
- 21 Wizard-specific proof tests: 19 of them (`sd13_wizard_level{2..10}_progression.rs`,
  `sd18_wizard_level{11..20}_widening.rs`) shared a uniform single-line
  `assert_eq!(wizard.support_state, SupportState::Partial);` /
  `assert_eq!(wizard.evidence_tier, EvidenceTier::Computed);` pair, updated via
  a bulk `perl -pi` substitution across all 19 files in one pass (with an
  inline promotion-note comment appended to the support_state line). The
  remaining 2 (`sd13_wizard_level1_prepared_spell_baseline.rs`,
  `sd13_wizard_evocation_school_powers.rs`) used multi-line
  `assert_eq!(...)` calls with custom failure messages and (for the
  evocation-school-powers file) an extra `assert_ne!(wizard.support_state,
  SupportState::Supported);` line that had to be removed rather than
  flipped — both hand-edited individually, mirroring the Cleric cycle's own
  level1-file hand-edit.

**Zero self-heal catches this cycle** (unlike Cleric's one-catch run): before
rewriting the Wizard row's `blocker_or_lossiness_note`/`next_required_uplift`,
the required substring tokens the level-1 baseline proof test's
`.contains()` assertions check for ("Scribe Scroll", "specialization
choice", "specialist bonus slot", "school powers", "opposed-school",
"spellbook", "spells prepared", "spell slots") were grepped against the
matrix row's full text up front, confirmed present outside the edited
region, and left untouched — applying the prior cycle's own handoff note
directly. `cargo test --locked` (3422/3422 green), `cd apps/desktop/src-tauri
&& cargo test` (75/75 green), and `cargo clippy --locked --tests -- -D
warnings` (clean) all passed on the first run, with no fix-and-recommit
cycle needed. This is the RED/GREEN evidence for this UI-surfacing/promotion
cycle (per Step 4's UI-surfacing branch, there is no corpus-existence RED
test in the compute-grounding sense; the full-suite and full-lint runs
before committing serve as the RED/GREEN gate).

**Live verification:** not repeated this cycle — the Class Progression
Catalog browser's live-verification evidence (screenshot proof of the full
class list, a class filter, and search against real corpus-derived data)
was already captured by the browser-build cycle (`9313e30`) and is
unchanged; per the loop instruction's own Step 5 guidance ("at least once
per new browser, not necessarily every cycle after that"), re-verifying per
individual row promotion is not required.

**Full-matrix closure status: 26 of 34 rows now Supported/ProductVisible**
(9 spell schools + 4 equipment categories + 7 races + Fighter (2 rows) +
Monk (1 row) + Druid (1 row) + Barbarian (1 row) + Cleric (1 row) + Wizard
(1 row)). Per `## Full-matrix closure`'s priority order, the next frontier
is the remaining 5 class rows (Rogue, Sorcerer, Bard, Paladin, Ranger) —
per-class file-footprint survey re-measured live this cycle for sequencing
(smallest first, should be re-measured live by the next cycle rather than
assumed unchanged): Rogue 31, Sorcerer 34, Bard 36, Paladin 39, Ranger 42 —
or the Human interaction-row judgment call (tackled last per the loop
instruction's own priority ordering, since it needs a written decision, not
a mechanical build). The permanently-excluded non-Human interaction row
remains untouched throughout, as required.

## Full-matrix closure: Class Progression row promotion, Rogue subset (2026-07-16, cycle 2026-07-16T2123)

Re-derived eligibility live per Step 1 rather than trusting the prior cycle's
summary: `git fetch origin tranche/3` + `git log origin/tranche/3 --oneline -8`
confirmed HEAD at `2157af4` (Wizard class-row promotion, no further class-row
work landed since), `git status --porcelain` clean, working tree on
`tranche/3`. In-flight check (`ps -eo pid,etime,stat,cmd | grep -iE 'claude' |
grep -v grep` plus tracing this shell's own ancestry via `ps -o pid,ppid,cmd
-p $$` up through `$PPID`) found one other live `claude -p` process (pid
3805589) running the identical generic SD-19 loop-cycle prompt with no
specific criterion named — traced directly to this session's own top-level
harness invocation (this shell's `$PPID` resolves straight to it), not a
second competing process; no process was found naming a specific SD-19
acceptance criterion, so no criterion was excluded by the in-flight check.
Also read the two required SD-18 investigation-cycle sections
(cycle-2026-07-15T0300 §3.4, cycle-2026-07-15T0400 §3.5 in
`programs/codex/requirements/SD-18-core-rules-breadth/artifacts/SD-18-core-rules-breadth-progress.md`,
the current on-disk location, not the workspace-root path named in SD-19's
own loop instruction's stale cross-reference) as read-only reference — no
write to that file; both remain fully superseded by SD-19's already-landed
capability slice, as every prior cycle has also found. Re-verified live
against `support_state_matrix.rs` directly (`grep -n "row_id:"` for all
`class.*` rows plus direct `support_state`/`evidence_tier` field reads): all
5 remaining class rows (Rogue, Sorcerer, Bard, Paladin, Ranger) confirmed
`SupportState::Partial` / `EvidenceTier::Computed` by direct read before
starting.

Per the prior cycle's own recommendation ("smallest first"), re-measured
live file-footprint via `grep -rl "class\.<name>\." tests/
apps/desktop/src-tauri/src/` for all 5 remaining classes: Rogue 31, Sorcerer
34, Bard 36, Paladin 39, Ranger 42 — matching the prior cycle's own estimate
exactly. Rogue is the smallest, so chosen next per the established
sequencing.

### cycle-2026-07-16T2123 | Rogue class-row promotion (class.rogue.bounded_progression) | commit `433fdc2` | card t_d3f47e43 | cargo test root 3422/3422 green | desktop backend 75/75 green | clippy clean

Promoted the Rogue row `Partial/Computed` -> `Supported/ProductVisible`.
Condition 2 (every named grounded milestone) was already satisfied — SD-13/
SD-18 compute grounding for Rogue levels 1-20 (base attack/save progression,
sneak-attack die count, trapfinding, Evasion, Trap Sense, Uncanny Dodge,
Improved Uncanny Dodge, and the numbered rogue-talent-slot recognition
records across every supported level through the level-20 Master Strike
gate) was complete and unchanged this cycle. Condition 1 (live UI surface)
was already satisfied too — the Class Progression Catalog browser
(`apps/desktop/src/classCatalog/ClassCatalogScreen.tsx`, `list_class_catalog`
Tauri command) shipped and was live-verified by an earlier cycle (`9313e30`),
so this cycle needed no new browser work, only the matrix-and-test
promotion.

**File surface (50 files, matching the pre-cycle 31-file class.rogue.
footprint plus the closed-world set not literally naming Rogue before this
cycle — 1 matrix carrier + 23 Rogue-specific proof tests + 7 cross-class
negative-control files + the full 25-file closed-world set (24
sibling-preservation tests plus the master `tests/sd13_support_state_matrix.rs`),
with `tests/sd13_rogue_level1_chassis_baseline.rs` counted once despite
belonging to both the Rogue-specific and closed-world sets):**
- `src/rules_core/support_state_matrix.rs`: the Rogue row's
  `support_state`/`evidence_tier` flipped to `Supported`/`ProductVisible`;
  `blocker_or_lossiness_note` extended (not rewritten) to name the level-20
  widening and the Class Progression Catalog browser as the UI-surfacing
  evidence; `next_required_uplift` rewritten to name the still-out-of-scope
  compute burdens (the general rogue-talent choice/effect engine and Master
  Strike's own execution) as future-SD-N scope rather than a live per-cycle
  target.
- 25 closed-world "no unexpected Supported row" files (24
  sibling-preservation tests plus the master `tests/sd13_support_state_matrix.rs`):
  found via the generic exclusion-list anchor
  (`grep -rl 'r.row_id != "class.wizard.progression_and_spell_burden"'`) per
  the prior cycle's own handoff note. The first bulk-insertion attempt via
  `perl -pi -e 's/^(\s*)(&&\s*)?(...)(,?)\s*$/$&\n.../'` corrupted all 24
  sibling files: the trailing `\s*$` in the match greedily consumed each
  matched line's own newline character, so the replacement's manually-added
  `\n` didn't restore it and the following original line printed jammed
  onto the same line as the insertion. Caught by directly reading one
  edited file before running any test, reverted every touched file via
  `git checkout --`, and redone with a newline-safe `awk` script that
  reprints every line unchanged and appends a brand-new line immediately
  after the matched line, never touching existing line boundaries.
  Verified clean on a sample file before applying to the full set; a
  post-insertion `grep -c` confirmed exactly one Rogue exclusion line
  landed in each of the 25 files. `tests/sd13_support_state_matrix.rs`
  additionally got a direct hand-edit to its own dedicated
  `rogue_row_is_partial_and_computed_with_blocker_note` test (function name
  kept unchanged, mirroring the Fighter-row precedent of not renaming a
  test function on promotion; support_state/evidence_tier assertions
  flipped; a promotion comment added).
- 7 cross-class negative-control files whose own standalone tests asserted
  Rogue stays Partial after their own class's slice (Barbarian,
  Fighter-level1-mandatory-milestone, Fighter-level2-3, Hybrid, Monk,
  Paladin, Ranger) — each edited individually to flip the pinned
  `rogue.support_state` assertion to `Supported` (adding an
  `evidence_tier` assertion alongside it where the original only checked
  `support_state`) and reword the surrounding comment/assert messages.
  Two of the seven (`sd13_fighter_level1_mandatory_milestone_classification.rs`,
  `sd13_fighter_level2_level3_progression.rs`) had their own dedicated
  `matrix_keeps_rogue_partial_after_its_own_recognition_slice` test
  functions, renamed to `matrix_keeps_rogue_supported_after_...` (mirroring
  that no other cross-class file needed a function rename, since the
  remaining five asserted Rogue inline inside a larger, differently-named
  test).
- 23 Rogue-specific proof tests (`sd13_rogue_level1_chassis_baseline.rs`,
  `sd13_rogue_level{2..10}_progression.rs`, `sd13_rogue_second_talent.rs`,
  `sd13_rogue_talent_choice.rs`, `sd13_rogue_talents_three_through_five.rs`,
  `sd18_rogue_level{11..20}_widening.rs`): unlike Wizard's split (19 uniform
  + 2 hand-edited), all 23 Rogue files shared the identical uniform
  single-line `assert_eq!(rogue.support_state, SupportState::Partial);` /
  `assert_eq!(rogue.evidence_tier, EvidenceTier::Computed);` pair with no
  `assert_ne!` complications anywhere (checked via a direct grep before
  editing), so a single bulk `perl -pi` substitution across all 23 files
  handled the whole set in one pass, with no hand-edited exceptions needed
  this cycle.

**One self-heal this cycle** (unlike Wizard's zero-catch run): the bulk
closed-world insertion's first attempt corrupted 24 files via the
newline-consuming regex bug described above. Caught before any `cargo test`
run, self-healed by reverting via `git checkout --` on the exact 24-file set
and redoing the insertion with a newline-safe `awk` script; no stale state
was ever committed or pushed. Before rewriting the Rogue row's
`blocker_or_lossiness_note`, the required substring tokens the level-1
baseline proof test's `.contains()` assertions check for ("base attack",
"base save", "sneak attack", "trapfinding", "check-execution engine",
"trap DC", "magic-trap disarm engine", "rogue talent", "damage-roll
execution", "defense.total_save") were grepped against the matrix row's
full text up front and confirmed present outside the edited region, plus
confirmed the stale-claim guards ("only trapfinding remains unproven",
"trapfinding remains unproven") stayed absent — applying the Wizard cycle's
own handoff note directly. `cargo test --locked` (3422/3422 green),
`cd apps/desktop/src-tauri && cargo test` (75/75 green), and
`cargo clippy --locked --tests -- -D warnings` (clean) all passed after the
self-heal, with no further fix-and-recommit cycle needed. This is the
RED/GREEN evidence for this UI-surfacing/promotion cycle (per Step 4's
UI-surfacing branch, there is no corpus-existence RED test in the
compute-grounding sense; the full-suite and full-lint runs before
committing serve as the RED/GREEN gate).

**Live verification:** not repeated this cycle — the Class Progression
Catalog browser's live-verification evidence (screenshot proof of the full
class list, a class filter, and search against real corpus-derived data)
was already captured by the browser-build cycle (`9313e30`) and is
unchanged; per the loop instruction's own Step 5 guidance ("at least once
per new browser, not necessarily every cycle after that"), re-verifying per
individual row promotion is not required.

**Full-matrix closure status: 27 of 34 rows now Supported/ProductVisible**
(9 spell schools + 4 equipment categories + 7 races + Fighter (2 rows) +
Monk (1 row) + Druid (1 row) + Barbarian (1 row) + Cleric (1 row) + Wizard
(1 row) + Rogue (1 row)). Per `## Full-matrix closure`'s priority order, the
next frontier is the remaining 4 class rows (Sorcerer, Bard, Paladin,
Ranger) — per-class file-footprint survey re-measured live this cycle for
sequencing (smallest first, should be re-measured live by the next cycle
rather than assumed unchanged): Sorcerer 34, Bard 36, Paladin 39, Ranger 42
— or the Human interaction-row judgment call (tackled last per the loop
instruction's own priority ordering, since it needs a written decision, not
a mechanical build). The permanently-excluded non-Human interaction row
remains untouched throughout, as required.

### cycle-log

cycle: 2026-07-16T2123
criterion touched: Class Progression row promotion, Rogue subset (class.rogue.bounded_progression)
row_or_kind: class.rogue.bounded_progression
commit: 433fdc2
card: t_d3f47e43
verify: cargo test root 3422/3422 green; desktop backend 75/75 green; clippy clean (npm test/typecheck/build not re-run — matrix-and-test-only promotion cycle, no browser/frontend code touched, mirroring the Monk/Druid/Barbarian/Cleric/Wizard cycles' own scope)
status: GREEN

## Full-matrix closure: Class Progression row promotion, Sorcerer subset (2026-07-16, cycle 2026-07-16T2145)

Re-derived eligibility live before starting. `git fetch origin tranche/3` +
`git log origin/tranche/3 --oneline -10`: HEAD `433fdc2` (Rogue promotion),
matching `snapshot_as_of`; `git status --porcelain` 0 lines; `git worktree
list --porcelain` showed only the primary worktree; branch `tranche/3`.
In-flight check (`ps -eo pid,ppid,etime,stat,cmd | grep claude`) found this
session's own top-level `claude -p` process (pid 3837447) as the only
`claude` process running the SD-19 loop prompt — traced its ancestry
(`ps -o pid,ppid,cmd -p $$` up through `$PPID`) directly to that pid,
confirming it is this very session, not a second competing process. No
in-flight collision, no criterion excluded by Step 1's in-flight check. Also
read the two required SD-18 investigation-cycle sections
(`cycle-2026-07-15T0300` §3.4, `cycle-2026-07-15T0400` §3.5 in
`SD-18-core-rules-breadth-progress.md`) as read-only reference — no write to
that file; both remain fully superseded by SD-19's already-landed capability
slice. Re-verified live against `support_state_matrix.rs` directly
(`grep -n "row_id:"` for all `class.*` rows plus direct
`support_state`/`evidence_tier` field reads): all 4 remaining class rows
(Sorcerer, Bard, Paladin, Ranger) confirmed `SupportState::Partial` /
`EvidenceTier::Computed` by direct read before starting.

Per the prior cycle's own recommendation ("smallest first"), re-measured
live file-footprint via `grep -rl "class\.<name>\." tests/
apps/desktop/src-tauri/src/` for all 4 remaining classes: Sorcerer 34, Bard
36, Paladin 39, Ranger 42 — matching the prior cycle's own estimate exactly.
Sorcerer is the smallest, so chosen next per the established sequencing.

### cycle-2026-07-16T2145 | Sorcerer class-row promotion (class.sorcerer.progression_and_spell_burden) | commit `c33951c` | card t_4c4d6350 | cargo test root 3422/3422 green | desktop backend 75/75 green | clippy clean

Promoted the Sorcerer row `Partial/Computed` -> `Supported/ProductVisible`.
Condition 2 (every named grounded milestone) was already satisfied — SD-13/
SD-18 compute grounding for Sorcerer levels 1-20 (base attack/save
progression, Eschew Materials, the bloodline choice and Arcane bloodline
class-skill recognitions, and the per-level arithmetic-widening records
through the level-20 spell-table ceiling) was complete and unchanged this
cycle. Condition 1 (live UI surface) was already satisfied too — the Class
Progression Catalog browser
(`apps/desktop/src/classCatalog/ClassCatalogScreen.tsx`, `list_class_catalog`
Tauri command) shipped and was live-verified by an earlier cycle (`9313e30`),
so this cycle needed no new browser work, only the matrix-and-test
promotion.

**File surface (51 files):**
- `src/rules_core/support_state_matrix.rs`: the Sorcerer row's
  `support_state`/`evidence_tier` flipped to `Supported`/`ProductVisible`;
  `blocker_or_lossiness_note` extended (not rewritten) to name the Class
  Progression Catalog browser as the UI-surfacing evidence; `next_required_uplift`
  rewritten to name the still-out-of-scope compute burdens (Arcane Bond
  execution and the spontaneous which-spells-known/casting-execution burden)
  as future-SD-N scope rather than a live per-cycle target.
- 25 Sorcerer-specific proof tests (`sd13_sorcerer_level{2..10}_progression.rs`,
  `sd18_sorcerer_level{11..20}_widening.rs`, `sd13_sorcerer_bonus_spells.rs`,
  `sd13_sorcerer_spell_save_dcs.rs`, `sd13_sorcerer_spells_per_day_counts.rs`,
  `sd13_sorcerer_spell_level_thresholds.rs`, `sd13_sorcerer_spells_known_counts.rs`,
  `sd13_sorcerer_total_spells_per_day.rs`): all 25 shared the identical
  uniform single-line `assert_eq!(sorcerer.support_state, SupportState::Partial);`
  / `assert_eq!(sorcerer.evidence_tier, EvidenceTier::Computed);` pair
  (checked via a direct grep before editing; `sd13_sorcerer_base_attack_and_saves.rs`
  and `sd13_sorcerer_bloodline_class_skill_choice.rs` were checked and found
  to carry no matrix-row assertion at all, so left untouched), so a single
  bulk `perl -pi` substitution across all 25 files handled the whole set in
  one pass, mirroring the Rogue cycle's own uniform-pair precedent.
- `tests/sd13_sorcerer_level1_spell_baseline.rs`: the Sorcerer row's own
  dedicated matrix-assertion test
  (`matrix_sorcerer_row_is_partial_computed_and_names_choice_arcane_bond_and_spontaneous`,
  function name kept unchanged per the Fighter/Rogue precedent of not
  renaming a test function on promotion) flipped to Supported/ProductVisible;
  this file's own closed-world exclusion test
  (`matrix_does_not_promote_any_row_to_supported_or_lossy`) got a new
  `class.sorcerer.progression_and_spell_burden` exclusion line.
- 7 cross-class negative-control files whose own level-1/foundational tests
  pinned a "sorcerer stays Partial" assert (Bard, Cleric, Druid, Monk,
  Paladin, Ranger, Wizard level-1 baseline files) — found via
  `grep -rln 'sorcerer\.support_state\|"class.sorcerer` across `tests/` and
  cross-checked against actual assertion sites (as opposed to files that
  merely load a Sorcerer fixture to prove another class's recognition
  doesn't leak onto it, which needed no edit). Two shapes:
  - Bard, Paladin, Wizard: Sorcerer asserted standalone (not sharing a loop
    with a row that stays Partial) — each flipped in place to
    Supported/ProductVisible, with `sd13_bard_level1_spell_baseline.rs`'s
    own test function renamed
    `matrix_keeps_sorcerer_and_paladin_blocked_computed_after_bard_slice` ->
    `matrix_keeps_sorcerer_supported_and_paladin_blocked_computed_after_bard_slice`
    and `sd13_wizard_level1_prepared_spell_baseline.rs`'s renamed
    `matrix_preserves_hybrid_paladin_and_sorcerer_blocked_computed_truth` ->
    `matrix_preserves_hybrid_paladin_blocked_computed_and_sorcerer_supported_truth`
    (mirroring the Rogue cycle's own precedent of renaming a function whose
    name asserted a now-stale state).
  - Cleric, Druid, Monk, Ranger: Sorcerer shared a `for row_id in [...]`
    loop with Bard (who stays Partial), so each loop was split into two
    standalone assertions — Sorcerer promoted, Bard left unchanged — rather
    than a blanket flip. `sd13_cleric_level1_spell_baseline.rs`'s
    `matrix_preserves_sorcerer_bard_wizard_and_hybrid_blocked_computed_truth`
    and `sd13_druid_level1_spell_baseline.rs`'s
    `matrix_preserves_sorcerer_bard_wizard_cleric_and_hybrid_blocked_computed_truth`
    were both renamed to name Bard/hybrid as staying blocked-computed and
    Sorcerer as newly supported, matching the same rename convention.
- 25 closed-world "no unexpected Supported row" files (24
  sibling-preservation tests plus the master `tests/sd13_support_state_matrix.rs`):
  found via the generic exclusion-list anchor
  (`grep -rl 'r.row_id != "class.rogue.bounded_progression"'`) per the Rogue
  cycle's own handoff note. Applied the newline-safe `awk` insertion directly
  this time (skipping the perl `\s*$` approach that corrupted files during
  the Rogue cycle), verified via a post-insertion `grep -c` that exactly one
  Sorcerer exclusion line landed in each of the 24 files (the 25th,
  `sd13_sorcerer_level1_spell_baseline.rs`, got its own closed-world line via
  the dedicated edit above, not the bulk pass, to avoid double-insertion).

**One self-heal this cycle:** splitting Sorcerer out of the Ranger file's
Bard+Sorcerer loop (`sd13_ranger_level1_chassis_and_class_feature_separation.rs`)
left a one-item `for id in ["class.bard.progression_and_spell_burden"] { ... }`
loop. `cargo clippy --locked --tests -- -D warnings` (run before committing,
per this cycle's own GREEN-evidence step) caught it as
`clippy::single_element_loop`, implied by `-D warnings`. Fixed by collapsing
the loop to a bare `{ let id = "..."; ... }` block, matching clippy's own
suggested rewrite, before any commit — no recommit needed. `cargo test
--locked` (3422/3422 green), `cd apps/desktop/src-tauri && cargo test`
(75/75 green), and `cargo clippy --locked --tests -- -D warnings` (clean)
all passed after the fix.

Kanban card minting hit the same CLI mismatch every prior SD-19 cycle has
hit: `hermes kanban create` rejects `--initial-status done` in the current
CLI. Self-healed identically — created with the default `ready` status
(`t_4c4d6350`), then `hermes kanban complete t_4c4d6350 --result "..."`.

**Live verification:** not repeated this cycle — the Class Progression
Catalog browser's live-verification evidence (screenshot proof of the full
class list, a class filter, and search against real corpus-derived data)
was already captured by the browser-build cycle (`9313e30`) and is
unchanged; per the loop instruction's own Step 5 guidance ("at least once
per new browser, not necessarily every cycle after that"), re-verifying per
individual row promotion is not required.

**Full-matrix closure status: 28 of 34 rows now Supported/ProductVisible**
(9 spell schools + 4 equipment categories + 7 races + Fighter (2 rows) +
Monk (1 row) + Druid (1 row) + Barbarian (1 row) + Cleric (1 row) + Wizard
(1 row) + Rogue (1 row) + Sorcerer (1 row)). Per `## Full-matrix closure`'s
priority order, the next frontier is the remaining 3 class rows (Bard,
Paladin, Ranger) — per-class file-footprint survey from the prior cycle
(should be re-measured live by the next cycle rather than assumed
unchanged): Bard 36, Paladin 39, Ranger 42 — or the Human interaction-row
judgment call (tackled last per the loop instruction's own priority
ordering, since it needs a written decision, not a mechanical build). The
permanently-excluded non-Human interaction row remains untouched throughout,
as required.

### cycle-log

cycle: 2026-07-16T2145
criterion touched: Class Progression row promotion, Sorcerer subset (class.sorcerer.progression_and_spell_burden)
row_or_kind: class.sorcerer.progression_and_spell_burden
commit: c33951c
card: t_4c4d6350
verify: cargo test root 3422/3422 green; desktop backend 75/75 green; clippy clean (npm test/typecheck/build not re-run — matrix-and-test-only promotion cycle, no browser/frontend code touched, mirroring the Monk/Druid/Barbarian/Cleric/Wizard/Rogue cycles' own scope)
status: GREEN

## Full-matrix closure: Class Progression row promotion, Bard subset (2026-07-16, cycle 2026-07-16T2200)

Re-derived eligibility live before starting. `git fetch origin tranche/3` +
`git log origin/tranche/3 --oneline -10`: HEAD `c33951c` (Sorcerer promotion),
matching `snapshot_as_of`; `git status --porcelain` 0 lines; `git worktree
list --porcelain` showed only the primary worktree; branch `tranche/3`.
In-flight check (`ps -eo pid,etime,stat,cmd | grep -iE 'claude' | grep -v
grep`) found a second `claude -p` process (pid 3872506) running with a
prompt textually identical to this session's own launch prompt. Traced its
ancestry directly (`ps -o pid,ppid,cmd -p $$` walked up through `$PPID` and
the grandparent) to `bash /home/ubuntu/workspace/sd19-loop-supervisor.sh`
(pid 3387203) — confirming pid 3872506 *is* this very session's own
top-level process, not a second competing one. A third `claude` process
(pid 2195929, ~13.5h uptime) was also live but carries no prompt naming any
SD-19 criterion (bare `claude`, an unrelated interactive session) — no
in-flight collision either way. Also read the two required SD-18
investigation-cycle sections (`cycle-2026-07-15T0300` §3.4,
`cycle-2026-07-15T0400` §3.5 in `SD-18-core-rules-breadth-progress.md`) as
read-only reference — no write to that file; both remain fully superseded
by SD-19's already-landed capability slice. Re-verified live against
`support_state_matrix.rs` directly (`grep -n "row_id:"` for all `class.*`
rows plus direct `support_state`/`evidence_tier` field reads): all 3
remaining class rows (Bard, Paladin, Ranger) confirmed `SupportState::Partial`
/ `EvidenceTier::Computed` by direct read before starting.

Re-measured live file-footprint via `grep -rl "class\.<name>\." tests/
apps/desktop/src-tauri/src/` for all 3 remaining classes: Bard 36, Paladin
39, Ranger 42 — matching the prior cycle's own estimate exactly. Bard is the
smallest, so chosen next per the established sequencing.

### cycle-2026-07-16T2200 | Bard class-row promotion (class.bard.progression_and_spell_burden) | commit `3725cd6` | card t_87cc96a7 | cargo test root 3422/3422 green | desktop backend 75/75 green | clippy clean

Promoted the Bard row `Partial/Computed` -> `Supported/ProductVisible`.
Condition 2 (every named grounded milestone) was already satisfied — SD-13/
SD-18 compute grounding for Bard levels 1-20 (base attack/save progression,
Bardic Knowledge, the flat Bardic Performance surface — rounds per day,
inspire courage, Fascinate DC/creature count, Well-Versed, Inspire
Competence, Lore Master, Soothing Performance, Frightening Tune, Inspire
Heroics, and the level-20 Deadly Performance capstone DC — all grounded for
real at every supported level) was complete and unchanged this cycle.
Condition 1 (live UI surface) was already satisfied too — the Class
Progression Catalog browser (`apps/desktop/src/classCatalog/ClassCatalogScreen.tsx`,
`list_class_catalog` Tauri command) shipped and was live-verified by an
earlier cycle (`9313e30`), so this cycle needed no new browser work, only
the matrix-and-test promotion.

**File surface (53 files):**
- `src/rules_core/support_state_matrix.rs`: the Bard row's
  `support_state`/`evidence_tier` flipped to `Supported`/`ProductVisible`;
  `blocker_or_lossiness_note` extended (not rewritten) to name the Class
  Progression Catalog browser as the UI-surfacing evidence; `next_required_uplift`
  rewritten to name the still-out-of-scope compute burdens (the bardic
  performance-execution engine and the spontaneous spell-slot burden) as
  future-SD-N scope rather than a live per-cycle target, mirroring the
  Sorcerer/Wizard/Rogue precedent's rewrite framing exactly.
- 27 Bard-specific proof tests (`sd13_bard_base_attack_and_saves.rs`,
  `sd13_bard_bonus_spells.rs`, `sd13_bard_level{2..10}_progression.rs`,
  `sd13_bard_spell_level_thresholds.rs`, `sd13_bard_spell_save_dcs.rs`,
  `sd13_bard_spells_known_counts.rs`, `sd13_bard_spells_per_day_counts.rs`,
  `sd13_bard_total_spells_per_day.rs`, `sd13_bard_versatile_performance_slots.rs`,
  `sd18_bard_level{11..20}_widening.rs`): 26 of the 27 shared the identical
  uniform single-line `assert_eq!(bard.support_state, SupportState::Partial);`
  / `assert_eq!(bard.evidence_tier, EvidenceTier::Computed);` pair (checked
  via a direct grep before editing), so a single bulk `perl -pi` substitution
  handled those 26 in one pass, mirroring the Rogue/Sorcerer cycles' own
  uniform-pair precedent. The 27th (`sd13_bard_base_attack_and_saves.rs`)
  carried a differently-shaped multi-line `assert_eq!(row.support_state,
  SupportState::Partial, "...")` with an inline failure message and a `row`
  binding instead of `bard` — caught by the bulk grep surfacing a non-matching
  line, hand-edited individually (function name kept unchanged, per the
  Fighter/Rogue/Sorcerer precedent of not renaming a test function on
  promotion; only the asserted value and message updated).
- `tests/sd13_bard_level1_spell_baseline.rs`: the Bard row's own dedicated
  matrix-assertion test (`matrix_bard_row_is_partial_computed_and_names_remaining_burdens`,
  function name kept unchanged per the Fighter/Rogue/Sorcerer precedent)
  flipped to Supported/ProductVisible; this file's own closed-world
  exclusion test (`matrix_does_not_promote_any_row_to_supported_or_lossy_after_bard_slice`)
  got a new `class.bard.progression_and_spell_burden` exclusion line.
- 7 cross-class negative-control files whose own level-1/foundational tests
  pinned a "bard stays Partial" assert (Cleric, Druid, Monk, Paladin, Ranger,
  Sorcerer, Wizard level-1/foundational files) — found via
  `grep -rln 'bard\.support_state\|"class.bard` across `tests/` and
  cross-checked against actual assertion sites. Unlike the Sorcerer cycle,
  none of the 7 shared a `for row_id in [...]` loop with a still-Partial
  sibling this cycle (Paladin and Ranger, the two classes still Partial
  after this cycle, were each asserted standalone in every file that also
  asserted Bard), so every file got a direct standalone flip, no loop-split
  needed. 4 of the 7 (Cleric, Druid, Sorcerer, Wizard) had their own
  dedicated test function renamed to drop a now-stale "bard blocked/partial"
  claim from the name (`matrix_preserves_bard_wizard_hybrid_blocked_computed_and_sorcerer_supported_truth`
  -> `matrix_preserves_wizard_hybrid_blocked_computed_and_sorcerer_bard_supported_truth`
  in Cleric; `matrix_preserves_bard_wizard_cleric_hybrid_blocked_computed_and_sorcerer_supported_truth`
  -> `matrix_preserves_wizard_hybrid_blocked_computed_and_sorcerer_bard_cleric_supported_truth`
  in Druid; `matrix_wizard_row_reflects_current_truth_and_preserves_bard_blocked_state`
  -> `matrix_wizard_row_reflects_current_truth_and_preserves_bard_supported_state`
  in Sorcerer; `matrix_preserves_bard_blocked_computed_truth` ->
  `matrix_preserves_bard_supported_product_visible_truth` in Wizard),
  mirroring the Rogue/Sorcerer cycles' own rename convention. Monk, Paladin,
  and Ranger needed no rename (their test function names carried no stale
  Bard-specific claim).
- 25 closed-world "no unexpected Supported row" files (24
  sibling-preservation tests plus the master `tests/sd13_support_state_matrix.rs`):
  found via the generic exclusion-list anchor
  (`grep -rl 'r.row_id != "class.sorcerer.progression_and_spell_burden"'`)
  per the Sorcerer cycle's own handoff note. Applied the newline-safe `awk`
  insertion throughout, verified via a post-insertion `grep -c` that exactly
  one Bard exclusion line landed in each of the 24 files (the 25th,
  `sd13_bard_level1_spell_baseline.rs`, got its own closed-world line via
  the dedicated edit above, not the bulk pass, to avoid double-insertion).

**Zero self-heals this cycle** (unlike Sorcerer's one clippy catch): `cargo
test --locked` (3422/3422 green), `cd apps/desktop/src-tauri && cargo test`
(75/75 green), and `cargo clippy --locked --tests -- -D warnings` (clean)
all passed on the first run, no fix-and-recommit needed.

Kanban card minting hit the same CLI mismatch every prior SD-19 cycle has
hit: `hermes kanban create` rejects `--initial-status done` in the current
CLI. Self-healed identically — created with the default `ready` status
(`t_87cc96a7`), then `hermes kanban complete t_87cc96a7 --result "..."`.

**Live verification:** not repeated this cycle — the Class Progression
Catalog browser's live-verification evidence (screenshot proof of the full
class list, a class filter, and search against real corpus-derived data)
was already captured by the browser-build cycle (`9313e30`) and is
unchanged; per the loop instruction's own Step 5 guidance ("at least once
per new browser, not necessarily every cycle after that"), re-verifying per
individual row promotion is not required.

**Full-matrix closure status: 29 of 34 rows now Supported/ProductVisible**
(9 spell schools + 4 equipment categories + 7 races + Fighter (2 rows) +
Monk (1 row) + Druid (1 row) + Barbarian (1 row) + Cleric (1 row) + Wizard
(1 row) + Rogue (1 row) + Sorcerer (1 row) + Bard (1 row)). Per
`## Full-matrix closure`'s priority order, the next frontier is the
remaining 2 class rows (Paladin, Ranger) — per-class file-footprint survey
re-measured live this cycle for sequencing (smallest first, should be
re-measured live by the next cycle rather than assumed unchanged): Paladin
39, Ranger 42 — or the Human interaction-row judgment call (tackled last per
the loop instruction's own priority ordering, since it needs a written
decision, not a mechanical build). The permanently-excluded non-Human
interaction row remains untouched throughout, as required.

### cycle-log

cycle: 2026-07-16T2200
criterion touched: Class Progression row promotion, Bard subset (class.bard.progression_and_spell_burden)
row_or_kind: class.bard.progression_and_spell_burden
commit: 3725cd6
card: t_87cc96a7
verify: cargo test root 3422/3422 green; desktop backend 75/75 green; clippy clean (npm test/typecheck/build not re-run — matrix-and-test-only promotion cycle, no browser/frontend code touched, mirroring the Monk/Druid/Barbarian/Cleric/Wizard/Rogue/Sorcerer cycles' own scope)
status: GREEN

## Full-matrix closure: Class Progression row promotion, Paladin subset (2026-07-17, cycle 2026-07-17T2200)

Re-derived eligibility live before starting. `git fetch origin tranche/3` +
`git log origin/tranche/3 --oneline -5`: HEAD `3725cd6` (full SHA
`3725cd67eecb3a1f39dba6f39533fe7f25fb811f`, Bard promotion), matching
`snapshot_as_of`; `git status --porcelain` 0
lines; `git worktree list --porcelain` showed only the primary worktree;
branch `tranche/3`. In-flight check (`ps -eo pid,etime,stat,cmd | grep -iE
'claude' | grep -v grep`) found this session's own top-level `claude -p`
process (pid 3903425) plus one unrelated long-running interactive session
(pid 2195929, ~13.75h uptime, no SD-19 criterion named). Traced pid
3903425's ancestry directly (`ps -o pid,ppid,cmd -p $$` / `-p $PPID`) to
`bash .../sd19-loop-supervisor.sh` (pid 3387203) — confirming it is this
very session, not a second competing process. No in-flight collision. Also
read the two required SD-18 investigation-cycle sections
(`cycle-2026-07-15T0300` §3.4, `cycle-2026-07-15T0400` §3.5 in
`SD-18-core-rules-breadth-progress.md`, found at
`../SD-18/artifacts/`
since the workspace-root copy is archived) as read-only reference — no
write to that file; both remain fully superseded by SD-19's already-landed
capability slice. Re-verified live against `support_state_matrix.rs`
directly (`grep -n "row_id:"` for the two remaining `class.*` rows plus
direct `support_state`/`evidence_tier` field reads): both Paladin and
Ranger confirmed `SupportState::Partial` / `EvidenceTier::Computed` by
direct read before starting.

Re-measured live file-footprint via `grep -rl "class\.<name>\." tests/
apps/desktop/src-tauri/src/` for both remaining classes: Paladin 39,
Ranger 42 — matching the prior cycle's own estimate exactly. Paladin is the
smaller, so chosen next per the established sequencing.

### cycle-2026-07-17T2200 | Paladin class-row promotion (class.paladin.hybrid_chassis_and_spell_burden) | commit `0cf9e77` | card t_7a1b6050 | cargo test root 3422/3422 green | desktop backend 75/75 green | clippy clean

Promoted the Paladin row `Partial/Computed` -> `Supported/ProductVisible`.
Condition 2 (every named grounded milestone) was already satisfied — SD-13/
SD-18 compute grounding for Paladin levels 1-20 (base attack/save
progression, smite evil, lay on hands, divine grace, mercy, channel
positive energy, the partial-caster effective-caster-level gate, Aura of
Justice/Faith/Righteousness, and the level-20 Holy Champion capstone) was
complete and unchanged this cycle. Condition 1 (live UI surface) was
already satisfied too — the Class Progression Catalog browser
(`apps/desktop/src/classCatalog/ClassCatalogScreen.tsx`, `list_class_catalog`
Tauri command) shipped and was live-verified by an earlier cycle (`9313e30`),
so this cycle needed no new browser work, only the matrix-and-test
promotion.

**File surface (53 files):**
- `src/rules_core/support_state_matrix.rs`: the Paladin row's
  `support_state`/`evidence_tier` flipped to `Supported`/`ProductVisible`;
  `blocker_or_lossiness_note` extended (not rewritten) to name the Class
  Progression Catalog browser as the UI-surfacing evidence; `next_required_uplift`
  rewritten to name the still-out-of-scope compute burdens (Divine Bond
  execution, Aura of Justice/Faith/Righteousness resolution, Holy Champion
  resolution, mercy EFFECT resolution, and the prepared-posture/
  spell-source-lineage spell burden) as future-SD-N scope rather than a
  live per-cycle target, mirroring the Sorcerer/Bard/etc. precedent's
  rewrite framing exactly.
- 27 Paladin-specific proof tests. Two distinct pre-existing shapes:
  - 19 files (`sd13_paladin_bonus_spells.rs`, `_level8/9/10_progression.rs`,
    `_mercies_two_and_three.rs`, `_spell_level_thresholds.rs`,
    `_spell_save_dcs.rs`, `_spells_per_day_counts.rs`,
    `_total_spells_per_day.rs`, `sd18_paladin_level11..20_widening.rs`)
    shared the identical uniform single-line `assert_eq!(paladin.support_state,
    SupportState::Partial); assert_eq!(paladin.evidence_tier,
    EvidenceTier::Computed);` pair (checked via a direct grep before
    editing), so a single bulk `perl -pi` substitution handled the whole
    set in one pass, mirroring the Rogue/Sorcerer/Bard cycles' own
    uniform-pair precedent.
  - 7 files (`sd13_paladin_base_attack_and_saves.rs`, `_level3_mercy.rs`,
    `_level4/5/6/7_progression.rs`, `_partial_caster_effective_caster_level.rs`)
    carried a different, previously-unseen shape: `assert_eq!(paladin.
    support_state, SupportState::Partial); assert_ne!(paladin.support_state,
    SupportState::Supported);` — the redundant `assert_ne!` would fail
    once promoted (Supported != Supported is false), so a second
    multiline `perl -0pi` pass collapsed both lines to a single
    `assert_eq!(paladin.support_state, SupportState::Supported);` across
    all 7 in one pass. One of the 7
    (`_partial_caster_effective_caster_level.rs`) also carried a stale
    inline comment ("does not promote the row past Partial") hand-edited
    to name the new Supported posture.
  - `tests/sd13_paladin_level1_chassis_and_spell_burden_separation.rs`: the
    Paladin row's own dedicated matrix-assertion test
    (`matrix_paladin_row_is_promoted_to_partial_with_honest_burden_note`,
    function name kept unchanged per the Fighter/Rogue/Sorcerer/Bard
    precedent of not renaming a test function on promotion) flipped to
    Supported/ProductVisible; this file's own closed-world exclusion test
    got a new `class.paladin.hybrid_chassis_and_spell_burden` exclusion
    line, hand-edited (not the bulk pass) to avoid double-insertion.
- 11 cross-class negative-control files whose own level-1/foundational
  tests pinned a "paladin stays Partial" assert (Barbarian, Bard, Cleric,
  Druid, Hybrid, Monk, Ranger (both
  `sd13_ranger_level1_chassis_and_class_feature_separation.rs` and
  `sd13_ranger_spell_burden_classification.rs`), Rogue, Sorcerer, Wizard) —
  found via `grep -rln 'paladin' across tests/` and cross-checked against
  actual assertion sites. Every file asserted Paladin standalone (Ranger,
  the only still-Partial sibling this cycle, was never asserted in the
  same loop as Paladin in any of these files, unlike the Sorcerer cycle's
  Cleric/Druid/Monk/Ranger loop-split), so every file got a direct
  standalone flip, no loop-split needed. `sd13_ranger_spell_burden_classification.rs`
  was the one file NOT surfaced by the bard-exclusion-anchor grep (it
  carries no closed-world exclusion list, only a direct standalone
  assertion) — missed by the initial file-surface plan and caught instead
  by a `cargo test` FAILED result before commit (see self-heal note
  below), then fixed the same way as the other 10. 3 of the 11 (Bard,
  Wizard, and Sorcerer's own paladin-preserving test) had their own
  dedicated test function renamed to drop a now-stale "paladin
  blocked/partial" claim from the name
  (`matrix_keeps_sorcerer_supported_and_paladin_blocked_computed_after_bard_slice`
  -> `matrix_keeps_sorcerer_and_paladin_supported_after_bard_slice` in
  Bard; `matrix_preserves_hybrid_paladin_blocked_computed_and_sorcerer_supported_truth`
  -> `matrix_preserves_hybrid_paladin_and_sorcerer_supported_truth` in
  Wizard; `matrix_preserves_paladin_hybrid_blocked_computed_truth` ->
  `matrix_preserves_paladin_hybrid_supported_product_visible_truth` in
  Sorcerer), mirroring the Rogue/Sorcerer/Bard cycles' own rename
  convention. Barbarian, Cleric, Druid, Hybrid, Monk, Ranger (both files),
  and Rogue needed no rename (their test function names carried no stale
  Paladin-specific claim).
- 25 closed-world "no unexpected Supported row" files (24
  sibling-preservation tests plus the master `tests/sd13_support_state_matrix.rs`):
  found via the generic exclusion-list anchor
  (`grep -rl 'r.row_id != "class.bard.progression_and_spell_burden"'`) per
  the Bard cycle's own handoff note. Applied the newline-safe `awk`
  insertion throughout, verified via a post-insertion `grep -c` that
  exactly one Paladin exclusion line landed in each of the 24 files (the
  25th, `sd13_paladin_level1_chassis_and_spell_burden_separation.rs`, got
  its own closed-world line via the dedicated edit above, not the bulk
  pass, to avoid double-insertion). The master
  `tests/sd13_support_state_matrix.rs` additionally needed a second, hand
  edit beyond the bulk exclusion-list line: its own dedicated
  `paladin_hybrid_row_is_partial_and_computed_with_named_burdens` test
  (native to this file since Paladin/Ranger's hybrid rows were introduced
  together with a master-file-level dedicated test, unlike Sorcerer/Bard
  which have no such master-file test) flipped to
  Supported/ProductVisible, dropping its `assert_ne!(hybrid.support_state,
  SupportState::Supported)` line (now tautologically false).

**One near-self-heal this cycle:** the initial file-surface plan (built
from the `class\.paladin\.` grep across `tests/` and `apps/desktop/src-tauri/src/`,
39 files) and the closed-world-anchor grep (25 files) together should have
covered every file needing an edit, but a full `cargo test --locked` run
caught one FAILED assertion in `tests/sd13_ranger_spell_burden_classification.rs`
(`matrix_ranger_row_is_not_misattributed_to_paladin_or_supported`, which
was in the original 39-file list but not cross-checked against the actual
edit plan before the first test run) — fixed the same way as the other
10 cross-class files, then `cargo test --locked` re-run clean. `cargo build`,
`cargo test --locked` (3422/3422 green), `cd apps/desktop/src-tauri && cargo
test` (75/75 green), and `cargo clippy --locked --tests -- -D warnings`
(clean) all passed after the fix.

Kanban card minting avoided the known `hermes kanban create
--initial-status done` CLI rejection (hit by every prior SD-19 cycle) by
not passing `--initial-status` at all — created with the default `ready`
status (`t_7a1b6050`), then `hermes kanban complete t_7a1b6050 --result
"..."`.

**Live verification:** not repeated this cycle — the Class Progression
Catalog browser's live-verification evidence (screenshot proof of the full
class list, a class filter, and search against real corpus-derived data)
was already captured by the browser-build cycle (`9313e30`) and is
unchanged; per the loop instruction's own Step 5 guidance ("at least once
per new browser, not necessarily every cycle after that"), re-verifying per
individual row promotion is not required.

**Full-matrix closure status: 30 of 34 rows now Supported/ProductVisible**
(9 spell schools + 4 equipment categories + 7 races + Fighter (2 rows) +
Monk (1 row) + Druid (1 row) + Barbarian (1 row) + Cleric (1 row) + Wizard
(1 row) + Rogue (1 row) + Sorcerer (1 row) + Bard (1 row) + Paladin (1
row)). Per `## Full-matrix closure`'s priority order, the next frontier is
the remaining 1 class row (Ranger, file-footprint 42 per the prior cycle's
own measurement, should be re-measured live by the next cycle) — or the
Human interaction-row judgment call (tackled last per the loop
instruction's own priority ordering, since it needs a written decision,
not a mechanical build). The permanently-excluded non-Human interaction row
remains untouched throughout, as required.

### cycle-log

cycle: 2026-07-17T2200
criterion touched: Class Progression row promotion, Paladin subset (class.paladin.hybrid_chassis_and_spell_burden)
row_or_kind: class.paladin.hybrid_chassis_and_spell_burden
commit: 0cf9e77
card: t_7a1b6050
verify: cargo test root 3422/3422 green; desktop backend 75/75 green; clippy clean (npm test/typecheck/build not re-run — matrix-and-test-only promotion cycle, no browser/frontend code touched, mirroring the Monk/Druid/Barbarian/Cleric/Wizard/Rogue/Sorcerer/Bard cycles' own scope)
status: GREEN

## Full-matrix closure: Class Progression row promotion, Ranger subset (2026-07-17, cycle 2026-07-17T2300)

Re-derived eligibility live before starting. `git fetch origin tranche/3` +
`git log origin/tranche/3 --oneline -5`: HEAD `0cf9e77` (full SHA
`0cf9e77c66353ac2e9b1c399f12a539054de2f1e`, Paladin promotion), matching
`snapshot_as_of`; `git status --porcelain` 0 lines; `git worktree list
--porcelain` showed only the primary worktree; branch `tranche/3`. In-flight
check (`ps -eo pid,etime,stat,cmd | grep -iE 'claude' | grep -v grep`) found
this session's own top-level `claude -p` process plus one unrelated
long-running interactive session (~13h57m uptime, no SD-19 criterion named in
its cmdline) — no in-flight collision. Also read the two required SD-18
investigation-cycle sections (`cycle-2026-07-15T0300` §3.4,
`cycle-2026-07-15T0400` §3.5 in `SD-18-core-rules-breadth-progress.md`, found
at `../SD-18/artifacts/`)
as read-only reference — no write to that file. Re-verified live against
`support_state_matrix.rs` directly (`grep -n "row_id:"` plus a direct
`support_state`/`evidence_tier` field read for `class.ranger.hybrid_chassis_and_spell_burden`):
confirmed `SupportState::Partial` / `EvidenceTier::Computed` by direct read
before starting — the last remaining `class.*` row per the Paladin cycle's
own handoff note.

Re-measured live file-footprint via `grep -rl "class\.ranger\." tests/
apps/desktop/src-tauri/src/`: 42 files — matching the prior cycle's own
estimate exactly.

### cycle-2026-07-17T2300 | Ranger class-row promotion (class.ranger.hybrid_chassis_and_spell_burden) | commit `c98c39c` | card t_706bc1a3 | cargo test root 3422/3422 green | desktop backend 75/75 green | clippy clean

Promoted the Ranger row `Partial/Computed` -> `Supported/ProductVisible`.
Condition 2 (every named grounded milestone) was already satisfied — SD-13/
SD-18 compute grounding for Ranger levels 1-20 (base attack/save progression,
Track, the favored-enemy flat surface, the combat-style choice-and-bonus-feat
recognition at its correct 2nd-level gate, Endurance, Favored Terrain,
Hunter's Bond, Woodland Stride, Swift Tracker, Quarry/Improved Quarry,
Camouflage, Hide in Plain Sight, Improved Evasion, and the level-20 Master
Hunter capstone) was complete and unchanged this cycle. Condition 1 (live UI
surface) was already satisfied too — the Class Progression Catalog browser
(`apps/desktop/src/classCatalog/ClassCatalogScreen.tsx`, `list_class_catalog`
Tauri command) shipped and was live-verified by an earlier cycle (`9313e30`),
so this cycle needed no new browser work, only the matrix-and-test promotion.

**File surface (56 files):** unlike the Fighter-through-Paladin cycles, the
initial file-surface grep (42 files, all literal matches on the exact
`class.ranger.hybrid_chassis_and_spell_burden` row_id string) undercounted
the true surface. A full `cargo test --locked --no-fail-fast` run against the
matrix-only edit surfaced 55 failing test binaries — 13 more than the grep's
42 — because several closed-world "no unexpected Supported row" files
(the 6 non-Human race-semantics files plus `sd13_fighter_bravery.rs` and
`sd13_fighter_level4` through `level9_level10_progression.rs`, 7 files) carry
an exclusion-list allowlist keyed by *other* rows' literal strings but had
never needed a Ranger-specific line before (Ranger was always Partial, so it
was never excluded from their "must not silently promote" check). This
cycle's process note for future cycles: **grep for the exact row_id string
undercounts the true file surface; a full `cargo test --no-fail-fast` run
against the matrix-only edit, before any test-file edits, is the reliable way
to discover every file needing a change** — this was done proactively this
cycle rather than caught as a late self-heal.

- `src/rules_core/support_state_matrix.rs`: the Ranger row's
  `support_state`/`evidence_tier` flipped to `Supported`/`ProductVisible`;
  `blocker_or_lossiness_note` extended (not rewritten) to name the Class
  Progression Catalog browser as the UI-surfacing evidence; `next_required_uplift`
  rewritten to name the still-out-of-scope compute burdens (Wisdom bonus
  spells, prepared-posture/spell-source-lineage spell burden, favored-terrain/
  favored-enemy conditional-application, the five combat-style bonus feats'
  own mechanics, Hunter's Bond ally-bonus/animal-companion subsystem, Woodland
  Stride/Swift Tracker/Quarry/Camouflage/Hide in Plain Sight/Evasion
  execution engines, and SD13-E4 ranger spell burden execution) as
  future-SD-N scope, mirroring the Paladin/Bard/etc. precedent's rewrite
  framing exactly.
- 31 Ranger-specific proof tests (21 `sd13_ranger_*.rs` + 10
  `sd18_ranger_*.rs`). Two distinct pre-existing shapes:
  - 26 files shared the identical uniform single-line `assert_eq!(ranger.support_state,
    SupportState::Partial); assert_eq!(ranger.evidence_tier, EvidenceTier::Computed);`
    pair, bulk-edited via `perl -pi` in one pass, mirroring the Rogue/Sorcerer/
    Bard/Paladin cycles' own uniform-pair precedent.
  - 2 files (`sd13_ranger_base_attack_and_saves.rs`,
    `sd13_ranger_favored_terrain_choice.rs`) carried the `assert_eq!(Partial);
    assert_ne!(Supported);` shape, collapsed via a `perl -0pi` multiline pass.
  - `tests/sd13_ranger_combat_style_level_gate.rs`: a differently-worded
    single assertion (`SupportState::Partial, "the combat-style level-gate
    correction does not promote the row past Partial"`), hand-edited.
  - `tests/sd13_ranger_level1_chassis_and_class_feature_separation.rs`: two
    functions hand-edited — the row's own dedicated promotion test (renamed
    `matrix_ranger_row_is_promoted_to_partial_and_names_remaining_pillars` ->
    `matrix_ranger_row_is_promoted_to_supported_and_names_remaining_pillars`,
    dropping the now-stale "partial" claim from the name, mirroring the
    Paladin/Bard/Wizard/Sorcerer rename precedent) and the file's own
    closed-world sibling-preservation test (`matrix_preserves_sibling_rows_after_ranger_promotion`),
    which needed a new `class.ranger.hybrid_chassis_and_spell_burden`
    exclusion line added to its own "no silent promotion" allowlist — Ranger's
    own row had never been excluded from this list before, since Ranger's own
    promotion is the intentional move this cycle makes.
  - `tests/sd13_ranger_spell_burden_classification.rs`: two functions
    hand-edited — `matrix_ranger_row_is_partial_computed_and_names_remaining_burdens`
    (assertions flipped to Supported/ProductVisible, function name kept since
    it does not itself embed a "partial" claim past the doc-comment) and
    `matrix_ranger_row_is_not_misattributed_to_paladin_or_supported` (renamed
    to `matrix_ranger_row_is_not_misattributed_to_paladin`, dropping the
    "_or_supported" clause since Ranger overshooting to Supported is now the
    correct, intentional outcome rather than a guarded-against regression;
    the redundant `assert_ne!(Supported)` replaced with the row's own
    `assert_eq!(Supported)` truth).
- 10 cross-class negative-control files whose own level-1/foundational tests
  pinned a "ranger stays Partial" assert (Barbarian, Bard, Cleric, Druid,
  Hybrid, Monk, Paladin, Rogue, Sorcerer, Wizard) — found via a direct grep
  for `class.ranger.hybrid_chassis_and_spell_burden` across `tests/`, cross-
  checked against the full-suite failure list. Every file asserted Ranger
  standalone (no still-Partial sibling existed to loop-split against, since
  Ranger was the last remaining Partial class row), so every file got a
  direct standalone flip via individual `Edit` calls (message text differed
  per file, so no single bulk-perl pattern covered all 8 of the
  barbarian/bard/cleric/druid/monk/rogue/sorcerer/wizard files). The Hybrid
  file additionally needed its own dedicated Ranger promotion test renamed
  (`matrix_ranger_row_is_partial_computed_and_names_remaining_burdens` ->
  `matrix_ranger_row_is_supported_and_names_remaining_burdens`) and its
  closed-world exclusion list extended. The Paladin file needed only its
  closed-world exclusion list extended (no dedicated Ranger-assertion
  function of its own). 8 of these 10 files (all but Hybrid and Paladin) also
  needed a second, separate edit: their own closed-world "no unexpected
  Supported row" exclusion list (a distinct test function from the
  Ranger-specific cross-class assertion) also lacked a Ranger exclusion line
  — found via the `class.paladin.hybrid_chassis_and_spell_burden` anchor grep
  per the Paladin cycle's own handoff note, then inserted via a line-numbered
  `awk` script (indentation captured per-file via `sed`, since indentation
  varied 12 vs 16 spaces across files) rather than the newline-safe multi-line
  `awk` idiom used by prior cycles, to sidestep having to hand-craft a
  distinct multi-line insertion pattern per file.
- 13 closed-world-only files (6 non-Human race-semantics files: Dwarf, Elf,
  Gnome, Half-Elf, Half-Orc, Halfling; 7 Fighter files: `sd13_fighter_bravery.rs`
  and `sd13_fighter_level4_progression.rs` through
  `sd13_fighter_level9_level10_progression.rs`) — each needed only the single
  `&& r.row_id != "class.ranger.hybrid_chassis_and_spell_burden"` exclusion
  line added to their own "must not promote any row to Supported or Lossy"
  allowlist, via the same line-numbered `awk` insertion. These 13 files were
  the ones this cycle's own process note (above) exists to flag: they do not
  contain the literal Ranger row_id string until this cycle adds it, so a
  grep-based file-surface plan misses them entirely; only a full test run
  surfaces them.
- `tests/sd13_support_state_matrix.rs` (master file): two hand edits beyond
  the bulk pattern — its own dedicated
  `ranger_hybrid_row_is_partial_and_computed_with_named_burdens` test
  (native to this file, mirroring Paladin/Ranger's master-file-level
  dedicated test) renamed to
  `ranger_hybrid_row_is_supported_and_product_visible_with_named_burdens` and
  flipped to Supported/ProductVisible, and the master `seed_contains_no_unexpectedly_supported_rows`
  closed-world test's exclusion list extended with the Ranger row_id line.

**Zero self-heals this cycle** in the sense of catching a missed file after
the fact — the file surface was fully and correctly discovered by running
`cargo test --locked --no-fail-fast` against the matrix-only edit *before*
starting any test-file edits (rather than discovering gaps via a post-edit
test run, as the Paladin cycle's own near-miss did for
`sd13_ranger_spell_burden_classification.rs`). `cargo build`, `cargo test
--locked` (3422/3422 green, first attempt after all 56 files edited), `cd
apps/desktop/src-tauri && cargo test` (75/75 green), and `cargo clippy
--locked --tests -- -D warnings` (clean) all passed clean.

Kanban card minting followed the Paladin cycle's own workaround for the known
`hermes kanban create --initial-status done` CLI rejection: created without
`--initial-status` (default `ready` status, `t_706bc1a3`), then `hermes
kanban complete t_706bc1a3 --result "..."`.

**Live verification:** not repeated this cycle — the Class Progression
Catalog browser's live-verification evidence (screenshot proof of the full
class list, a class filter, and search against real corpus-derived data) was
already captured by the browser-build cycle (`9313e30`) and is unchanged; per
the loop instruction's own Step 5 guidance ("at least once per new browser,
not necessarily every cycle after that"), re-verifying per individual row
promotion is not required.

**Full-matrix closure status: 31 of 34 rows now Supported/ProductVisible**
(9 spell schools + 4 equipment categories + 7 races + all 12 classes: Fighter
(2 rows) + Monk + Druid + Barbarian + Cleric + Wizard + Rogue + Sorcerer +
Bard + Paladin + Ranger (1 row each)). This closes the entire
`## Full-matrix closure` Catalog Browser priority list items 1-3 (Spell
Catalog Browser, Class Progression Browser, Race Trait Browser — all built
AND all their rows promoted). The sole remaining item is priority 4, the
**Human interaction-row judgment call**
(`interaction.human_bonus_feat_ability_bonus.pilot_pressure`) — tackled last
per the loop instruction's own priority ordering, since it needs a written
decision (option (a) or (b) per `## Full-matrix closure`), not a mechanical
build. The permanently-excluded non-Human interaction row remains untouched
throughout, as required. Once the Human interaction-row judgment call lands,
the loop reaches its terminal state: either 33/34 (option (a)) or 32/34
(option (b)) rows Supported/ProductVisible, with the loop's only remaining
work being to route around the one (or two) permanently-excluded row(s).

## Full-matrix closure: Human interaction-row judgment call (2026-07-17, cycle 2026-07-17T2400)

Re-derived eligibility live before starting. `git fetch origin tranche/3` +
`git log origin/tranche/3 --oneline -5`: HEAD `c98c39c` (full SHA
`c98c39c17477c290668bc85410eeeaeefad1eb3c`, Ranger promotion), matching
`snapshot_as_of`; `git status --porcelain` 0 lines; `git worktree list
--porcelain` showed only the primary worktree; branch `tranche/3`. In-flight
check (`ps -eo pid,etime,stat,cmd | grep -iE 'claude' | grep -v grep`) found
this session's own top-level `claude -p` process (cmdline matches this exact
loop invocation) plus one unrelated long-running interactive session and its
MCP server child — no in-flight collision on a specific SD-19 criterion. Also
read the two required SD-18 investigation-cycle sections
(`cycle-2026-07-15T0300` §3.4, `cycle-2026-07-15T0400` §3.5 in
`SD-18-core-rules-breadth-progress.md`, found at
`../SD-18/artifacts/`)
as read-only reference — no write to that file. Per the progress doc's own
prior handoff note, the sole remaining live-directive item was the Human
interaction-row judgment call
(`interaction.human_bonus_feat_ability_bonus.pilot_pressure`) — re-verified
live against `support_state_matrix.rs` directly before starting: confirmed
`SupportState::Partial` / `EvidenceTier::ProductVisible` by direct read.

**The judgment call.** Read the row's own `next_required_uplift`
("generalize the named Human pilot pressure into the interaction-row model
once a second computed interaction pressure exists") against the loop
instruction's own explicit framing of this decision (`## Full-matrix
closure`'s "The judgment call on the Human interaction row" section).
**Decision: option (a).** Reasoning recorded here per the instruction's
explicit requirement to write the choice and reasoning, not silently pick
one:

- The row's own documented trigger for generalization — "once a second
  computed interaction pressure exists" — names exactly one candidate per
  the non-Human interaction row's own warrant condition:
  `interaction.non_human_any_class.progression_pressure`. That row is
  permanently and explicitly excluded from this loop's target set. The
  uplift path is therefore not merely unfinished — it is structurally
  unreachable without inventing a non-Human interaction pressure, which the
  loop instruction explicitly forbids ("Do not invent a non-Human
  interaction pressure to satisfy it").
- Re-read this loop's own generic bar in `## What "supported /
  Product-visible" actually means for SD-19`: a row reaches
  Supported/Product-visible when (1) evidence_tier is Product-visible and
  (2) every named sample in the row's own `blocker_or_lossiness_note` is
  grounded as a real compute explanation. Checked both against the live
  row: (1) was already true — `evidence_tier: EvidenceTier::ProductVisible`
  was set by the SD18-PRELOOP cycle, grounded by
  `tests/sd18_preloop_consumer_compose.rs`. (2) was already true — the row's
  own note names exactly two samples (`human_bonus_feat -> feat:dodge`,
  `human_ability_bonus -> ability:strength`), both grounded as explicit
  compute explanations, not diagnostic strings.
- This is the identical bar every other Full-matrix-closure row was promoted
  on. Every class row (Fighter through Ranger) carries its own
  `next_required_uplift` naming real, currently out-of-scope compute
  burdens (spell slot math, spellbook engines, favored-terrain conditional
  application, etc.) — those uplifts were named as future-SD-N scope rather
  than treated as blockers to the row's own Supported promotion, because
  condition 2's bar is about the row's own *named* claim, not about closing
  every future uplift. Applying the same standard here is the consistent,
  non-arbitrary choice: this row's own named claim is exactly as fully
  grounded as any promoted class row's named claim.
- Option (b) (treat generalization as required, add a second permanent
  exception) was rejected because it would apply a *stricter* bar to this
  one interaction row than every other promoted row in the matrix received
  — the same "not yet generalized further" caveat exists on essentially
  every promoted row's own uplift note, and singling out the interaction row
  for a permanent-exception treatment when its own named claim is fully
  grounded would be inconsistent, not more honest.

**Implementation (matrix-and-test-only promotion, no new compute grounding,
mirroring the Fighter-through-Ranger class-row promotion cycles' own
shape):**

- `src/rules_core/support_state_matrix.rs`: the row's `support_state`
  flipped `Partial` -> `Supported` (evidence_tier unchanged, already
  `ProductVisible`); `blocker_or_lossiness_note` rewritten (not silently
  overwritten — reasoning preserved in an inline comment above the row) to
  state the row's own named claim is fully grounded and that "Supported"
  here means that, not that the interaction-row model itself has been
  generalized; `next_required_uplift` rewritten to name the permanent
  decoupling and its (currently unreachable) real trigger condition, mirroring
  the future-SD-N-scope framing used by every class row's own uplift note.
- A full `cargo test --locked --no-fail-fast` run against the matrix-only
  edit (done proactively before any test-file edits, per the Ranger cycle's
  own process note) surfaced 25 failing test binaries: 23 closed-world-only
  files (`sd13_barbarian_level1_chassis_baseline.rs`,
  `sd13_bard_level1_spell_baseline.rs`, `sd13_cleric_level1_spell_baseline.rs`,
  `sd13_druid_level1_spell_baseline.rs`, `sd13_dwarf_bounded_race_semantics.rs`,
  `sd13_elf_race_semantics_recognition.rs`, `sd13_fighter_bravery.rs`,
  `sd13_fighter_level4_progression.rs` through
  `sd13_fighter_level9_level10_progression.rs` (6 files),
  `sd13_gnome_race_semantics_recognition.rs`,
  `sd13_half_elf_race_semantics_recognition.rs`,
  `sd13_half_orc_race_semantics_recognition.rs`,
  `sd13_halfling_race_semantics_recognition.rs`,
  `sd13_hybrid_level1_chassis_baseline.rs`,
  `sd13_monk_level1_chassis_baseline.rs`,
  `sd13_ranger_level1_chassis_and_class_feature_separation.rs`,
  `sd13_rogue_level1_chassis_baseline.rs`,
  `sd13_sorcerer_level1_spell_baseline.rs`,
  `sd13_wizard_level1_prepared_spell_baseline.rs`), each needing exactly one
  new `&& r.row_id != "interaction.human_bonus_feat_ability_bonus.pilot_pressure"`
  exclusion line in their own "no unexpected Supported row" allowlist —
  applied via a single `sed` insert-after-match pass keyed on the identical,
  most-recently-added `class.ranger.hybrid_chassis_and_spell_burden`
  exclusion line, since every one of these files' allowlists already carried
  that line at 16-space indentation. 1 file
  (`sd13_paladin_level1_chassis_and_spell_burden_separation.rs`) had a
  differently-shaped allowlist (12-space indentation, closing `),` on the
  same line as the last exclusion) and was hand-edited via `Edit` to insert
  the new exclusion line before the closing paren rather than after it. The
  master `tests/sd13_support_state_matrix.rs` needed its own dedicated
  `human_interaction_row_is_partial_and_product_visible` test renamed to
  `human_interaction_row_is_supported_and_product_visible` (assertion
  flipped `Partial` -> `Supported`) plus the same sed-driven closed-world
  exclusion-list edit as the other 23 files.

**Zero self-heals** in the after-the-fact sense — the full 25-file surface
was discovered proactively via `cargo test --no-fail-fast` before any
test-file edits (mirroring the Ranger cycle's own now-standard process),
rather than caught via a post-edit failure. `cargo test --locked`
(3422/3422 green — identical total to the Ranger cycle, as expected for a
matrix-only promotion that adds no new tests), `cd apps/desktop/src-tauri &&
cargo test` (75/75 green, desktop backend untouched by this cycle), and
`cargo clippy --locked --tests -- -D warnings` run from the repo root (clean)
all passed. (Note: an initial clippy invocation accidentally ran from within
`apps/desktop/src-tauri` due to working-directory carryover from the prior
desktop-test command, surfacing 6 pre-existing, unrelated clippy findings in
`apps/desktop/src-tauri/src/update/transaction.rs` — confirmed unrelated to
this cycle's file surface and not touched by this cycle; the correct
root-crate clippy run, which is what Step 5 actually gates on for a
matrix-and-test-only cycle, is clean.)

**Live verification:** not applicable — this cycle adds no new UI surface;
the row's Product-visible evidence tier was already established by the
SD18-PRELOOP cycle's own live-verification evidence
(`tests/sd18_preloop_consumer_compose.rs`), unchanged by this cycle.

**Full-matrix closure status: 33 of 34 rows now Supported/ProductVisible**
(9 spell schools + 4 equipment categories + 7 races + 12 classes + the Human
interaction row). This is the loop's terminal criterion per
`## Full-matrix closure`'s own priority list (item 4, tackled last). The
sole remaining row, `interaction.non_human_any_class.progression_pressure`,
stays permanently `Unverified/Observed` by explicit, standing exclusion —
not an open gap requiring a future cycle. **No further Full-matrix-closure
criteria are eligible for a future cycle** unless a future SD ingests a
sourcebook that grounds a genuine non-Human race x class compute pressure
(per this row's own `next_required_uplift`), at which point a new,
non-excluded interaction row (not this permanently-excluded one) could be
added to a future matrix version.

## Full-matrix closure: terminal-state re-confirmation (2026-07-17, cycle 2026-07-17T0221)

Re-derived eligibility live rather than trusting the prior cycle's own
closing summary, per the loop instruction's Step 1/§5 requirements:

- `git fetch origin tranche/3` + `git log origin/tranche/3 --oneline -20`
  confirm HEAD is still `509b7be` (the Human interaction-row promotion
  commit) — no drift since the prior two cycles. `git status --porcelain`
  returns 0 (clean tree); `git branch --show-current` is `tranche/3`;
  `git worktree list --porcelain` shows only the single primary worktree,
  no stray parallel worktree.
- In-flight check (`ps -eo pid,etime,stat,cmd | grep -iE 'claude'`) found
  this cycle's own `claude -p` process (running the identical SD-19 loop
  prompt — that is this session itself, not a competing claim), plus one
  unrelated long-running interactive session (pid 2195929, matching the
  same PID every prior SD-18/SD-19 cycle has independently noted as
  unrelated) and its Honcho MCP server child. No second competing SD-19
  cycle process naming a specific acceptance criterion — no Hard-stop-#3
  collision.
- Direct grep of `src/rules_core/support_state_matrix.rs` for all 34
  `row_id:` occurrences (36 raw matches; 2 are the struct-field and
  method-signature declarations, not data rows — 34 real rows), then a
  per-row extraction of each row's own `support_state`/`evidence_tier`
  pair (not a prior cycle's summary): all 7 race rows, all 12 class rows
  (including the Fighter split into `level_1_pilot`/`levels_2_10`), all 9
  school rows, all 4 equipment rows, and the Human interaction row
  (`interaction.human_bonus_feat_ability_bonus.pilot_pressure`) each read
  `support_state: Supported` / `evidence_tier: ProductVisible` (33 rows).
  `interaction.non_human_any_class.progression_pressure` reads
  `support_state: Unverified` / `evidence_tier: Observed`, exactly as the
  permanent exclusion requires. This independently corroborates the prior
  two cycles' closing summaries byte-for-byte — no drift, no doc/matrix
  disagreement.
- Read the two required SD-18 investigation-cycle sections
  (`cycle-2026-07-15T0300` §3.4 spell-school and `cycle-2026-07-15T0400`
  §3.5 equipment-category reachability-chain investigations, at lines
  2796 and 2816 of
  `programs/codex/requirements/SD-18-core-rules-breadth/artifacts/SD-18-core-rules-breadth-progress.md`)
  as read-only reference, per this bundle's inherited requirement. Both
  document the historical structural gap (`pilot_compute.rs` had zero
  corpus-aware compute path for spells or equipment, `CharacterInput` had
  no spell-content-selection mechanism) that SD-19's foundation and
  capability slices went on to close. Neither contradicts the live state
  confirmed above. No write made to that file.

Checked Step 1's priority list against the live matrix: items 1-2
(§2.4/§2.5) were already `Done` at scope-doc ship; items 3-5 (Spell/
Class/Race Catalog Browsers, plus their row promotions) are fully landed
per the row greps above; item 6 (Human interaction-row judgment call) was
resolved at cycle-2026-07-17T2400. The one row Step 1 explicitly forbids
ever picking (`interaction.non_human_any_class.progression_pressure`)
correctly remains untouched at `Unverified/Observed`. No criterion
satisfies eligibility rule 1 (has not yet reached `Supported/Product-
visible`) other than the permanently-excluded row, which eligibility rule
1 itself carves out. **No loop-eligible work exists this cycle.** This is
the expected terminal state per the loop instruction's own "How the loop
will end" section (33 of 34 rows `Supported/Product-visible`, one row
permanently and explicitly excluded) — not a blocker requiring operator
intervention, and not a hard-stop condition. No code touched, no test
run, no commit, no kanban card (nothing eligible to attach one to). The
standing supervisor will re-run this same live check on its next
back-to-back cycle; it will keep finding the same terminal state until a
future SD ingests a non-CRB sourcebook that grounds a genuine non-Human
race×class compute pressure (per the excluded row's own
`next_required_uplift`), at which point a new, non-excluded interaction
row could reopen loop-eligible work.

### cycle-log

cycle: 2026-07-17T0221
criterion touched: none (terminal-state re-confirmation — no loop-eligible criterion exists)
row_or_kind: n/a (all 33 targeted rows already Supported/ProductVisible; interaction.non_human_any_class.progression_pressure permanently excluded)
commit: no commit: nothing eligible; live matrix re-derivation confirmed the loop's terminal state (33/34 rows Supported/ProductVisible) with no drift since cycle-2026-07-17T0218/T2400
card: no card: no eligible work attempted, nothing to attach a post-mortem record to
verify: cargo test not run (no code change attempted); live grep of support_state_matrix.rs (34/34 rows) independently confirms terminal state
status: NO-OP

## Full-matrix closure: terminal-state re-confirmation (2026-07-17, cycle 2026-07-17T0218)

Re-derived eligibility live rather than trusting the prior cycle's own
closing summary, per the loop instruction's Step 1/§5 requirements:

- `git fetch origin tranche/3` + `git log origin/tranche/3 --oneline -10`
  confirm HEAD is still `509b7be` (the Human interaction-row promotion
  commit) — no drift since the prior cycle. `git status --porcelain`
  returns 0 (clean tree); `git worktree list --porcelain` shows only the
  single primary worktree, no stray parallel worktree.
- In-flight check (`ps -eo pid,etime,stat,cmd | grep -iE 'claude'`) finds
  exactly one `claude -p` process, this cycle's own (pid 4012880, ppid
  3387203 — the standing supervisor), plus one unrelated long-running
  interactive session (pid 2195929, matching the same PID the SD-18
  investigation cycles independently noted as unrelated) and its Honcho
  MCP server child. No second competing SD-19 cycle process — no
  Hard-stop-#3 collision.
- Direct grep of `src/rules_core/support_state_matrix.rs` for every
  `row_id:`/`support_state:`/`evidence_tier:` triple (34 rows total,
  independent of any doc summary) confirms: all 7 race rows, all 12
  class rows, all 9 school rows, all 4 equipment rows, and the Human
  interaction row read `Supported` / `ProductVisible` (33 rows); the
  `interaction.non_human_any_class.progression_pressure` row reads
  `Unverified` / `Observed`, exactly as the permanent exclusion requires.
  This independently corroborates the prior cycle's own closing summary
  byte-for-byte — no drift, no doc/matrix disagreement.
- Read the two required SD-18 investigation-cycle sections
  (`cycle-2026-07-15T0300` §3.4 spell-school and `cycle-2026-07-15T0400`
  §3.5 equipment-category reachability-chain investigations, in
  `programs/codex/requirements/SD-18-core-rules-breadth/artifacts/SD-18-core-rules-breadth-progress.md`)
  as read-only reference, per this bundle's inherited requirement. Both
  document the historical structural gap (`pilot_compute.rs` had zero
  corpus-aware compute path) that SD-19's foundation/capability slices
  went on to close; neither contradicts the live state confirmed above.
  No write made to that file.

Checked Step 1's priority list against the live matrix: items 1-2
(§2.4/§2.5) were already `Done` at scope-doc ship; items 3-5 (Spell/
Class/Race Catalog Browsers, plus their row promotions) are fully landed
per the row greps above; item 6 (Human interaction-row judgment call) was
resolved last cycle. The one row Step 1 explicitly forbids ever picking
(`interaction.non_human_any_class.progression_pressure`) correctly remains
untouched at `Unverified/Observed`. No criterion satisfies eligibility
rule 1 (has not yet reached `Supported/Product-visible`) other than the
permanently-excluded row, which eligibility rule 1 itself carves out. **No
loop-eligible work exists this cycle.** This is the expected terminal
state per the loop instruction's own "How the loop will end" section (33
of 34 rows `Supported/Product-visible`, one row permanently and
explicitly excluded) — not a blocker requiring operator intervention, and
not a hard-stop condition. No code touched, no test run, no commit, no
kanban card (nothing eligible to attach one to). The standing supervisor
will re-run this same live check on its next back-to-back cycle; it will
keep finding the same terminal state until a future SD ingests a
non-CRB sourcebook that grounds a genuine non-Human race×class compute
pressure (per the excluded row's own `next_required_uplift`), at which
point a new, non-excluded interaction row could reopen loop-eligible
work.

### cycle-log

cycle: 2026-07-17T0218
criterion touched: none (terminal-state re-confirmation — no loop-eligible criterion exists)
row_or_kind: n/a (all 33 targeted rows already Supported/ProductVisible; interaction.non_human_any_class.progression_pressure permanently excluded)
commit: no commit: nothing eligible; live matrix re-derivation confirmed the loop's terminal state (33/34 rows Supported/ProductVisible) with no drift since cycle-2026-07-17T2400
card: no card: no eligible work attempted, nothing to attach a post-mortem record to
verify: cargo test not run (no code change attempted); live grep of support_state_matrix.rs (34/34 rows) independently confirms terminal state
status: NO-OP

cycle: 2026-07-17T2400
criterion touched: Human interaction-row judgment call (interaction.human_bonus_feat_ability_bonus.pilot_pressure)
row_or_kind: interaction.human_bonus_feat_ability_bonus.pilot_pressure
commit: 509b7be
card: t_0b7dbf68
verify: cargo test root 3422/3422 green; desktop backend 75/75 green; root clippy clean (npm test/typecheck/build not re-run — matrix-and-test-only promotion cycle, no browser/frontend code touched)
status: GREEN

cycle: 2026-07-17T2300
criterion touched: Class Progression row promotion, Ranger subset (class.ranger.hybrid_chassis_and_spell_burden)
row_or_kind: class.ranger.hybrid_chassis_and_spell_burden
commit: c98c39c
card: t_706bc1a3
verify: cargo test root 3422/3422 green; desktop backend 75/75 green; clippy clean (npm test/typecheck/build not re-run — matrix-and-test-only promotion cycle, no browser/frontend code touched, mirroring the Monk/Druid/Barbarian/Cleric/Wizard/Rogue/Sorcerer/Bard/Paladin cycles' own scope)
status: GREEN

## Full-matrix closure: terminal-state re-confirmation (2026-07-17, cycle 2026-07-17T0223)

Re-derived eligibility live rather than trusting the prior two cycles'
own closing summaries, per the loop instruction's Step 1/§5 requirements:

- `git fetch origin tranche/3` + `git log origin/tranche/3 --oneline -10`
  confirm HEAD is still `509b7be` (the Human interaction-row promotion
  commit) — no drift since the prior three cycles. `git status --porcelain`
  returns 0 (clean tree); `git branch --show-current` is `tranche/3`;
  `git rev-parse HEAD` and `git rev-parse origin/tranche/3` are identical;
  `git worktree list --porcelain` shows only the single primary worktree,
  no stray parallel worktree.
- In-flight check (`ps -eo pid,etime,stat,cmd | grep -iE 'claude'`) found
  this cycle's own `claude -p` process (pid 4015666, running the identical
  SD-19 loop prompt — that is this session itself, not a competing claim),
  plus one unrelated long-running interactive session (pid 2195929,
  matching the same PID every prior SD-18/SD-19 cycle has independently
  noted as unrelated) and its Honcho MCP server child. No second
  competing SD-19 cycle process naming a specific acceptance criterion —
  no Hard-stop-#3 collision.
- Direct extraction (via a Python pass over `support_state_matrix.rs`,
  not a prior cycle's summary) of all 34 `row_id:`/`support_state:`/
  `evidence_tier:` triples: all 7 race rows, all 12 class rows (including
  the Fighter split into `level_1_pilot`/`levels_2_10`), all 9 school
  rows, all 4 equipment rows, and the Human interaction row
  (`interaction.human_bonus_feat_ability_bonus.pilot_pressure`) each read
  `support_state: Supported` / `evidence_tier: ProductVisible` (33 rows,
  independently re-verified with a targeted `sed` extraction per row
  after the first pass's naive fixed-window regex under-matched several
  rows with long inline comment blocks — e.g. the Human interaction row's
  own judgment-call reasoning comment — and returned false `None`s that a
  wider-window re-check resolved).
  `interaction.non_human_any_class.progression_pressure` reads
  `support_state: Unverified` / `evidence_tier: Observed`, exactly as the
  permanent exclusion requires. This independently corroborates the prior
  three cycles' closing summaries byte-for-byte — no drift, no
  doc/matrix disagreement.
- Read the two required SD-18 investigation-cycle sections (`## 3.4`
  spell-school and `## 3.5` equipment-category reachability-chain
  investigations, at lines 2796 and 2816 of
  `programs/codex/requirements/SD-18-core-rules-breadth/artifacts/SD-18-core-rules-breadth-progress.md`,
  cycle-dated `cycle-2026-07-15T0300` and `cycle-2026-07-15T0400`
  respectively) as read-only reference, per this bundle's inherited
  requirement. Both document the historical structural gap
  (`pilot_compute.rs` had zero corpus-aware compute path for spells or
  equipment, `CharacterInput` had no spell-content-selection mechanism)
  that SD-19's foundation and capability slices went on to close. Neither
  contradicts the live state confirmed above. No write made to that file.

Checked Step 1's priority list against the live matrix: items 1-2
(§2.4/§2.5) were already `Done` at scope-doc ship; items 3-5 (Spell/
Class/Race Catalog Browsers, plus their row promotions) are fully landed
per the row greps above; item 6 (Human interaction-row judgment call) was
resolved at cycle-2026-07-17T2400. The one row Step 1 explicitly forbids
ever picking (`interaction.non_human_any_class.progression_pressure`)
correctly remains untouched at `Unverified/Observed`. No criterion
satisfies eligibility rule 1 (has not yet reached `Supported/Product-
visible`) other than the permanently-excluded row, which eligibility rule
1 itself carves out. **No loop-eligible work exists this cycle.** This is
the expected terminal state per the loop instruction's own "How the loop
will end" section (33 of 34 rows `Supported/Product-visible`, one row
permanently and explicitly excluded) — not a blocker requiring operator
intervention, and not a hard-stop condition. No code touched, no test
run, no commit, no kanban card (nothing eligible to attach one to). The
standing supervisor will re-run this same live check on its next
back-to-back cycle; it will keep finding the same terminal state until a
future SD ingests a non-CRB sourcebook that grounds a genuine non-Human
race×class compute pressure (per the excluded row's own
`next_required_uplift`), at which point a new, non-excluded interaction
row could reopen loop-eligible work.

### cycle-log

cycle: 2026-07-17T0223
criterion touched: none (terminal-state re-confirmation — no loop-eligible criterion exists)
row_or_kind: n/a (all 33 targeted rows already Supported/ProductVisible; interaction.non_human_any_class.progression_pressure permanently excluded)
commit: no commit: nothing eligible; live matrix re-derivation confirmed the loop's terminal state (33/34 rows Supported/ProductVisible) with no drift since cycle-2026-07-17T0221/T0218/T2400
card: no card: no eligible work attempted, nothing to attach a post-mortem record to
verify: cargo test not run (no code change attempted); live grep of support_state_matrix.rs (34/34 rows) independently confirms terminal state
status: NO-OP

## Full-matrix closure: terminal-state re-confirmation (2026-07-17, cycle 2026-07-17T0226)

Re-derived eligibility live rather than trusting the prior four cycles'
own closing summaries, per the loop instruction's Step 1/§5 requirements:

- `git fetch origin tranche/3` + `git log origin/tranche/3 --oneline -10`
  confirm HEAD is still `509b7be` (the Human interaction-row promotion
  commit) — no drift since the prior four cycles. `git status --porcelain`
  returns 0 (clean tree); `git branch --show-current` is `tranche/3`;
  `git worktree list --porcelain` shows only the single primary worktree,
  no stray parallel worktree.
- In-flight check (`ps -eo pid,etime,stat,cmd | grep -iE 'claude'`) found
  this cycle's own `claude -p` process (pid 4016815, running the identical
  SD-19 loop prompt — this session itself, not a competing claim), plus
  one unrelated long-running interactive session (pid 2195929, matching
  the same PID every prior SD-18/SD-19 cycle has independently noted as
  unrelated) and its Honcho MCP server child (pid 2195992/4016836). No
  second competing SD-19 cycle process naming a specific acceptance
  criterion — no Hard-stop-#3 collision.
- Direct extraction (via a Python pass over `support_state_matrix.rs`,
  not a prior cycle's summary) of all 34 `row_id:`/`support_state:`/
  `evidence_tier:` triples: all 7 race rows, all 12 class rows (including
  the Fighter split into `level_1_pilot`/`levels_2_10`), all 9 school
  rows, all 4 equipment rows, and the Human interaction row
  (`interaction.human_bonus_feat_ability_bonus.pilot_pressure`) each read
  `support_state: Supported` / `evidence_tier: ProductVisible` (33 rows).
  `interaction.non_human_any_class.progression_pressure` reads
  `support_state: Unverified` / `evidence_tier: Observed`, exactly as the
  permanent exclusion requires. This independently corroborates the prior
  four cycles' closing summaries byte-for-byte — no drift, no
  doc/matrix disagreement.
- Read the two required SD-18 investigation-cycle sections
  (`cycle-2026-07-15T0300` §3.4 spell-school and `cycle-2026-07-15T0400`
  §3.5 equipment-category reachability-chain investigations, at lines
  7721 and 7927 of
  `programs/codex/requirements/SD-18-core-rules-breadth/artifacts/SD-18-core-rules-breadth-progress.md`)
  as read-only reference, per this bundle's inherited requirement. Both
  document the historical structural gap (`pilot_compute.rs` had zero
  corpus-aware compute path for spells or equipment, `CharacterInput` had
  no spell-content-selection mechanism) that SD-19's foundation and
  capability slices went on to close. Neither contradicts the live state
  confirmed above. No write made to that file.

Checked Step 1's priority list against the live matrix: items 1-2
(§2.4/§2.5) were already `Done` at scope-doc ship; items 3-5 (Spell/
Class/Race Catalog Browsers, plus their row promotions) are fully landed
per the row greps above; item 6 (Human interaction-row judgment call) was
resolved at cycle-2026-07-17T2400. The one row Step 1 explicitly forbids
ever picking (`interaction.non_human_any_class.progression_pressure`)
correctly remains untouched at `Unverified/Observed`. No criterion
satisfies eligibility rule 1 (has not yet reached `Supported/Product-
visible`) other than the permanently-excluded row, which eligibility rule
1 itself carves out. **No loop-eligible work exists this cycle.** This is
the expected terminal state per the loop instruction's own "How the loop
will end" section (33 of 34 rows `Supported/Product-visible`, one row
permanently and explicitly excluded) — not a blocker requiring operator
intervention, and not a hard-stop condition. No code touched, no test
run, no commit, no kanban card (nothing eligible to attach one to). The
standing supervisor will re-run this same live check on its next
back-to-back cycle; it will keep finding the same terminal state until a
future SD ingests a non-CRB sourcebook that grounds a genuine non-Human
race×class compute pressure (per the excluded row's own
`next_required_uplift`), at which point a new, non-excluded interaction
row could reopen loop-eligible work.

### cycle-log

cycle: 2026-07-17T0226
criterion touched: none (terminal-state re-confirmation — no loop-eligible criterion exists)
row_or_kind: n/a (all 33 targeted rows already Supported/ProductVisible; interaction.non_human_any_class.progression_pressure permanently excluded)
commit: no commit: nothing eligible; live matrix re-derivation confirmed the loop's terminal state (33/34 rows Supported/ProductVisible) with no drift since cycle-2026-07-17T0223/T0221/T0218/T2400
card: no card: no eligible work attempted, nothing to attach a post-mortem record to
verify: cargo test not run (no code change attempted); live grep of support_state_matrix.rs (34/34 rows) independently confirms terminal state
status: NO-OP

## Full-matrix closure: terminal-state re-confirmation (2026-07-17, cycle 2026-07-17T0229)

Re-derived eligibility live rather than trusting any prior cycle's own
closing summary, per the loop instruction's Step 1/§5 requirements:

- `git fetch origin tranche/3` + `git log origin/tranche/3 --oneline -10`
  confirm HEAD is still `509b7be` (the Human interaction-row promotion
  commit) — no drift since the prior five cycles. `git status --porcelain`
  returns 0 (clean tree); `git branch --show-current` is `tranche/3`.
- In-flight check (`ps -eo pid,etime,stat,cmd | grep -iE 'claude'`) found
  this cycle's own `claude -p` process (pid 4018647, running the identical
  SD-19 loop prompt — this session itself, not a competing claim), plus
  one unrelated long-running interactive session (pid 2195929, matching
  the same PID every prior SD-18/SD-19 cycle has independently noted as
  unrelated) and its Honcho MCP server child. No second competing SD-19
  cycle process naming a specific acceptance criterion — no Hard-stop-#3
  collision.
- Direct extraction (`grep -n "row_id:\|support_state:\|evidence_tier:"
  src/rules_core/support_state_matrix.rs`) of all 34 `row_id:`/
  `support_state:`/`evidence_tier:` triples: all 7 race rows, all 12
  class rows (including the Fighter split into `level_1_pilot`/
  `levels_2_10`), all 9 school rows, all 4 equipment rows, and the Human
  interaction row (`interaction.human_bonus_feat_ability_bonus.pilot_pressure`)
  each read `support_state: Supported` / `evidence_tier: ProductVisible`
  (33 rows). `interaction.non_human_any_class.progression_pressure` reads
  `support_state: Unverified` / `evidence_tier: Observed`, exactly as the
  permanent exclusion requires. This independently corroborates the prior
  five cycles' closing summaries byte-for-byte — no drift, no
  doc/matrix disagreement.
- Read the two required SD-18 investigation-cycle sections (`## 3.4`
  spell-school and `## 3.5` equipment-category reachability-chain
  investigations, at lines 2796 and 2816 of
  `programs/codex/requirements/SD-18-core-rules-breadth/artifacts/SD-18-core-rules-breadth-progress.md`,
  cycle-dated `cycle-2026-07-15T0300` and `cycle-2026-07-15T0400`
  respectively) as read-only reference, per this bundle's inherited
  requirement. Both document the historical structural gap
  (`pilot_compute.rs` had zero corpus-aware compute path for spells or
  equipment, `CharacterInput` had no spell-content-selection mechanism)
  that SD-19's foundation and capability slices went on to close. Neither
  contradicts the live state confirmed above. No write made to that file.

Checked Step 1's priority list against the live matrix: items 1-2
(§2.4/§2.5) were already `Done` at scope-doc ship; items 3-5 (Spell/
Class/Race Catalog Browsers, plus their row promotions) are fully landed
per the row greps above; item 6 (Human interaction-row judgment call) was
resolved at cycle-2026-07-17T2400. The one row Step 1 explicitly forbids
ever picking (`interaction.non_human_any_class.progression_pressure`)
correctly remains untouched at `Unverified/Observed`. No criterion
satisfies eligibility rule 1 (has not yet reached `Supported/Product-
visible`) other than the permanently-excluded row, which eligibility rule
1 itself carves out. **No loop-eligible work exists this cycle.** This is
the expected terminal state per the loop instruction's own "How the loop
will end" section (33 of 34 rows `Supported/Product-visible`, one row
permanently and explicitly excluded) — not a blocker requiring operator
intervention, and not a hard-stop condition. No code touched, no test
run, no commit, no kanban card (nothing eligible to attach one to). This
was executed as a single serial cycle per the operator's explicit
instruction not to fan out into parallel worktree agents or a batch/fleet
of workers — trivially satisfied here since there was no code-bearing
work to parallelize in the first place. The next invocation of this loop
(scheduled or supervisor-driven) will re-run this same live check; it
will keep finding the same terminal state until a future SD ingests a
non-CRB sourcebook that grounds a genuine non-Human race×class compute
pressure (per the excluded row's own `next_required_uplift`), at which
point a new, non-excluded interaction row could reopen loop-eligible
work.

### cycle-log

cycle: 2026-07-17T0229
criterion touched: none (terminal-state re-confirmation — no loop-eligible criterion exists)
row_or_kind: n/a (all 33 targeted rows already Supported/ProductVisible; interaction.non_human_any_class.progression_pressure permanently excluded)
commit: no commit: nothing eligible; live matrix re-derivation confirmed the loop's terminal state (33/34 rows Supported/ProductVisible) with no drift since cycle-2026-07-17T0226/T0223/T0221/T0218/T2400
card: no card: no eligible work attempted, nothing to attach a post-mortem record to
verify: cargo test not run (no code change attempted); live grep of support_state_matrix.rs (34/34 rows) independently confirms terminal state
status: NO-OP

## Full-matrix closure: terminal-state re-confirmation (2026-07-17, cycle-2026-07-17T0232)

Re-derived eligibility live rather than trusting any prior cycle's own
closing summary, per the loop instruction's Step 1/§5 requirements:

- `git fetch origin tranche/3` + `git log origin/tranche/3 --oneline -5`
  confirm HEAD is still `509b7be` (the Human interaction-row promotion
  commit) — no drift since the prior six cycles. `git status --porcelain`
  returns 0 (clean tree); `git branch --show-current` is `tranche/3`;
  `git worktree list --porcelain` shows only the single primary worktree.
- In-flight check (`ps -eo pid,etime,stat,cmd | grep -iE 'claude'`) found
  this cycle's own `claude -p` process (pid 4020038, running the identical
  SD-19 loop prompt — this session itself, not a competing claim), plus
  one unrelated long-running interactive session (pid 2195929, the same
  PID every prior SD-18/SD-19 cycle has independently noted as unrelated)
  and its Honcho MCP server child. No second competing SD-19 cycle process
  naming a specific acceptance criterion — no Hard-stop-#3 collision.
- Direct extraction (`grep -n "row_id:\|support_state:\|evidence_tier:"
  src/rules_core/support_state_matrix.rs`, parsed independently rather
  than trusting the doc) of all 34 `row_id:`/`support_state:`/
  `evidence_tier:` triples: all 7 race rows, all 12 class rows (Fighter
  split into `level_1_pilot`/`levels_2_10`), all 9 school rows, all 4
  equipment rows, and the Human interaction row
  (`interaction.human_bonus_feat_ability_bonus.pilot_pressure`) each read
  `support_state: Supported` / `evidence_tier: ProductVisible` (33 rows).
  `interaction.non_human_any_class.progression_pressure` reads
  `support_state: Unverified` / `evidence_tier: Observed`, exactly as the
  permanent exclusion requires. This independently corroborates every
  prior cycle's closing summary byte-for-byte — no drift, no doc/matrix
  disagreement.
- Read the two required SD-18 investigation-cycle sections
  (`cycle-2026-07-15T0300` §3.4 spell-school and `cycle-2026-07-15T0400`
  §3.5 equipment-category reachability-chain investigations, at lines
  7721 and 7927 of
  `programs/codex/requirements/SD-18-core-rules-breadth/artifacts/SD-18-core-rules-breadth-progress.md`)
  as read-only reference, per this bundle's inherited requirement. Both
  document the historical structural gap (`pilot_compute.rs` had zero
  corpus-aware compute path for spells or equipment, `CharacterInput` had
  no spell-content-selection mechanism) that SD-19's foundation and
  capability slices went on to close. Neither contradicts the live state
  confirmed above. No write made to that file.

Checked Step 1's priority list against the live matrix: items 1-2
(§2.4/§2.5) were already `Done` at scope-doc ship; items 3-5 (Spell/
Class/Race Catalog Browsers, plus their row promotions) are fully landed
per the row greps above; item 6 (Human interaction-row judgment call) was
resolved at cycle-2026-07-17T2400. The one row Step 1 explicitly forbids
ever picking (`interaction.non_human_any_class.progression_pressure`)
correctly remains untouched at `Unverified/Observed`. No criterion
satisfies eligibility rule 1 (has not yet reached `Supported/Product-
visible`) other than the permanently-excluded row, which eligibility rule
1 itself carves out. **No loop-eligible work exists this cycle.** This is
the expected terminal state per the loop instruction's own "How the loop
will end" section (33 of 34 rows `Supported/Product-visible`, one row
permanently and explicitly excluded) — not a blocker requiring operator
intervention, and not a hard-stop condition. No code touched, no test
run, no commit, no kanban card (nothing eligible to attach one to). This
cycle ran as a single serial procedure per the operator's explicit
instruction not to fan out into parallel worktree agents or a batch/fleet
of workers — trivially satisfied here since there was no code-bearing
work to parallelize. The next invocation of this loop will re-run this
same live check; it will keep finding the same terminal state until a
future SD ingests a non-CRB sourcebook that grounds a genuine non-Human
race×class compute pressure (per the excluded row's own
`next_required_uplift`), at which point a new, non-excluded interaction
row could reopen loop-eligible work.

### cycle-log

cycle: 2026-07-17T0232
criterion touched: none (terminal-state re-confirmation — no loop-eligible criterion exists)
row_or_kind: n/a (all 33 targeted rows already Supported/ProductVisible; interaction.non_human_any_class.progression_pressure permanently excluded)
commit: no commit: nothing eligible; live matrix re-derivation confirmed the loop's terminal state (33/34 rows Supported/ProductVisible) with no drift since cycle-2026-07-17T0229/T0226/T0223/T0221/T0218/T2400
card: no card: no eligible work attempted, nothing to attach a post-mortem record to
verify: cargo test not run (no code change attempted); live grep of support_state_matrix.rs (34/34 rows) independently confirms terminal state
status: NO-OP
