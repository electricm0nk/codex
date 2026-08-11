# SD-29 — Local-file Work Queue (replaces Hermes board `codex-tranche-9`)

Per operator directive 2026-08-01, the Hermes board is retired. SD-29's
work queue is a local-file Markdown table. The supervisor reads this file
at top of each cycle tick to identify the next ready card; the
file-touch partition ensures only one cycle claims a card at a time.

**Re-cut 2026-08-10 (`decisions.md §37`).** Cards are now lane-scoped
(kind, or merged-kind-pair), not per-book. A lane epic's cycle-batches
fan out per book internally — see `epic-breakdown.md` for each lane's
per-book unit counts.

**Re-scoped corpus-wide 2026-08-10 (`decisions.md §38`).** Every lane below now fans out across
all 37 in-scope books, not the retired seven-book set. Epic 4 (Tier 1, proven-path) is split into
three cards since it now covers six kinds at corpus scale; Epics 5-7 (Tier 2, mechanism-gated)
each pilot on one small book before extending corpus-wide.

> ## 🔴 BUNDLE REOPENED — 2026-08-11 (operator directive, `decisions.md §42`)
>
> **Operator, verbatim:** "this is part of sd-29's scope. sd-29 isn't done. let's get after it."
>
> SD-29 was closed on 2026-08-11 (`73f1421f`, `ac217788`) with **three lanes unfinished**. That
> closure is **rescinded**. Cards **8, 9, 10, 11, 12** (monster extend ingest; race-trait pilot and
> extend ingest; the whole companion lane) are back to `READY` and are **SD-29 scope** — not SD-30's,
> not `successor-forward-scope-register.md`'s, not any successor bundle's. Cards **15** and **16**
> (`epic-10-review`, `epic-11-closure`) are reopened with them, because both ran against a bundle
> state that is no longer final.
>
> **PR #360 (`tranche/9` → `develop`) stays OPEN and unmerged** until real closure.
>
> **Prior receipts are preserved, not erased.** Every reopened row keeps its `progress.md` receipt
> and its landed half; only the *status* is corrected. Where a card delivered one half (a chassis, a
> classifier fix) and left another undelivered (the ingest), the row now reads `READY` for the
> undelivered half — an undelivered half does not inherit the delivered half's status.

## Status legend

- `READY` — not yet claimed. Cycle can pick up once every `Depends-on` card is `COMPLETE`.
- `IN-FLIGHT` — claimed by a cycle, in progress. Other cycles must wait.
- `BLOCKED` — cycle claims the block, captures the gap, surfaces in `progress.md` as a blocker.
- `COMPLETE` — cycle receipt in `progress.md` closes the card.
- `DECISION-BLOCKED` — the card required a decision the playbook routes to the operator; under
  UNATTENDED MODE (`loop-instruction.md` item 4) the cycle ruled it itself, took the safe default,
  and recorded the ruling and its evidence in `progress.md`. A terminal state, not a wait: the card
  is closed for this bundle and downstream cards treat it as settled. Do not re-dispatch it.
  **Narrowed 2026-08-11 (`decisions.md §42`), because this legend was misapplied to close the
  bundle early:** `DECISION-BLOCKED` is terminal only for a card that genuinely needed an operator
  decision and got one. It is **not** a disposal chute for work that was simply never dispatched.
  A card whose ingest half was never attempted is `READY`, not `DECISION-BLOCKED`, regardless of
  what its other half delivered.

**Dispatch tiebreak:** next card = lowest `Order` among `READY` cards whose
every `Depends-on` card is `COMPLETE`. A card whose `Depends-on` is not
fully `COMPLETE` is not eligible regardless of `Order` or `Status`.

## Cards (one row per lane epic cycle-batch), in dispatch order

| Order | ID | Status | Lane / Scope | Cycle-type | Depends-on | Claimed-by | Claimed-at | Cycle-id |
|---|----|--------|------|-----------|------------|------------|------------|----------|
| 1 | `epic-1-identifier` | COMPLETE | Identifier Cleanup | identifier-discipline audit pass | none | sd29-e1-identifier | 2026-08-10T00:00:00Z | SD29-E1-F1-001 |
| 2 | `epic-2-prelaunch` | COMPLETE | Operator Pre-Launch | corpus-wide (37-book) cycle-0 trap-report + work-inventory | `epic-1-identifier` | sd29-e2-prelaunch | 2026-08-10T23:56:00Z | SD29-E2-F1-001 |
| 2.5 | `epic-1b-naming-sweep` | COMPLETE | Function-Based Naming Sweep | corpus-source rename sweep: SD-NN + GE-NN tags out of file names, directory names, and identifiers (operator directive 2026-08-11) | `epic-1-identifier` | sd29-e1b-naming | 2026-08-11T00:00:00Z | SD29-E1B-F1-001 |
| 3 | `epic-3-provenance` | COMPLETE | Provenance Gate | PI-screening wired into each lane's extraction step; license-matrix citation for OGL/attribution, corpus-wide | `epic-2-prelaunch` | sd29-e3-provenance | 2026-08-11T00:00:00Z | SD29-E3-F1-001 |
| 4 | `epic-4-proven-equip-mod` | COMPLETE | Proven-Path Lanes — equipment + equipment_modifier | corpus-wide, 1,144 + 812 remaining units (equipment corrected from 1,163 by Epic 2 — the old figure counted `beginner_box`'s 19 excluded units; see `corpus-shape-37-books.md` §3) | `epic-3-provenance` | sd29-e4-equip | 2026-08-11T00:00:00Z | SD29-E4-F1-001 |
| 5 | `epic-4-proven-spell` | COMPLETE | Proven-Path Lanes — spell | corpus-wide, **1,561** remaining units (was 1,754: 192 ARG+UI units were already ingested AND already on screen, but a two-list divergence between `build_spell_catalog` (5 books) and `v06_work_inventory`'s `spell_levels` (3 books) reported them `not-ingested` — closed by this card's cycle; see `progress.md`). Residual splits 622 in rule-set-bearing books / 939 in books with no compiled rule set | `epic-3-provenance` | sd29-e4-spell | 2026-08-11T00:00:00Z | SD29-E4-F1-001 |
| 6 | `epic-4-proven-feat-race-class` | COMPLETE — settled by `epic-10-review` 2026-08-11: its actor never obtained an exit code (turn budget ran out mid-`root-full`) and left the row `IN-FLIGHT` with the lane's 83 feat rows landed but two downstream count pins RED. Those pins are fixed (`b4cff429`) and the lane's whole diff is now covered by a green full gate (`verify.sh` exit `0`, `root-full` 6170/543). Not re-dispatched: the work is landed and gate-verified, only the receipt was missing | Proven-Path Lanes — feat + race + class | corpus-wide, 1,348 + 96 + 158 remaining units (feat: the prior 1,350 counted the kind's 2 `deferred-with-reason` units as remaining — predicate difference, not an arithmetic error; see `corpus-shape-37-books.md` §3) | `epic-3-provenance` | sd29-e4-frc | 2026-08-11T02:40:00Z | SD29-E4-F2-001 |
| 7 | `epic-5-monster-lane-pilot` | COMPLETE | Monster / Monster-Ability Chassis Lane — pilot | Bonus Bestiary end-to-end (14 monster + 17 monster_ability — both re-derived from the corpus, both now `grounded`) | `epic-3-provenance` | sd29-e5-monster-pilot | 2026-08-11T00:00:00Z | SD29-E5-F1-001 |
| 8 | `epic-5-monster-lane-extend` | **READY — round 2** (round 1 landed 2026-08-11 by `sd29-monster-r1`, commits `4aa0fb4b` + `92f7abc3`; receipt `## Cycle SD29-E5-F2-002`). **Round 1 ingested `monster_codex` in full (2 monster + 3 monster_ability = 5 units, the book's ENTIRE monster family) and spent the rest of its budget making round 2 cheap:** the pilot's throwaway transcriber is now checked in as `scripts/transcribe_monster_tables.py` (proven by reproducing the pilot's 31-record table across all 352 field lines, zero diffs), and the chassis is book-generic (`rules_tables::monster_chassis::MONSTER_BOOKS`; the inventory classifier, cache generator, catalog mapping, diagnostic row and reach claims all iterate the registry instead of naming books). **Round-2 targets, from the link-shape table in the receipt (NOT the densest books): `book_of_the_damned_volume_1` (41 units) and `_volume_2` (21 units) are the only remaining books with ZERO orphan abilities.** `ultimate_psionics` looks cheap (already `in_scope`) and is not: 66 of its 79 abilities are Astral Construct menu selections carried in the `TYPE:` token, a third link shape the chassis does not model. Ten books totalling 703 units are `monster_ability`-only with a 100% orphan rate and need a surface decision, not an ingest. Prior reopen note (2026-08-11, `decisions.md §42`) — the once-per-kind chassis IS merged on `origin/tranche/9` and pilot-proven; the per-book **ingest is outstanding and was never dispatched**. Prior status `PARTIAL/decision-blocked` is superseded: a chassis is not a lane. Receipts for the chassis half stand in `progress.md` `## Cycle — epic-5-monster-lane-extend (SD29-E5-F2-001)` and are not erased | Monster / Monster-Ability Chassis Lane — extend | corpus-wide, every remaining book. **Denominators re-derived 2026-08-11T19:26Z by round 1, confirming the prior figure and then moving it: BEFORE `monster` 1,210 remaining + `monster_ability` 3,090 remaining = 4,300; AFTER round 1 **`monster` 1,208 + `monster_ability` 3,087 = 4,295 remaining**. Grounded monsters are `bestiary` 46 (SD-22) + `bonus_bestiary` 14 (pilot) + `monster_codex` 2 (round 1); grounded monster_ability is `bonus_bestiary` 17 + `monster_codex` 3. The prior "1,224 monster" figure in this row was wrong — corrected to 1,270 total. The prior "1,224 monster" figure in this row was wrong — corrected to 1,270 total. Grounded monsters are `bestiary` 46 (SD-22) + `bonus_bestiary` 14 (this bundle's pilot); grounded monster_ability is `bonus_bestiary` 17, all pilot.** **Cost, from the pilot's receipt: do NOT extrapolate a per-unit rate** — essentially all of the pilot's cost was the once-per-*kind* chassis (new `RuleSetId`, rules-table module, generator arm, wire DTO, `CORPUS_KIND_NAMES` entry, reach claims, diagnostic row, frontend path, and 8 whole-catalog assertions re-scoped to their own book). The next book inherits all of it. Expect to fund a published-text dice-grounding decision: 13 of the pilot book's 14 named natural attacks carry no dice in the corpus. | `epic-5-monster-lane-pilot` | sd29-e5-monster-extend | 2026-08-11T12:00:00Z | SD29-E5-F2-001 |
| 9 | `epic-6-race-trait-lane-pilot` | **COMPLETE** (2026-08-11, `decisions.md §43`) — both halves are now delivered. The classifier defect fix stood already; this cycle **re-pinned the pilot book to `monster_codex`** and landed the ingest end-to-end: 5 records at `data/corpus/monster_codex/race_trait/`, a real reach claim `("monster_codex", "race_traits")`, and the retirement of the standing `beastiary1/race_traits` finding from BOTH `OPEN_FINDINGS` and `UNREACHED_RECORD_FINDINGS` (DoD item 6). Prior receipts preserved | Race-Trait Lane — pilot | classifier defect fix (DONE) + `monster_codex` pilot ingest (DONE) | `epic-3-provenance` | sd29-racetrait-repin | 2026-08-11T16:30:00Z | SD29-E6-F1-002 |
| 10 | `epic-6-race-trait-lane-extend` | **READY (round 2)** — this is a **loop-until-dry** lane; a round delivers as much gate-verified work as its turn holds and a successor resumes from its receipt. **Round 1 landed 2026-08-11** (`sd29-racetrait-r1`, commits `4d362e2e`/`577d8e55`, receipt `progress.md` `## Cycle — epic-6-race-trait-lane-extend, ROUND 1`, `decisions.md §44`): `§43.5`'s probe repair (race_trait grounded **21 → 336**), the `beastiary`/`bestiary` book-spelling defect that was silently under-reporting 108 Bestiary 1 records, a live picker-vs-engine stub the Epic 6 pilot shipped for Monster Codex's 4 alternates, and APG's `Half-Orc ~ Plagueborn` (closing `decisions.md §39`'s deferral). **Round 2 starts at `core_essentials`' 48 and `bestiary`'s 3** — no new mechanism, chassis already loaded, and the probe repair means they ground the moment they land. Round 1's gate went RED first and is recorded that way (`progress.md` §6b): 3 root-full failures and 3 clippy warnings, every one attributed to this lane, all fixed at the source, ceiling not raised. **Round 2 also owns one finding round 1's on-screen pass caught** (`progress.md` §8b): the Race Traits browse screen's standard-trait column does not recompute when an alternate is selected — a render bug, evidenced as such, with the engine's own suppression proven green. Prior status `READY` (reopened by `epic-12-reopen`, `decisions.md §42`); the companion mis-classification fix IS landed and stays landed. The race-chassis ceiling (`crb::race_traits()` models exactly 7 races) is a real structural finding and SD-29 now owns confronting it — it is not a licence to close the lane. Prior receipts preserved | Race-Trait Lane — extend | corpus-wide, **26** books. **Denominators re-derived at the end of round 1 (2026-08-11) from `docs/work-inventory.json`: `race_trait` 3,447 total / **336 grounded** / 3,111 remaining by status.** **3,111 is not the lane's workload** (`decisions.md §44.4`): only **553** of the 3,447 name one of the 18 races the product models, and a race trait whose race has no chassis is unreachable by construction — `RaceCorpus::resolve` returns `None` without one. Within the 553, the genuinely ingestable remainder is **167**: `inner_sea_races` 72, `core_essentials` 48, `horror_adventures` 44, `bestiary` 3. Two residuals are deliberately not gap: APG's 49 ARG-key collisions (`§39`) and Monster Codex's `Oversized Goblin` (mechanism-blocked, finding recorded). The pre-round-1 figures this row carried — 21 grounded / 3,426 remaining — were correct when written and reproduced exactly before being superseded | `epic-6-race-trait-lane-pilot` | sd29-racetrait-r1 (round 1; round 2 unclaimed) | 2026-08-11T18:00:00Z | SD29-E6-F2-002 (round 1 of a loop-until-dry lane) |
| 11 | `epic-7-companion-lane-pilot` | **READY** (reopened 2026-08-11 by `epic-12-reopen`, `decisions.md §42`) — **never started; SD-29 owes it.** Its cycle refused at Cycle-mechanics step 1c (`preflight-disk` EXIT=1, 91% used / 47G free, twice, with `reclaim.sh --apply` in between) and correctly left the row unclaimed rather than parking it `IN-FLIGHT` under an agent that did no bounded work — that refusal was an **environmental condition, never a scope ruling**. Receipt preserved: `progress.md` `## Cycle SD29-E7-F1-001`. The disk condition has cleared (80% used, 97G available, re-verified 2026-08-11 by `./scripts/verify.sh --only preflight-disk` EXIT=0). NOT deferred to a successor bundle | Companion Lane — pilot | mechanism-build + a pilot book's ingest. **`inner_sea_combat` (10 units) needs re-confirming before dispatch** — the Epic 6 companion mis-classification fix moved 13 units into `companion` corpus-wide, so per-book companion counts have shifted since this row was written | `epic-3-provenance` | — (unclaimed; prior actor `sd29-e7-companion-pilot` refused at preflight and did no bounded work) | — | SD29-E7-F1-001 (refusal receipt only) |
| 12 | `epic-7-companion-lane-extend` | **READY** (reopened 2026-08-11 by `epic-12-reopen`, `decisions.md §42`) — **never started; SD-29 owes it.** Never eligible, because its `Depends-on` (card 11) never reached COMPLETE. No cycle claimed it and no work was attempted. NOT deferred to a successor bundle | Companion Lane — extend | corpus-wide, **17** books. **Denominators re-derived 2026-08-11 from `docs/work-inventory.json`: `companion` 1,696 total / **0 grounded** / **1,696 remaining** (1,363 `not-ingested` + 333 `not-started`). The prior "1,683 remaining minus the pilot's 10" figure predates the Epic 6 companion mis-classification fix that added 13 units (1,683 + 13 = 1,696)** | `epic-7-companion-lane-pilot` | — | — | — |
| 13 | `epic-9-version` | COMPLETE | Build Version Numbering | first concrete value `0.9.<build>` | `epic-1-identifier` | sd29-e9-version | 2026-08-11T00:00:00Z | SD29-E9-F1-001 |
| 14 | `epic-8-toolkit` | DECISION-BLOCKED | DM Toolkit extension | RULED 2026-08-11 under UNATTENDED MODE item 4: does NOT land inside SD-29; surfaces as the Class 3 retrofit C3.1. Criterion (`epic-breakdown.md` Epic 8 / `loop-instruction.md` "Epic ordering") is "a lane cycle needed the consumer surface to satisfy its reach claim" — unmet: both of Epic 5's pilot reach claims assess the already-shipped `list_monster_catalog`, zero assess a toolkit surface | `epic-5-monster-lane-pilot` | sd29-e8-toolkit | 2026-08-11T00:00:00Z | SD29-E8-F1-001 |
| 15 | `epic-10-review` | **READY** (reopened 2026-08-11 by `epic-12-reopen`, `decisions.md §42`) — its 2026-08-11 pass (`73f1421f`) ran against a bundle state that is **no longer final**. A full-bundle diff review is only meaningful against the diff that actually ships, and cards 8-12 will add to that diff. The prior pass's own findings stand (it fixed two unowned count pins, `b4cff429`); its *coverage claim* does not. Receipt preserved: `progress.md` `## Cycle SD29-E10-F1-001` | Bundle Code Review | full-bundle diff review vs. branch point (`decisions.md §27`), re-run against the reopened bundle's final diff | `epic-4-proven-equip-mod`, `epic-4-proven-spell`, `epic-4-proven-feat-race-class`, `epic-5-monster-lane-extend`, `epic-6-race-trait-lane-pilot`, `epic-6-race-trait-lane-extend`, `epic-7-companion-lane-pilot`, `epic-7-companion-lane-extend`, `epic-9-version`, `epic-8-toolkit` (COMPLETE or `decision-blocked`) | — (unclaimed; prior actor `sd29-e10-review`) | — | SD29-E10-F1-001 (superseded pass) |
| 16 | `epic-11-closure` | **READY** (reopened 2026-08-11 by `epic-12-reopen`, `decisions.md §42`) — the 2026-08-11 closure (`ac217788`) was **premature and is rescinded by operator directive**. It closed the bundle with three lanes unfinished and disposed of them outward. **PR #360 (`tranche/9` → `develop`) stays OPEN and unmerged**; the operator merges it at real closure. Receipt preserved: `progress.md` `## Cycle SD29-E11-F1-001`, annotated by the correction entry at the end of that file | Closure Epilogue | closure rollup + tranche promotion PR (#360 already open — do NOT open a second one) | all cards above (COMPLETE or `decision-blocked`) | — (unclaimed; prior actor `sd29-e11-closure`) | — | SD29-E11-F1-001 (rescinded pass) |
| 18 | `epic-13-desktop-driver-fix` | COMPLETE | Desktop Driver Fix — make Definition-of-done item 8 (on-screen verification) satisfiable again | diagnose + fix `apps/desktop/.claude/skills/run-desktop/driver.sh`; add its self-test to the gate; backfill on-screen checks for the families run 1 shipped without item 8 | none (tooling; blocks item 8 for every other card) | sd29-driver-fix | 2026-08-11T12:30:00Z | SD29-E13-F1-001 |
| 17 | `epic-12-reopen` | COMPLETE | Reopen Correction — make the package's own documents tell the truth after the premature closure | doc-only: record `decisions.md §42`, reset cards 8-12 and 15-16, correct `progress.md` + `release-notes.md`, re-derive the four lane denominators, emit the correction event | none (supersedes `epic-11-closure`) | sd29-reopen | 2026-08-11T15:00:00Z | SD29-E12-F1-001 |

> **Cycle-id collision, recorded not silently fixed (2026-08-11).** Cards `epic-4-proven-equip-mod`
> and `epic-4-proven-spell` both minted `SD29-E4-F1-001`: the two lanes ran concurrently in
> isolated worktrees and neither could see the other's claim before pushing. The ids are therefore
> **not** unique in this bundle — disambiguate a receipt by its card id and `Claimed-by` actor
> (`sd29-e4-equip` vs `sd29-e4-spell`), both of which are unique, not by cycle-id alone. Neither
> receipt is rewritten, because the id is already committed in each lane's commit messages and
> `progress.md` heading. A future concurrent split of one epic should suffix the lane
> (`SD29-E4-F1-001-equip` / `-spell`) at claim time.

> **Epic 6 pilot book carries zero race traits — operator re-pin needed (2026-08-11,
> `sd29-e6-racetrait-pilot`).** Card `epic-6-race-trait-lane-pilot` split on delivery. The
> **classifier defect fix is COMPLETE** (`corpus-work-channels.md` §9.3's name-coincidence defect;
> grounded race traits 44 → 21, see `progress.md`). The **per-book pilot ingest is
> `decision-blocked`**: all 9 of `inner_sea_intrigue`'s `race_trait`-kinded units come from
> `isi_abilities_race_companion.lst` and are Clockwork Familiar / Clockwork Spy *construct-companion*
> abilities, not racial traits of any player race — `file_kind()` types that file `race_trait` by
> filename. The unit count (9) was right; the kind was not. This is loop-instruction.md's named hard
> stop ("a book's derived shape contradicts its recorded ingest subtype — the cycle reports; the
> operator re-pins the book list"), so no pilot book was substituted unilaterally.
>
> **Re-pin candidates**, smallest first, re-derived from `docs/work-inventory.json` excluding
> `*companion*.lst` sources: `ultimate_intrigue` (3), `ultimate_magic` (3), `inner_sea_bestiary` (4),
> `ultimate_combat` (4), `monster_codex` (14), `bestiary` (21). `monster_codex` is the recommended
> pick — it is the book DoD item 6 already expects to retire the standing
> `beastiary1/race_traits` `OPEN_FINDINGS` entry, so the pilot and that retirement land together.
>
> **RE-PIN LANDED, 2026-08-11 (`sd29-racetrait-repin`, card 9, `decisions.md §43`).** The pilot
> book is **`monster_codex`**. The candidate figures above were re-derived independently and
> reproduce exactly, with two additions the note omitted (`book_of_the_damned_volume_1`/`_2` at
> **1** unit each, not 2). The decisive reason for the pick is stronger than the one recorded above:
> reading the candidates' actual unit keys rather than their counts, **five of the six carry the
> same defect that disqualified `inner_sea_intrigue`** — eidolon rows, `Racial SLA ~ …` rows,
> favoured-class rows, and monster racial abilities (Drow Noble, Rust Monster, Treant, Unicorn,
> `Template ~ +2 <Stat>`) under a `_abilities_race` filename. `monster_codex` is the only candidate
> carrying genuine **player-race** alternate racial traits, so it is not the best pilot but the only
> viable one. Card 10 is now eligible.
>
> The dispatch brief's claim that `inner_sea_bestiary` is out of scope is itself a transcription of
> a passage `loop-instruction.md` strikes through and corrects; the book is one of the 37. Nothing
> turned on it — it was not picked — but the correction is recorded (`decisions.md §43.2`).
>
> **RESOLVED IN PART, 2026-08-11 (`sd29-e6-racetrait-extend`, card 10).** The re-pin is no longer
> needed to unblock the *classifier* half: `file_kind()` now types an `_abilities_race*` basename
> that also carries a `companion`/`familiar` marker as `Companion`, so the 9 Clockwork units left
> `race_trait` for `companion` where they belong and `inner_sea_intrigue` no longer appears as a
> race-trait book at all. Corpus-wide the fix moved **9** units (`inner_sea_intrigue`, all 9) and
> added **13** companion units (`inner_sea_intrigue` 9 + `bestiary_4` 4 from
> `b4_abilities_race_ce_companion.lst`, whose rows survive the companion path that the race-trait
> row exclusion had been dropping). The **pilot's re-pin is still outstanding**, but its purpose is
> now narrower: a per-book ingest exemplar, not a classifier probe.
>
> **Card 10's own ingest half was recorded `decision-blocked` on "the engine models exactly 7 races
> (CRB's hardcoded `race_traits()`)". CORRECTED 2026-08-11 (`decisions.md §43.5`): that is true of
> ONE INSTRUMENT and false of the product.** The player surface —
> `race_resolver::load_race_corpus` → `race_trait_picker` → `list_alternate_racial_traits` — models
> **18** races read off disk at runtime (CRB 7 + B1 11; re-derived with
> `ls -d data/corpus/{core_rulebook,beastiary,advanced_race_guide}/race_trait/*/ | xargs -n1 basename | sort -u | wc -l`
> → 18). Duergar and Goblin are both in it, which is why card 9's pilot records reach a player and
> carry a passing reach claim. What is pinned to 7 is **`v06_work_inventory`'s grounding probe**:
> `race_names` from `RaceId::ALL`, `race_trait_ids` solely from `crb::race_traits()`. So ARG's 156,
> B1's 108, APG's 1 and the pilot's 5 all report `race_trait_race_not_modelled` while reaching a
> player. **Card 10's real first task is repairing that probe**, not waiting on a race chassis. Its
> blast radius (several hundred units' status across the dashboard) is why it was recorded with its
> evidence rather than attempted inside a pilot cycle.

## Cycle claims (cycle-supervisor protocol)

When a cycle claims a card:

1. Edit the card's `Status` to `IN-FLIGHT`.
2. Edit `Claimed-by` to the cycle's harness identifier.
3. Edit `Claimed-at` to the cycle's ISO-8601 timestamp.
4. Edit `Cycle-id` to the cycle's audit ID (e.g., `SD29-E4-F1-001`).
5. Append the cycle's per-cycle facts to `progress.md` (write to
   `progress.md` after writing the kanban claim; the supervisor reads
   progress.md to verify the prior cycle complete before claiming the
   next).
6. On cycle completion, edit `Status` to `COMPLETE` and append the
   completion receipt to `progress.md`.

## Operator override slot

Operator may add or remove cards directly by editing this file. Cycle
dispatch honors the post-edit state.

## Resolution to operator directives

This file is the load-bearing replacement for the Hermes `codex-tranche-9`
board (operator-confirmed 2026-08-01). When a Hermes board card is
referenced from prior doctrine (`decisions.md`, `scope-draft.md`,
`loop-instruction.md`, etc.), the reference resolves to a `kanban.md`
card id at the time of cycle dispatch.
