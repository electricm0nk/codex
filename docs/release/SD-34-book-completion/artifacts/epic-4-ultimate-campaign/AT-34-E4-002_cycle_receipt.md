# Cycle AT-34-E4-002 — Epic 4 (Ultimate Campaign) / AT-34-E4-002

- **Commit SHA:** `4005925ae2f70777d3191a13f9895cdec91c4c6f`
- **Files touched:** `src/bin/v06_work_inventory.rs`, `scripts/completion_atlas.py`,
  `docs/work-inventory.json`,
  `docs/release/SD-34-book-completion/artifacts/epic-1-atlas/completion-atlas.json`,
  `docs/release/SD-34-book-completion/artifacts/epic-4-ultimate-campaign/AT-34-E4-002_cycle_receipt.md`,
  `docs/release/SD-34-book-completion/kanban.md`, `docs/release/SD-34-book-completion/progress.md`,
  `docs/retro/events/sd34-e4002-*.jsonl`
- **Identifier audit result:** OK_NO_BUNDLE_TAGS (own-diff check against the epic's file-touch
  set; the diff vs. `merge-base HEAD origin/develop` also matches inherited, already-shipped
  `vacuous_placeholder_row_no_corpus_content_to_render` evidence strings from earlier cycles —
  a legitimate corpus-data term, not a stub token, and not introduced by this cycle)
- **Wired-integration audit result:** OK_NO_TOKENS (same scope/caveat as above)
- **Acceptance criterion (verbatim, `epic-breakdown.md`):**
  > **Evidence:** `python3 scripts/completion_atlas.py --book ultimate_campaign --check` exits 0
  > with `DONE=265 of 265`, every other bucket zero, plus
  > `artifacts/epic-4-ultimate-campaign/ultimate-campaign-completion-manifest.json`.

## Status: PARTIAL — the bar is not met this cycle; population closed and remainder named

## Starting population (re-derived at HEAD before this cycle's fix)

```
python3 scripts/completion_atlas.py --book ultimate_campaign --check
```
```
book=ultimate_campaign population=265 unclassified=0 overlap=0
  DONE: 127   A: 0   B: 5   C: 0   D: 4   M: 88   V: 18   U: 21   X: 2   Z: 0
```
(AT-34-E4-001 closed the U/X question by proof without moving buckets; this cycle is the first
to move real population toward `DONE=265`.)

## The fix — bucket B (5 units) closed, real cause: a missing PI-coordinate wire-up

All 5 of `ultimate_campaign`'s bucket-B trait units are `NAMEISPI:YES` deity-linked traits
(`Corpse Cannibal (Urgathoa)`, `Pain Is Pleasure (Zon-Kuthon)`, `Shadow Whispers (Norgorber)`,
`Voice of Monsters (Lamashtu)`, `Wrecking Wrath (Rovagug)` — `uca_abilities_traits.lst:280-284`).
PI-masking rewrites their corpus record's `key`/`name` to `Codex-Named Unit (...)` at ingestion,
so a plain key/name `resolve` never finds the record even though it physically exists
(`data/corpus/ultimate_campaign/trait_generic/codex_named_unit_trait_ultimate_campaign_uca_abilities_traits_lst_280.json`
holds `Corpse Cannibal`'s real description, verified by direct read).

`decisions.md §14` already built this exact fallback — `simple_kind_verdict`'s `coordinate`
parameter + `SimpleKindTable::resolve_by_coordinate`, wired for `Kind::Domain` (AT-34-E3-001) and
`Kind::Deity` — but `Kind::Trait`'s call site in `classify()` still passed `None`. The
`by_coordinate` index itself is built generically for every kind in `simple_kind_tables.rs`, so
this was a pure missing wire-up, not new capability.

**TDD:** RED test `a_pi_renamed_trait_record_resolves_by_coordinate_and_leaves_bucket_b` added
first (real corpus record, `ultimate_campaign`/`uca_abilities_traits.lst:280`/`Trait ~ Corpse
Cannibal`), confirmed failing for the intended reason (`engine-does-not-hold` == `engine-does-not-hold`,
i.e. still bucket B) before the fix. Fixed `Kind::Trait`'s arm to build and pass the same
`"{engine_book}:{file}:{line}"` coordinate `Kind::Domain`/`Kind::Deity` already pass. GREEN.
Added the monotonicity sibling `a_trait_record_absent_from_the_table_and_with_no_matching_coordinate_stays_bucket_b`
(a genuinely absent coordinate must still refuse cleanly, never fabricate a hit) — also GREEN.
Full `src/bin/v06_work_inventory.rs` suite: 414/414 passed (0 regressions).

## Population re-derived at HEAD after the fix

```
python3 scripts/completion_atlas.py --book ultimate_campaign --check
```
```
book=ultimate_campaign population=265 unclassified=0 overlap=0
  DONE: 130   A: 0   B: 0   C: 0   D: 5   M: 89   V: 18   U: 21   X: 2   Z: 0
```
Denominator: 265 units in `ultimate_campaign` (`docs/work-inventory.json`, HEAD). `B: 5 -> 0`.
The 5 units split honestly on their real corpus shape, not forced into one bucket: 3
(`Corpse Cannibal`, `Pain Is Pleasure`, `Voice of Monsters`) are zero-magnitude, real-description,
`display`-class -> promoted to `text-complete`/`DONE` (`DONE +3`, `127->130`). 1 (`Shadow
Whispers`) carries a real `BONUS:SKILL` magnitude token -> `ingested-magnitude`/bucket `M`
(`M +1`, `88->89`, correct: this table is a lookup, not a compute path, `decisions.md §2a`). 1
(`Wrecking Wrath`) has genuine prose ability-scaling ("add your Strength modifier to the damage
roll a second time") -> `ambiguous` wiring class, correctly falls to bucket `D` pending
wiring-class review (`D +1`, `4->5`), not silently forced to DONE.

## Corpus-wide side effect (same fix, same mechanism, other books)

`python3 scripts/completion_atlas.py --check` at HEAD: `population=49438 buckets=10
unclassified=0 overlap=0`, `B: 11921 (pre-existing, per progress.md) -> 11831` (`-90`, corpus-wide:
every other book's PI-masked trait record that only lacked this same wire-up also resolved).
`DONE` and other buckets moved correspondingly for the same reason as ultimate_campaign's own
3/1/1 split, applied corpus-wide. No unit outside `trait`-kind PI-masked-coordinate records was
touched; `unclassified=0`, `overlap=0` before and after.

**Instrument correction, logged:** the 22-line insertion (test + production fix) shifted
`completion_atlas.py`'s bucket-`V` citation off `src/bin/v06_work_inventory.rs:11707`, caught by
`AT-34-E1-002` condition 6 (`citation_failures=1` on the first re-run after the fix). Re-derived
to the real current line (`11722`, `item.verdict.status = "literal-verified"`), re-verified
`citation_failures=0`.

## Remainder — 135 of 265 units NOT yet `DONE`, named by sub-cause with population

The criterion's full bar (`DONE=265 of 265`, every other bucket zero) is **not met**. Every
remaining unit is named, by mechanism, summing exactly to the gap:

| Bucket | Population | Sub-cause | Clearing mechanism (`technical-design.md §5` / `decisions.md §17`) |
|---|---:|---|---|
| `M` | 89 | 59 `trait` + 30 `ability`, magnitude ingested, never computed (`trait_content_table_holds_record_magnitude_not_yet_computed` / `ability_...`) | run the existing compute path and apply the result — real engine work, not a lookup fix; unmeasured per-unit cost, must be measured on a sample before a population run (`decisions.md §12` L8-adjacent throughput discipline) |
| `V` | 18 | `ability`, verified by proxy only | run through the SD-33 oracle harness (`scripts/oracle_harness/`) |
| `D` | 3 | `ability` (`Blood of Dragons ~ Saving Throw`, `Deathtouched ~ Mind-Affecting`, `Loyalty across Lifetimes ~ Eidolon Bonus`): each has real player-facing text ONLY under an `ASPECT:` token, no `DESC:`/`SPROP:`/`BENEFIT:` — `closure_has_real_description` reads only those three prefixes, so `has_real_description=false` even though `wiring_class=display` (diagnosed by direct corpus read; a candidate widening for a future cycle — ASPECT semantics differ book-to-book and were not audited beyond these 3 units, so not attempted this cycle) | widen `closure_has_real_description`'s prefix set to `ASPECT:`, or a narrower book-scoped rule, after auditing ASPECT usage corpus-wide (not attempted this cycle) |
| `D` | 1 (pre-existing) | `trait` (`Alchemical Intuition`): real `DESC:` prose scaling a trait bonus with the character's Charisma modifier — genuinely `ambiguous` wiring class (`prose_ability_scaling`), correctly not display-promoted; carried over unchanged from before this cycle | wiring-class review + a compute-path build (out of scope this cycle) |
| `D` | 1 (new) | `trait` (`Wrecking Wrath`, this cycle's own new finding — reached only because this cycle's fix resolved it out of bucket B): real prose ability-scaling ("add your Strength modifier to the damage roll a second time") — genuinely `ambiguous` wiring class, correctly not display-promoted | wiring-class review + a compute-path build (out of scope this cycle) |
| `U` | 21 | `feat`, `[Not Implemented]`/PI-marker-in-served-description shape — proven by `AT-34-E4-001` to be a deliberate, correct, final verdict (instrument is NOT wrong for this shape); marker-stripping is a **named forward capability**, not decided by this cycle, filed at `AT-34-E5-002` | instrument correction (marker-stripping capability) — out of this cycle's scope by AT-34-E4-001's own prior ruling, not re-litigated here |
| `X` | 2 | `Fearless Zeal`, `Magnum Opus` — splice/truncation corpus defects, proven real/current/unrepairable by `AT-34-E4-001`; **not** the option-pool shape `decisions.md §17` names a new choice-filter mechanism for — these are unrelated corpus-fidelity defects | revisiting the stated (unrepairable) condition; no new mechanism applies |
| **Total** | **135** | | 89 + 18 + 3 + 1 + 1 + 21 + 2 = 135; 130 (DONE) + 135 = 265 |

`M` (89) and `V` (18) are Epic 3's own bucket-clearing mechanisms (compute path, oracle harness)
applied to a different book population — real, substantial engine work each, not addressable as
a wire-up fix. `D`'s 5 need per-unit review, one of which (`Wrecking Wrath`) is this cycle's own
newly surfaced finding, not carried from AT-34-E4-001. `U`/`X` are unchanged from AT-34-E4-001's
proof and are correctly NOT reopened here — `§2`'s rule is that an unpredicted step is a defect
in the atlas, but these are both *predicted and already resolved by proof*, not a gap.

## Figures + re-derive commands (denominator stated with each)

- `population=265` (ultimate_campaign units) — `python3 scripts/completion_atlas.py --book ultimate_campaign --check`, denominator: `docs/work-inventory.json` at HEAD
- `DONE=130 of 265`, `B=0 of 265` — same command
- `population=49438` corpus-wide, `B=11831 of 49438` — `python3 scripts/completion_atlas.py --check`
- `corpus_literal_sweep`: `48708 examined of 51482 read, 0 findings` — `cargo run --release --locked --bin corpus_literal_sweep -- --json-out <path>`, denominator: 51,482 total corpus files (no `data/corpus/**` file was touched this cycle; the pre-existing baseline in `decisions.md` is `48699`; delta `+9` is from other lanes' concurrent corpus work between the baseline's derivation and this run, not from this cycle)
- `derived_evaluator_fixture_check`: `1839 unit(s) cleared over 2580 fixture row(s); 0 failed` — `cargo run --release --locked --bin derived_evaluator_fixture_check -- --json-out <path>`, denominator: 2,580 fixture rows
- 414/414 unit tests in `v06_work_inventory.rs` — `cargo test --bin v06_work_inventory`

## Row-count command output (this cycle's own artifact)

```
$ python3 scripts/completion_atlas.py --book ultimate_campaign --check
book=ultimate_campaign population=265 unclassified=0 overlap=0
  DONE: 130
  A: 0
  B: 0
  C: 0
  D: 5
  M: 89
  V: 18
  U: 21
  X: 2
  Z: 0
```
`DONE=130 of 265`. The criterion's bar (`DONE=265 of 265`, every other bucket zero) is **not
met**. `kanban.md` row is marked `partial`, not `complete`, on the strength of this count alone.

## Build scope verified

- `cargo test --locked --no-run`: exit 0, full workspace, run at `4005925ae2f70777d3191a13f9895cdec91c4c6f`
- `apps/desktop/src-tauri`: not touched, not run (no file in that crate's tree was changed)

## Sweep population

`corpus_literal_sweep`: `48708 examined of 51482 read` both before and after this cycle (no
`data/corpus/**` file was regenerated or touched — this cycle is pure `src/`/`scripts/`/derived-
JSON work). Delta `0` records added by this cycle, consistent with `48708 -> 48708`.

## Oracle pin

Not applicable — no figure in this cycle came from the pinned PCGen oracle checkout; the fix
reads only the live repo's `data/corpus/` tree.

## Movement, four buckets

- **Closure:** 3 `ultimate_campaign` units (`Corpse Cannibal`, `Pain Is Pleasure`, `Voice of
  Monsters`) moved bucket `B` -> `DONE`. Corpus-wide, the same mechanism closed additional
  PI-masked trait records in other books (net `B: -90` corpus-wide; not separately itemized here
  — out of this cycle's book scope).
- **Reclassification:** 1 unit (`Shadow Whispers`) moved `B` -> `M` (a real magnitude was found;
  this table is a lookup, not a compute path — the record is more accurately placed, not closed).
- **Reachability:** 1 unit (`Wrecking Wrath`) moved `B` -> `D` — newly reachable (no longer
  silently absent from the table) but not yet computed; its real prose-scaling shape is a new,
  named finding for the next cycle.
- **Instrument-correction:** `completion_atlas.py`'s bucket-`V` citation (`11707 -> 11722`),
  caught by `AT-34-E1-002` condition 6 on this cycle's own line-shift, re-derived and reverified.

## Notes

- The criterion as written (`DONE=265 of 265`, every other bucket zero, in one cycle) has no
  prior cycle against this criterion — `kanban.md` showed `not-started`. The remaining 135-unit
  population (`M` 89, `V` 18, `D` 5, `U` 21, `X` 2) requires real engine compute-path work and an
  oracle-harness run, each of which is Epic 3-scale work applied to a different book, not
  addressable in a single wire-up fix. Per-unit cost for `M`/`V`/`D` on this shallow book is not
  yet measured; `AT-34-E4-003` (a separate criterion, not attempted this cycle) is where that
  measurement belongs.
- `U` (21) and `X` (2) are deliberately NOT reopened: `AT-34-E4-001` already proved both correct
  and final for `ultimate_campaign`'s specific shapes (feat `[Not Implemented]` markers; two
  corpus splice/truncation defects). `decisions.md §17`'s 2026-08-28 ruling on bucket `U`/`X`
  targeted a *different* shape (`equipment_modifier` zero-magnitude/no-description for `U`;
  option-pool choice-filtering for `X`) that does not match either of `ultimate_campaign`'s
  remaining `U`/`X` populations — verified by direct read of both.
- `Wrecking Wrath`'s newly-surfaced prose-scaling shape is a genuine finding, not a defect in the
  atlas (`§2`): bucket `D`'s own definition already covers "other engine gap, per named
  sub-cause", and `pending_wiring_class_review` is exactly that sub-cause.

## Next-cycle plan

1. `D` (5 units): per-unit wiring-class review — widen `closure_has_real_description` for the 3
   `ability` `ASPECT:`-only records (root cause already diagnosed this cycle), and build a
   compute path for `Alchemical Intuition`'s and `Wrecking Wrath`'s prose ability-scaling (2
   `trait` units, likely small given the count).
2. `M` (89 units) and `V` (18 units): measure per-unit cost on a small sample of each (compute
   path application; oracle-harness run) **before** any population-scoped run, per
   `technical-design.md §6`'s throughput discipline — this doubles as useful early signal for
   `AT-34-E4-003`'s required second cost measurement.
3. Re-run `python3 scripts/completion_atlas.py --book ultimate_campaign --check` after each
   sub-wave; mark `kanban.md` `complete` only when the live count reads `DONE=265 of 265`.
