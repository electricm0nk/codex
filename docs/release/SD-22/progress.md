---
title: SD-22 — Content-Source Ingest (APG + ACG + Bestiary 1) + DM Toolkit + Closure Readiness — Progress
mirrors: /home/ubuntu/workspace/SD-22-content-source-ingest-and-dm-toolkit-scope-draft.md
created: 2026-07-19
snapshot_as_of: 4c8e6a4
---

# SD-22 — Progress

## SD-22 STATUS: LOOP RUNNING (cycle 1)

Loop launched 2026-07-19 per `decisions.md §5` amendments (corpus generation in-bundle,
`/batch` deferred). Running from a remote execution session — `hermes` CLI is not
available in this environment, so kanban card minting (Step 10) is recorded here as a
markdown note instead of a live board card; the operator should backfill cards on
`codex-tranche-5` from this log when next at a terminal with `hermes` available.

---

SD-22's own progress doc. Loop's claim protocol and per-cycle history live here under
`## SD-22 cycles`.

## Status matrix

| ID | Epic | row_or_kind | Description | Status | Commit |
|---|---|---|---|---|---|
| E1.1 | 1 — Identifier Cleanup | identifier:audit | `sd22_\|SD22_\|Sd22\|SD-22-[A-Z][0-9]` grep across `apps/desktop/`, `apps/desktop/src-tauri/`, `src/rules_core/` | **complete** (0 hits; defensive audit found nothing to clean) | n/a (verification-only) |
| E1.2 | 1 — Identifier Cleanup | identifier:regression_check | Per-rename tests pass | **complete (vacuous)** — no renames needed; baseline `cargo test --locked` green (14 tests, 0 failed) before Epic 3/4/5 work began | n/a |
| E2.3 | 2 — Operator Pre-Launch | prelaunch:board | `codex-tranche-5` kanban board set as SD-22 default | **complete** — `hermes kanban boards switch codex-tranche-5` ran locally 2026-07-19; persistent state file `~/.hermes/kanban/current` = `codex-tranche-5`; loop's per-invocation `hermes kanban --board codex-tranche-5` (per loop-instruction Step 10b) resolves to the same board. NB: session env `HERMES_KANBAN_BOARD=codex-tranche-4` was overriding the on-disk default until unset; not persisted in any shell init file. | n/a |
| E2.4 | 2 — Operator Pre-Launch | prelaunch:branch | `tranche/5` pushed to origin | **complete** — `git ls-remote origin tranche/5` = `233c426...` matches local HEAD | 233c426 |
| E2.5 | 2 — Operator Pre-Launch | prelaunch:no_inflight | No other `claude` processes touching `rules_tables/<book>/` | **complete** — `ps -eo pid,etime,stat,cmd \| grep claude` shows only this session's own process | n/a |
| E3.6-9 | 3 — APG ingest | ingest:apg_class | Alchemist (1/6), Cavalier (2/6), Inquisitor (3/6), Oracle (4/6), Summoner (5/6), Witch (6/6); shared spell/equipment tables | **complete — criteria 6-9, Epic 3 (APG) fully closed out.** `rules_tables/apg/mod.rs` populated, `RuleSetId::Apg` registered, all six classes' BAB/save chassis land with cross-book invariant tests (criteria 6-8). Criterion 9 lands this cycle as `apg/spell_list.rs` (4-entry bootstrap sample: Bomber's Eye/Alchemist, Burst Bonds/Inquisitor, Borrow Fortune/Oracle, Ill Omen/Witch) and `apg/equipment_tables.rs` (3-entry bootstrap sample: Iron Spike, Arrow (Blunt), Knucklebone of Fickle Fortune) — bootstrap/representative coverage per the `crb/equipment_tables.rs` precedent, not exhaustive; Summoner has no active spell record anywhere in the real corpus (its dedicated block is entirely `#`-commented out) and Cavalier casts no spells, both by design not omission. Gunslinger and Magus are permanently excluded (roster corrected to 6 real classes, commit `6923e54`). See `artifacts/apg/class_alchemist_cycle_receipt.md`, `artifacts/apg/class_cavalier_cycle_receipt.md`, `artifacts/apg/class_inquisitor_cycle_receipt.md`, `artifacts/apg/class_oracle_cycle_receipt.md`, `artifacts/apg/class_summoner_cycle_receipt.md`, `artifacts/apg/class_witch_cycle_receipt.md`, `artifacts/apg/spell_list_cycle_receipt.md`, `artifacts/apg/equipment_tables_cycle_receipt.md` | see `## Cycle log` |
| E4.10-13 | 4 — ACG ingest | ingest:acg_class | Arcanist (1/10), Bloodrager (2/10), Brawler (3/10), Hunter (4/10) of the corrected 10-class roster; "Alchemist-ACG" dropped — no real `CLASS:Alchemist` record in `acg_classes.lst`, same roster-defect shape as Gunslinger/Magus; `Slayer` added — has a real record, was missing from `decisions.md`'s stated order) | **complete (criteria 10-12 for Arcanist + Bloodrager + Brawler + Hunter)** — `rules_tables/acg/mod.rs` grown to four classes, `RuleSetId::Acg` cross-book invariant tests hold for all four. Hunter is the third ACG spellcasting class (widened `spellcasting_class.rs`'s `SPELLCASTING_CLASS_NAMES`, same allowlist as Arcanist/Bloodrager; Brawler alone used `class.rs`'s `MARTIAL_CLASS_NAMES`). See `artifacts/acg/class_arcanist_cycle_receipt.md`, `artifacts/acg/class_bloodrager_cycle_receipt.md`, `artifacts/acg/class_brawler_cycle_receipt.md`, `artifacts/acg/class_hunter_cycle_receipt.md` | see `## Cycle log` |
| E5.14-17 | 5 — Bestiary 1 ingest | ingest:beastiary1_subset | Subset 01 (CR 1: Goblin/Kobold/Orc/Skeleton/Zombie) | see cycle log | pending |
| E6.18-21 | 6 — DM Toolkit | dm:encounter, dm:party_cr | Not started (requires ≥1 book ingested) | open | — |
| E7.22-26 | 7 — Closure Epilogue | closure:* | Not started (fires last) | open | — |
| E8.27 | 8 — Build Version | version:patch_bump | Version fields set to `0.5.95` (`package.json`, `tauri.conf.json`, `Cargo.toml`) | **complete** — see `artifacts/epic_8/three_version_fields_cycle_receipt.md` | (this cycle's commit, see `## Cycle log`) |
| E8.28 | 8 — Build Version | version:build_label_format | `BUILD_PREFIX = 'Codex'` / `${BUILD_PREFIX} ${buildVersion}` format ships (inherited from SD-21 E5.26); this cycle re-anchored the format's own test fixtures from the pre-bump `Codex 0.4.94-test` literal to the current `Codex 0.5.95-test` | **complete** — see `artifacts/epic_8/build_label_format_cycle_receipt.md` | (this cycle's commit, see `## Cycle log`) |
| E8.29 | 8 — Build Version | version:closure_checklist | `docs/SD-22/release-closure-checklist.md` — four-step version-bump process, mirrors SD-21's E5.27 doc | **complete** — see `artifacts/epic_8/release_closure_checklist_cycle_receipt.md` | (this cycle's commit, see `## Cycle log`) |
| E8.30 | 8 — Build Version | version:* | Per-cycle tests pass at closure — standing verification gate (not a one-shot artifact), re-verified by every cycle's own `cargo test`/`cargo clippy` run; closed out by Epic 9's criterion-31 eval | open (standing gate; re-verified this cycle: `cargo test` all green, clippy clean) | — |
| E9.31 | 9 — Closure Readiness | closure_readiness:* | Not started (fires after Epic 8, before Epic 7) | open | — |

## Open blockers

### [SELF-HEALED IN-CYCLE 2026-07-19T20:18:28Z] E4.10-13 (Epic 4, "Alchemist (ACG-side)" row 1) — `corpus-source-inventory.md §2.1`'s ACG roster is wrong for this row: no `CLASS:Alchemist` record exists anywhere in `acg_classes.lst`

**Same defect shape as the resolved Gunslinger/Magus blocker below.** Before
writing any RED test for Epic 4's first cycle, verified the real
`acg_classes.lst` directly (not `corpus-source-inventory.md §2.1`'s
"Content shape" prose, which that file's own corrective banner already
marks non-authoritative — but the *class roster itself*, like the
Gunslinger/Magus case, turned out to be a routing-level defect too, not
just illustrative prose):

```
$ grep -n "^CLASS:Alchemist" acg_classes.lst
(0 hits)

$ grep -oP "^CLASS:\K[A-Za-z-]+" acg_classes.lst | sort -u
Arcanist
Bloodrager
Brawler
Ex-Warpriest
Hunter
Investigator
Shaman
Skald
Slayer
Swashbuckler
Warpriest
```

Alchemist is APG-only content (already ingested in Epic 3); ACG never
republishes a distinct Alchemist chassis — there is no ACG-side
Alchemist bomb/archetype chassis in the real corpus at all, contradicting
`corpus-source-inventory.md §2.1` row 1's "Content shape" text describing
one. Separately, `decisions.md`'s recorded ACG class order ("Alchemist →
Arcanist → Bloodrager → Brawler → Hunter → Investigator → Shaman → Skald →
Swashbuckler → Warpriest") both wrongly includes "Alchemist" **and**
omits `Slayer`, which does have a real `CLASS:Slayer` record
(`acg_classes.lst:327`) — the roster is off by one in two different ways
that happen to cancel out to the same total count (10), which is likely
why it wasn't caught until a cycle checked the actual `.lst` roster
directly.

**Self-healed in-cycle, not left as a standing blocker**, because — unlike
Gunslinger/Magus, which required an operator judgment call on excluded
book scope — this is a pure roster-correctness fact fully resolvable by
reading the real corpus, with a clear, unambiguous corrected 10-class list
and an obvious next-eligible class (Arcanist, the first class in both the
real file's line order and `corpus-source-inventory.md §2.1`'s own
existing row order). This cycle did not fabricate an Alchemist-ACG
chassis; it proceeded directly to Arcanist and logged this entry for the
audit trail. `corpus-source-inventory.md §2.1`, `decisions.md`'s stated
ACG ordering, and `epic-breakdown.md`'s ACG class list still need an
operator/doc-correction pass (mirroring commit `6923e54`'s APG roster fix)
to formally replace "Alchemist (ACG-side)" with `Slayer` in the row list
— left as a follow-on note; not blocking further Epic 4 per-class cycles,
which can keep using the real corpus roster (Arcanist, Bloodrager,
Brawler, Hunter, Investigator, Shaman, Skald, Slayer, Swashbuckler,
Warpriest) directly regardless of whether the doc text is corrected first.

### [RESOLVED 2026-07-19T09:43:58-04:00, commit `6923e54`] E3.6-9 (Epic 3, Gunslinger + Magus specifically) — `corpus-source-inventory.md` §1.1's 8-class APG roster is wrong for these 2 rows: neither class has a real record anywhere under `advanced_players_guide/`

**Resolution:** the operator landed option 1 of the three recommended below
(commit `6923e54`, `docs(sd22): correct APG roster to 6 real classes`):
narrowed Epic 3's class count from 8 to 6 real APG classes (Alchemist,
Cavalier, Inquisitor, Oracle, Summoner, Witch), corrected everywhere the
8-class roster appeared (`corpus-source-inventory.md`, `epic-breakdown.md`,
`decisions.md`, `risks-and-open-questions.md` Flag A,
`scope-draft.md`, `technical-design.md`, `acceptance-and-verification.md`).
Gunslinger and Magus are not blocked — they are genuinely not APG's job;
they are Ultimate Combat / Ultimate Magic content. Left below for the
audit trail per this file's own "edit in place, don't rewrite" convention.

Discovered 2026-07-19T16:00:00Z, cycle 4 (Inquisitor), while attempting the
next class in the operator-pinned ordering (Alchemist → Cavalier →
**Gunslinger** → Inquisitor → **Magus** → Oracle → Summoner → Witch).
Before writing any RED test, checked the real record — and found no
`CLASS:Gunslinger` line anywhere in `apg_classes.lst` (`grep -n
"Gunslinger" apg_classes.lst` → 0 hits, not even outside a `CLASS:` line).
Searched the whole PCGen tree: `CLASS:Gunslinger` exists only in
`ultimate_combat/uc_classes.lst`; `CLASS:Magus` exists only in
`ultimate_magic/um_classes.lst`. Neither is APG content in the real
corpus — they're Ultimate Combat and Ultimate Magic content respectively.

`apg_classes.lst`'s actual `CLASS:` roster (every line, confirmed by
listing them all) is exactly 6 classes: Alchemist, Cavalier, Inquisitor,
Oracle, Summoner, Witch. That matches the real, published Pathfinder 1e
Advanced Player's Guide's actual class list (6 base classes) — Gunslinger
and Magus are real PF1 classes, but from different books entirely.

This is **not** the same shape as the earlier (now-resolved)
parser-allowlist blocker: this isn't "the record exists but the parser
doesn't recognize it yet." The record does not exist under
`advanced_players_guide/` at all. Per `loop-instruction.md`'s
SD-22-specific hard stop ("the specific record isn't present in the
resolved tree"), this routes straight to `## Open blockers` — there is no
mechanical unblock available (widening a parser allowlist can't manufacture
a `.lst` line that doesn't exist).

**Also material:** `decisions.md §1` explicitly and repeatedly states
Ultimate Combat / Ultimate Magic are **not** in SD-22's scope ("SD-22 does
NOT own Ultimate Combat / Ultimate Magic / any other 'Ultimate'-line
book"). So this isn't just a missing-source problem solvable by reaching
into `ultimate_combat/` or `ultimate_magic/` for the record — doing that
would itself violate the bundle's own recorded scope boundary. The
`corpus-source-inventory.md` §1.1 table's 8-class APG list is the thing
that's wrong: 2 of its 8 rows (Gunslinger, Magus) name content from
explicitly out-of-scope books under the APG epic. This is a routing-table
defect, not just the "Content shape" prose the file's corrective banner
already flagged as non-authoritative — the banner said routing columns
"remain valid and authoritative," but the class roster itself (which
class belongs to which book) is a routing-level fact, and it's wrong here.

**Not self-healing this inline.** No commit lands for Gunslinger or Magus
specifically this cycle (Inquisitor, the next-eligible class in the
ordering, landed instead — see cycle log below). Recommend, operator's
call:
1. Narrow Epic 3's class count from 8 to 6 (drop Gunslinger and Magus from
   `corpus-source-inventory.md` §1.1 and `epic-breakdown.md`'s class list),
   matching the real APG's actual 6-class roster, or
2. Explicitly expand SD-22's scope to add Ultimate Combat + Ultimate Magic
   as new source books (contradicts `decisions.md §1`'s explicit
   exclusion; would need its own operator directive to amend that
   decision), or
3. Re-route Gunslinger to a future Ultimate-Combat-scoped epic and Magus
   to a future Ultimate-Magic-scoped epic in a later bundle, leaving
   Epic 3's own 8-class list as aspirational-but-not-APG's-job for 2 of
   its rows.

Oracle (class 5 of 8 in the existing ordering) is next-eligible for Epic 3
regardless of which option the operator picks — Oracle, Summoner, and
Witch all have real records in `apg_classes.lst` (confirmed: `CLASS:Oracle`
at line 107, `CLASS:Summoner` at line 139, `CLASS:Witch` at line 172).

### E3.6-9 / E4.10-13 / E5.14-17 (Epics 3/4/5, first cycle of each) — real LST source exists, but the existing `pcgen_import` parsers do not recognize these records (new parsing code required, out of this cycle's file-touch partition)

`decisions.md §5` (corrected 2026-07-19, commit `9cd7708`) resolved the *prior* blocker below: a real, reachable corpus source does exist (`/home/user/pcgen` — a live checkout of `https://github.com/PCGen/pcgen`, confirmed this cycle via `git remote -v`), and `apg_classes.lst` does contain a real `CLASS:Alchemist` record (confirmed this cycle: `apg_classes.lst:11`). So the *original* fabrication-risk framing no longer applies, and this cycle re-opened E3.6-9 to attempt it rather than re-logging the same NO-OP.

Before writing any RED test, this cycle read `decisions.md §5`'s ingest-shape instructions and verified the specific claim that "no new parsing code is needed" against the actual parser source (not from memory):

- `src/pcgen_import/lst_parser/class.rs`'s `CLASS:` parser (`parse_class_entries`) is hard-scoped to `MARTIAL_CLASS_NAMES = ["Fighter","Barbarian","Monk","Rogue","Ranger","Paladin"]` (class.rs:25-26). A `CLASS:Alchemist` line is silently skipped — "Out of scope: skip the line entirely. No diagnostic, no record" (class.rs:252-257). Confirmed by reading the parser's own doc comment (class.rs:1-12): scope is explicitly "the six martial classes named in the [SD-17 B-1] slice card."
- `src/pcgen_import/lst_parser/spellcasting_class.rs` (the only other `CLASS:` parser) is hard-scoped to `SPELLCASTING_CLASS_NAMES = ["Cleric","Druid","Wizard","Sorcerer","Bard"]` (spellcasting_class.rs:47-48) — also excludes Alchemist and every other APG/ACG class.
- Together these two parsers cover exactly the 11 classes in `rules_tables/crb/class_tables.rs`'s `CLASS_META` (the CRB roster) and nothing else. No APG class name and no ACG class name is in either allowlist. `acg_classes.lst` was checked too (`grep CLASS:Alchemist` — 0 hits, correctly: Alchemist is APG-only; ACG's own classes — Arcanist, Bloodrager, etc. — are equally absent from both allowlists).
- The Bestiary 1 case is not better: `src/pcgen_import/lst_parser/race_ability.rs`'s `parse_lst_entry` only recognizes `RACE:`/`RACES:` *pointer* lines (an include-target string, used in PCC files) and `ABILITY:` lines — its own doc comment (race_ability.rs:19-29) scopes it to that pointer/ability shape only. `b1_races.lst`'s actual monster records (confirmed this cycle: `b1_races.lst:9-15`, e.g. `Aboleth\tSTARTFEATS:1\tSIZE:H\t...`) are bare tab-delimited rows with the race name as the unprefixed first field — no `RACE:` key prefix at all (`grep -c "RACE:" b1_races.lst` → 0). `race_ability.rs` would extract zero records from this file.
- `src/pcgen_import/lst_parser/metadata.rs` (the fourth parser) is scoped to six unrelated directive kinds (`DEITY:`, `DOMAIN:`, `KITS:`, `LANGUAGE:`, `TEMPLATE:`, `COMPANIONMOD:` — metadata.rs:4-5) and explicitly disclaims `CLASS:`/`RACE:` (metadata.rs:22-23).
- Also checked whether `rules_tables/crb/class_tables.rs` (the file `decisions.md §5` says this cycle's output should match "same shape as") itself calls any of these parsers at runtime: it does not. Its own doc comment (class_tables.rs:1-16) says its BAB/save cells come from `pilot_compute.rs`'s hand-implemented formula functions, not from `lst_parser`/`ir_converter` output, and that "named per-level features and exact spell-per-day cells are deliberately out of scope." So the precedent this cycle was told to mirror was never itself an LST-parser consumer for class data.

**Conclusion:** `decisions.md §5`'s "no new parsing code is needed; APG/ACG/Bestiary-1 are new *inputs* to an engine that already exists" is not accurate for any of the three book epics as the parsers exist today. Making Epic 3/4/5's first cycle land would require extending `class.rs`'s and/or `spellcasting_class.rs`'s class-name allowlist (or adding a new APG/ACG-scoped class parser module) and writing a new bare-tab-delimited race/monster parser for `race_ability.rs`'s gap — i.e., real new parsing code inside `src/pcgen_import/`. That is:
1. Outside this cycle's (and every SD-22 cycle's) file-touch partition, which scopes Epic 3/4/5 cycles to `rules_tables/<book>/*.rs` + `tests/sd22_*.rs` only — `src/pcgen_import/` is not a partition any SD-22 cycle owns.
2. A nontrivial, multi-way design decision (which classes to add to which allowlist vs. a new parser module; whether the existing SD-17 martial/spellcasting split stays meaningful once APG/ACG classes are added; how to shape a monster-stat-block parser for the unprefixed-row format) that a single bounded autonomous cycle should not improvise per `AGENTS.md`'s Role Boundaries ("[upstream planning artifacts] define intent and constraints, not permission to improvise beyond the bounded run").

**Not self-healing this inline.** No commit lands this cycle; no parser code was written. Recommend, operator's call:
1. Scope a dedicated SD-22 (or SD-17-follow-on) cycle/criterion specifically to extend `src/pcgen_import/lst_parser/{class,spellcasting_class}.rs`'s allowlists (or add sibling APG/ACG-scoped parser modules) and to add a bare-row race/monster parser, with its own RED/GREEN tests against the SD-17 parser test suite (`tests/sd17_b_*`) so the existing 6+5-class scope doesn't regress; then re-open Epic 3/4/5's first cycles against the now-real engine, or
2. Amend `decisions.md §5` to explicitly fold "extend `pcgen_import`'s class/race parsers to cover APG/ACG/Bestiary-1 record shapes" into Epic 3/4/5's own file-touch partition and cycle shape (since as written, no epic's partition currently owns `src/pcgen_import/`), or
3. Explicitly re-affirm a narrower Epic 3/4/5 acceptance shape that only requires data derivable without new parsing (e.g., transcribing raw token key/value pairs already visible in the `.lst` text by simple line-splitting in the new `rules_tables/<book>/*.rs` module itself, without going through `pcgen_import`) — mirroring how `crb/class_tables.rs` itself never actually depended on the LST parser pipeline.

This supersedes the corpus-generation/fabrication-risk framing below (that framing assumed no real source existed at all; a real source now exists and this cycle used it to test the actual claim in `decisions.md §5`, rather than re-deriving the same already-resolved objection). The original entry is left below for the audit trail per this file's own "edit in place, don't rewrite" convention — do not re-read it as the live blocker; the live blocker is this entry.

---

### [SUPERSEDED 2026-07-19 — see entry above] E3.6-9 (Epic 3, Alchemist, cycle 3) — corpus generation would require fabricating unverifiable game content

`corpus-source-inventory.md` §1.1 and `decisions.md §5` direct this cycle to
"generate `corpus/apg_alchemist.json` from PF1 OGL/SRD content" by having the
model recall and transcribe the APG Alchemist class table (bomb list,
discoveries, spell progression, etc.) from memory, with no in-repo source file
and no operator-supplied corpus. Before writing anything, this cycle tried to
ground that content against a real source:

- `WebFetch` to `aonprd.com` (Archives of Nethys) → **HTTP 403**
- `WebFetch` to `d20pfsrd.com` → **HTTP 403**

Neither OGL/SRD mirror is reachable from this sandbox, and no corpus or
reference file exists in-repo (`corpus/` doesn't exist; `docs/release/SD-22/artifacts/`
holds only its README). That leaves one path to close this criterion: transcribe
the Alchemist's bombs/discoveries/spell-list content purely from the model's own
training-data recall and commit it to `tranche/5` as if it were verified SRD
data.

This repo already has a documented precedent against exactly that move.
`src/rules_core/rules_tables/crb/class_tables.rs`'s header comment (SD-19)
explicitly scoped CRB's class tables down to BAB/save formulas and left out
named per-level features and spell-per-day cells for this reason, in its own
words: *"hand-transcribing exhaustive per-level feature text without a
verifiable in-repo source would be exactly the fabricated-data risk `AGENTS.md`
rules out."* `AGENTS.md`'s non-negotiable rules (`## Non-Negotiable Rules`,
esp. "No fake completion" and "Fix the source, not the symptom") apply
repo-wide and aren't something a bundle-local planning doc can waive for
itself — per `AGENTS.md`'s own "Role Boundaries": upstream planning artifacts
"define intent and constraints, not permission to improvise beyond the
bounded run."

`decisions.md §5` / `risks-and-open-questions.md` frame "missing corpus file"
as always self-healable by in-cycle generation from memory. That framing is
what's in tension with `AGENTS.md` here — a missing *file* is self-healable;
a missing *verifiable source* for detailed rules-text content is not the same
problem, and self-healing it by fabricating the content is the thing
`AGENTS.md` and the CRB precedent both rule out.

**Not self-healing this inline.** No commit lands this cycle. Recommend one of,
operator's call:
1. Supply a real corpus/reference file (e.g. a licensed text dump or a
   reachable SRD mirror) so the cycle has something verifiable to transcribe
   against, or
2. Narrow Epic 3/4/5's acceptance shape to formula-derivable data only
   (BAB/saves/simple numeric progressions), mirroring the CRB precedent, and
   drop the named-item/named-feature resolution requirements from
   `corpus-source-inventory.md` §1.1/§1.3, or
3. Explicitly re-affirm (outside this bundle's own self-referential docs) that
   memory-recalled OGL content is acceptable here, accepting the fabrication
   risk knowingly.

Logged as a real `## Open blockers` entry per the loop-instruction's hard-stop
clause (unresolvable source ambiguity), rather than force a cycle forward.
E1.1, E1.2, E2.3, E2.4, E2.5 remain **complete** (see cycle log above) — this
blocker is scoped to Epic 3 onward (and, by the same content shape, Epic 4 and
Epic 5, which will hit the identical wall on their first cycles).

## Cycle log

### cycle-2026-07-19T00:00:00Z | Epic 1 + Epic 2 pre-flight | n/a (verification-only) | no card (hermes unavailable; logged here) | open → **complete** (E1.1, E1.2, E2.4, E2.5); E2.3 → **blocked (environment)**

Ran the Epic 1 identifier-audit grep gate scoped to SD-22-specific patterns
(`sd22_|SD22_|Sd22|SD-22-[A-Z][0-9]`) across `apps/desktop/`, `apps/desktop/src-tauri/`,
`src/rules_core/` — zero hits. (The broader `sd[0-9]+_` pattern in the criterion's
verification command also matches pre-existing `sd19_*`/`sd13_*`/`sd16_*` identifiers
from already-shipped, unrelated spec domains — those are out of Epic 1's scope per
`epic-breakdown.md`'s own scope-doctrine note and AGENTS.md's no-scope-expansion rule;
not touched.) Ran baseline `cargo test --locked` — 14 tests passed, 0 failed, confirming
a clean starting tree before Epic 3/4/5 cycles begin. Verified `tranche/5` is pushed to
origin (E2.4) and no other `claude` processes are in-flight (E2.5). E2.3 (kanban board)
requires operator-local `hermes`, unavailable here — recorded as a blocker, non-gating.

### cycle-2026-07-19T03:50:00Z | Epic 2 follow-up: E2.3 + receipts-doctrine amendment | n/a (operator-local + doctrine) | no card (operator-local action; amendment commits land as `1df00d0` and `3c9fa6a`) | E2.3 → **complete**; no other row touched

Operator ran `hermes kanban boards switch codex-tranche-5` from a local terminal with
`hermes` available — the persistent state file `~/.hermes/kanban/current` now reads
`codex-tranche-5`. The loop's per-invocation `hermes kanban --board codex-tranche-5`
calls (loop-instruction Step 10b) will resolve to the same board. One snag: the
session's `HERMES_KANBAN_BOARD=codex-tranche-4` env var was masking the on-disk default
in `hermes kanban boards current` output; the env var is not in any shell init file,
so it is session-scoped only and will not survive into the next launched loop session.
Loop launch will need either `unset HERMES_KANBAN_BOARD` first, or to rely on the
explicit `--board codex-tranche-5` flag (which is what Step 10b does already, so the
loop is correct as written).

Between cycles, the operator landed a doctrine amendment on top of cloud cycle 1:
- `1df00d0 feat(sd22): repo-resident receipts.md + Step 10a/10b split` — adds
  `docs/release/SD-22/receipts.md` (durability backbone for cloud cycles) and splits
  Step 10 into 10a (always-write the repo-resident receipt) and 10b (best-effort
  kanban card mint). Cycle-receipt schema lives at the top of `receipts.md`.
- The amendment post-dates the cloud cycle that wrote `progress.md`, so the cycle
  log here does not retroactively reference Step 10a. Future cycles will.

No Epic 3/4/5 cycles have started yet (correct per dependency graph: Epic 1 vacuous
done, Epic 2 fully done as of this entry). Loop is ready for the first ingest cycle
on next restart.

### cycle-2026-07-19T04:00:00Z | Epic 3, Alchemist (cycle 1 of 8) | ingest:apg_class | no card (blocked, no commit) | open → **blocked**

Attempted Step 4's RED-phase reading for the Alchemist cycle. `corpus-source-inventory.md`
§1.1 and `decisions.md §5` call for generating `corpus/apg_alchemist.json` from
"PF1 OGL/SRD content" in-cycle since no corpus file exists yet. Tried to ground
that against a real source before writing anything: `WebFetch` to `aonprd.com`
and to `d20pfsrd.com` both returned HTTP 403 (unreachable from this sandbox).
With no in-repo corpus/reference file either, the only way to produce the
content this criterion wants (bomb list, discoveries, spell progression, named
class features) is to transcribe it from the model's own training-data recall
and commit it as if it were verified SRD data — which is the exact fabrication
risk `src/rules_core/rules_tables/crb/class_tables.rs`'s own SD-19 doc comment
says it deliberately avoided, citing `AGENTS.md`. Did not write `corpus/`,
`src/rules_core/rules_tables/apg/`, or any `tests/sd22_apg_*` files this cycle.
No commit landed. Full detail and recommended paths forward in `## Open
blockers` above. `cargo test --locked` was not re-run since no production code
changed this cycle (last known-green baseline: 14/14, recorded in the E1+E2
pre-flight cycle above).

### cycle-2026-07-19T05:02:04Z | Epic 8, criterion 27 (three version fields) | version:patch_bump | no card (hermes unavailable; logged here + `receipts.md`) | open → **complete**

Re-checked the Epic 3 Alchemist blocker before picking a criterion: nothing has
changed (`decisions.md §5` still frames "missing corpus file" as always
self-healable by memory-recall from PF1 OGL/SRD content, which is still in
tension with `AGENTS.md`'s "No fake completion" rule and the
`crb/class_tables.rs` precedent). Epic 4 and Epic 5 would hit the identical
wall on their first cycles (same corpus-generation instruction, same absence
of a verifiable source), so did not re-attempt Epic 3/4/5 this cycle. Epic 6
needs ≥1 book ingested (blocked transitively). Per Step 1's priority order,
picked the next eligible, non-transitively-blocked criterion: Epic 8's
criterion 27, which `loop-instruction.md`'s file-touch-partition section
documents as independent of Epics 1-6, and which is a mechanical version bump
with a derivable (not invented) target value.

RED: added `apps/desktop/src/sd22/buildVersionTriple.test.ts` (mirrors SD-21's
`sd21/buildVersionTriple.test.ts`), asserting the version triple starts with
`0.5.`; failed against the pre-bump `0.4.94` tree for the intended reason.
GREEN: bumped `apps/desktop/package.json`, `apps/desktop/src-tauri/tauri.conf.json`,
and `apps/desktop/src-tauri/Cargo.toml` from `0.4.94` to `0.5.95` (major=0 until
first main-publish; tranche=5 for `tranche/5`; build=95, the next monotonic
counter after SD-21's last committed build of 94 on this line per
`decisions.md §2`). Re-ran `npm install` to re-sync `package-lock.json`'s
embedded version (it had already drifted to a stale `0.1.0` pre-cycle).

One sibling regression surfaced and was fixed in the same commit:
`apps/desktop/src/sd21/buildVersionTriple.test.ts` (inherited onto `tranche/5`
via the `aea478c` merge) hard-codes an assertion that the tranche stays at 4
"until promoted" — `tranche/5` *is* that promotion, so the assertion was stale,
not a real regression from this change. Updated its anchor from `0.4.` to
`0.5.` with an explanatory comment rather than leave a known-broken sibling
test on the branch (sibling-preservation + AGENTS.md's "fix the source, not
the symptom").

Verification: `npm test` 46/46 JS test files green (including the new
`sd22/buildVersionTriple.test.ts` and the fixed `sd21/buildVersionTriple.test.ts`).
`cargo test --locked` at repo root (independent Cargo package from
`apps/desktop/src-tauri`) — all suites green, 0 failures. `cargo clippy --locked
--tests -- -D warnings` clean. `cargo check` on `apps/desktop/src-tauri` itself
fails in this sandbox on missing GTK system libs (`gdk-3.0` via pkg-config) —
pre-existing environment limitation unrelated to this change; it got far enough
to resolve and rewrite `Cargo.lock`'s `codex-desktop` entry to `0.5.95` before
failing at the native-linking stage.

Full RED/GREEN evidence, file list, and reasoning:
`artifacts/epic_8/three_version_fields_cycle_receipt.md`. Receipt block appended
to `receipts.md`. Criterion 28 (build-label format) was NOT touched or marked
complete this cycle — `createSd11WorkbenchStatus.ts` already carries the
`BUILD_PREFIX = 'Codex'` / `${BUILD_PREFIX} ${buildVersion}` shape from SD-21's
E5.26, but a future cycle should explicitly verify and close it rather than
this cycle assuming it.

### cycle-2026-07-19T06:15:00Z | Epic 8, criterion 28 (build-label format fixtures) | version:build_label_format | no card (hermes unavailable; logged here + `receipts.md`) | open → **complete**

Re-checked the Epic 3/4/5 corpus-generation blocker first: `corpus/` still
doesn't exist and no SRD mirror is reachable from this sandbox, so nothing
has changed and re-attempting those epics would just re-log the same
blocker. Epic 6 remains transitively blocked (needs ≥1 book ingested). Per
Step 1's priority order, picked Epic 8's remaining open item: criterion 28,
which the prior cycle's receipt explicitly flagged as verified-but-not-closed
(the `Codex ${buildVersion}` format already ships via SD-21 E5.26, but its
own test fixtures still hard-coded the pre-bump `Codex 0.4.94-test` literal).

`node_modules` was missing at cycle start (all 46 JS test files failed for
an environment reason); ran `npm install` to restore it, confirming a clean
46/46 baseline before touching anything.

RED: added `apps/desktop/src/sd22/buildLabelFixtureFreshness.test.ts`,
scanning the three fixture files named in `loop-instruction.md`'s file-touch
partition for the pre-bump literal and asserting each carries
`Codex <current package.json version>-test` instead. Ran against the
pre-edit fixtures (re-verified via `git stash`) — failed for the intended
reason: `"...loadSd11TesterWorkbenchSurface.test.ts still carries the
pre-bump build-label fixture \"Codex 0.4.94-test\""`. (An earlier draft used
a blanket regex that false-positived on an unrelated arbitrary-input fixture,
`'Codex 0.0.0-test'`, used by `createSd11WorkbenchStatus.test.ts`'s
`verifiesLinuxAlphaStatusTruth` case; narrowed to the specific known-stale
literal before trusting RED.)

GREEN: re-anchored `sd11/loadSd11TesterWorkbenchSurface.test.ts`,
`sd11/status/createSd11WorkbenchStatus.test.ts`, and `testSupport/makeSurface.ts`
from `Codex 0.4.94-test` to `Codex 0.5.95-test`. One sibling regression
surfaced from `makeSurface.ts` being the shared fixture factory: four
consumer test files (`sd11/feedback/bug/composeBugReport.test.ts`,
`sd11/feedback/enhancement/composeEnhancementRequest.test.ts`,
`sd11/feedback/evidence/captureFeedbackEvidence.test.ts`,
`sd15/buildSd15OperatorTriageDraft.test.ts`) independently hard-coded the
same stale literal in their own assertions and broke as a direct,
mechanical consequence of this cycle's edit — fixed in the same commit per
sibling-preservation + AGENTS.md's "fix the source, not the symptom," even
though they're outside Epic 8's file-touch partition.

Verification: `npm test` 47/47 green. `cargo test --locked` at repo root —
all suites green, 0 failures (unaffected; this criterion is JS-only).
`cargo clippy --locked --tests -- -D warnings` clean.

Full RED/GREEN evidence, file list, and reasoning:
`artifacts/epic_8/build_label_format_cycle_receipt.md`. Receipt block
appended to `receipts.md`. Next-eligible: Epic 8 criterion 29
(`docs/SD-22/release-closure-checklist.md`) — untouched this cycle.

### cycle-2026-07-19T07:00:00Z | Epic 8, criterion 29 (release closure checklist doc) | version:closure_checklist | no card (hermes unavailable; logged here + `receipts.md`) | open → **complete**

Re-checked the Epic 3/4/5 corpus-generation blocker first: `corpus/` still
doesn't exist and no SRD mirror is reachable from this sandbox — nothing
has changed since the blocker was logged, so re-attempting those epics
would just re-log the same fabrication-risk wall. Epic 6 remains
transitively blocked (needs ≥1 book ingested). Per Step 1's priority
order, picked Epic 8's remaining open item: criterion 29.

`node_modules` was missing at cycle start; ran `npm install` to restore it.

RED: added `apps/desktop/src/sd22/releaseClosureChecklistDoc.test.ts`
(mirrors SD-21's `sd21/releaseClosureChecklistDoc.test.ts`), asserting
`docs/SD-22/release-closure-checklist.md` exists and names all four steps
(three version files, workflow stamp, build-label check, `cargo check`,
the `feat(sd22): bump version to` commit shape, the
`<major>.<tranche-base>.<build>` triple). Failed for the intended reason:
the doc didn't exist yet.

GREEN: added `docs/SD-22/release-closure-checklist.md`, mirroring SD-21's
doc content with `<tranche>` renamed to `<tranche-base>` (matching
`decisions.md §2`'s terminology), the worked example updated to `0.5.95`
(this branch's current version, landed by criteria 27/28), and the
commit-message shape changed to `feat(sd22):`.

Verification: `npm test` 48/48 green. `cargo test --locked` at repo root —
all suites green, 0 failures (unaffected; this criterion is docs+JS-only).
`cargo clippy --locked --tests -- -D warnings` clean.

One note, not fixed this cycle: `.github/workflows/publish-tester-release.yml`'s
stamp line still reads `VERSION="0.4.${GITHUB_RUN_NUMBER}"` — one tranche
behind the `0.5.95` already in the three repo version files. Not in Epic
8's file-touch-partition scope; flagged in the cycle artifact as a
candidate Epic 9 self-heal item (mechanically verifiable drift, not a
judgment call).

Criterion 30 ("per-cycle tests pass at closure") is a standing
verification gate re-verified by every cycle's own `cargo test`/`cargo
clippy` run (including this one), not a one-shot artifact — left `open`
in the status matrix pending Epic 9's criterion-31 eval closing it out.

Full RED/GREEN evidence, file list, and reasoning:
`artifacts/epic_8/release_closure_checklist_cycle_receipt.md`. Receipt
block appended to `receipts.md`. All of Epic 8's file-touch-partition-scoped
criteria (27, 28, 29) are now complete. Next-eligible: Epic 3/4/5 remain
blocked; Epic 6 transitively blocked; Epic 9 (criterion 31) is now
eligible per Step 1's priority order (fires after Epic 8's criterion-30 is
`complete` per `epic-breakdown.md` line 179 — criterion 30 is the standing
gate discussed above, satisfied by this cycle's own green run, so Epic 9
could reasonably start next cycle) but Epic 9 fires "after Epic 8 lands,"
and Epic 8's own criteria 27-29 (the three file-touch-partition-scoped
ones) are now all `complete` — a future cycle should make the explicit
call on whether Epic 9 is now unblocked or whether criterion 30 needs its
own discrete landing first.

### cycle-2026-07-19T08:00:00Z | scheduled loop firing | n/a (verification-only, no production change) | no card (NO-OP, nothing to mint) | no row transition

Re-checked the state this cycle inherited before picking a criterion:

- `corpus/` still does not exist anywhere in the repo; no operator-supplied
  reference/corpus file has been added since the E3.6-9 blocker above was
  logged.
- That blocker's own text already states the identical-wall reasoning
  applies to Epic 4 (`E4.10-13`) and Epic 5 (`E5.14-17`) first cycles, not
  just Epic 3 — so attempting either this cycle would just re-derive the
  same "no fake completion" conflict (`AGENTS.md`) against the same
  unresolved `decisions.md §5` fabrication-risk trade-off, for no new
  information.
- Epic 6 depends on ≥1 book ingested (still none). Epic 8's discrete
  criteria (27-29) are `complete`; E8.30 is a standing re-verification
  gate, not a fresh unit of work. Epic 9 and Epic 7 are both gated behind
  Epic 3/4/5/6 closing.

No criterion this cycle is both unclaimed and un-blocked. Per the
loop-instruction's own exit condition ("if every criterion is already
complete or already has a real `## Open blockers` entry, exit NO-OP
immediately... do not force work"), this firing lands no production
change and mints no fabricated content. Recommending (again) that the
three options listed under the E3.6-9 blocker get an explicit operator
decision — the loop will otherwise NO-OP every firing indefinitely
without one.

### cycle-2026-07-19T08:56:08Z | scheduled loop firing | n/a (verification-only, no production change) | no card (NO-OP, nothing to mint) | no row transition

Re-checked state; nothing has changed since the prior cycle's NO-OP:

- `corpus/` still does not exist in the repo; no operator-supplied
  reference/corpus file has appeared.
- Re-attempted `https://www.aonprd.com/` directly (not just d20pfsrd.com) —
  still HTTP 403 Forbidden. No SRD/OGL mirror is reachable from this
  sandbox.
- Epic 3/4/5 remain blocked for the identical reason; Epic 6 remains
  transitively blocked; Epic 8's discrete criteria (27-29) remain
  complete; Epic 9/7 remain gated behind Epic 3/4/5/6.

This is the second consecutive NO-OP firing on the exact same blocker,
which the prior cycle explicitly predicted would keep happening without
an operator decision. Per that prediction, this firing surfaces the stall
to the operator directly (push notification) rather than silently
NO-OPing indefinitely. No production change, no fabricated content.

### cycle-2026-07-19T13:00:00Z | Epic 3, Alchemist (cycle 3, re-attempt after §5 correction) | ingest:apg_class | no card (blocked, no commit) | open → **blocked (new, more specific reason)**

`decisions.md §5` was corrected (commit `9cd7708`, landed since the last
cycle) to say the real corpus source is PCGen's `.lst` data via the
existing `pcgen_import` engine, and that no new parsing code is needed.
Since that changes the premise of the standing E3.6-9 blocker, this cycle
re-opened Epic 3's Alchemist criterion instead of re-logging a NO-OP.

Verified the corrected premise before writing any RED test: confirmed
`/home/user/pcgen` is a real, reachable checkout of `github.com/PCGen/pcgen`
and that `apg_classes.lst` has a real `CLASS:Alchemist` record (line 11) —
so the *original* blocker (no verifiable source) is genuinely resolved.
But reading the actual parser source (not `decisions.md`'s description of
it) found the "no new parsing code needed" half of the correction does
not hold: `lst_parser::class`'s `CLASS:` parser and `lst_parser::
spellcasting_class`'s `CLASS:` parser are both hard-scoped by name
allowlist to the 11 CRB classes only (silently skip anything else, no
diagnostic); `lst_parser::race_ability`'s `RACE:`/`RACES:` parser expects
pointer-style lines, but `b1_races.lst`'s actual monster records are
unprefixed bare tab-delimited rows (0 matches for `RACE:` in that file);
`lst_parser::metadata` is scoped to six unrelated directive kinds. None of
the four existing parsers can ingest APG/ACG classes or Bestiary 1
monsters as they're actually shaped in the corpus today. Full evidence
(file:line citations) recorded in the `## Open blockers` entry above,
which supersedes the prior (now-resolved) fabrication-risk entry.

Did not write any `rules_tables/apg/*`, `tests/sd22_apg_*`, or
`src/pcgen_import/*` files this cycle — extending the parsers is real new
code outside every SD-22 epic's file-touch partition and is a multi-way
design decision, not a mechanical unblock. No commit lands. Re-verified
Epic 4 (`acg_classes.lst` has no `CLASS:Alchemist`, confirming ACG's own
classes are equally outside the allowlist) and Epic 5 hit the identical
parser-coverage wall, not per-epic-distinct issues — so did not
separately re-attempt their first cycles this firing. Epic 6 remains
transitively blocked. Epic 8's three file-touch-partition criteria (27-29)
remain complete; criterion 30 remains a standing gate. Epic 9/7 remain
gated behind Epic 3/4/5/6. `cargo test --locked` re-run at cycle start as
a clean-tree baseline check: all suites green, 0 failures (unaffected;
no production code touched this cycle).

This is new, actionable information for the operator (a corrected,
narrower, mechanically-specific blocker — not a repeat of the prior
fabrication-risk stall), so this cycle sends a push notification rather
than treating it as a duplicate of the earlier stall alert.

### cycle-2026-07-19T09:53:00Z | scheduled loop firing | n/a (verification-only, no production change) | no card (NO-OP, nothing to mint) | no row transition

Re-checked state; nothing has changed since the prior cycle's NO-OP:

- `corpus/` still does not exist in the repo.
- `decisions.md` and `risks-and-open-questions.md` are unchanged since
  before the blocker was first logged (`git log` shows no commits to
  either file after `233c426`) — no operator decision has landed on the
  three options recorded under the E3.6-9 blocker (supply a real
  corpus/reference source; narrow Epic 3/4/5's acceptance shape to
  formula-derivable data; or explicitly accept memory-recall fabrication
  risk).
- `origin/tranche/5` has no commits past this session's own last cycle
  (`4f07d75`) — no other stream landed work in the interim.
- Epic 3/4/5 remain blocked for the identical corpus/fabrication-risk
  reason; Epic 6 remains transitively blocked (needs ≥1 book ingested);
  Epic 9 is not actually eligible yet despite Epic 8's discrete criteria
  (27-29) being complete — `epic-breakdown.md` line 208 places Epic 8
  "after Epics 1+3+4+5+6 land," and Epic 9 evaluates criteria 1-30 as a
  30/30-clean gate, which Epic 3/4/5/6 being blocked would fail
  non-mechanically (the same corpus blocker, not a self-healable
  shortfall) — dispatching Epic 9 now would just re-log the identical
  judgment call under a different epic heading, not make progress; Epic 7
  remains gated behind Epic 9.

Third consecutive NO-OP on the same unresolved blocker. The prior cycle
already surfaced this stall to the operator via push notification; since
nothing new has happened since then (no operator decision, no new source,
no new commits), this firing does not send a duplicate notification —
repeating the same unactioned alert would be noise, not signal. No
production change, no fabricated content. The loop will keep NO-OPing
every firing until one of the three recorded options gets an explicit
operator decision.

### cycle-2026-07-19T10:55:05Z | scheduled loop firing | n/a (verification-only, no production change) | no card (NO-OP, nothing to mint) | no row transition

Re-checked state; nothing has changed since the prior cycle's NO-OP:

- `corpus/` still does not exist in the repo.
- `git log` on `decisions.md` and `risks-and-open-questions.md` still shows
  no commits after `233c426` — no operator decision has landed on the
  three options recorded under the E3.6-9 blocker.
- `origin/tranche/5` HEAD matches this session's local HEAD (`3c3cf81`,
  the prior cycle's own commit) — no other stream landed work in the
  interim.
- Epic 3/4/5 remain blocked for the identical corpus/fabrication-risk
  reason; Epic 6 remains transitively blocked; Epic 9/7 remain gated
  behind Epic 3/4/5/6.

Fourth consecutive NO-OP on the same unresolved blocker. No new
information since the last notified cycle, so — per that cycle's own
precedent — this firing does not send another push notification;
repeating an unactioned alert with nothing new to report would be noise.
No production change, no fabricated content.

### cycle-2026-07-19T11:54:17Z | scheduled loop firing | n/a (verification-only, no production change) | no card (NO-OP, nothing to mint) | no row transition

Re-checked state; nothing has changed since the prior cycle's NO-OP:

- `corpus/` still does not exist in the repo.
- `git log 233c426..HEAD` on `decisions.md`, `risks-and-open-questions.md`,
  `epic-breakdown.md`, `scope-draft.md`, and `corpus-source-inventory.md`
  shows zero commits to any of them — no operator decision has landed on
  the three options recorded under the E3.6-9 blocker.
- `origin/tranche/5` HEAD matches this session's local HEAD (`c641656`,
  the prior cycle's own commit) — no other stream landed work in the
  interim.
- Re-tried `WebFetch` to `https://www.aonprd.com/` directly — still
  HTTP 403 Forbidden. No SRD/OGL mirror is reachable from this sandbox.
- `docs/release/SD-22/artifacts/` holds only `README.md` and `epic_8/` —
  no new Epic 3/4/5/6/9 artifacts have appeared.
- Epic 3/4/5 remain blocked for the identical corpus/fabrication-risk
  reason; Epic 6 remains transitively blocked; Epic 9/7 remain gated
  behind Epic 3/4/5/6.

Fifth consecutive NO-OP on the same unresolved blocker. No new
information since the last notified cycle (cycle 2, `4f07d75`), so this
firing does not send another push notification — same reasoning as
cycles 3 and 4. No production change, no fabricated content. This cycle
does note, for the operator's eventual return, that the loop has now run
5 consecutive hourly firings (roughly 4 hours) with zero landed criteria
past Epic 8's three discrete items — the stall is not self-resolving and
remains squarely an operator-decision item, not a mechanical one.

### cycle-2026-07-19T12:55:40Z | scheduled loop firing | n/a (verification-only, no production change) | no card (NO-OP, nothing to mint) | no row transition

Re-checked state before picking a criterion, per Step 1:

- `git log 233c426..HEAD` on `decisions.md`, `risks-and-open-questions.md`,
  `epic-breakdown.md`, and `corpus-source-inventory.md` shows no commits
  past `9cd7708` (the §5 LST-sourcing correction, already reflected in the
  live E3.6-9/E4.10-13/E5.14-17 blocker text) — no new operator decision
  has landed on the three options recorded under that blocker.
- Re-verified the blocker's own claims directly against source rather than
  trusting the doc text: `class.rs`'s `MARTIAL_CLASS_NAMES` and
  `spellcasting_class.rs`'s `SPELLCASTING_CLASS_NAMES` allowlists are
  unchanged (still the 11 CRB classes only); `apg_classes.lst` still has a
  real `CLASS:Alchemist` record (line 11); `acg_classes.lst` has zero
  `CLASS:Alchemist` hits (ACG's own classes are separately absent from
  both allowlists, confirming this isn't an APG-only gap); `b1_races.lst`
  still has zero `RACE:`-prefixed lines (bare tab-delimited rows, not the
  pointer shape `race_ability.rs` parses). The parser-coverage blocker is
  unchanged and still accurate — extending it is real new parsing code in
  `src/pcgen_import/`, outside every SD-22 epic's file-touch partition.
- `origin/tranche/5` HEAD (`ada161e`) matches this session's local HEAD —
  no other stream landed work in the interim.
- Epic 3/4/5 remain blocked for the identical, unchanged reason; Epic 6
  remains transitively blocked (needs ≥1 book ingested); Epic 8's three
  file-touch-partition criteria (27-29) remain complete, criterion 30
  remains the standing re-verification gate; Epic 9/7 remain gated behind
  Epic 3/4/5/6.

Per Step 1's own exit condition and this file's "do not repeat a NO-OP for
a criterion whose blocker reason has not changed" instruction: the
blocker's reason is identical to the one already recorded by the prior
cycle (`ada161e`, cycle-2026-07-19T13:00:00Z log entry above). No new
information to report, so — consistent with the immediately preceding
NO-OP cycles' own precedent — this firing does not send a push
notification. No production change, no fabricated content, no card
minted. `snapshot_as_of` bumped to `ada161e` in this cycle's edit (the
prior cycle's own commit had not updated it).

### cycle-2026-07-19T13:xx:xxZ | operator-side: pcgen_import parser widening (unblocks Epic 3 Alchemist) | fix:pcgen_import_allowlist | no card (operator-driven local session, not a loop firing) | E3.6-9's blocker → **narrowed, not yet closed**

Responding to the `ada161e` cycle's finding (the two existing `CLASS:`
parsers are name-allowlisted and neither `MARTIAL_CLASS_NAMES` nor
`SPELLCASTING_CLASS_NAMES` includes any APG/ACG class), the operator
session widened `SPELLCASTING_CLASS_NAMES` in
`src/pcgen_import/lst_parser/spellcasting_class.rs` to add `"Alchemist"`
— chosen over `class.rs`'s martial-class list because the real
`apg_classes.lst:11` `CLASS:Alchemist` line carries
`SPELLSTAT:INT MEMORIZE:YES SPELLBOOK:YES`, the same posture-bearing
shape as the five original CRB spellcasting classes.

Added a new gated real-corpus test,
`parses_real_alchemist_record_from_apg_classes_lst` in
`tests/sd17_b_spellcasting_class.rs`, mirroring the file's existing
`PCGEN_CORPUS_ROOT`-gated pattern: parses the real `apg_classes.lst` and
asserts Alchemist is now recognized with `CastingPosture::Spellbook`.
Ran and confirmed green against `PCGEN_CORPUS_ROOT=/home/ubuntu/workspace/repos/pcgen/data`.
Full `cargo test --locked` (0 failures across every suite) and
`cargo clippy --locked --tests -- -D warnings` (clean) both re-run and
green after the change — no regression on any existing class (the
`treats_class_lines_for_non_spellcasting_class_names_as_out_of_scope`
test does not use Alchemist as its out-of-scope example, so this
widening doesn't collide with an existing negative assertion).

`docs/release/SD-22/loop-instruction.md`'s file-touch partition is
amended to document this as an intended, bounded widening pattern:
Epic 3/4 per-class cycles may add exactly one class name to
`MARTIAL_CLASS_NAMES` or `SPELLCASTING_CLASS_NAMES` (never both at once,
never a broader rewrite) when — and only when — that specific class's
real record isn't yet recognized, with a small real-corpus test
accompanying the widening. This is not a general license to redesign
`src/pcgen_import`; it mirrors the SD-17 doc comments' own stated design
("owned by later B-slices").

**What this does NOT do:** does not write `rules_tables/apg/*`, does not
add `RuleSetId::Apg`, does not write the Epic 3 Alchemist acceptance
test (`tests/sd22_apg_class_alchemist_resolves.rs`). Epic 3's Alchemist
criterion (E3.6-9) is **not yet complete** — the corpus is now reachable
and the class name is now recognized, but the actual ingest cycle
(RED → GREEN per loop-instruction Step 4-5, populating
`rules_tables/apg/class_alchemist.rs`, and the cross-book invariant
tests) is still open work for the next loop firing. Re-triggering the
cloud routine now that this blocker is narrowed.

Bestiary 1's parser gap (bare tab-delimited monster rows in
`b1_races.lst`, no `RACE:` prefix — a different, larger gap than the
class-name allowlist) is **not** addressed by this fix and remains open;
Epic 5's first cycle will hit it and should route to `## Open blockers`
again with that specific finding, which is expected and correct (not a
regression — a new parser module for that record shape is out of scope
for this narrow fix).

### cycle-2026-07-19T14:00:00Z | Epic 3, Alchemist (cycle 1 of 8) | ingest:apg_class | no card (hermes unavailable; logged here + `receipts.md`) | blocked → **complete (criteria 6-8)**

Picked up the open work the operator-side widening cycle (`d1b2f80`,
merged via `e2d7194`) explicitly left for the next firing: the parser
allowlist gap was narrowed (Alchemist now recognized by
`spellcasting_class.rs`), but `rules_tables/apg/` didn't exist yet and
`RuleSetId::Apg` wasn't registered. Re-verified the premise first: `git
log` on `decisions.md`/`corpus-source-inventory.md` shows no new commits
since `9cd7708`; `apg_classes.lst:11`'s real `CLASS:Alchemist` record
still carries `BONUS:COMBAT|BASEAB|classlevel(...)*3/4`,
`BONUS:SAVE|BASE.Will|classlevel(...)/3`, and
`BONUS:SAVE|BASE.Fortitude,BASE.Reflex|classlevel(...)/2+2`, with
`MAXLEVEL:20` — confirmed via direct `grep`, not from the doc's prose
(which the corpus-source-inventory.md corrective banner already flags as
non-authoritative).

RED: added `tests/sd22_apg_class_alchemist_resolves.rs` asserting
`class_chassis_resolve(ApgClassId::Alchemist, level, RuleSetId::Apg)`
resolves the expected BAB/save cells at levels 1 and 20, returns `None`
past the real record's `MAXLEVEL:20`, and returns `None` for
`RuleSetId::Crb` (the Epic 3 cross-book invariant per
`corpus-source-inventory.md` §1.3). Ran against the unchanged tree: failed
to compile (`E0432`/`E0599` — `rules_tables::apg` and `RuleSetId::Apg`
didn't exist) for the intended reason.

GREEN: added `src/rules_core/rules_tables/apg/mod.rs` (`ApgClassId` enum,
`class_chassis_resolve`) and `class_alchemist.rs` (the BAB/save table,
scope-bounded to formula-derived chassis data only — same boundary
`rules_tables/crb/class_tables.rs` already established; named per-level
features like Bombs/Discoveries/Mutagen are out of scope for this cycle,
same fabrication-risk rationale). Added `RuleSetId::Apg` to
`rules_tables/mod.rs`. Also added and ran (real-corpus-gated on
`PCGEN_CORPUS_ROOT=/home/user/pcgen/data`) a grounding test that
re-parses the real `CLASS:Alchemist` line and asserts the exact
`BASEAB`/`SAVE` bonus-formula tokens the hand-transcribed constants are
derived from — both the default 4-test run and the `--ignored` real-corpus
run are green.

Verification: `cargo test --locked --test sd22_apg_class_alchemist_resolves`
4/4 passed (1 additional real-corpus-gated test passed separately under
`--ignored`). Full `cargo test --locked` — every suite green, 0 failures
(grep across the full run output for any `N failed` with `N > 0` found
none — sibling-preservation holds). `cargo clippy --locked --tests -- -D
warnings` clean.

Criterion 9 (per-cycle APG spell/equipment resolution) is **not** closed
this cycle — Alchemist's bombs/extracts require `apg/spell_list.rs` /
`apg/equipment_tables.rs`, which don't exist yet; a future cycle should
land those explicitly rather than this cycle assuming they're covered.
Epic 4 (ACG) and Epic 5 (Bestiary 1) remain blocked on their own,
separate parser gaps (no `CLASS:` allowlist entry for any ACG class; no
parser recognizes `b1_races.lst`'s unprefixed bare-row monster records) —
unaffected by this cycle, since this cycle only widened the *chassis*
surface for one already-allowlisted class.

Full RED/GREEN evidence, file list, and reasoning:
`artifacts/apg/class_alchemist_cycle_receipt.md`. Receipt block appended
to `receipts.md`. Next-eligible for Epic 3: Cavalier (class 2 of 8), or a
dedicated cycle for Alchemist's spell/equipment tables (criterion 9).

### cycle-2026-07-19T15:00:00Z | Epic 3, Cavalier (cycle 2 of 8) | ingest:apg_class | no card (hermes unavailable; logged here + `receipts.md`) | open → **complete (criteria 7-8)**

Re-checked state before picking a criterion: `git log 9c187a7..HEAD` on
`decisions.md`/`corpus-source-inventory.md`/`risks-and-open-questions.md`
shows no new commits; `origin/tranche/5` HEAD (`9c187a7`) matches this
session's local HEAD — no other stream landed work in the interim. Per
Step 1's priority order, Epic 3 remains the next-eligible open lane
(Epic 4/5 stay blocked on their own separate parser-coverage gaps, unchanged;
Epic 6 stays transitively blocked). Picked up the prior cycle's own
"next-eligible" recommendation: Cavalier (class 2 of 8), the per-class
chassis unit, over criterion 9's shared spell/equipment tables (a
distinct work-unit better suited to its own cycle).

Verified the real record before writing any test (not the
`corpus-source-inventory.md` "Content shape" prose, which that file's own
corrective banner marks non-authoritative): `apg_classes.lst:42`'s
`CLASS:Cavalier` line carries `BONUS:COMBAT|BASEAB|classlevel(...)` (full
BAB, no fractional divisor — unlike Alchemist's three-quarter BAB),
`BONUS:SAVE|BASE.Fortitude|classlevel(...)/2+2` (good Fortitude),
`BONUS:SAVE|BASE.Will,BASE.Reflex|classlevel(...)/3` (poor Will *and*
Reflex), `MAXLEVEL:20`, and no `SPELLSTAT:` line (non-caster, unlike
Alchemist) — confirming Cavalier belongs in `lst_parser::class`'s
martial-class allowlist, not the spellcasting-class parser.

**Widening RED**: added `parses_real_cavalier_record_from_apg_classes_lst`
to `tests/sd17_b1_martial_class.rs` (real-corpus-gated on
`PCGEN_CORPUS_ROOT`); ran against the unchanged tree — failed for the
intended reason (`Cavalier` not yet in `MARTIAL_CLASS_NAMES`, silently
skipped).

**Acceptance RED**: added `tests/sd22_apg_class_cavalier_resolves.rs`
mirroring the Alchemist test's shape; ran against the unchanged tree —
failed to compile (`E0599`: `ApgClassId::Cavalier` did not exist) for the
intended reason.

GREEN: widened `MARTIAL_CLASS_NAMES` in
`src/pcgen_import/lst_parser/class.rs` by exactly one name (`Cavalier`),
per the file-touch-partition's bounded-widening pattern (mirrors the
Alchemist cycle's `SPELLCASTING_CLASS_NAMES` widening). Added
`src/rules_core/rules_tables/apg/class_cavalier.rs` (BAB/save chassis
only, same scope boundary as `class_alchemist.rs`) and
`ApgClassId::Cavalier` + a match arm in `apg/mod.rs`. Lifted the
previously Alchemist-only `ClassTableRow` struct up into `apg/mod.rs` so
`class_chassis_resolve` has one return type across classes — a mechanical
consequence of landing a second class, not a scope expansion.

Verification: `cargo test --locked --test sd22_apg_class_cavalier_resolves`
4/4 passed (1 additional real-corpus-gated test passed separately under
`--ignored`). `cargo test --locked --test sd17_b1_martial_class
-- --include-ignored` 16/16 passed, including the new widening test and
the pre-existing real-corpus core-rulebook test (unaffected by the
widening — Cavalier doesn't appear in `cr_classes.lst`). Full `cargo test
--locked` — every suite green, 0 failures (no `N failed` with `N > 0`
anywhere; sibling-preservation holds, including the untouched Alchemist
suite). `cargo clippy --locked --tests -- -D warnings` clean.

Criterion 9 (per-cycle APG spell/equipment resolution) remains open for
both Alchemist and Cavalier — neither `apg/spell_list.rs` nor
`apg/equipment_tables.rs` exists yet. Epic 4 (ACG) and Epic 5 (Bestiary 1)
remain blocked on their own, separate parser gaps (no `CLASS:` allowlist
entry for any ACG class; no parser recognizes `b1_races.lst`'s unprefixed
bare-row monster records) — unaffected by this cycle.

Full RED/GREEN evidence, file list, and reasoning:
`artifacts/apg/class_cavalier_cycle_receipt.md`. Receipt block appended
to `receipts.md`. Next-eligible for Epic 3: Gunslinger (class 3 of 8), or
a dedicated cycle for criterion 9's shared spell/equipment tables.

### cycle-2026-07-19T16:00:00Z | Epic 3, Inquisitor (cycle 3 landed; Gunslinger found blocked) | ingest:apg_class | no card (hermes unavailable; logged here + `receipts.md`) | open → **complete (criteria 7-8 for Inquisitor)**; new blocker logged for Gunslinger/Magus

Re-checked state before picking a criterion: `git log 9c187a7..HEAD` on
`decisions.md`/`corpus-source-inventory.md`/`risks-and-open-questions.md`
shows no new commits; `origin/tranche/5` HEAD (`675ca65`) matches this
session's local HEAD — no other stream landed work in the interim. Per
Step 1's priority order and the operator-pinned ordering, Gunslinger
(class 3 of 8) was next.

Before writing any RED test, verified the real `.lst` record and found
`apg_classes.lst` has **no `CLASS:Gunslinger` line anywhere** — not just
an unrecognized one, genuinely absent from the file. Searched the full
PCGen tree: `CLASS:Gunslinger` lives in `ultimate_combat/uc_classes.lst`;
`CLASS:Magus` (the class 5 of 8 after Inquisitor) lives in
`ultimate_magic/um_classes.lst`. Neither is real APG content — both are
from books `decisions.md §1` explicitly excludes from SD-22 scope. Full
detail in the new `## Open blockers` entry above (E3.6-9, Gunslinger +
Magus specifically). This is new, actionable, previously-unknown
information — a genuine defect in `corpus-source-inventory.md` §1.1's
class roster, not a repeat of the earlier parser-allowlist blocker — so
this cycle sends a push notification.

Per Step 1 ("pick the smallest unclaimed eligible acceptance criterion"),
did not stall on the blocked class — picked Inquisitor (class 4 of 8),
the next-eligible class with a real record. Verified
`apg_classes.lst:50`'s `CLASS:Inquisitor` line directly: three-quarter
BAB (`*3/4`, same posture as Alchemist), good Fortitude **and** Will
(`/2+2` — a different good/poor split than Alchemist or Cavalier), poor
Reflex only (`/3`), `MAXLEVEL:20`, and `SPELLSTAT:WIS MEMORIZE:NO`
(spontaneous divine, same posture as Sorcerer/Bard) — confirming
Inquisitor belongs in `spellcasting_class.rs`'s allowlist, not `class.rs`'s.

**Widening RED**: added `parses_real_inquisitor_record_from_apg_classes_lst`
to `tests/sd17_b_spellcasting_class.rs` (real-corpus-gated on
`PCGEN_CORPUS_ROOT`); ran against the unchanged tree — failed for the
intended reason (`Inquisitor` not yet in `SPELLCASTING_CLASS_NAMES`,
silently skipped).

**Acceptance RED**: added `tests/sd22_apg_class_inquisitor_resolves.rs`
mirroring the Alchemist/Cavalier tests' shape; ran against the unchanged
tree — failed to compile (`E0599`: `ApgClassId::Inquisitor` did not
exist) for the intended reason.

GREEN: widened `SPELLCASTING_CLASS_NAMES` in
`src/pcgen_import/lst_parser/spellcasting_class.rs` by exactly one name
(`Inquisitor`), per the file-touch-partition's bounded-widening pattern.
Added `src/rules_core/rules_tables/apg/class_inquisitor.rs` (BAB/save
chassis only, same scope boundary as the prior two classes) and
`ApgClassId::Inquisitor` + a match arm in `apg/mod.rs`, plus a doc-comment
note recording why Gunslinger/Magus are absent from this module.

Verification: `cargo test --locked --test sd22_apg_class_inquisitor_resolves`
4/4 passed (1 additional real-corpus-gated test passed separately under
`--ignored`). `cargo test --locked --test sd17_b_spellcasting_class --
--ignored` 4/4 passed, including the new widening test. Full `cargo test
--locked` — every suite green, 0 failures (no `N failed` with `N > 0`
anywhere; sibling-preservation holds). `cargo clippy --locked --tests --
-D warnings` clean.

Criterion 9 (per-cycle APG spell/equipment resolution) remains open for
Alchemist, Cavalier, and Inquisitor alike — no `apg/spell_list.rs` or
`apg/equipment_tables.rs` exists yet. Epic 4 (ACG) and Epic 5 (Bestiary 1)
remain blocked on their own, separate parser gaps — unaffected by this
cycle.

Full RED/GREEN evidence, file list, and reasoning:
`artifacts/apg/class_inquisitor_cycle_receipt.md`. Receipt block appended
to `receipts.md`. Next-eligible for Epic 3: Oracle (class 5 of 8 in the
existing ordering — Magus is skipped per the new blocker above), or a
dedicated cycle for criterion 9's shared spell/equipment tables.

### cycle-2026-07-19T17:00:00Z | Epic 3, Oracle (class 4 of 6, corrected ordering) | ingest:apg_class | no card (hermes unavailable; logged here + `receipts.md`) | open → **complete (criteria 7-8)**

Re-checked state before picking a criterion: `git log 675ca65..HEAD` shows
one new commit, `6923e54` (`docs(sd22): correct APG roster to 6 real
classes`), landed by the operator between this firing and the prior
Inquisitor cycle. Read the diff directly rather than trusting the commit
message alone: it corrects `corpus-source-inventory.md`, `decisions.md`,
`epic-breakdown.md`, `risks-and-open-questions.md` (Flag A),
`scope-draft.md`, `technical-design.md`, and
`acceptance-and-verification.md` to a 6-class APG roster (Alchemist,
Cavalier, Inquisitor, Oracle, Summoner, Witch), removing Gunslinger and
Magus everywhere. This is the operator's own option 1 from the standing
E3.6-9 blocker's recommendation list — marked **resolved** in `## Open
blockers` above (left in place per this file's edit-in-place convention,
not deleted). With the blocker resolved, Oracle (class 4 of 6 in the
corrected ordering) is next-eligible per Step 1's priority order.

Verified the real `CLASS:Oracle` record directly before writing any test
(not `corpus-source-inventory.md`'s non-authoritative prose):
`apg_classes.lst:107` carries three-quarter BAB (`*3/4`, same posture as
Alchemist/Inquisitor), good Will only (`/2+2`), poor Fortitude **and**
Reflex (`/3` — a different good/poor split than any prior class landed),
`MAXLEVEL:20`, and `SPELLSTAT:CHA MEMORIZE:NO` (spontaneous divine,
same posture as Sorcerer/Bard/Inquisitor) — confirming Oracle belongs in
`spellcasting_class.rs`'s allowlist, not `class.rs`'s.

**Widening RED**: added `parses_real_oracle_record_from_apg_classes_lst`
to `tests/sd17_b_spellcasting_class.rs` (real-corpus-gated on
`PCGEN_CORPUS_ROOT`); ran against the unchanged tree — failed for the
intended reason (`Oracle` not yet in `SPELLCASTING_CLASS_NAMES`, silently
skipped).

**Acceptance RED**: added `tests/sd22_apg_class_oracle_resolves.rs`
mirroring the prior three classes' test shape; ran against the unchanged
tree — failed to compile (`E0599`: `ApgClassId::Oracle` did not exist)
for the intended reason.

GREEN: widened `SPELLCASTING_CLASS_NAMES` in
`src/pcgen_import/lst_parser/spellcasting_class.rs` by exactly one name
(`Oracle`), per the file-touch-partition's bounded-widening pattern.
Added `src/rules_core/rules_tables/apg/class_oracle.rs` (BAB/save chassis
only, same scope boundary as the prior three classes) and
`ApgClassId::Oracle` + a match arm in `apg/mod.rs`, and updated `apg/mod.rs`'s
doc comment to record the Gunslinger/Magus exclusion as now permanent
(per `6923e54`) rather than an ordering skip.

Verification: `cargo test --locked --test sd22_apg_class_oracle_resolves
-- --include-ignored` 5/5 passed (including the real-corpus-gated
grounding test). `cargo test --locked --test sd17_b_spellcasting_class --
--ignored` 5/5 passed, including the new widening test. Full `cargo test
--locked` — every suite green, 0 failed (no `N failed` with `N > 0`
anywhere; sibling-preservation holds). `cargo clippy --locked --tests --
-D warnings` clean.

Criterion 9 (per-cycle APG spell/equipment resolution) remains open for
Alchemist, Cavalier, Inquisitor, and Oracle alike — no
`apg/spell_list.rs` or `apg/equipment_tables.rs` exists yet. Epic 4 (ACG)
and Epic 5 (Bestiary 1) remain blocked on their own, separate parser
gaps — unaffected by this cycle.

Full RED/GREEN evidence, file list, and reasoning:
`artifacts/apg/class_oracle_cycle_receipt.md`. Receipt block appended to
`receipts.md`. Next-eligible for Epic 3: Summoner (class 5 of 6), Witch
(class 6 of 6), or a dedicated cycle for criterion 9's shared
spell/equipment tables.

**Concurrency note:** `git push` was rejected on the first attempt — a
concurrent operator commit, `f933ecf` (`feat(sd22): corpus source
surfaces...`), landed on `origin/tranche/5` after this cycle's local base
(`6923e54`) but before this cycle's push. Inspected its diff before
rebasing: it touches `.gitignore`, `docs/release/SD-22/README.md`,
`acceptance-and-verification.md`, `artifacts/README.md`,
`artifacts/corpus/**`, `corpus-source-inventory.md`, `ingest.md`, and
`loop-instruction.md` — no overlap with this cycle's file set
(`spellcasting_class.rs`, `apg/mod.rs`, `apg/class_oracle.rs`,
`sd17_b_spellcasting_class.rs`, `sd22_apg_class_oracle_resolves.rs`,
`receipts.md`, `progress.md`). Rebased cleanly (`git rebase
origin/tranche/5`, no conflicts) and re-pointed `snapshot_as_of` at the
new parent (`f933ecf`) before pushing.

### cycle-2026-07-19T18:00:00Z | Epic 3, Summoner (class 5 of 6) | ingest:apg_class | no card (hermes unavailable; logged here + `receipts.md`) | open → **complete (criteria 7-8)**

Re-checked state before picking a criterion: `git log aa9b924..origin/tranche/5`
showed no new commits — `aa9b924` (the prior Oracle cycle's own commit) is
still the tip. Per Step 1's priority order and the corrected 6-class
ordering (Alchemist, Cavalier, Inquisitor, Oracle all `complete`),
Summoner (class 5 of 6) is next-eligible.

Verified the real `CLASS:Summoner` record directly before writing any
test (not `corpus-source-inventory.md`'s non-authoritative prose):
`apg_classes.lst:139` carries three-quarter BAB (`*3/4`, same posture as
Alchemist/Inquisitor/Oracle), good Will only (`/2+2`), poor Fortitude
**and** Reflex (`/3` — the identical good/poor split to Oracle),
`MAXLEVEL:20`, and `SPELLSTAT:CHA MEMORIZE:NO` (spontaneous casting,
arcane rather than divine per `TYPE:Base.PC.SpontaneousArcane.Spontaneous`
— a distinction that doesn't affect the parser's posture derivation or
the chassis formulas) — confirming Summoner belongs in
`spellcasting_class.rs`'s allowlist, not `class.rs`'s.

**Widening RED**: added `parses_real_summoner_record_from_apg_classes_lst`
to `tests/sd17_b_spellcasting_class.rs` (real-corpus-gated on
`PCGEN_CORPUS_ROOT`); ran against the unchanged tree — failed for the
intended reason (`Summoner` not yet in `SPELLCASTING_CLASS_NAMES`,
silently skipped).

**Acceptance RED**: added `tests/sd22_apg_class_summoner_resolves.rs`
mirroring the prior four classes' test shape; ran against the unchanged
tree — failed to compile (`E0599`: `ApgClassId::Summoner` did not exist)
for the intended reason.

GREEN: widened `SPELLCASTING_CLASS_NAMES` in
`src/pcgen_import/lst_parser/spellcasting_class.rs` by exactly one name
(`Summoner`), per the file-touch-partition's bounded-widening pattern.
Added `src/rules_core/rules_tables/apg/class_summoner.rs` (BAB/save
chassis only, same scope boundary as the prior four classes) and
`ApgClassId::Summoner` + a match arm in `apg/mod.rs`.

Verification: `cargo test --locked --test sd22_apg_class_summoner_resolves
-- --include-ignored` 5/5 passed (including the real-corpus-gated
grounding test). `cargo test --locked --test sd17_b_spellcasting_class --
--ignored` 6/6 passed, including the new widening test. Full `cargo test
--locked` — every suite green, 0 failed (no `N failed` with `N > 0`
anywhere; sibling-preservation holds). `cargo clippy --locked --tests --
-D warnings` clean.

Criterion 9 (per-cycle APG spell/equipment resolution) remains open for
all five landed classes — no `apg/spell_list.rs` or
`apg/equipment_tables.rs` exists yet. Epic 4 (ACG) and Epic 5
(Bestiary 1) remain blocked on their own, separate parser gaps —
unaffected by this cycle.

Full RED/GREEN evidence, file list, and reasoning:
`artifacts/apg/class_summoner_cycle_receipt.md`. Receipt block appended
to `receipts.md`. Next-eligible for Epic 3: Witch (class 6 of 6, the
last real APG class), or a dedicated cycle for criterion 9's shared
spell/equipment tables.

### cycle-2026-07-19T14:00:00Z (this firing) | concurrent-cycle collision on Oracle | ingest:apg_class | no card (no commit; work discarded) | CLAIM-EXISTS

This scheduled hourly firing picked Oracle independently (branch tip at
firing start: `f933ecf`, before this firing was aware `6923e54` had
already corrected the APG roster to 6 classes or that another stream had
picked up Oracle too) and completed the full RED→GREEN cycle locally:
widened `SPELLCASTING_CLASS_NAMES` with `Oracle`, added
`rules_tables/apg/class_oracle.rs` + `ApgClassId::Oracle`, added the
widening test and `tests/sd22_apg_class_oracle_resolves.rs`, and verified
`cargo test --locked` (every suite green) + `cargo clippy --locked
--tests -- -D warnings` (clean) before attempting to push.

`git push origin tranche/5` was rejected (non-fast-forward, `403`
followed by "fetch first"): `git fetch` showed `origin/tranche/5` had
moved to `b160857` — a **different, concurrent** stream had already
landed both `aa9b924` (Oracle) and `b160857` (Summoner) on top of the
same `f933ecf` base this firing started from. Per
`loop-instruction.md`'s hard stop ("Two live `claude` processes are
working on cycles that would both touch `src/rules_core/rules_tables/
<book>/` or any per-epic module file"), this is exactly that collision —
it had already happened by the time this firing tried to push, rather
than being avoidable in advance (this environment's adapted concurrency
guard relies on git state, not `ps`, per the routine's cloud-sandbox
adaptation, and git state was clean — no divergence — when this firing's
Step 3 checkout ran).

Rather than force-push or attempt a manual merge/rebase of duplicate
Oracle content, this firing's local commit (never pushed, so discarding
it loses no shared work) was dropped and the local branch was reset to
match `origin/tranche/5` (`git reset --hard origin/tranche/5`). The
concurrent stream's Oracle (`aa9b924`) and Summoner (`b160857`) commits,
and their own `progress.md`/`receipts.md` cycle-log entries, are left
untouched by this entry — this entry only records this firing's own
redundant, discarded attempt for the audit trail.

No new commit lands from this firing. Per Step 1, the next-eligible
criterion is now Witch (class 6 of 6), already identified as such by the
concurrent stream's own Summoner cycle log entry above. This firing does
not also attempt Witch — picking up a second criterion in the same
firing right after detecting a live concurrency collision would risk
racing a still-active concurrent stream a second time in one cycle,
which is exactly what the "1 cycle at a time" default budget and the
per-cycle atomicity rules exist to prevent. Sending a push notification:
this is new, actionable information (evidence of a second, concurrently
running SD-22 loop stream) that the operator should be aware of, not a
repeat of a previously-notified condition.

### cycle-2026-07-19T19:00:00Z | Epic 3, Witch (class 6 of 6, the last real APG class) | ingest:apg_class | no card (hermes unavailable; logged here + `receipts.md`) | open → **complete (criteria 7-8)**

Re-checked state before picking a criterion: `git log` on `decisions.md`,
`corpus-source-inventory.md`, `risks-and-open-questions.md`,
`epic-breakdown.md` shows no new commits past `f8b4aae`/`6f2a13e` (the
parallel-session doctrine reconciliation merge that landed the
ingest.md rewrite and roster-count fixes, already reflected in the tree
this cycle read). `origin/tranche/5` HEAD (`6f2a13e`) matched local HEAD
after this cycle's initial fetch/checkout/pull — no other stream landed
work in the interim. Per Step 1's priority order and the corrected
6-class ordering (Alchemist, Cavalier, Inquisitor, Oracle, Summoner all
`complete`), Witch (class 6 of 6, the last real APG class) is
next-eligible, exactly as the prior Summoner cycle's own log entry
predicted.

Verified the real `CLASS:Witch` record directly before writing any test
(not `corpus-source-inventory.md`'s non-authoritative prose):
`apg_classes.lst:172` carries `BONUS:COMBAT|BASEAB|classlevel(...)/2`
(half BAB, poor — the first poor-BAB class landed in this roster; every
prior class was full or three-quarter), `BONUS:SAVE|BASE.Will|classlevel(...)/2+2`
(good Will only), `BONUS:SAVE|BASE.Fortitude,BASE.Reflex|classlevel(...)/3`
(poor Fortitude and Reflex — same split as Oracle/Summoner), `MAXLEVEL:20`,
and (line 176) `SPELLSTAT:INT` with no `MEMORIZE:NO`/`SPELLBOOK:YES` token
— the same absent-signals prepared-casting posture as Cleric/Druid —
confirming Witch belongs in `spellcasting_class.rs`'s allowlist, not
`class.rs`'s.

**Widening RED**: added `parses_real_witch_record_from_apg_classes_lst`
to `tests/sd17_b_spellcasting_class.rs` (real-corpus-gated on
`PCGEN_CORPUS_ROOT`); ran against the unchanged tree — failed for the
intended reason (`Witch` not yet in `SPELLCASTING_CLASS_NAMES`, silently
skipped).

**Acceptance RED**: added `tests/sd22_apg_class_witch_resolves.rs`
mirroring the prior five classes' test shape; ran against the unchanged
tree — failed to compile (`E0599`: `ApgClassId::Witch` did not exist)
for the intended reason.

GREEN: widened `SPELLCASTING_CLASS_NAMES` in
`src/pcgen_import/lst_parser/spellcasting_class.rs` by exactly one name
(`Witch`), per the file-touch-partition's bounded-widening pattern.
Added `src/rules_core/rules_tables/apg/class_witch.rs` (BAB/save chassis
only, same scope boundary as the prior five classes — half-BAB formula
`level/2` since Witch is the first poor-BAB class in this roster) and
`ApgClassId::Witch` + a match arm in `apg/mod.rs`, and updated `apg/mod.rs`'s
doc comment to record the roster as complete.

Verification: `cargo test --locked --test sd22_apg_class_witch_resolves
-- --include-ignored` 5/5 passed (including the real-corpus-gated
grounding test). `cargo test --locked --test sd17_b_spellcasting_class --
--include-ignored` 20/20 passed, including the new widening test. Full
`cargo test --locked` — every suite green, 0 failed (no `N failed` with
`N > 0` anywhere; sibling-preservation holds). `cargo clippy --locked
--tests -- -D warnings` clean.

With Witch landed, all six real APG classes now have chassis tables and
`RuleSetId::Apg` resolution — criteria 7-8 are complete for the full
roster. Criterion 9 (per-cycle APG spell/equipment resolution) remains
open for all six classes — no `apg/spell_list.rs` or
`apg/equipment_tables.rs` exists yet; that is a distinct work-unit for a
future cycle. Epic 4 (ACG) and Epic 5 (Bestiary 1) remain blocked on
their own, separate parser gaps (no `CLASS:` allowlist entry for any ACG
class; no parser recognizes `b1_races.lst`'s unprefixed bare-row monster
records) — unaffected by this cycle.

Full RED/GREEN evidence, file list, and reasoning:
`artifacts/apg/class_witch_cycle_receipt.md`. Receipt block appended to
`receipts.md`. Next-eligible: Epic 3 criterion 9 (`apg/spell_list.rs` +
`apg/equipment_tables.rs`), or Epic 4/Epic 5's first cycles (both remain
blocked on their own parser-coverage gaps, unchanged by this cycle).

**Concurrency note:** `git push` was rejected on the first attempt — a
concurrent operator commit, `6ab616b` (`docs(sd22): make corpus-sourcing
genuinely self-serve; document the Oracle collision's real cause`),
landed on `origin/tranche/5` after this cycle's local base (`6f2a13e`)
but before this cycle's push. Inspected its diff before rebasing: it
touches only `decisions.md` and `loop-instruction.md` (self-serve corpus
cloning + a retrospective lesson-learned note) — no overlap with this
cycle's file set. Rebased cleanly (`git rebase origin/tranche/5`, no
conflicts), re-pointed `snapshot_as_of` at the new parent (`6ab616b`),
re-verified `cargo test --locked` and `cargo clippy` still green
post-rebase, then pushed successfully as `18a963b`.

### cycle-2026-07-19T19:51:46Z | Epic 3, criterion 9 (APG shared spell/equipment tables) | ingest:apg_class | card `t_1d2c1dce` on `codex-tranche-5` (status=done) | open → **complete**

`hermes` is reachable from this session (unlike the prior cloud-run
cycles this progress doc's earlier entries describe) — Step 10b's kanban
mint ran for real this cycle. Two syntax corrections against the goal
file's documented example were needed: `--board <slug>` is a flag on
`hermes kanban` itself (before the subcommand), not on `create`; and
`--initial-status` only accepts `blocked`/`running`, not `done` — created
the card with default status then used `hermes kanban complete
t_1d2c1dce --summary "..."` to reach `done`. Card verified via `hermes
kanban show t_1d2c1dce`.

Re-checked state before picking a criterion: `git log 18a963b..HEAD` showed
one new commit, `e134bb4` (`feat(sd22): per-content-type product surfaces
(races, mitems, feats, archetypes, monster-abilities, monster-templates)`),
landed by the operator between this firing and the prior Witch cycle.
Inspected its diff before proceeding (not just the commit message): it adds
12 corpus-stub files under `artifacts/corpus/{races,magic-items,feats,
archetypes,monster-abilities,monster-templates}/`, extends
`corpus-source-inventory.md` with routing sections §7-§12, and extends
`ingest.md` with a §9 "Per-content-type extensions" doctrine section for
*future* extension-epic work (explicitly scoped as landing "after the
primary 31-criteria loop closes" per that commit's own message and
`corpus-source-inventory.md §10`). It touches no Rust source, no tests,
and does not add or remove any of the 31 primary criteria — it does not
change Epic 3's eligibility or criterion 9's open status. `origin/tranche/5`
HEAD (`e134bb4`) matched local HEAD after fetch/checkout/pull — no other
stream landed conflicting work. Per Step 1's priority order, with criteria
6-8 all `complete` (six-class roster landed), criterion 9 (APG shared
spell and equipment tables) is next-eligible for Epic 3.

Read `corpus-source-inventory.md §1.2` (routing: `apg/spell_list.rs` →
`tests/sd22_apg_spell_list_resolves.rs`; `apg/equipment_tables.rs` →
`tests/sd22_apg_equipment_resolves.rs`) and `ingest.md` before RED. Verified
real corpus records directly (not `corpus-source-inventory.md §1.1`'s
non-authoritative "Content shape" prose, which names Alchemist bombs as if
they were equipment — confirmed by grep across all three
`apg_equip_*.lst` files that no `Bomb`/`Acid Bomb` record exists anywhere;
bombs are a `Su` class feature computed by formula, not a purchasable
item, so this cycle does not fabricate one). Found real, active
(non-`.MOD`, non-commented) spell records for 4 of APG's 5 caster classes
in `apg_spells.lst`'s "Main Spell List" block: `Bomber's Eye` (line 44,
Alchemist=1, Transmutation), `Burst Bonds` (line 53, Inquisitor=1,
Evocation), `Borrow Fortune` (line 277, Oracle=3, Evocation), `Ill Omen`
(line 150, Witch=1, Enchantment). Summoner's dedicated "Summoner Spells -
APG" block (line 471 onward) is entirely `#`-commented out in the real
corpus — confirmed by direct grep, a real corpus gap, not an omission.
For equipment, found real verbatim `COST:`/`WT:` records: `Iron Spike`
(`apg_equip_general.lst`, `COST:0.05`), `Arrow (Blunt)`
(`apg_equip_arms_armor.lst`, `COST:0.1`), `Knucklebone of Fickle Fortune`
(`apg_equip_magic_items.lst`, `COST:0`).

RED: added `tests/sd22_apg_spell_list_resolves.rs` and
`tests/sd22_apg_equipment_resolves.rs`, referencing
`apg::spell_list::spell_resolve` / `apg::equipment_tables::equipment_resolve`
(neither module existed yet). Ran against the unchanged tree: both failed
to compile (`E0432: could not find spell_list/equipment_tables in apg`)
for the intended reason.

GREEN: added `src/rules_core/rules_tables/apg/spell_list.rs` (bootstrap
4-entry `SPELL_LIST` + `spell_resolve`, gated on `RuleSetId::Apg` same as
`class_chassis_resolve`) and `apg/equipment_tables.rs` (bootstrap 3-entry
`EQUIPMENT_TABLE` + `equipment_resolve`, same gating pattern); registered
both as `pub mod` in `apg/mod.rs`. Investigated the existing
`equipment_id_resolve`/`spell_id_resolve` in `src/rules_core/` (referenced
by criterion 6's wording) — both already accept a `RuleSetId` parameter
but are hard-wired to the CRB tables regardless of its value; widening
their dispatch is a cross-cutting change to files outside every SD-22
epic's file-touch partition (`equipment_resolver.rs`/`spell_resolver.rs`
aren't listed as cycle-touchable), so this cycle kept the new
`spell_resolve`/`equipment_resolve` functions self-contained inside
`apg/`, mirroring `class_chassis_resolve`'s own established shape, and did
not touch the global resolvers — left as a follow-on note in the receipt.

Verification: `cargo test --locked --test sd22_apg_spell_list_resolves --
--include-ignored` 7/7 passed (including the real-corpus-gated grounding
test, run with `PCGEN_CORPUS_ROOT=/home/ubuntu/workspace/repos/pcgen/data`).
`cargo test --locked --test sd22_apg_equipment_resolves -- --include-ignored`
6/6 passed (same grounding-test gating). Full `cargo test --locked` —
every suite green, 0 failures anywhere (grepped full output for
`FAILED`/`N failed` with `N > 0`, found none; sibling-preservation holds,
including all six untouched APG class-chassis suites). `cargo clippy
--locked --tests -- -D warnings` clean.

With criterion 9 landed, Epic 3 (APG) is now **fully closed out**
(criteria 6-9 all complete). Epic 4 (ACG) and Epic 5 (Bestiary 1) remain
blocked on their own, separate parser-coverage gaps (no `CLASS:`
allowlist entry for any ACG class; no parser recognizes `b1_races.lst`'s
unprefixed bare-row monster records) — unaffected by this cycle.

Full RED/GREEN evidence, file list, and reasoning:
`artifacts/apg/spell_list_cycle_receipt.md`,
`artifacts/apg/equipment_tables_cycle_receipt.md`. Receipt block appended
to `receipts.md`. Next-eligible: Epic 4/Epic 5's first cycles (both remain
blocked on their own parser-coverage gaps), or an operator decision on how
to unblock them (widen `pcgen_import`'s allowlists for ACG classes; add a
bare-row monster parser for Bestiary 1).

### cycle-2026-07-19T20:18:28Z | Epic 4, Arcanist (cycle 1, first real ACG class) | ingest:acg_class | card see receipts.md/kanban | open → **complete (criteria 10-12)**

Re-checked state before picking a criterion: `git log 87e7ec3..origin/tranche/5`
showed no new commits — `87e7ec3` is still the tip, tree clean. With Epic 3
(APG) fully closed out (criteria 6-9), Epic 4 (ACG) is next-eligible per
Step 1's priority order. `epic-breakdown.md`/`decisions.md` list Epic 4's
first class as "Alchemist (ACG-side)"; per this cycle's brief and the
established Gunslinger/Magus precedent, verified the real record before
writing any test rather than trusting the doc.

Verified directly against `acg_classes.lst`: **zero** `CLASS:Alchemist`
hits anywhere in the file. Full roster grep
(`grep -oP "^CLASS:\K[A-Za-z-]+" acg_classes.lst | sort -u`) returned the
real 10-class base roster: Arcanist, Bloodrager, Brawler, Hunter,
Investigator, Shaman, Skald, Slayer, Swashbuckler, Warpriest (plus the
internal `Ex-Warpriest` `VISIBLE:NO` variant, correctly excluded). This is
the identical defect shape as the resolved Gunslinger/Magus blocker:
`corpus-source-inventory.md §2.1`'s routing table names a class with no
real record (Alchemist is APG-only) and separately omits a class that
does have one (`Slayer`). Logged a new `## Open blockers` entry
(self-healed in-cycle, not left standing — see above) and proceeded
directly to **Arcanist**, the first class with a real record.

Verified `apg_classes.lst:11`'s `CLASS:Arcanist` record directly:
`BONUS:COMBAT|BASEAB|classlevel(...)/2` (poor/half BAB — same shape as
APG's Witch), `BONUS:SAVE|BASE.Will|classlevel(...)/2+2` (good Will only),
`BONUS:SAVE|BASE.Fortitude,BASE.Reflex|classlevel(...)/3` (poor Fortitude
and Reflex), `MAXLEVEL:20`, and `SPELLSTAT:INT MEMORIZE:YES SPELLBOOK:YES`
(spellbook-prepared posture, same shape as APG's Alchemist) — confirming
Arcanist belongs in `spellcasting_class.rs`'s allowlist, not `class.rs`'s.

**Widening RED**: added `parses_real_arcanist_record_from_acg_classes_lst`
to `tests/sd17_b_spellcasting_class.rs` (real-corpus-gated on
`PCGEN_CORPUS_ROOT`, new `real_acg_classes_lst()` helper reading
`advanced_class_guide/acg_classes.lst`); ran against the unchanged tree —
failed for the intended reason (`Arcanist` not yet in
`SPELLCASTING_CLASS_NAMES`, silently skipped).

**Acceptance RED**: added `tests/sd22_acg_class_arcanist_resolves.rs`;
ran against the unchanged tree — failed to compile (`E0432`/`E0599`:
`rules_tables::acg` module and `RuleSetId::Acg` did not exist yet) for the
intended reason.

GREEN: widened `SPELLCASTING_CLASS_NAMES` in
`src/pcgen_import/lst_parser/spellcasting_class.rs` by exactly one name
(`Arcanist`). Added `src/rules_core/rules_tables/acg/mod.rs` (new
`AcgClassId` enum, book-local `ClassTableRow`, `class_chassis_resolve`,
with the roster-correction finding recorded in the module doc comment)
and `acg/class_arcanist.rs` (BAB/save chassis only, same scope boundary
as every APG class module). Added `pub mod acg;` and `RuleSetId::Acg` to
`src/rules_core/rules_tables/mod.rs`.

Verification: `cargo test --locked --test sd22_acg_class_arcanist_resolves
-- --include-ignored` 6/6 passed (including the real-corpus-gated
grounding test). `cargo test --locked --test sd17_b_spellcasting_class --
--include-ignored` 21/21 passed, including the new widening test. Full
`cargo test --locked` — every suite green, 0 failed anywhere (grepped
full output for `N failed` with `N > 0`, found none; sibling-preservation
holds, including all six untouched APG class-chassis suites and both
untouched APG spell/equipment suites). `cargo clippy --locked --tests --
-D warnings` clean.

With Arcanist landed, Epic 4 (ACG) has its first class chassis and
`RuleSetId::Acg` resolution — criteria 10-12 complete for Arcanist.
Criterion 13 (per-cycle ACG spell/equipment resolution) remains open
(mirrors APG's criterion 9, a separate future cycle). Nine more real ACG
classes remain (Bloodrager, Brawler, Hunter, Investigator, Shaman, Skald,
Slayer, Swashbuckler, Warpriest). Epic 5 (Bestiary 1) remains blocked on
its own, separate parser gap (no parser recognizes `b1_races.lst`'s
unprefixed bare-row monster records) — unaffected by this cycle.

Full RED/GREEN evidence, file list, and reasoning:
`artifacts/acg/class_arcanist_cycle_receipt.md`. Receipt block appended to
`receipts.md`. Next-eligible for Epic 4: Bloodrager (class 2 of the
corrected 10-class roster), or a dedicated cycle for criterion 13's shared
spell/equipment tables once more classes land.

### cycle-2026-07-19T21:15:57Z | Epic 4, Bloodrager (cycle 2 of corrected 10-class roster) | ingest:acg_class | card `t_5cc43e43` on `codex-tranche-5` (status=done) | open → **complete (criteria 10-12 for Bloodrager)**

Re-checked state before picking a criterion: `git log 3f8df8a..origin/tranche/5`
showed no new commits — `3f8df8a` (the prior Arcanist cycle's own commit) is
still the tip, tree clean. Per Step 1's priority order and the prior cycle's
own `next_required_uplift`, Bloodrager (class 2 of the corrected 10-class
roster) is next-eligible. Re-verified the real `acg_classes.lst` roster
directly before picking (not from memory of the prior cycle's finding):
`grep -oP "^CLASS:\K[A-Za-z-]+" acg_classes.lst | sort -u` still returns the
same 10-class roster (Arcanist, Bloodrager, Brawler, Hunter, Investigator,
Shaman, Skald, Slayer, Swashbuckler, Warpriest, plus the internal
`Ex-Warpriest` variant) — `Bloodrager` has a real `CLASS:Bloodrager` record
at `acg_classes.lst:40`.

Verified the real record directly before writing any test: `BONUS:COMBAT|
BASEAB|classlevel("APPLIEDAS=NONEPIC")|TYPE=Base.REPLACE` (full BAB — no
fractional divisor, unlike Arcanist's poor/half BAB),
`BONUS:SAVE|BASE.Fortitude|classlevel(...)/2+2` (good Fortitude),
`BONUS:SAVE|BASE.Reflex,BASE.Will|classlevel(...)/3` (poor Reflex and Will),
`MAXLEVEL:20`, and (line 44) `SPELLSTAT:CHA MEMORIZE:NO` (spontaneous
casting, same posture as Sorcerer/Bard/Oracle/Summoner) — confirming
Bloodrager belongs in `spellcasting_class.rs`'s allowlist, not `class.rs`'s.

**Widening RED**: added `parses_real_bloodrager_record_from_acg_classes_lst`
to `tests/sd17_b_spellcasting_class.rs` (real-corpus-gated on
`PCGEN_CORPUS_ROOT`, reusing the existing `real_acg_classes_lst()` helper);
ran against the unchanged tree — failed for the intended reason
(`Bloodrager` not yet in `SPELLCASTING_CLASS_NAMES`, silently skipped).

**Acceptance RED**: added `tests/sd22_acg_class_bloodrager_resolves.rs`
mirroring the Arcanist test's shape (plus a cross-class regression test
confirming Arcanist still resolves once Bloodrager lands); ran against the
unchanged tree — failed to compile (`E0599`: `AcgClassId::Bloodrager` did
not exist, 5 call sites) for the intended reason.

GREEN: widened `SPELLCASTING_CLASS_NAMES` in
`src/pcgen_import/lst_parser/spellcasting_class.rs` by exactly one name
(`Bloodrager`), per the file-touch-partition's bounded-widening pattern.
Added `src/rules_core/rules_tables/acg/class_bloodrager.rs` (BAB/save
chassis only, same scope boundary as `class_arcanist.rs`) and
`AcgClassId::Bloodrager` + a match arm in `acg/mod.rs`.

Verification: `cargo test --locked --test sd22_acg_class_bloodrager_resolves
-- --include-ignored` 7/7 passed (including the real-corpus-gated grounding
test and the Arcanist-still-resolves regression check). `cargo test --locked
--test sd17_b_spellcasting_class -- --include-ignored` 22/22 passed,
including the new widening test. Full `cargo test --locked` — every suite
green, 0 failed anywhere (grepped full output for `FAILED`/`error\[`/`N
failed` with `N > 0`, found none; sibling-preservation holds, including the
untouched Arcanist suite, all six APG class-chassis suites, and both APG
spell/equipment suites). `cargo clippy --locked --tests -- -D warnings`
clean.

With Bloodrager landed, Epic 4 (ACG) has two of ten real classes chassis'd —
criteria 10-12 complete for Arcanist and Bloodrager. Criterion 13 (per-cycle
ACG spell/equipment resolution) remains open (mirrors APG's criterion 9, a
separate future cycle). Eight more real ACG classes remain (Brawler, Hunter,
Investigator, Shaman, Skald, Slayer, Swashbuckler, Warpriest). Epic 5
(Bestiary 1) remains blocked on its own, separate parser gap (no parser
recognizes `b1_races.lst`'s unprefixed bare-row monster records) —
unaffected by this cycle.

Full RED/GREEN evidence, file list, and reasoning:
`artifacts/acg/class_bloodrager_cycle_receipt.md`. Receipt block appended to
`receipts.md`. Next-eligible for Epic 4: Brawler (class 3 of the corrected
10-class roster), or a dedicated cycle for criterion 13's shared
spell/equipment tables once more classes land.

### cycle-2026-07-19T22:17:13Z | Epic 4, Brawler (cycle 3 of corrected 10-class roster) | ingest:acg_class | card `t_41a3578f` on `codex-tranche-5` (status=done) | open → **complete (criteria 10-12 for Brawler)**

Re-checked state before picking a criterion: `git status --porcelain | wc -l`
returned 0 and `git fetch origin tranche/5` showed `origin/tranche/5` HEAD
(`143dea6`, the prior Bloodrager backfill cycle's own commit) matching local
HEAD — no other stream landed work in the interim. Per Step 1's priority
order and the prior cycle's own `next_required_uplift`, Brawler (class 3 of
the corrected 10-class roster) is next-eligible. Re-verified the real
`acg_classes.lst` roster directly before picking (not from memory of the
prior cycle's finding): `grep -oP "^CLASS:\K[A-Za-z-]+" acg_classes.lst |
sort -u` still returns the same 10-class roster (Arcanist, Bloodrager,
Brawler, Hunter, Investigator, Shaman, Skald, Slayer, Swashbuckler,
Warpriest, plus the internal `Ex-Warpriest` variant) — `Brawler` has a real
`CLASS:Brawler` record at `acg_classes.lst:84`.

Verified the real record directly before writing any test: `BONUS:COMBAT|
BASEAB|classlevel("APPLIEDAS=NONEPIC")|TYPE=Base.REPLACE|PREVAREQ:
UseAlternateBABProgression,0` (full BAB — no fractional divisor, same
posture as Bloodrager), `BONUS:SAVE|BASE.Fortitude,BASE.Reflex|
classlevel("APPLIEDAS=NONEPIC")/2+2` (good Fortitude **and** Reflex, one
combined token — a new shape not seen in Arcanist or Bloodrager, which each
split their good/poor saves across separate single-save tokens),
`BONUS:SAVE|BASE.Will|CL/3` (poor Will, using the `CL` abbreviation for
`classlevel` rather than the full function-call form used elsewhere in the
same record — same arithmetic, different token spelling), `MAXLEVEL:20`,
and **no `SPELLSTAT:` line anywhere in the Brawler block** — confirming
Brawler is a non-caster and belongs in `lst_parser::class`'s
`MARTIAL_CLASS_NAMES` allowlist (the same allowlist Cavalier widened in
Epic 3), not `lst_parser::spellcasting_class`'s (which both prior ACG
classes, Arcanist and Bloodrager, used).

**Widening RED**: added `parses_real_brawler_record_from_acg_classes_lst`
to `tests/sd17_b1_martial_class.rs` (real-corpus-gated on
`PCGEN_CORPUS_ROOT`, mirroring the existing
`parses_real_cavalier_record_from_apg_classes_lst` pattern); ran against
the unchanged tree — failed for the intended reason (`Brawler` not yet in
`MARTIAL_CLASS_NAMES`, silently skipped, no diagnostic).

**Acceptance RED**: added `tests/sd22_acg_class_brawler_resolves.rs`
mirroring the Arcanist/Bloodrager tests' shape (plus a cross-class
regression test confirming Arcanist and Bloodrager both still resolve); ran
against the unchanged tree — failed to compile (`E0599`:
`AcgClassId::Brawler` did not exist, 5 call sites) for the intended reason.

GREEN: widened `MARTIAL_CLASS_NAMES` in `src/pcgen_import/lst_parser/
class.rs` by exactly one name (`Brawler`), per the file-touch-partition's
bounded-widening pattern — the first ACG class to widen this allowlist
rather than `SPELLCASTING_CLASS_NAMES`. Added
`src/rules_core/rules_tables/acg/class_brawler.rs` (BAB/save chassis only,
same scope boundary as `class_arcanist.rs`/`class_bloodrager.rs`) and
`AcgClassId::Brawler` + a match arm in `acg/mod.rs`.

Verification: `cargo test --locked --test sd22_acg_class_brawler_resolves
-- --include-ignored` 7/7 passed (including the real-corpus-gated
grounding test and the Arcanist+Bloodrager-still-resolve regression
check). `cargo test --locked --test sd17_b1_martial_class --
--include-ignored` 17/17 passed, including the new widening test and every
pre-existing martial-class test (Fighter/Barbarian/Monk/Rogue/Ranger/
Paladin/Cavalier all unaffected). Full `cargo test --locked` — 408
`test result: ok` blocks across every suite, 0 failed anywhere (grepped
full output for `N failed` with `N > 0`, found none; sibling-preservation
holds, including the untouched Arcanist and Bloodrager suites, all six APG
class-chassis suites, and both APG spell/equipment suites). `cargo clippy
--locked --tests -- -D warnings` clean (exit code 0).

With Brawler landed, Epic 4 (ACG) has three of ten real classes chassis'd —
criteria 10-12 complete for Arcanist, Bloodrager, and Brawler. Criterion 13
(per-cycle ACG spell/equipment resolution) remains open (mirrors APG's
criterion 9, a separate future cycle). Seven more real ACG classes remain
(Hunter, Investigator, Shaman, Skald, Slayer, Swashbuckler, Warpriest).
Epic 5 (Bestiary 1) remains blocked on its own, separate parser gap (no
parser recognizes `b1_races.lst`'s unprefixed bare-row monster records) —
unaffected by this cycle.

Full RED/GREEN evidence, file list, and reasoning:
`artifacts/acg/class_brawler_cycle_receipt.md`. Receipt block appended to
`receipts.md`. Kanban card `t_41a3578f` minted and completed on
`codex-tranche-5`. Next-eligible for Epic 4: Hunter (class 4 of the
corrected 10-class roster), or a dedicated cycle for criterion 13's shared
spell/equipment tables once more classes land.

### cycle-2026-07-19T23:16:42Z | Epic 4, Hunter (class 4 of corrected 10-class roster) | ingest:acg_class | card `t_3e37745a` on `codex-tranche-5` (status=done) | open → **complete (criteria 10-12 for Hunter)**

Re-checked state before picking a criterion: `git fetch origin tranche/5` and
`git status --porcelain | wc -l` returned 0; `origin/tranche/5` HEAD
(`3e10397`, the prior Brawler backfill cycle's own commit) matched local HEAD
— no other stream landed work in the interim. Per Step 1's priority order and
the prior cycle's own `next_required_uplift`, Hunter (class 4 of the
corrected 10-class roster) is next-eligible. Re-verified the real
`acg_classes.lst` roster directly before picking (not from memory of the
prior cycle's finding): `grep -oP "^CLASS:\K[A-Za-z-]+" acg_classes.lst |
sort -u` still returns the same 10-class roster (Arcanist, Bloodrager,
Brawler, Hunter, Investigator, Shaman, Skald, Slayer, Swashbuckler,
Warpriest, plus the internal `Ex-Warpriest` variant) — `Hunter` has a real
`CLASS:Hunter` record at `acg_classes.lst:108`.

Verified the real record directly before writing any test: `BONUS:COMBAT|
BASEAB|classlevel("APPLIEDAS=NONEPIC")*3/4|TYPE=Base.REPLACE|PREVAREQ:
UseAlternateBABProgression,0` (three-quarter BAB — same posture as APG's
Alchemist/Inquisitor/Oracle/Summoner), `BONUS:SAVE|BASE.Fortitude,
BASE.Reflex|classlevel("APPLIEDAS=NONEPIC")/2+2` (good Fortitude and
Reflex, one combined token — same shape as Brawler's save token),
`BONUS:SAVE|BASE.Will|classlevel("APPLIEDAS=NONEPIC")/3` (poor Will),
`MAXLEVEL:20`, and (a separate `CLASS:Hunter` line further down the block)
`SPELLSTAT:WIS MEMORIZE:NO` (spontaneous divine casting, same posture as
Bloodrager/Oracle/Summoner) — confirming Hunter belongs in
`spellcasting_class.rs`'s `SPELLCASTING_CLASS_NAMES` allowlist, not
`class.rs`'s `MARTIAL_CLASS_NAMES` (which Brawler widened last cycle).

**Widening RED**: added `parses_real_hunter_record_from_acg_classes_lst`
to `tests/sd17_b_spellcasting_class.rs` (real-corpus-gated on
`PCGEN_CORPUS_ROOT`, reusing the existing `real_acg_classes_lst()`
helper); ran against the unchanged tree — failed for the intended reason
(`Hunter` not yet in `SPELLCASTING_CLASS_NAMES`, silently skipped).

**Acceptance RED**: added `tests/sd22_acg_class_hunter_resolves.rs`
mirroring the Arcanist/Bloodrager/Brawler tests' shape (plus a cross-class
regression test confirming Arcanist, Bloodrager, and Brawler all still
resolve); ran against the unchanged tree — failed to compile (`E0599`:
`AcgClassId::Hunter` did not exist, 5 call sites) for the intended reason.

GREEN: widened `SPELLCASTING_CLASS_NAMES` in
`src/pcgen_import/lst_parser/spellcasting_class.rs` by exactly one name
(`Hunter`), per the file-touch-partition's bounded-widening pattern.
Added `src/rules_core/rules_tables/acg/class_hunter.rs` (BAB/save chassis
only, same scope boundary as `class_arcanist.rs`/`class_bloodrager.rs`/
`class_brawler.rs`) and `AcgClassId::Hunter` + a match arm in `acg/mod.rs`.

Verification: `cargo test --locked --test sd22_acg_class_hunter_resolves
-- --include-ignored` 7/7 passed (including the real-corpus-gated
grounding test and the Arcanist+Bloodrager+Brawler-still-resolve
regression check). `cargo test --locked --test sd17_b_spellcasting_class
-- --include-ignored` 23/23 passed, including the new widening test. Full
`cargo test --locked` — every suite green, 0 failed anywhere (grepped
full output for `FAILED`/`error\[`/`N failed` with `N > 0`, found none;
sibling-preservation holds, including the untouched Arcanist, Bloodrager,
and Brawler suites, all six APG class-chassis suites, and both APG
spell/equipment suites). `cargo clippy --locked --tests -- -D warnings`
clean.

With Hunter landed, Epic 4 (ACG) has four of ten real classes chassis'd —
criteria 10-12 complete for Arcanist, Bloodrager, Brawler, and Hunter.
Criterion 13 (per-cycle ACG spell/equipment resolution) remains open
(mirrors APG's criterion 9, a separate future cycle). Six more real ACG
classes remain (Investigator, Shaman, Skald, Slayer, Swashbuckler,
Warpriest). Epic 5 (Bestiary 1) remains blocked on its own, separate
parser gap (no parser recognizes `b1_races.lst`'s unprefixed bare-row
monster records) — unaffected by this cycle.

Full RED/GREEN evidence, file list, and reasoning:
`artifacts/acg/class_hunter_cycle_receipt.md`. Receipt block appended to
`receipts.md`. Next-eligible for Epic 4: Investigator (class 5 of the
corrected 10-class roster), or a dedicated cycle for criterion 13's shared
spell/equipment tables once more classes land.
