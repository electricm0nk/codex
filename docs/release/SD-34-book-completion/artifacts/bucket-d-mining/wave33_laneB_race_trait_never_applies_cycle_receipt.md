# Cycle — SD-34 wave 33, Lane B — the 53 `race_trait_record_loaded_but_never_applies` units

**Status: partial.** Zero units reach `DONE` this cycle. Every one of the 53 is named by an
exact sub-cause with a population that sums to 53, and 27 of them get a genuinely more honest
`engine-does-not-hold` evidence string than the blanket "never applies" they carried before
(instrument-correction — zero bucket movement). The dispatch's own "Shape 8" citation is corrected
in place: it names a *different* mechanism (`companion_absent`'s familiar-pool cross-book
ownership gap), not this one — see "Correcting the dispatch's own citation" below.

**Recovered from a server crash** (2026-09-02 kernel soft-lockup under heavy parallel
`rust-lld` link jobs, confirmed via `journalctl -b -1`, unrelated to this cycle's own work — the
crash hit mid-flight before this lane could commit) via its preserved worktree diff, rebased onto
`tranche/14` (which had moved to include wave 33 lane A, `e8fc4f8ff9`, in the meantime), then
landed. The rebase's own real conflicts (four shared instrument files, both lanes' substantive
changes kept, every disputed `completion_atlas.py` citation line re-derived fresh against the
merged source rather than trusted from either lane's own hand-computed offset) surfaced a real
error in this cycle's own pre-fold draft — see "Post-fold correction" immediately below.

**Post-fold correction (this cycle's own pre-fold draft, caught before landing):** the draft
claimed all 21 "Adopted Race" selector records resolve real grants through
`trait_pool::resolve_adopted_race_options`. Re-verified fresh at this cycle's own final commit,
only **20 of 21** do. Bestiary 6's `Rougarou` selector (`rougarou_abilities_race.lst:29`, `KEY:
Adopted Race ~ Rougarou`) chooses from `TYPE=Rougarou Race Trait`, and that literal has exactly
one match anywhere in the corpus — the upstream `No Race Trait Available` placeholder itself. The
pool is genuinely empty by design, so the resolver correctly returns nothing and the record
correctly stays under the blanket evidence string — not a defect in this cycle's own code, a
wrong figure in its own draft prose (present before the crash, unrelated to the rebase). Every
count below (28→27, the sub-cause table, the remainder table, the next-cycle plan) is corrected
in place; retro-logged as a `correction` event (`docs/retro/events/wave33-laneb.jsonl`,
`1788362671809-wave33-laneb-67b09d`).

- **Wave/lane, per the repo, not the brief.** This worktree's system prompt says "wave 32"; the
  repo's own ledger (`scripts/wave_ledger.py`'s `KNOWN_WAVES["wf_cbb90b15-7b0"]`, this cycle's
  own commit `aee47d3c5a`) registers it as **wave 33, lane B**: "the 53 race_traits that load but
  never apply." The repo wins — this receipt and its ledger entries use wave 33.
- **Files touched this cycle:**
  - `src/bin/v06_work_inventory.rs` — two new `RaceTraitProbe` fields
    (`adopted_race_selector_grants`, `adoptive_parentage_rendered`), populated in
    `probe_race_trait_corpus` by calling the SAME two resolver functions
    `race_trait_picker.rs`'s own `list_alternate_racial_traits` Tauri command calls
    (`race_resolver::adopted_race_choose_selectors` + `trait_pool::resolve_adopted_race_options`,
    and `race_resolver::adoptive_parentage_options`); two new `EngineFacts` accessors; two new
    `classify()` checks in the `Kind::RaceTrait` arm giving 27 of the 53 units a precise,
    non-`done` `engine-does-not-hold` evidence string in place of the blanket one; 4 new tests
    (2 RED→GREEN proof cases against the real corpus, 2 regression negatives).
  - `scripts/completion_atlas.py` — five `BUCKET_DEFINITIONS` citation lines re-derived fresh
    against the post-fold merged `v06_work_inventory.rs` (both lane A's and this cycle's own
    insertions present), replacing both lanes' own conflicting hand-computed offset guesses.
  - `docs/work-inventory.json`, `docs/release/SD-34-book-completion/artifacts/epic-1-atlas/
    completion-atlas.json` — regenerated via the guarded path (below), never hand-merged.
  - This receipt, `progress.md`.

- **Commit SHA:** `9ebf638f6f` (code + guarded regen); receipt/progress land in a second,
  docs-only commit on top, per this bundle's own two-commit precedent (wave 33 lane A,
  `9558f4a774` code / `e8fc4f8ff9` docs).
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS` — `git diff --unified=0
  9ebf638f6f^..9ebf638f6f -- src/bin/v06_work_inventory.rs scripts/completion_atlas.py
  docs/work-inventory.json docs/release/SD-34-book-completion/artifacts/epic-1-atlas/
  completion-atlas.json | grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})'` finds
  nothing.
- **Wired-integration audit result:** `OK_NO_TOKENS` — same diff range, `grep -nE
  '\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b'` finds nothing.
- **Acceptance criterion:** *(no `AT-34-E#` card names this specific shape — this is a
  bucket-D mining cycle, continuing wave 32 lane C's own reconnaissance and this wave's own
  dispatch instruction: "establish whether it SHOULD apply and does not (a real defect, fix it)
  or whether it is correctly inert (then it needs an honest evidence string, not a pretended
  one)" for the 53-unit `race_trait_record_loaded_but_never_applies` shape.)*
- **Build scope verified:** scoped to `--bin v06_work_inventory` (§ RED→GREEN above); full
  `cargo test --locked --no-run` at the widest workspace scope deferred to wave-end per this
  fold's own dispatch instruction, run once by a different agent after all three lanes land.

## Correcting the dispatch's own citation — "Shape 8" names a different mechanism

The dispatch brief and the wave-32 Lane C reconnaissance receipt it cites both say this
population's diagnosis already exists: `AT-34-E3-001`'s `race_trait_absent` receipt "already
named this needs a cross-book ownership shape (Shape 8)." Read that receipt in full this cycle
(`artifacts/epic-3-core-rulebook/AT-34-E3-001_race_trait_absent_cycle_receipt.md`) — it never
mentions "Shape 8" or "cross-book ownership" anywhere. Its own subject is a *different*
mechanism, `race_trait_absent_from_race_traits` (9 CRB units moving from bucket B to bucket D,
closed 2026-08-27), and its own "Movement" section explicitly defers the reachability question
for the 9 units it produced ("Whether/how they should ever become reachable... is a different
bucket's own mechanism, not this cycle's") without naming Shape 8 as the answer.

"Shape 8 (cross-book ownership)" is real, but it belongs to a *sibling* receipt about a
*different* content kind: `AT-34-E3-001_companion_absent_cycle_receipt.md` (`grep -c 'Shape 8'`
→ 5 hits there, 0 in `race_trait_absent`'s own file) defines it for the **familiar-pool special-
ability** shape — 14 `companion` records whose true owner is a DIFFERENT book's chassis, needing
"either a cross-book ownership shape (Shape 8, not built this cycle) or a master-side ability-
pool record type this chassis does not have." Nothing about that description matches the 53
`race_trait` units this cycle owns — a shared name ("cross-book"), not a shared thing. Retro-
logged as a `correction` event (`docs/retro/events/wave33-laneb.jsonl`,
`1788346370019-wave33-laneb-597a5f`), caught before any code was written against the false
premise.

## The brief's magnitude/description claim, also corrected

The dispatch brief states the population is "zero magnitude, 0 of 53 carry a real description."
Re-derived fresh against the real corpus this cycle (`docs/work-inventory.json`'s
`magnitude_token_count` field plus each record's own `data/corpus/*/race_trait/**/*.json`
`description`):

```
python3 -c "
import json
from collections import Counter
with open('docs/work-inventory.json') as f:
    units = json.load(f)['units']
sel = [u for u in units if u.get('evidence')=='race_trait_record_loaded_but_never_applies']
print('total', len(sel))
print('magnitude_token_count distribution', dict(Counter(u['magnitude_token_count'] for u in sel)))
"
total 53
magnitude_token_count distribution {0: 32, 2: 15, 1: 6}
```

Only **32 of 53** are zero-magnitude; **21** (the 20 Skinwalker `Change Shape (<Option>)`
components at 1–2 tokens each, plus `Suli ~ Trusted Mediator` at 2) carry a real, non-zero
`magnitude_token_count`. Of the zero-magnitude 32, **8** — not 0 — carry a real corpus
description: ARG's 7 `Adoptive Parentage` options (`Drow`/`Dwarf`/`Elf`/`Gnome`/`Grippli`/
`Halfling`/`Orc`, each `"You were adopted and raised by <race>."`) and `inner_sea_races`' `Human
~ Tribalistic Languages`. Retro-logged as a `correction` event
(`1788346377897-wave33-laneb-2bb3f2`) — caught before implementation, not shipped.

## The real shape: six sub-causes, re-derived from the live corpus, not carried forward

Every one of the 53 was matched to its exact corpus record this cycle (join on `(source_file,
source_line)`, the same identity `race_resolver.rs`'s own probe uses — 53 of 53 matched, 0
misses):

| Sub-cause | Population | Real consumer? | Disposition |
|---|---:|---|---|
| "Adopted Race" selectors (`TYPE:AdoptiveRace`, `CATEGORY:Special Ability`, no `DESC:`) — CRB 7, `bestiary_2` 7, `bestiary_3` 5, `bestiary_5`'s `Skinwalker` 1 | 20 | **Yes** — resolves real, non-empty grants through `trait_pool::resolve_adopted_race_options`, the exact call `list_alternate_racial_traits` makes | Engine-resolved, UI-unwired (see below) — real, buildable next-cycle work |
| `advanced_race_guide`'s "Adoptive Parentage" options (`CATEGORY:Adoptive Parentage`, real `DESC:`) | 7 | **Yes** — `adoptive_parentage_options()`, the exact call the same Tauri command makes | Engine-resolved, UI-unwired — real, buildable next-cycle work |
| `bestiary_5`'s Skinwalker `Change Shape (<Option>)` components (`VISIBLE:NO`, TYPE-pool-referenced, real magnitude 1–2) | 20 | No | Needs a NEW mechanism (a TYPE-pool option picker) — `reach_gate.rs`'s own named remedy, unchanged |
| `core_rulebook`'s Human Ethnicity placeholders (`None`/`Unknown`, `CATEGORY:Background`, no gate) | 2 | No | Needs a NEW mechanism (a `HumanEthnicity` picker) — `reach_gate.rs`'s own named remedy, unchanged |
| `monster_codex`'s `Oversized Goblin` (real DESC, grants two sibling `Alternate` records) | 1 | No | Needs a NEW mechanism (an ability-pool variant picker) — `reach_gate.rs`'s own named remedy, unchanged |
| `inner_sea_races`' `Human ~ Tribalistic Languages` (real DESC) + `Suli ~ Trusted Mediator` (PI-redacted DESC, magnitude 2) | 2 | No | Upstream data gap (`Tribalistic Languages`) / no project-side remedy at all (`Trusted Mediator`, upstream PCGen omits the flag) — `reach_gate.rs`'s own named remedy, unchanged |
| `bestiary_6`'s `Rougarou` "Adopted Race" selector (`TYPE:AdoptiveRace`, same shape as row 1) | 1 | **No** — `TYPE=Rougarou Race Trait` matches exactly one row corpus-wide: the upstream `No Race Trait Available` placeholder itself | Correctly inert — the pool is genuinely empty by design, no real content exists to wire; no remedy possible short of an upstream PCGen data addition |

**Sum: 20 + 7 + 20 + 2 + 1 + 2 + 1 = 53** — matches the population exactly.

This shape decomposition is corroborated, not invented: `apps/desktop/src-tauri/src/
reach_gate.rs`'s own `OPEN_FINDINGS`/`UNREACHED_RECORD_FINDINGS` tables (a real, tested, dated,
code-embedded instrument — "runs the real IPC builder... and checks the records are in the
response") independently name the identical 25-record "no real consumer" set by exact key, with
the identical remedy characterization ("a new mechanism, not a missing wire") for every one of
the 5 named shapes, dated 2026-08-26/27, predating this cycle.

## The real defect this cycle found and fixed: an instrument gap, not a resolver gap

28 of the 53 (the 21 "Adopted Race" selectors + the 7 ARG "Adoptive Parentage" options) are
deliberately `TraitRole::Unclassified` — no readable default/replace/grant gate of their own,
so `race_resolver`'s role vocabulary was never written to describe them (`race_resolver.rs`'s
own module doc names this seam: "`Unclassified` exists so that a row with no readable gate is
*visible*"). But BOTH shapes apply through a real, different, already-shipped consumer:
`race_resolver::adopted_race_choose_selectors`/`trait_pool::resolve_adopted_race_options` for the
selectors, `race_resolver::adoptive_parentage_options` for the parentage options — the exact two
functions `race_trait_picker.rs`'s own `list_alternate_racial_traits` Tauri command calls.
`probe_race_trait_corpus`'s `reachable` set, built from `role != TraitRole::Unclassified` alone,
could not see either consumer, so `v06_work_inventory.rs`'s classifier reported the blanket
`race_trait_record_loaded_but_never_applies` for all 28 — literally false for **27** of them: the
engine DOES resolve real content, verified independently by `reach_gate.rs`'s own dated finding
(`"crb"`/`"race_traits"`, 2026-08-27: "the 7 newly-ingested `Adopted Race ~ <Race>` selectors
resolve through... `adopted_race_choose_selectors`/`trait_pool` path with real payload... 4 real
grants apiece for Dwarf/Elf/Gnome/Half-Elf/Half-Orc/Human, 3 for Halfling"). The 28th, `bestiary_
6`'s `Rougarou` selector, is the one exception in this same `TraitRole::Unclassified` group: its
own resolver call correctly returns empty (the pool has no real member, see the corrected
sub-cause table above), so "never applies" was already the true statement for it and it correctly
stays there — not a bug this cycle needed to fix.

**Fixed:** `probe_race_trait_corpus` now calls both functions (the identical calls
`race_trait_picker.rs` makes, never re-implemented) and records which records they resolve with
real, non-empty output. `classify()`'s `Kind::RaceTrait` arm checks this before falling to the
"never applies" fallback, and gives these 27 a new, precise `engine-does-not-hold` evidence
string instead: `race_trait_adopted_race_selector_resolves_real_grants_but_no_desktop_ui_surface_
reads_them` (20) / `race_trait_adoptive_parentage_option_rendered_by_the_engine_but_no_desktop_
ui_surface_reads_it` (7).

## Why this does NOT reach `done` — the "twin the player reads" bar, checked and failed

The obvious next question: since the engine genuinely resolves real content for these 27, why
not credit `grounded`/`text-complete` (bucket DONE — `completion_atlas.py`'s own `BUCKET_
DEFINITIONS["DONE"]` accepts either status)? Checked, and refused, this cycle:

`reach_gate.rs`'s own module doc states its bar precisely: "runs the real IPC builder... and
checks the records are in the response" — real, but a strictly weaker claim than what
`race_trait_rendered_description`'s own doc comment (and `AGENTS.md`'s "a magnitude is not wired
until it moves on the twin the player reads") requires for `text-complete`/`grounded`: the
SHIPPED SCREEN must read the field, not merely the IPC response carry it. Verified directly, not
assumed:

```
grep -rn "adoptiveParentageOptions\|adoptedRaceOptions" apps/desktop/src
-> no matches (0 files)
```

`apps/desktop/src/boundary/loadAlternateRacialTraits.ts`'s own `AlternateRacialTraitsResponse`
TypeScript interface declares only `races`/`diagnostics`/`findings` — the Rust DTO's
`adoptive_parentage_options`/`adopted_race_options` fields are never typed on the frontend
boundary, and no `.tsx` file under `apps/desktop/src` reads either camelCase name anywhere. The
Rust struct's own doc comment for `adopted_race_options` confirms this was known and deliberate:
"Additive field — a consumer that does not read it is unaffected." (The PRE-EXISTING, already-
`text-complete`/`grounded` `alternates[]`/`standardTraits[]` fields on the SAME response ARE
rendered — confirmed by `grep -rln '\.alternates\b' apps/desktop/src --include=*.tsx`, 3 real
`.tsx` consumers including `raceCatalog/AlternateTraitPicker.tsx` — so this is a genuine,
specific gap in these two NEW fields, not a blanket claim that the picker doesn't work.)

So these 27 stay in bucket D (`engine-does-not-hold`, unchanged population) with a MORE PRECISE,
more honest evidence string than before — not promoted to `done`, because a player does not
today see this content on any real screen. This is exactly the disposition the brief's own bar
calls for: "a unit that loads but never applies is not automatically incomplete — establish
whether it SHOULD apply and does not (a real defect, fix it) or whether it is correctly inert
(then it needs an honest evidence string, not a pretended one)." These 27 are neither of the
brief's two named outcomes exactly: the engine-level "never applies" WAS false and is now fixed
(a real, if narrower, defect closed at the instrument level); "correctly inert" would also be
false (the engine does apply them, for real) — the honest third answer is "applies at the engine
level, not yet wired to any screen," and that is now what the evidence string says. The
`Rougarou` selector, by contrast, genuinely IS the brief's "correctly inert" outcome — checked and
confirmed, not assumed (see the corrected sub-cause table).

## RED → GREEN

- RED (before the fix, confirmed by compiling the new tests against the pre-fix code): both new
  tests failed to compile (`E0599: no method named
  race_trait_adoptive_parentage_rendered_description`) — the intended reason, a missing
  capability, not a typo.
- GREEN, re-run post-fold at this cycle's own final commit:
  `cargo test --locked --bin v06_work_inventory race_trait_grounding_tests:: -j 6` →
  **38 passed; 0 failed** (34 pre-existing + 4 new: 2 proof cases against the real corpus, 2
  regressions pinning `Oversized Goblin` and a Skinwalker `Change Shape` component are
  unaffected).
- Full binary suite, post-fold (includes wave 33 lane A's own new tests, landed by the rebase):
  `cargo test --locked --bin v06_work_inventory -j 6` → **511 passed; 0 failed**.
- `cargo clippy --locked --bin v06_work_inventory -j 6` → clean, 0 warnings.
- `cargo test --locked --no-run` (full workspace) — run once at wave-end by a different agent
  after all three lanes land, per this fold's own dispatch instruction; not re-run here.

## Guarded regeneration

`docs/work-inventory.json` regenerated through the guarded path, no `--allow-stamp-loss` needed,
after the rebase and both source-file corrections above landed:

```
SCRATCH=<any writable scratch dir>; mkdir -p "$SCRATCH/regen"
cargo run --locked --bin corpus_literal_sweep -- --json-out "$SCRATCH/regen/corpus_literal_sweep_report.json" --quiet
-> corpus-literal-sweep: CLEAN
cargo run --locked --bin derived_evaluator_fixture_check -- --json-out "$SCRATCH/regen/derived_evaluator_fixture_check_report.json" --quiet
-> (no findings)
CORPUS_LITERAL_SWEEP_REPORT="$SCRATCH/regen/corpus_literal_sweep_report.json" \
DERIVED_FIXTURE_CHECK_REPORT="$SCRATCH/regen/derived_evaluator_fixture_check_report.json" \
cargo run --locked --bin v06_work_inventory -j 6
-> docs/work-inventory.json regenerated, 49438 units, generated_at 2026-09-02T15:21:49Z
```

`python3 -c "import json; print(len(json.load(open('docs/work-inventory.json'))['units']))"` →
**49438** — matches the population `completion_atlas.py --check` reports below, unchanged from
before the regen (no corpus records added or removed this cycle, only two new probe fields and
two new `classify()` checks).

## Movement (four buckets, this cycle)

- **Closure (bucket → DONE):** 0.
- **Reclassification (bucket → different non-DONE bucket):** 0 — all 53 stay in bucket D
  (`engine-does-not-hold`, no A/B/C marker).
- **Reachability:** 0 — none of the 53 becomes newly reachable to a player this cycle (27 became
  newly *engine-resolved*, which is a real, verified fact, but not player-reachability;
  `Rougarou`'s selector was checked and confirmed to genuinely have no real content, so it is not
  counted here).
- **Instrument-correction:** 27 evidence strings replaced (a false blanket "never applies" with
  a precise, verified "engine resolves it for real, no desktop UI surface reads the response
  field yet"), plus 5 `completion_atlas.py` citation lines re-derived fresh against the post-fold
  merged source (both lanes' own conflicting hand-computed offsets discarded), plus 3
  dispatch/draft-receipt corrections logged to retro (Shape 8 misattribution, the
  magnitude/description figure, this cycle's own pre-fold "21 resolve" claim corrected to 20).

## Figures (every number, its command, its denominator)

- Population, this shape: **53** —
  `python3 -c "import json; d=json.load(open('docs/work-inventory.json')); print(len([u for u in d['units'] if u.get('evidence') in ('race_trait_record_loaded_but_never_applies','race_trait_adopted_race_selector_resolves_real_grants_but_no_desktop_ui_surface_reads_them','race_trait_adoptive_parentage_option_rendered_by_the_engine_but_no_desktop_ui_surface_reads_it')]))"`,
  denominator: all `race_trait` units originally carrying this shape at dispatch time.
- New `..._resolves_real_grants_but_no_desktop_ui_surface_reads_them` evidence: **20** —
  `python3 -c "import json; d=json.load(open('docs/work-inventory.json')); print(len([u for u in d['units'] if u.get('evidence')=='race_trait_adopted_race_selector_resolves_real_grants_but_no_desktop_ui_surface_reads_them']))"`,
  denominator: the 21-record "Adopted Race" selector sub-cause.
- New `..._adoptive_parentage_option_rendered_by_the_engine_but_no_desktop_ui_surface_reads_it`
  evidence: **7** —
  `python3 -c "import json; d=json.load(open('docs/work-inventory.json')); print(len([u for u in d['units'] if u.get('evidence')=='race_trait_adoptive_parentage_option_rendered_by_the_engine_but_no_desktop_ui_surface_reads_it']))"`,
  denominator: the 7-record `advanced_race_guide` "Adoptive Parentage" sub-cause (all 7 resolve).
- Still-blanket `race_trait_record_loaded_but_never_applies`: **26** —
  `python3 -c "import json; d=json.load(open('docs/work-inventory.json')); print(len([u for u in d['units'] if u.get('evidence')=='race_trait_record_loaded_but_never_applies']))"`,
  denominator: same 53; sum with the two lines above is 20+7+26=53, matches exactly.
- `docs/work-inventory.json` total population: **49438** —
  `python3 -c "import json; print(len(json.load(open('docs/work-inventory.json'))['units']))"`,
  denominator: whole corpus, unchanged from pre-cycle (no records added/removed).
- `completion_atlas.py --check`: `population=49438 buckets=10 unclassified=0 overlap=0`, `DONE`
  24985, `D` 2933 (unchanged from wave 33 lane A's own post-cycle figure — this cycle moves no
  unit between buckets, only evidence-string precision within D) —
  `python3 scripts/completion_atlas.py --check`, denominator: whole corpus.

## Remainder, named by mechanism (all 53, none unnamed)

1. **20** — `race_trait_adopted_race_selector_resolves_real_grants_but_no_desktop_ui_surface_
   reads_them` (CRB 7 / `bestiary_2` 7 / `bestiary_3` 5 / `bestiary_5` 1). Real next-cycle work:
   declare `adoptedRaceOptions`/`adoptiveParentageOptions` on the TypeScript
   `AlternateRacialTraitsResponse` boundary and add a real picker section to
   `AlternateTraitPicker.tsx` (or a sibling component) that renders them — genuinely cheap,
   the backend data is already correct and tested.
2. **7** — `race_trait_adoptive_parentage_option_rendered_by_the_engine_but_no_desktop_ui_
   surface_reads_it` (`advanced_race_guide`). Same remedy as #1 — likely the SAME UI change
   closes both in one pass.
3. **20** — Skinwalker `Change Shape (<Option>)` components (`bestiary_5`). Needs a NEW
   TYPE-pool option picker mechanism (per-kin choice of one row out of a `TYPE=Skinwalker Change
   Shape <Kin>` pool) — `reach_gate.rs`'s own `OPEN_FINDINGS` names this exact remedy, "outside
   this fold's scope." Not zero-magnitude (1–2 real tokens each), so the text-only ruling never
   applied here regardless.
4. **2** — Human Ethnicity placeholders (`core_rulebook`: `None`/`Unknown`). Needs a NEW
   `HumanEthnicity`-category picker UI (race-flavor choice, no mechanical grant) —
   `reach_gate.rs`'s own named remedy, operator-ruling territory per `AT-34-E3-001`'s own
   receipt ("outside AT-34-E3-001's scope... a new, mechanically-inert UI surface").
5. **1** — `monster_codex`'s `Oversized Goblin`. Needs a NEW ability-pool variant mechanism (a
   race-level choice of one row out of a `BONUS:ABILITYPOOL|<Pool>|n` pool) —
   `reach_gate.rs`'s own named remedy.
6. **2** — `inner_sea_races`: `Human ~ Tribalistic Languages` (an upstream `TEMPLATE:`-borne
   grant this project does not yet read, or a `HumanEthnicity`-shaped remedy — a new mechanism
   either way) and `Suli ~ Trusted Mediator` (a genuine upstream PCGen data omission — the row
   sets no `FACT:` flag its structurally-identical siblings all set — "REMEDY: none available
   project-side," `reach_gate.rs`'s own words).
7. **1** — `bestiary_6`'s `Rougarou` "Adopted Race" selector. Correctly inert — the `TYPE=
   Rougarou Race Trait` pool it selects from has exactly one member corpus-wide, the upstream
   `No Race Trait Available` placeholder itself. No project-side remedy exists; would need an
   upstream PCGen data addition (a real Rougarou race trait row typed into that pool) before any
   engine or UI work here could matter. Name it as permanently blocked pending upstream, not a
   to-do — the same disposition as `Suli ~ Trusted Mediator` in item 6.

No unit is left as "the rest" — every one of the 53 is inside a named mechanism above, and the
seven populations sum exactly to 53.

## Verification

- `python3 scripts/completion_atlas.py --check` → `population=49438 buckets=10 unclassified=0
  overlap=0`; `DONE=24985 A=449 B=11769 C=4173 D=2933 M=4449 V=289 U=202 X=170 Z=19`;
  `done_evidence_violations=0 missing_clearing_mechanisms=0 stale_derived_at=False
  citation_failures=0`. Run fresh at this cycle's own final pre-commit state, post-fold.
- `python3 scripts/denominator_gate.py --check` → `files_checked=155 violations=0`.
- `python3 scripts/denominator_gate.py --check-provenance` → `files_checked=85
  figures_examined=128 violations=0`.
- `git status --porcelain` clean before every write this cycle; no `git add -A`; each
  `git diff --cached --numstat` read before committing.

## Next-cycle plan

1. **27** (sub-causes 1–2 above) — wire `adoptedRaceOptions`/`adoptiveParentageOptions` into the
   desktop TypeScript boundary and a real picker UI section. Cheapest, highest-value: the
   backend is done and tested; this is pure frontend wiring, likely closable to `done` in one
   cycle once shipped (both are `text_only`/zero-magnitude, so a real rendered description is
   the whole bar).
2. **20** (Skinwalker `Change Shape`) — scope a TYPE-pool option picker; largest remaining
   sub-population, real magnitude, needs both a resolver-side pool mechanism and a UI surface.
3. **2+1** (Human Ethnicity, Oversized Goblin) — each needs an operator ruling on whether a
   dedicated, mechanically-inert picker UI is in scope, or whether these stay a named, accepted
   gap.
4. **2+1** (`inner_sea_races`, `Rougarou`) — `Human ~ Tribalistic Languages` needs a
   `TEMPLATE:`-reading mechanism (new); `Suli ~ Trusted Mediator` and `Rougarou`'s selector have
   no project-side remedy at all (upstream data gaps) — name both as permanently blocked pending
   an upstream PCGen fix, not a to-do.
