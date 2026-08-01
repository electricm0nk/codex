# SD-27 — Release Notes

> **Phase 1 (ingestion) populated at closure, cycle E4.3, 2026-07-28.**
> **Phase 2 (player reachability) populated at closure, tranche/7-1, 2026-08-01.**
>
> Every figure in §§8–13 was re-derived by command at closure, from `git log`, the on-disk corpus and
> a full `scripts/verify.sh --full` run — **not** from the planning documents' predictions, and not
> from the cycle reports that preceded this one. Where a prior document's number turned out wrong, the
> correction is stated rather than the number quietly replaced.

---

## 1. Summary

SD-27 resolves the operator's 2026-07-25 "tune, then go wide" directive: prove the per-book Shape B v1
ingestion pattern on 2 future-state books before fanning out to the remaining 17. In scope:
**Advanced Race Guide (ARG)** and **Pathfinder Unchained (PU)** — matching the operator's live
dashboard workchannel `SD-27 (ARG + PU)`. Adventurer's Guide, though present in the bundle's original
authoring, is **not** part of this closure; it is routed to SD-30.

The bundle also lands Shape B v1 — a license-aware schema extension (`license`/`pi_field`/`pi_marker`)
— and retro-fits all 4 already-ingested in-scope books (Core Rulebook, Advanced Player's Guide,
Advanced Class Guide, Bestiary 1) to it, ahead of building the 2 new books natively in v1.

19 cycles ran across Epics 1–4 (of the pre-scoped 45-cycle full-19-book plan; the other 17 books remain
correctly deferred to SD-28+). All 19 completed; 0 failed; 0 blocked.

**The bundle then acquired a second phase.** The operator's 2026-07-30 directive redefined SD-27's
definition of done from "content is ingested" to full player reachability — *"all data is ingested,
compute is available, and can reach the end user through the ui. there is not a single thing left to be
done for that thing to be utilized by a user."* That directive governs; the original content-only
framing (`README.md §1`, `technical-design.md:156`) is superseded on this point, as recorded in
`decisions.md §24.2` and `§28`. §§8–13 below are that second phase.

## 2. User-Visible Changes (Phase 1, as recorded 2026-07-28)

| Book | Records | Content kinds | License | Registry status |
|---|---|---|---|---|
| Advanced Race Guide | 479 | 92 spell + 200 equipment + 187 feat | 479 OGL, 0 redacted | `#0003` → Resolved |
| Pathfinder Unchained | 59 | 17 feat + 42 equipment (no new spells — honest absence) | 59 OGL, 0 redacted | `#0017` → Resolved |

> **Superseded by Phase 2.** Both books grew. ARG is now 635 records (the 156 alternate-racial-trait
> records landed in Phase 2) and PU is now 127 (4 class + 64 class_feature). Derived counts are in
> §8. The "racial-trait and class-ability-formula content is out of scope" statement below was true
> when written and is **no longer true**: `decisions.md §24` ruled that content in, hand-modelled per
> feature, and Phase 2 delivered it.

Both books' racial-trait, race-builder, and class-ability-formula content (PCGen's low-level
ability/BONUS/DEFINE/PREREQ syntax) was deliberately outside the Phase-1 Shape B cache — no book in
this codebase, including Core Rulebook itself, had ever represented that content shape in Shape B
JSON. Documented per-book in each cycle's receipt, not silently dropped.

**4 in-scope books retro-fitted to Shape B v1** (license-aware, additive over v0):

| Book | Records | Redacted |
|---|---|---|
| Core Rulebook | 3,326 | 0 |
| Advanced Player's Guide | 641 | 0 |
| Advanced Class Guide | 423 | 1 (an example NPC name in spell flavor text, flagged for operator review) |
| Bestiary 1 | 45 | 0 |

**Total at Phase 1: 4,973 Shape B v1 records across 6 books.**

**Real PCGen parity baselines** for both new books (E3.1/E3.2) — see §5.

## 3. Defects Fixed (Phase 1)

- **Two shared-test staleness gaps**, both surfaced by this bundle's own real execution and fixed once
  by the orchestrator rather than left to race across parallel cycles:
  - `tests/sd27_license_stripping_shape_v1.rs`'s Audit 1 asserted every on-disk record had
    `license: None` — true only pre-retrofit. Rewritten to assert the post-retrofit invariant
    (populated `license` + `validate_license()` clean) against the real corpus.
  - The same test's file-walk didn't exclude `LICENSE.json` or `_parity/` output — both non-`CorpusRecordV1`
    shapes by design. Both exclusions added.
- **A real regression in SD-26's own, already-shipped `tests/sd26_cache_core_rulebook.rs`**: the new
  `core_rulebook/LICENSE.json` broke its generic file-walk. Fixed with the same exclusion pattern —
  a direct, necessary consequence of this bundle's own change, not scope creep.

## 4. Operational Notes (Phase 1)

- **File-touch partition** (`decisions.md §8`, `loop-instruction.md §6`): enforced per-cycle throughout.
  One real, self-corrected near-miss: the two per-book pre-build cycles (E2.1/E2.2) ran concurrently in
  the same shared working directory (no git worktree isolation) and briefly collided on
  `src/rules_core/rules_tables/mod.rs`, which the partition doesn't allow-list. Self-corrected in-place;
  recorded in `progress.md`'s Open Blockers as a real gap for future concurrent per-book batches — not
  swept under the rug. As a direct consequence, both new books' `rules_tables` modules were reachable
  only via the shared codegen binary's `#[path]` include, not `codex::rules_core::rules_tables::*`.
  **Closed in Phase 2** — both modules are registered in `rules_tables/mod.rs`; see §12.
- **Tier model**: all Phase-1 cycles ran at Sonnet; the free/discounted-model option (`decisions.md §11`)
  was available but not exercised.
- **v0.6 coordination**: confirmed via `git log` against `origin/tranche/6` before dispatching the
  4-book retrofit — v0.6's concurrent activity was confined to `src/rules_core/rules_tables/{crb,apg,acg}/`
  (off-limits to SD-27 at the time), zero collision on the `data/corpus/` files that bundle touched.
  **`decisions.md §28` records that partition as spent**: v0.6 closed, and both prohibitions were lifted
  for Phase 2 because two shipped defects could not be closed without `pilot_compute.rs`.

## 5. Verification Evidence (Phase 1, as recorded 2026-07-28)

| Criterion | Verification | Result |
|---|---|---|
| 1.1 | Full-tree code-identifier scan | 0 genuine bundle-tagged identifiers |
| 2.0 | Label resolution propagation | 21 stubs + registry + SD-26 decisions.md + v0.6 risks doc, all `"SD-27"` |
| 2.0.5 | Schema + PI-blacklist tests | 10/10 new tests passing |
| 2.0.6–2.0.9 | Per-book retrofit + 5th audit | 4,435 records, 0 PI-blacklist defects |
| 2.0.10 | 23-book conformance sweep | 23/23 accounted for, 0 defects |
| 2.1/2.1', 2.2/2.2' | Cache-shape tests + orchestrator re-verification | 15 new tests; sha256/line citations independently confirmed against real LST |
| 3.1 | Real PCGen Gradle pipeline (ARG) | 13/15 dimensions match |
| 3.2 | Real PCGen Gradle pipeline (PU) | 14/15 dimensions match |
| Full suite | `cargo test --workspace --locked --no-fail-fast` (with `PCGEN_REPO_DIR` set) | **4,820 passed / 2 pre-existing, environment-path-dependent failures** unrelated to this bundle |
| Dual-audit | `identifier-discipline` + `wired-integration` (4-check) | Clean at every cycle boundary |
| 4.1 | Final criterion scan, 3 independent sources | 14/14 pre-closure criteria, 0 discrepancies |
| 4.2 | Architecture truth-up | Clean, no architecture-doc impact. Graphify: genuine environment gap (no CLI installed), honestly reported |

## 6. Known Issues (Phase 1) — current status

- **Inherited CG-03 baseline** (`decisions.md §10`, v0.6's lane): both books' parity runs show
  `combat.baseline_melee_attack_bonus` mismatched (PCGen's generic melee-export field doesn't fold in a
  weapon-specific `Weapon Focus` to-hit bonus). Same root cause SD-26 already documented; inherited,
  not chased by this bundle. **Still open.**
- **`encumbrance.rs` CRB-only weight lookup** — real equipment from other books resolved correctly
  against the book-agnostic corpus resolver but its weight was silently dropped (PCGen 30 lbs, Codex
  29 lbs on the ARG Dogslicer). **Still open**; `encumbrance.rs` was outside every Phase-1 partition
  and no Phase-2 cycle owned it either.
- **`ACG` retrofit's one PI redaction** (an example NPC name, "Jarn," in spell flavor text) is a
  judgment call flagged for operator review. **Still open.**
- **PI-blacklist classifications throughout are a heuristic first-pass screen**, not an exhaustive legal
  review — stated in every book's `LICENSE.json` (`operator_sign_off.signed_off: false` throughout).
  **Unchanged and deliberate.**
- **`rules_tables::{advanced_race_guide,pathfinder_unchained}` not wired into `codex`'s public module
  tree.** **CLOSED in Phase 2** — both are registered. One residue remains: `src/bin/sd27_gen_book_cache.rs`
  still `#[path]`-includes the ARG module a *second* time into its own crate, which is the sole reason
  several ARG items report as dead code. Deferred; see §13.
- **Graphify CLI not installed** in this execution environment. **Unchanged.**

## 7. Update Eligibility

This closure is the bundle-of-record for Advanced Race Guide and Pathfinder Unchained's Shape B v1
ingestion and player reachability, and for Core Rulebook/Advanced Player's Guide/Advanced Class
Guide/Bestiary 1's license retrofit. Version bumps `0.6.0 → 0.6.1` (cycle E4.4).

**Migration path for the 17 deferred future-state books (SD-28+):** the per-book pre-build → verify →
parity cycle pattern established here is templated and reusable. SD-28 (Ultimate line, 6 books), SD-29
(Bestiary line, 7 books), and SD-30 (Adventure+ line, 4 books, including Adventurer's Guide) are the
operator's next-batch routing. Recommend `isolation: 'worktree'` for any future batch running 2+
per-book cycles concurrently — Phase 1's near-miss on shared `rules_tables/mod.rs` recurred in Phase 2
as three separate `git stash` incidents in a shared working tree (§13).

---

# Phase 2 — Player Reachability (tranche/7-1, 2026-07-31 → 2026-08-01)

## 8. What actually shipped, derived from the corpus

**Corpus totals, counted on disk at closure.** Command:
`find data/corpus -name '*.json' -not -name 'LICENSE.json' -not -path '*/_parity/*' | wc -l`

| Book | Total records | By content kind |
|---|---:|---|
| Core Rulebook | 3,400 | 2,663 equipment · 652 spell · 67 race_trait · 11 class · 7 race |
| Advanced Player's Guide | 641 | 338 equipment · 297 spell · 6 class |
| Advanced Race Guide | **635** | 200 equipment · 187 feat · **156 race_trait** · 92 spell |
| Advanced Class Guide | 423 | 269 equipment · 144 spell · 10 class |
| Bestiary 1 | 164 | **108 race_trait** · 41 monster · **11 race** · 4 equipment |
| Pathfinder Unchained | **127** | **64 class_feature** · 42 equipment · 17 feat · **4 class** |
| **Total** | **5,390** | across 6 books |

**Phase 2 added exactly 417 corpus records** (`git diff --name-status c79884f8..HEAD --diff-filter=A -- data/corpus`),
and every one is a content kind no book had ever ingested:

| Records | Book / kind |
|---:|---|
| 156 | `advanced_race_guide/race_trait` — the Alternate Racial Traits corpus, ARG's own contribution |
| 108 | `beastiary/race_trait` |
| 67 | `core_rulebook/race_trait` |
| 64 | `pathfinder_unchained/class_feature` |
| 11 | `beastiary/race` |
| 7 | `core_rulebook/race` |
| 4 | `pathfinder_unchained/class` |
| **417** | |

Before this tranche, **no `data/corpus/*/race/` directory existed anywhere** (`decisions.md §25.5`):
the Core Rulebook's 7 races lived only as a hardcoded 7-variant `RaceId` enum. Race content is now
corpus-driven for all 18 in-scope races, with the pre-existing CRB 7 pinned before/after by
`tests/sd27_crb_race_corpus_pin.rs`.

**Scale of the change** (`git diff --stat c79884f8..HEAD`, 21 commits): **601 files changed,
+70,739 / −958**, plus a 12-file uncommitted working tree at closure (+2,665 / −40).

## 9. Player-reachable surfaces, derived from the live IPC responses

Every count below is asserted by a test that ran green in the closure sweep — it is what the
command actually returns, not what a table says it returns.

| Surface | Serves | Pinned at |
|---|---|---|
| `list_feat_catalog` | **690** feats: 185 CRB · 172 APG · 129 ACG · **187 ARG** · **17 PU** | `rules_tables/feats_all.rs:645` |
| `list_spell_catalog` | **1,185** spells: 652 CRB · 297 APG · 144 ACG · **92 ARG** | `spell_catalog.rs:250` |
| `list_equipment_catalog` | **3,830** items: 2,977 CRB · 338 APG · 269 ACG · 4 B1 · **200 ARG** · **42 PU** | `equipment_catalog.rs:580` |
| `list_race_catalog` | **18** races, 173 rows | `race_catalog.rs:442` |
| `list_alternate_racial_traits` | **18** races' standard + alternate trait menus | `race_trait_picker.rs:770` |
| `list_monster_catalog` | **41** Bestiary 1 monsters | `reach_gate.rs` per-record claim |
| character creation | **31** classes (`CLASS_OPTIONS`), **18** races (corpus-derived roster) | `characterHubModel.ts:409`; `raceRoster.raceOptionsFromChassis` |
| `v06_class_state_dump` | **31/31** classes computing at every level | verify.sh `class-dump` stage |

The 7-entry `RACE_OPTIONS` constant is **gone** from `characterHubModel.ts` — the creation form now
builds its roster from the backend's corpus-derived chassis, which is why widening to 18 was possible
at all.

## 10. Defects fixed in Phase 2

Each of these was a wrong number or a dead affordance **on a shipped screen**, most of them
pre-dating SD-27 and found by it.

1. **Size modifiers did not exist for any race, at any pillar.** A live Goblin Fighter 1 showed
   AC 18 / touch 14 / CMB +3 / CMD 17; PF1's Small column is 19 / 15 / +2 / 16. Carrying capacity was
   the only consumer of `SizeCategory` in the entire crate. Now grounded in the engine and plumbed to
   the sheet (`tests/sd27_size_modifiers_to_armor_class.rs`,
   `tests/sd27_size_modifiers_to_touch_cmb_cmd_and_attack.rs`). **This pre-dated the race widening —
   Gnome and Halfling shipped with it.**
2. **The two-compute-twins trap.** Five feats (three of them CRB) computed one number in the test
   suite and a different one on the player's sheet. Closed by a single shared seam plus a structural
   *and* a behavioural guard. Full account: `decisions.md §29.1`.
3. **Feat prerequisites were not enforced.** Now evaluated; **599 of the catalog's 690 records** carry
   at least one prerequisite (`feats_all.rs:738`). Of the **29 distinct PRE token kinds** the catalog
   uses, **14 are modelled**, 14 are declared unmodelled with a stated reason, and `PRETEXT` is PCGen
   display prose. The unmodelled ones return `Unmodelled` and **never block**, rather than guessing —
   see §13.
4. **The prepared-spell level gate covered CRB only.** A Wizard 1 could prepare `Tsunami` (APG,
   9th level) and the engine reached `Computed` without complaint, because the resolver looked the key
   up in `crb::spell_list` alone and `filter_map` silently dropped what it could not resolve.
   Closed by per-class spell levels for APG/ACG/ARG (`tests/sd27_non_crb_spell_level_gate.rs`).
5. **Raw PCGen syntax reached players.** Two independent leaks: 17 of 681 served feat descriptions
   carried unrendered `DESC:` syntax into the Add Feat picker's detail line; 54 equipment records
   carried the `%%` literal-percent escape (ARG's Dwarven Boulder helmet read *"adds 20%% to the
   wearer's arcane spell failure chance"*). Both closed, with a cross-catalog guard over 5,394 served
   strings so a third leak fails loudly.
6. **PU's 4 Unchained classes were grounded but unselectable** — 4 class + 64 class_feature records
   and 69 passing library tests, and no player could pick one. Now wired; all 64 class features reach
   `load_saved_character -> explanations` carrying real payload, claimed per corpus record.
7. **ARG's alternate racial traits were browse-only.** `CreateCharacterRequest` carried no
   alternate-trait field. All 153 alternates now persist onto a character, compute, and reach the sheet.
8. **9 Aasimar alternate-trait rows were dead affordances** — selectable and incapable of applying.
   Closed by the globalvar gate (`tests/sd27_aasimar_globalvar_gate_closes_the_dead_affordance.rs`).
9. **105 dead equipment rows** and the cross-book equipmod picker's unattachable offers, closed to the
   extent honestly possible (the residual is §13's cross-book cost resolver).
10. **No post-creation removal existed for anything.** Feats, spells and equipment all had an add path
    and no remove path. All three now have one, on the same load → mutate → recompute → re-save spine,
    with a dependency guard that refuses a removal another held feat depends on and names the reason.
11. **Flat-footed AC moved into the engine** (`defense.flat_footed_armor_class`), off the React sheet.

## 11. Reach: what "reach-complete" means, per book, per content kind

**The measurement.** `apps/desktop/src-tauri/src/reach_gate.rs` unions three independent discovery
sources (the shipped ingest diagnostic, a source scan of `rules_tables/`, and the `data/corpus/` tree),
yielding **27 (book, content-kind) families**. For each it *executes* a claim against the live IPC
boundary — no claim consults a doc comment or a recorded count — and requires the record to arrive
carrying real payload, not just an identity. The findings list is pinned bidirectionally: an unlisted
gap fails, and a *fixed* listed gap also fails until its entry is deleted.

**Result: 26 of 27 families reach a player. 1 does not, and is upstream-blocked.**

Two different things are being claimed, and they are not the same:

* **Catalog-reach** — the record arrives at a real screen carrying renderable fields (name,
  description, cost, level…). This is what the reach gate measures.
* **Magnitude-reach** — holding the thing changes a number the engine computes and the sheet prints.
  The reach gate does **not** measure this, except for PU class features, whose claim is executed
  against computed `explanations` rows.

| Book | Content kind | Catalog-reach | Magnitude-reach |
|---|---|---|---|
| **Core Rulebook** | feats (185) | ✅ complete | partial — the wired subset moves numbers; not enumerated as a ratio |
| | spells (652) | ✅ complete | ✅ level gate + slot consumption enforced |
| | equipment (2,977 served) | ✅ complete | partial — formula-priced equipmods attach free (§13) |
| | weapons | ✅ complete | ✅ |
| | races (7) | ✅ complete | ✅ size, ability adjustments, vision, speed all compute |
| | race_traits (67) | ✅ complete | partial — Dwarf's 6 swap-suppress; 6 other race seams do not (§13) |
| | classes (11) | ✅ complete | ✅ 11/11 compute at every level |
| **Advanced Player's Guide** | spells (297) | ✅ complete | ✅ per-class spell levels ingested |
| | equipment (338) | ✅ complete | partial — no `apg_equipmods.lst` exists; nothing is missing |
| | classes (6) | ✅ complete | ✅ compute at every level |
| **Advanced Class Guide** | spells (144) | ✅ complete | ✅ |
| | equipment (269) | ✅ complete | partial — 48 equipmods, cross-book attach deferred (§13) |
| | classes (10) | ✅ complete | ✅ |
| **Advanced Race Guide** | feats (187) | ✅ complete | **24 of the 49 unconditionally-bonused feats move a number on their own**, proven by running the real pipeline once per catalog key and differencing the result. A 25th (Bestow Luck) is wired and moves a number only alongside its own prerequisite, Defiant Luck — correctly, since alone there is no ability for its extra use to attach to. **24 remain ungrounded** — §13. The other 138 of the 187 carry no unconditional bonus token at all. |
| | spells (92) | ✅ complete | ✅ per-class spell levels ingested |
| | equipment (200) | ✅ complete | partial — 15 equipmods; cross-book attach deferred (§13) |
| | race_traits (156) | ✅ complete — all 156, both stragglers closed | **153 alternates persist, compute and reach the sheet; 11 of the 153 carry a bonus that lands on a total this engine computes.** 142 do not, because the total does not exist yet — §13 |
| **Bestiary 1** | races (11) | ✅ complete | ✅ all 11 creatable and computing |
| | race_traits (108) | ❌ **107 of 108** | the 107 reach; the 1 is unreachable |
| | monsters (41) | ✅ complete, record by record | n/a — browse-only content |
| | equipment (4) | ✅ complete | ✅ |
| **Pathfinder Unchained** | feats (17) | ✅ complete | partial |
| | equipment (42 equipmods) | ✅ complete, with honest null costs | partial — `pu_equipmods.lst` carries no `COST:` on any row, so cost arrives null rather than a fabricated 0 |
| | classes (4) | ✅ complete | ✅ all 4 selectable and computing |
| | class_features (64) | ✅ complete | ✅ **all 64** reach `load_saved_character -> explanations` with real payload, claimed per corpus record |

**SD-27's own two books are 8 of 8 families catalog-reach-complete** (ARG: feats, spells, equipment,
race_traits; PU: feats, equipment, classes, class_features).

**The one NO, stated exactly.** `beastiary1/race_traits` — `Duergar ~ Spell-Like Ability ~ Invisibility`.
Its positive gate is `FACT:Duergar_ReplaceSLAEnlargePerson|True`, and **no record in any ingested book
sets that flag.** Its only setter anywhere in the PCGen checkout is `Duergar ~ Ironskinned` at
`monster_codex/mc_abilities_race.lst:16` — a book this project has not registered, audited or ingested.
This is **upstream-blocked, not broken**, and the distinction is proven by an executable test rather
than asserted: `tests/sd27_duergar_invisibility_sla_is_upstream_blocked.rs` derives the empty setter set
from the on-disk corpus, proves no Duergar selection reaches the row, **and proves the mirror row does**
(`Duergar ~ Blood Enmity` sets `Duergar_ReplaceSLAInvisibility` and really does grant
`Duergar ~ Spell-Like Ability ~ Enlarge Person`). That test goes RED the day Monster Codex is ingested,
which is how the finding closes. Routed to **SD-29**.

**What reach-complete does not mean, stated plainly so nobody rounds up:**

* It does not mean every record changes a number. For ARG's 187 feats and 153 alternate racial traits
  the honest ratios are **24 of 49** and **11 of 153** respectively, and the shortfall is a missing
  *engine dimension*, not missing wiring — see §13.
* It does not mean the reach gate is complete. It has two structural blind spots
  (`decisions.md §29.3`), one of which — hand-modelled pure functions being invisible to a source
  scan — is **permanent by construction** under `decisions.md §24`.
* It does not mean every screen renders every field that crosses the boundary. Equipment descriptions
  are served for 2,856 records and stop at the IPC boundary; no TypeScript consumer reads them yet.

## 12. Verification — the closure gate

`scripts/verify.sh --full --show-actuals -j 8`, run 2026-08-01 on `tranche/7-1` at `06641a54` plus the
12-file working tree. **8 stages pass, 1 fails.**

| Stage | Result | Recorded floor / ceiling |
|---|---|---|
| root-lib | **PASS** — 1,426 passed, 0 failed | floor raised 1,412 → 1,426 |
| root-full | **FAIL** — 5,848 passed / **5 failed** / 526 suites | floor raised 5,825 → 5,848; binaries unchanged at 526 |
| desktop | **PASS** — 400 passed, 0 failed | floor raised 385 → 400 |
| reach | **PASS** — 16 passed | no floor of its own |
| frontend-install | **PASS** | — |
| frontend-test | **PASS** — 97/97 files | floor raised 96 → 97 |
| frontend-typecheck | **PASS** — `tsc --noEmit` clean | — |
| clippy | **PASS** — root 75, desktop 7, **0 errors** | at ceiling; **not raised** |
| class-dump | **PASS** — 31/31 computing | floor 31, unchanged |

**The 5 root-full failures are environmental and pre-existing, proven rather than asserted.** All five
need PCGen or pilot fixtures under a hardcoded `/home/ubuntu/...` path that does not exist on this box:

| Suite | Missing fixture |
|---|---|
| `sd26_pcgen_runner` | `/home/ubuntu/workspace/repos/pcgen/gradlew` (+ `pf_Paladin.pcg`) |
| `sd26_pilot_case_verification` | `.../GE-05-.../pf1-crb-human-fighter-level1-provisional-ge05-e1-f2.pcg` |
| `sd27_advanced_race_guide_parity` | PCGen Gradle wrapper |
| `sd27_pathfinder_unchained_parity` | PCGen Gradle wrapper |
| `v06_wizard_pilot_case_verification` | `.../pf1-crb-human-wizard-level1-v06-alpha-swarm.pcg` |

Pre-existence proven per file, two ways: `git diff --stat c79884f8..HEAD -- <file>` **and**
`git diff --stat HEAD -- <file>` are **both empty (0 bytes) for all five**. The failure count is
cross-checked two ways so a larger failure cannot hide behind the suite count: summed `failed;` = 5,
and `test result: FAILED` lines = 5.

**Counting method.** Every number above was taken with `awk` (verify.sh's own `count_passed` form), not
with this harness's shimmed `grep -o`, which silently drops matches on large logs. The commands are
recorded in `scripts/verify-baselines.env` beside each raised floor.

**Floor deltas reconcile exactly**, derived by counting the tests themselves rather than by subtracting
two measurements — this file's standing method:

| Floor | Δ | Reconciliation |
|---|---:|---|
| root-lib 1,412 → 1,426 | +14 | `feat_effects.rs` +9, `pcgen_desc.rs` +5, `pilot_compute.rs` +0 |
| root-full 5,825 → 5,848 | +23 | root-lib's +14 **plus** `tests/sd27_arg_and_pu_feat_effects.rs` +9 |
| desktop 385 → 400 | +15 | `character_hub.rs` +10, `equipment_catalog.rs` +4, `feat_catalog.rs` +1, `main.rs` +0 |
| frontend 96 → 97 | +1 | `apps/desktop/src/boundary/removeSelection.test.ts` |

Clippy ceilings were **not** raised. Both crates sit exactly at ceiling with 0 errors.

## 13. Known Issues and Deferrals (Phase 2)

The full deferral register — **72 `deferral` events across 51 actor shards** in
`docs/retro/events/*.jsonl`, alongside 85 corrections, 26 incidents, 4 near-misses and 4 reworks — is
the honest record of what was left. The material ones, grouped by the reason they were left and routed
to where they should land:

### A. Blocked on an engine dimension that does not exist (→ the cycle that builds the dimension)

| Deferral | Measured shortfall | Needs |
|---|---|---|
| ARG feat magnitudes | **24 of 49** unconditionally-bonused feats ungrounded (25 wired, of which 24 move a number unaccompanied) | 10 need an `ABILITYPOOL` chooser surface; 6 need spell-like abilities; 3 need racial per-day resource pools; then fly manoeuvrability (Angel Wings), companion levels (Beast Rider), glide ratio (Draconic Glide — whose corpus row also carries two contradictory unconditional tokens, `5` and `25`), weapon-size treatment (Goblin Gunslinger), natural-weapon damage dice (Blood Beak). Great Hatred is excluded separately: its bonus depends on the *opponent's* creature type. |
| ARG alternate racial traits | **142 of 153** change no computed number | The engine computes six totals an ART can land on (3 saves, Climb/Intimidate/Swim). Perception, Fly, Profession, Linguistics, initiative, caster level, spell DCs and ability pools have no total to land on. Wiring one would mean inventing the total first. Re-derived every run by `tests/sd27_alternate_racial_trait_reachability.rs`, which **fails and names them** the day the surface widens. |
| Unchained Summoner spell list | 0 of 202 spells | 46 of the 202 are defined only in Ultimate Magic / Ultimate Combat, neither ingested. Blocked on **SD-28**. Substituting the APG Summoner list would be the *wrong* list — the corpus's `StandardSummoner` flag switches it off. A non-claim-blocking diagnostic rides on every Unchained Summoner receipt. |
| Feat prerequisites | **14 of the 29 distinct PRE token kinds** unverifiable — **51 of 1,592 clause occurrences, 3.2%** (both re-derived at closure from the pinned census; an earlier report said "14 of 31", which counted `!PREABILITY` and `!PREALIGN` as kinds of their own) | `CharacterInput` carries no alignment, deity, domain, proficiency roster, class-feature/archetype roster or vision modes. These return `Unmodelled` and **never block** — a guessed denial is the failure mode this engine exists to prevent. The `PREABILITY` Special-Ability arm alone would close ~100 clauses. |
| Archetype suppression | not applied for any PU class feature | No archetype engine exists anywhere in this repo. Each function models the unsuppressed progression and its doc comment says so. |

### B. Upstream-blocked on an unregistered book (→ SD-28 / SD-29)

* **`Duergar ~ Spell-Like Ability ~ Invisibility`** — Monster Codex. **SD-29.** Executable proof, §11.
* **19 of ARG's 37 races** — B2/B3/B4/ISWG. Ingesting one here would invent provenance for a tome
  nobody has audited (`decisions.md §25.3`). **SD-28** (B2–B4); ISWG unscheduled.
* **46 of the Unchained Summoner's 202 spells** — Ultimate Magic / Ultimate Combat. **SD-28.**

### C. Correct-but-incomplete: a partial fix would be worse than the gap

* **Flat-footed AC dodge-denial.** The cell moved into the engine; it does not yet drop dodge-typed
  bonuses. Subtracting only the Dodge *feat* would be right for one build and wrong for every other —
  `pilot_compute.rs` carries at least six further dodge-typed AC terms (dwarf Defensive Training,
  swashbuckler Nimble, Dodging Panache, Dizzying Defense, Trap Sense, Inspire Heroics). Needs an
  engine-side inventory of every dodge-typed contribution first.
* **Cross-book equipmod attach.** 57 non-CRB equipmods the picker offers cannot attach. The
  recognition gate *and* the cost resolver are both CRB-only, so widening the gate alone would attach
  a 500gp ARG Whipwood **for free** — silent mispricing, strictly worse than today's honest refusal.
* **Two pre-existing pricing defects, pinned rather than fixed**: CRB's Holy Symbol (Wooden/Silver)
  display 1gp/25gp and attach free (duplicate `KEY:` across two categories, first match wins); and
  formula-priced equipmods (CRB +1..+10, ARG's `COST:WT*375` Darkleaf Cloth) attach free because
  `COST:` is a PCGen formula `decisions.md §24` forbids evaluating.
* **Non-caster spell picker.** A Fighter is still offered all 1,185 spells. `list_class_spell_levels`
  reports one bit, and that bit conflates *"casts, but no list ingested"* (Magus, Summoner, Oracle)
  with *"casts nothing"* (Fighter, Barbarian, Rogue). Narrowing on it would hide legal spells from the
  first group. Needs a third state.
* **PF1's minimum-ability rule for learning a spell** (Int ≥ 10 + spell level). Landing it alone would
  convert a creation-time defect into un-loadable saved characters: `pf1_adapter`'s own wizard fixture
  is Int 10. Creation-time validation must come first.
* **`Halfling ~ Adaptable Luck`'s reduced-bonus magnitude** is dropped from the served description
  (`DESC` arg 2 is `Halfling_AdaptableLuck_Bonus-1`, an *expression*). Reading a constant off the row
  is transcription and allowed; evaluating `<Var>-1` is not, under §24. The clause is kept and only the
  unread number omitted, because dropping the clause would imply no downside.
* **`Armor of the Pit`'s Scaled Skin branch** grounds no resistance number: the token names all three
  of cold/electricity/fire but the rule grants resistance 5 to **two**, player-chosen. Grounding three
  overstates it; grounding a guessed two fabricates the choice. A +0 record names the branch instead.

### D. Ran out of scope, not out of reason (→ next cycle owning the file)

* **Equipment descriptions stop at the IPC boundary.** The DTO now carries a rendered `description`
  for 2,856 records; `loadEquipmentCatalog.ts`, `EquipmentCatalogScreen.tsx` and `itemPickerFilter.ts`
  do not read it yet. A genuine half-hop, flagged rather than claimed as reach.
* **The headline HIT POINTS panel contradicts the Defense tab** for any Toughness holder — the third
  TypeScript twin. `decisions.md §29.2`.
* **The sheet's Vision cell ignores a vision-replacing alternate** (ARG Halo). The *engine* is correct;
  `character_hub::race_creation_chassis` resolves vision at race level with no alternate selections.
* **Standard-trait suppression is gated for Dwarf only.** The other 6 CRB race seams emit their
  standard traits unconditionally, so an alternate replacing one removes nothing from the sheet's
  explanation list. Bounded deliberately to keep the `pilot_compute.rs` change reviewable under §28's
  standing guard; the remaining 6 are the same mechanical edit repeated.
* **ARG feat magnitudes are not yet read by `pilot_compute_corpus.rs`** for two specific producers
  (`armor_of_the_pit` natural armor; `arg_computed_climb_bonus`), plus the parity test that would fail
  when a term exists on one twin and not the other. Found *during verification*; a verifier silently
  patching the thing it is measuring destroys the measurement.
* **The Known/spellbook path is ungated.** A Wizard 1 can record `Antilife Shell` (Cleric/Druid-only)
  as a Wizard spell. The off-*list* half is illegal under any reading; the out-of-*level* half is a
  genuine rules question that should not be settled by an agent inventing a rule.
* **The Add Spell picker offers the whole 1,185-row catalog** with no class or level filter. The
  backend's refusal is *correct* PF1 enforcement — the defect is the unfiltered menu.
* **3 ARG racial-trait records reach no picker surface** (`Feral ~ Languages`,
  `Scion of Humanity ~ Languages`, `Saltbeard ~ Dwarf ~ Greed`): `build_menu` filters to
  `Default|Alternate`. All three do reach through `resolve_race_alternate_selection`, which is why the
  family is catalog-reach-complete; being absent from the *menu* is a product decision.
* **`monster_catalog.rs::race_subtype` serves a raw PCGen `|` to the player** — Hell Hound reads
  `Evil|Extraplanar|Fire|Lawful`. A display join, not a `DESC:` render, so `render_pcgen_desc` is the
  wrong tool.
* **Post-creation removal of alternate racial traits** was not built: there is no post-creation *add*
  path to mirror, and a remove-only path would be a one-way door in the other direction.
* **16 APG/ACG classes absent from the Class Progression browser** (300 rows across 15 of the 31
  creatable classes). Pre-existing; PU's rows landing there made the omission more visible, not less.
* **3 APG spell records still carry `%%` on disk**
  (`chameleon_stride`, `fiery_body`, `ghostbane_dirge`). **No player is affected** — the screen path
  renders through `render_pcgen_desc`. Fix at `cache_gen/apg.rs:580` and `:780`.
* **One PU class_feature record carries a literal `&nl;`** (`unchained_rogue_debilitating_injury.json`)
  because `ingest_pu_classes.rs` has its own copy of the desc treatment. Not player-reaching.
* **`src/bin/sd27_gen_book_cache.rs` double-includes the ARG module**, which is the sole cause of
  several ARG dead-code warnings. Deleting the include should drop warnings off the root clippy
  baseline.
* **`biosettings` height/weight profiles absent for the 11 newly-creatable Bestiary races.** No book's
  ingest reads `<race>_biosettings.lst` for *any* book; the 7 shipped profiles are hand-entered
  constants with no corpus source. Those races honestly show "No height/weight profile" and offer no
  reroll button, rather than being given invented numbers.

### E. Stale data fields, uncorrected because the file was not owned

* **`core_rulebook/LICENSE.json` reports 3,326; the corpus holds 3,400.** The 74-record gap is exactly
  this tranche's 7 `race` + 67 `race_trait` additions.
* **`beastiary/LICENSE.json` reports 45; the corpus holds 164.** The 119-record gap is exactly this
  tranche's 11 `race` + 108 `race_trait` additions.
* `tests/sd27_book_license_record_counts.rs`'s `BOOKS` covers `pathfinder_unchained` and
  `advanced_race_guide` only — **both of which are correct** (127 and 635, verified). Extending it to
  the other two is the one-line change that would have caught the above, and is deliberately left as
  the only edit needed.
* **`records_processed` is written from what one generator binary emitted**
  (`sd27_gen_book_cache.rs`), so re-running the generator resets corrected values. A drift test now
  fails loudly if the regression returns.
* **`v06_work_inventory.rs::rule_set_for()` still maps four books**, so ARG and PU corpus units read
  as `future_state` with evidence `no_compiled_rule_set_for_book` — false since Phase 1.

### F. Process incidents worth carrying forward

* **Three separate `git stash` incidents in a shared working tree.** Agents measuring a "before"
  baseline by stashing repeatedly stashed *concurrent agents'* uncommitted work; one stash was popped
  by another process mid-run, producing a junk 13-failure measurement. No work was lost. **The method
  is unsafe and the remedy is `isolation: 'worktree'`,** which Phase 1's release notes already
  recommended for a different reason.
* **Two TDD deviations self-reported** rather than glossed: implementation written before the test,
  with RED reconstructed afterwards by stubbing the guard and confirming the intended failure.
* **Two agents wrote outside their declared file scope and said so** — `feat_catalog.rs` (because the
  brief's own deliverable, "a test that fails if ANY catalog serves raw PCGen syntax," was
  unsatisfiable without it, and pinning 17 known player-visible leaks as *expected* would be theatre)
  and `boundary/removeSelection.ts` (because the brief required following the add-path pattern
  exactly, and that pattern puts the `invoke()` wrapper in `boundary/`). Both are recorded as
  judgement calls made rather than questions asked.
