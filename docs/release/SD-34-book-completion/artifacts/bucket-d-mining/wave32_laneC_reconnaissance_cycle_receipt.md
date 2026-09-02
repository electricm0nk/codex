# Cycle — SD-34 wave 32, Lane C — mine bucket D (reconnaissance + one instrument fix, zero content closures)

**Status: partial.** This cycle re-derived bucket D's real population and full
per-mechanism shape at the branch tip, fixed a real instrument defect that was
blocking `--check` for every lane in the wave, investigated the "text-only,
zero-magnitude, description-shown-to-player = COMPLETE" ruling's applicability
against bucket D's *current* (heavily-mined) shape, and — after a real guarded
regeneration to rule out a stale-cache hypothesis — found **zero bucket-D units
closable to DONE within this cycle's scope**. The remainder is named exactly,
by mechanism, summing to the population (`figures` below).

- **Commit SHA (pushed):** `65c891e277` (citation re-pin; the only content
  change this cycle produced). This receipt's own commit SHA is reported in
  the structured output, not here (the receipt cannot cite its own future
  hash).
- **Files touched this cycle:**
  - `scripts/completion_atlas.py` — re-pinned all 10 `BUCKET_DEFINITIONS`
    citation lines (see "Instrument-correction" below). Already committed and
    pushed as `65c891e277` before the bucket-D investigation began, since it
    blocked deriving a trustworthy population for anything downstream.
  - This receipt, `progress.md`, `kanban.md` (this commit).
  - No `src/`, `data/corpus/`, or `docs/work-inventory.json` changes ship
    this cycle — the guarded regen run (below) produced a byte-identical
    `units` array (only `generated_at` moved), so the pre-regen committed
    copy was restored rather than committing a no-op timestamp bump.

## Population derived fresh, not trusted from the brief

The brief's dispatch-time figures (`population=49438`, `DONE=24963`, 50.5% of 49,438, `python3 scripts/completion_atlas.py --check`) matched exactly at
this cycle's own re-derivation:

```
python3 scripts/completion_atlas.py --check
-> population=49438 buckets=10 unclassified=0 overlap=0
   DONE: 24963  A: 449  B: 11769  C: 4173  D: 2955  M: 4449  V: 289  U: 202  X: 170  Z: 19
   done_evidence_violations=0 missing_clearing_mechanisms=0 citation_failures=10 (before this cycle's fix)
```

Bucket D's real population is **2,955 of 49,438** — the same figure the brief
stated; not smaller or reshaped at a coarse level. The brief's framing ("one
shape, zero magnitude") does **not** hold at the mechanism level, though — see
"Discoveries" below. Per-book split (`python3 scripts/completion_atlas.py
--by-book`, D column only, largest first): `ultimate_psionics` 465,
`inner_sea_gods` 322, `adventurers_guide` 261, `core_rulebook` 366,
`bestiary_2` 158, `bestiary_4` 143, `advanced_race_guide` 107, `bestiary_3` 92,
`occult_adventures` 78, `ultimate_magic` 60, remaining 22 books smaller. No
single book's D population is dominated by one closable mechanism (see the
per-mechanism table, which cuts across books).

## Instrument-correction (landed, pushed, not a bucket-D closure)

`--check` reported `citation_failures=10` at this cycle's own start SHA
(`4df2c3fa0a`) — every one of `completion_atlas.py`'s 10 `BUCKET_DEFINITIONS`
citations had drifted from a prior wave's own uncredited edits to
`v06_work_inventory.rs` (`199ec991e0`/`9d2e7d9e28`, clippy remediation).
Confirmed pre-existing, not self-caused, via a clean `git status --porcelain`
before any local edit. Re-derived all 10 by fresh `grep -n` against the live
file's real construction-site literal for each marker: DONE/M/D/Z/X/U all
landed at exactly `old_line+23`; A/B/C/V all landed at exactly `old_line+14`
(two separate uniform insertions). Fixed and pushed as `65c891e277` before
population-deriving work began, since an unclean `--check` cannot be trusted
to report a real bucket-D population. `python3 scripts/completion_atlas.py
--check` → `citation_failures=0`, exit 0, confirmed again at this receipt's
own final HEAD. Wave 32's own Lane B (independently dispatched at the *same*
defect) confirmed this fix byte-identical to its own independent derivation
(`0eaba444bc`, "independent audit confirms lane C's citation re-pin") — no
double-fix collision; the two lanes converged on the same 10 lines because the
lines are a deterministic function of the live file, not a judgment call.

This did not move any bucket population (`DONE`/`D`/etc. counts are identical
before and after) — reported here as instrument-correction, per the wave's
four-bucket movement taxonomy, not as a closure.

## Discoveries: why "one shape, zero magnitude" does not unlock bucket D right now

Bucket D's 2,955 units decompose into exactly six shapes (enumerated by
`evidence` string via `scripts/completion_atlas.py`'s own `_bucket_of`,
re-derived at this cycle's HEAD, summing exactly):

| Shape | Population | Magnitude | Real description? | What actually blocks DONE |
|---|---:|---|---|---|
| `*_content_table_holds_zero_magnitude_record_pending_wiring_class_review` (template/deity/ability/race_trait_generic/language/domain/skill/trait — Epic 2's 8 simple-kind tables' shared fallthrough) | 1,727 | 0 (`text_only`) | 427 of 1,727 (25%) | See below — the genuine "zero magnitude" shape, and the one the ruling targets, but already fully mined |
| `class_feature_of_unmodelled_corpus_class:<class>` (75 distinct prestige/base classes, `book_of_the_damned_volume_1`'s `diabolist` among them) | 931 | mostly 1–2 (NOT zero) | not checked (moot — not text-only) | a whole new class chassis per class (BAB/save progression + feature tree) — Epic 4/5 scope, does not match "zero magnitude" at all |
| `class_feature_no_dedicated_magnitude_id_matched_the_record_slug` | 179 | mostly 1–3 (NOT zero) | not checked (moot) | real magnitude-id matching work, Epic 3 scope |
| `race_trait_record_loaded_but_never_applies` | 53 | 0 | 0 of 53 | `RaceCorpus` terminal `TraitRole::Unclassified` state; a prior cycle (`AT-34-E3-001` race_trait_absent receipt) already named this "needs a cross-book ownership shape (Shape 8)" |
| `class_modelled_but_no_observed_delta_on_the_rendered_snapshot` (`Kind::Class`, not `class_feature`) | 38 | 1–2 | moot (class-level, not description-shaped) | class-level rendered-snapshot delta observation, Epic 5 scope |
| `class_feature_*_held_by_*_table` (class-skill-list / wizard-school-spell-list / weapon-and-armor-proficiency / weapon-proficiency, `AT-34-E3-001` cycles 5–9's own four rungs) | 27 | 0 | 0 of 27 (`description: null` by the code's own comment, verified against the corpus, not assumed) | genuinely no player-facing text exists for these records; "leaving the display gap for whichever mechanism owns `has_real_description`" (the rung's own doc comment) |

**Sum: 1,727 + 931 + 179 + 53 + 38 + 27 = 2,955 — matches the D population
exactly**, re-derived via `scripts/completion_atlas.py`'s own `_bucket_of`
over `docs/work-inventory.json` (script at
`/tmp/.../scratchpad/final_d_breakdown.py` this session, reproducible by
grouping `evidence` strings the same way).

**Only the first shape (1,727 units) is genuinely `zero-magnitude` and is the
one the brief's ruling targets.** I traced it to its exact code path —
`simple_kind_verdict` (`src/bin/v06_work_inventory.rs:10130`), the function
shared by all seven of Epic 2's simple-kind tables plus `class_feature`'s
sibling rungs — and its promotion gate (`is_display_wiring_class_for_
promotion` + `has_real_description` + `!universal_sheet_modifier`,
`v06_work_inventory.rs:10180-10191`) is **exactly** the "text-only, real
description = COMPLETE" ruling already, landed by an earlier wave (the doc
comment at `v06_work_inventory.rs:770-774` states the ruling verbatim: "A
guard or a choice on a row with no magnitude token gates TEXT, and text is
`display` work under the standing ruling").

Cross-referencing all 1,727 units against their real corpus `DESC:` tokens
(script this session, `scratchpad/scan_d_desc.py`): **only 427 of 1,727 (25%,
same command) carry a real description at all** — the rest (1,300) are structurally invisible
internal template rows (`VISIBLE:NO`, e.g. `advanced_class_guide`'s
`Arcanist ~ Acid Damage` template — raw corpus record has no `DESC:` token,
confirmed by direct read of the corpus JSON) or PI-masked deity/domain/ability
stat-block rows carrying only structured facts (`DOMAINS`, `ALIGN`,
`FACTSET`, ...) with no prose. Of the 427 that DO have a real description,
cross-referencing against `wiring_class` (`scratchpad/scan_intersect.py`)
finds only **3** where `wiring_class == "display"` (the gate's own
requirement) — all three (`ultimate_magic`'s `altered_form_spontaneous`/
`bestial_form_spontaneous`/`monstrous_form_spontaneous`) are universal
size-bonus modifiers, correctly excluded by the SAME `universal_sheet_
modifier` gate the shipped `gnome_size_is_demoted_from_done_by_the_
universal_modifier_gate` test (`v06_work_inventory.rs:18148`) already proves
must never read `text-complete`. **Zero of the 1,727-unit shape is a
promotable-but-blocked record right now** — the ruling has already been
mined to its floor here by prior Epic 2/3 waves.

**Self-corrected before shipping, not silently discarded.** My first
hypothesis, after finding 401 `deity`-kind units with real portfolio/title
descriptions (`"Master of the Final Incantation; Demon lord of forbidden
lore, magic, snakes"`) stuck at `wiring_class: "computed"` reason
`"pre_guard"`, was that `docs/work-inventory.json` was **stale** relative to
`wiring_class.rs`'s current `signals_with_rules` (whose `pre_guard` detection
is gated `if !mags.is_empty()`, and these deity rows carry zero
`MAGNITUDE_TOKENS` fields — `DOMAINS`/`ALIGN`/`FACTSET`/`!PRECAMPAIGN` are
none of them magnitude tokens). I ran the full guarded regeneration to test
this: `cargo run --locked --bin corpus_literal_sweep -- --json-out ...`
(CLEAN, `48706 of 51476` examined, `0 findings`), `cargo run --locked --bin
derived_evaluator_fixture_check -- --json-out ...` (`1839 unit(s) cleared
... 0 failed`), then `CORPUS_LITERAL_SWEEP_REPORT=... DERIVED_FIXTURE_CHECK_
REPORT=... cargo run --locked --bin v06_work_inventory` (no
`--allow-stamp-loss`, none needed) — a real, live-executed re-derivation, not
a partial or synthetic one. Result: `docs/work-inventory.json`'s `units`
array is **byte-identical** before and after (`git diff --stat
docs/work-inventory.json` → only `generated_at`'s timestamp moved, 1 line).
The hypothesis was **wrong** — the current binary, run fresh against the
current corpus, reproduces `wiring_class: computed`/`pre_guard` for these
deity units exactly as committed. I did not fully re-derive *why* `has_guard`
fires on a row I read as carrying zero `MAGNITUDE_TOKENS` fields (the
`token_closure_rows` aggregation may pull in a related row I have not traced,
or `!PRECAMPAIGN:1,Inner Sea World Guide` participates through a path
`signals_with_rules`'s doc comment does not spell out) — flagging this
explicitly as unresolved rather than asserting a mechanism I have not
verified. What IS verified: the committed inventory is **not stale**, so the
next lane does not need to re-run this same regen to check the same
hypothesis. Logged as a `correction` retro event (own hypothesis, corrected
by the guarded regen itself, verified-by: `git diff --stat
docs/work-inventory.json` showing 0 unit-level delta).

## Movement (four buckets, this cycle)

- **Closure (bucket → DONE):** 0. Verified by bucket-diff:
  `python3 scripts/completion_atlas.py --check` before and after this
  cycle's own commit report identical counts in every bucket
  (`DONE: 24963` both times, `D: 2955` both times).
- **Reclassification (bucket → different non-DONE bucket):** 0. No unit
  changed `evidence` or `status` this cycle (`docs/work-inventory.json`
  content is byte-identical to the cycle's start, confirmed above).
- **Reachability:** 0 units newly reached or lost reachability.
- **Instrument-correction:** 10 `BUCKET_DEFINITIONS` citation-line pins in
  `scripts/completion_atlas.py` (`65c891e277`, detailed above); plus this
  cycle's own falsified stale-cache hypothesis, which is itself a (negative)
  instrument-correction finding for the next lane to inherit rather than
  re-derive.

## Figures (every number, its command, its denominator)

- `population=49438` — `python3 scripts/completion_atlas.py --check`, of the
  full corpus (`docs/work-inventory.json`'s `units` array length).
- `D: 2955` (before and after this cycle) — same command, of `population=49438`.
- `citation_failures: 10 -> 0` — same command's own `citation_failures=` line,
  of `BUCKET_DEFINITIONS`'s 10 entries.
- D-shape sum `1727+931+179+53+38+27=2955` — re-derived this session via a
  fresh `_bucket_of`/`evidence`-string grouping over `docs/work-inventory.json`
  (script content reproduced inline above), of `D: 2955`.
- `427 of 1727` (25%) simple-kind-table D units carry a real corpus `DESC:`
  token — this session's direct corpus-JSON cross-reference
  (`data/corpus/<book>/<kind>/*.json`'s `data.raw_tokens` `DESC` key), of the
  1,727-unit shape.
- `3 of 427` pass `wiring_class == "display"` (the promotion gate's own
  requirement) — same cross-reference, of the 427 with a real description;
  all 3 independently confirmed `universal_sheet_modifier`-excluded by the
  shipped `gnome_size_is_demoted_from_done_by_the_universal_modifier_gate`
  test's own pattern.
- Guarded regen: `docs/work-inventory.json` unit-level diff `0` —
  `git diff --stat docs/work-inventory.json` (1 line changed, `generated_at`
  only), of the file's `units` array, cross-checked by `corpus_literal_sweep`
  (`48706 of 51476 examined, 0 findings, CLEAN`) and
  `derived_evaluator_fixture_check` (`1839 unit(s) cleared ... 0 failed`).

## Remainder, named by mechanism (all 2,955 D units, none unnamed)

1. **1,727** — Epic 2 simple-kind-table zero-magnitude fallthrough
   (template/deity/ability/race_trait_generic/language/domain/skill/trait).
   Already mined to its floor (see Discoveries). Real next-cycle work: (a)
   ingest real `DESC:` prose for the 1,300 description-less records where one
   plausibly exists upstream (PCGen oracle re-check per record — a real
   per-record content-ingestion pass, not a code fix), or (b) accept the
   1,300 invisible/PI-stat-block records as correctly-not-text-only-eligible
   and re-scope them under a different disposition (operator ruling territory
   — `decisions.md §2`-shaped, not this lane's to invent).
2. **931** — `class_feature_of_unmodelled_corpus_class` across 75 classes.
   Needs a real chassis per class (Epic 4/5, `prestige_class_entry_gate.rs`-
   shaped registration where a chassis pattern already exists, a genuinely
   new build otherwise). Largest single class: `phrenic_slayer` (47,
   `ultimate_psionics`).
3. **179** — `class_feature_no_dedicated_magnitude_id_matched_the_record_slug`,
   magnitude-bearing (not zero), needs real magnitude-id matching (Epic 3).
4. **53** — `race_trait_record_loaded_but_never_applies`. Already named by a
   prior `AT-34-E3-001` cycle's own receipt as needing "a cross-book ownership
   shape (Shape 8)" — not re-derived fresh here, cited as the standing
   diagnosis.
5. **38** — `class_modelled_but_no_observed_delta_on_the_rendered_snapshot`
   (`Kind::Class`). Class-level rendered-snapshot-delta wiring, Epic 5.
6. **27** — the four `AT-34-E3-001` cycles 5–9 "held by table" rungs
   (class-skill-list 10, wizard-school-spell-list 9, weapon-and-armor-
   proficiency 5, weapon-proficiency 3). `description: null` confirmed
   against the corpus; blocked on real prose ingestion for these exact 27
   records, or a non-text disposition ruling.

No unit is left as "the rest" — every one of the 2,955 is inside a named
mechanism above, and the six populations sum exactly to 2,955.

## Verification

- `python3 scripts/completion_atlas.py --check` → clean, `citation_failures=0`,
  `done_evidence_violations=0`, exit 0 (this receipt's own final state).
- `python3 scripts/denominator_gate.py --check` → `files_checked=151
  violations=0` (run against this receipt after writing it).
- `python3 scripts/denominator_gate.py --check-provenance` → run against this
  receipt after writing it (see structured output for the literal line).
- No `src/`/`Cargo.toml`/test files touched this cycle (only
  `scripts/completion_atlas.py`, already verified above, plus docs) — a full
  `cargo test`/`cargo build --no-run` re-run is not warranted by this
  cycle's own diff and was not run; the corpus-facing binaries this cycle DID
  run (`corpus_literal_sweep`, `derived_evaluator_fixture_check`,
  `v06_work_inventory`) each exited `0` with the outputs quoted above.
- `git status --porcelain` clean before every write this cycle; no `git add
  -A`; each `git diff --cached --numstat` read before committing.

## Next-cycle plan

Cheapest-first, per the six named mechanisms above:

1. **27** — the four "held by table" rungs: confirm with the operator whether
   these 27 description-less records get a non-text disposition (e.g. a new
   evidence string distinct from `text-complete` that still counts as DONE
   for a set/list-shaped record with no prose) or whether real DESC prose
   exists upstream and was never ingested. Smallest population, cleanest
   question, already fully diagnosed.
2. **53** — `race_trait_record_loaded_but_never_applies`: pick up the prior
   cycle's own named "cross-book ownership shape (Shape 8)" investigation.
3. **38** — `class_modelled_but_no_observed_delta_on_the_rendered_snapshot`:
   scope the class-level snapshot-delta wiring (likely shares plumbing with
   the existing `--class-probe` instrument already in `v06_work_inventory`'s
   `main()`).
4. **179** — `class_feature_no_dedicated_magnitude_id_matched_the_record_slug`:
   needs its own per-mechanism scoping pass (magnitude present, not
   zero-magnitude — outside this wave's "zero magnitude" framing entirely).
5. **1,727** simple-kind-table shape and **931** unmodelled-class shape are
   both real but large (content-ingestion and new-chassis-building
   respectively) — decompose per-book or per-class before dispatching, the
   same way `AT-34-E3-001`'s nine mechanisms and `class_feature_of_
   unmodelled_corpus_class`'s 75 classes each already are.
