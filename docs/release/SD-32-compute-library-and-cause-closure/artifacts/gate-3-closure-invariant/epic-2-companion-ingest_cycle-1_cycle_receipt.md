# Cycle epic-2-companion-ingest — gate-3-closure-invariant / `decisions.md §20`

- **Card ID:** `epic-2-cause-closure` (kanban row 11; rows 11 and 15 left `in-progress` per
  dispatch instruction).
- **Corpus SHA:** `7f818006e371188e5717fd18d74d18a420747fc6` (`scripts/pcgen-oracle-pin.env`;
  `scripts/verify.sh --only preflight-oracle` → PASS, oracle bootstrapped fresh into the repo-local
  `artifacts/corpus/operator-supplied/pcgen` slot, never `~/workspace/repos/pcgen`).

## Scope

`decisions.md §20`: `Kind::Companion`'s enumerated units carried 769 `join_status: no_record`
(`scripts/shape_ledger.py`) — enumerated but never ingested, so Gate 1's "every unit's shape is
measured" was unmet. This cycle's own dispatch brief flagged an unresolved contradiction: a prior
T9 cycle had already closed `companion` scope "at zero net new records," so this cycle's first job
was establishing whether the 769 was a different population, or a mechanism gap that cycle did not
hit.

## §17a re-derivation, and resolving the contradiction, before building anything

`python3 scripts/shape_ledger.py --inventory docs/work-inventory.json --output /tmp/l.json` then
`Counter(x['kind'] for x in rows if x['join_status']=='no_record')`: **`companion` 769**, confirming
the brief's figure exactly.

Read the prior T9 cycle's own kanban entry (`epic-2-t9-feat-equipment-companion-monster`) in full:
it closed a **4-unit** `companion` population — both `bestiary_4` units are `.COPY=`/`.MOD` deltas
`transcribe_companion_tables.py`'s own tested contract correctly refuses, and both `bestiary_5`
units are `PRECAMPAIGN`-gated behind un-ingested Occult Adventures. That finding is correct and
unchanged by this cycle; it is not the same population.

Ran `python3 scripts/classify_companion_rows.py` (no book arg — all 16 companion books) fresh
against the pinned oracle:

```
total companion units in scope : 1696
orphan ability rows            : 730
PRECAMPAIGN-gated on an uningested campaign : 2
`*_classes_companion.lst` class rows the chassis drops : 7
`.COPY=`/`.MOD` delta rows the chassis drops : 30
distinct excluded rows (the UNION, not the sum) : 768
```

Then confirmed by exact key-set diff (`(book, corpus_key)` pairs) against the ledger's
`status == "not-ingested"` companion units: **768 of 769 are exactly `classify()`'s own
orphan/delta/class/gated exclusion union.** The lone residual,
`bestiary:companion:pseudodragon_tail`, is a rendering-side `engine_book` resolution gap
(`v06_work_inventory.rs`'s `Kind::Companion => not_ingested("companion_content_has_no_engine_
table")` fall-through arm), not a missing citation — named, not investigated further this cycle.

**This settles the contradiction**: 769 is a genuinely different, much larger population than the
4 the earlier cycle examined, and `transcribe_companion_tables.py`'s own exclusion logic — a
**mechanism gap it was designed to have**, not a defect — is exactly why these units never reached
a corpus record. That pipeline resolves each ability row's OWNING creature so the render layer can
attach it correctly, and deliberately drops every row it cannot prove an owner for
(`decisions.md §1a`: never fabricate an ownership claim the corpus doesn't state).

## Search for an existing ingest path — found the right question, not the wrong mechanism

`scripts/transcribe_companion_tables.py` + `src/bin/gen_book_cache.rs::gen_companion_book` is the
companion analogue of the brief's other named precedents, but it answers a **harder, later**
question (ownership, for rendering) than `shape_ledger.py`'s `no_record` join asks (does a corpus
record exist at all, so the shape can be measured — `decisions.md §20`). Widening its ownership
resolution (a 7th shape beyond row-named/prerace/prefix/relay/granted/`.COPY=`, traced concretely
in a prior sibling cycle's own notes: `BONUS:ABILITYPOOL|<PoolName>|<Count>` grants whose pool name
does not equal the ability `KEY` prefix, via a two-hop `CATEGORY:Internal` relay) is real,
per-pool-traceable engineering work belonging to a Gate-2 (reachability) cycle — attempting it here
under `no_record` pressure would risk exactly the false-ownership fabrication `decisions.md §1a`
forbids.

Following `scripts/ingest_ability.py`'s method (the wave-1 precedent this bundle's own brief names):
**a generic, per-unit literal transcriber that makes no ownership claim at all.** New
`scripts/ingest_companion.py` (~300 lines): resolves each `status: not-ingested` companion unit's
own `(book, source_file, source_line)` citation, reads the cited row byte-verbatim, tab-tokenizes it
(skip identity column, split each field on its first `:`), PI-screens it, and writes one JSON
record with `owners: []` and `data.origin` carrying the unit's own `declared`/`copy`/`mod_only`
value — a `.COPY=`/`.MOD` row's tokens are its OWN row's tokens, never a merged/resolved record, and
`origin` states that plainly so a later reader is never misled.

## Product Identity — the exact §19a/§19c-approved companion chain, reused a third time

Imports, not re-derives: `scripts/sd32_t9_pi_exposure_audit.py::classify_row` (declared
`NAMEISPI:`/`DESCISPI:` union with the 60-term blacklist scan) and
`scripts/sd32_t9_pi_review_companion_monsterability.py::normalized_scan`/
`classify_uncertain_content` — the operator-approved normalized (case-fold + OCR-fold) re-scan and
per-record content classifier that resolved companion's 443-unit uncertain bucket under
`decisions.md §18`/`§19a`/`§19c`. Same disposition chain as that review script:

1. `classify_row` blocked → skip.
2. `normalized_scan` hit → skip.
3. Free-text tag present, no hit above, `classify_uncertain_content` returns
   `still_undecidable` → skip. A legitimate disposition per `decisions.md §18`'s standing
   constraint, not forced to `clear` for a tidier number.
4. Otherwise → transcribe.

Every skip is named in the run's own report, never silently dropped (`decisions.md §15`).

## Population, before and after

**This cycle's own run**
(`PCGEN_CORPUS_ROOT=<oracle>/data python3 scripts/ingest_companion.py --out <report.json>`):

```
population:  769
written:     552
pi_skipped:  217  (all "still_undecidable" — 0 "blocked")
unresolved:  0
```

**`shape_ledger.py`** before/after:

| | before | after | delta |
|---|---:|---:|---:|
| `companion` `no_record` | 769 | 217 | **−552** |
| every other kind | unchanged | unchanged | 0 |

(Full `Counter` diffed both sides — `race_trait` 1883, `monster_ability` 1146, `template` 1062,
`feat` 901, `spell` 686, `ability` 576, `deity` 459, `equipment` 316, `equipment_modifier` 237,
`class` 157, `class_feature` 140, `race` 59, `monster` 28, `language` 15, identical before and
after.)

The 217 residual `no_record` units are EXACTLY this run's own `still_undecidable` count — every
un-ingested unit is accounted for by a named §15/§18 stop, none silently missing.

## §15/§18 — Product Identity: 217 records stopped, named by the run's own report

Every one of the 217 records this cycle did NOT transcribe is listed by
`(book, source_file, line, name, key, bucket, reason)` in
`docs/release/SD-32-compute-library-and-cause-closure/artifacts/gate-3-closure-invariant/epic-2-companion-ingest_cycle-1_cycle_receipt_pi-skipped.json`,
committed with this cycle. All 217 are the `still_undecidable` bucket (a capitalized token outside
the operator-approved allowlist, or a lowercase creature-species-shaped reference) — 0 `blocked`
(no declared `NAMEISPI:`/`DESCISPI:`, no exact or normalized blacklist-term hit, in this
population). None were transcribed, none silently skipped. **Widening the allowlist further to
resolve more of these 217 is explicitly operator-scope** (`decisions.md §19c`: "a token added
without a stated reason is a defect, not a shortcut") — not attempted by this cycle on its own
authority.

## §16 — a unit moved out of a shape is not a unit closed

No `companion` unit was reclassified into another kind by this cycle. All 552 written records keep
`kind: companion` — they are still exactly the units `docs/work-inventory.json` already enumerates
as `companion`; this cycle only adds their corpus record. `population: "in_scope"` on every written
record.

## Fixture discipline (`decisions.md §3`) — `corpus_literal_sweep`

The repo's own full-corpus sweep fatals immediately on a **pre-existing, unrelated** defect:
`data/corpus/advanced_class_guide/domain/battle_spirit.json`'s `source.path` is missing its leading
`pathfinder/` segment (landed by a sibling `card15-simple-filename-kinds-ingest` cycle before this
one; the brief itself flagged "a repair lane is running now — check the current state" — checked,
still present, not fixed by this cycle: a different kind, a different generator, out of scope per
AGENTS.md rule 3). Worked around for verification only, non-destructively: built a scratch
`--repo-root` whose `data/corpus/<book>/companion` entries are symlinks into the real directories
for all 9 touched books (`beastiary` alias included), leaving every other corpus directory (and the
defect) untouched and out of the walk:

```bash
export PCGEN_CORPUS_ROOT=<oracle>/data
target/release/corpus_literal_sweep --repo-root /tmp/companion_scratch/reporoot
corpus-literal-sweep: 1314 records examined of 1314 read, 16602 tokens compared (0 synthesized), 1314 digests checked, 0 findings
corpus-literal-sweep: CLEAN
```

1,314 = 552 new + 762 pre-existing companion records across those 9 books, all CLEAN. Every one of
the 552 written records' `raw_tokens` matches the pinned oracle's cited bytes exactly.

## Corpus-write safety — no existing record or verification stamp touched

```bash
git status --porcelain -- data/corpus   # 552 lines, all `??` (untracked/new), 0 modifications
git status --porcelain -- docs/work-inventory.json   # empty
```

No `gen_book_cache`/`gen_companion_book` regeneration was run — this cycle only adds new,
previously-nonexistent files (collision-checked against files already on disk before slugging, not
just this run's own output). `docs/work-inventory.json` was neither read-then-written nor
regenerated, so no `literal-verified`/`fixture-verified` stamp was at risk (no diff of the status
distribution was needed because nothing that carries stamps was touched — confirmed by the `git
status` above, not assumed).

## Reachability (`apps/desktop/src-tauri/src/reach_gate.rs`) — honest claim: 0

`reach_gate.rs`'s only companion entry is `CompanionAbilityRecord`, sourced from
`companion_chassis::COMPANION_BOOKS` — the ownership-resolving pipeline this cycle deliberately does
not touch. **None of the 552 new records reach a player through any existing engine path.** This
cycle closes Gate-1 measurability (the shape can now be read from a real corpus record) for 552 of
769 units; it makes no reachability claim. That is separate, unattempted Gate-2 work.

## Tests / RED → GREEN

- No Rust code touched this cycle (Python-only generator + JSON data). `docs/work-inventory.json`
  is untouched (0 stamps at risk), so no Rust test suite is affected by this cycle's own diff.
- `scripts/ingest_companion.py` has no committed unit tests of its own — reported, not hidden, same
  gap `ingest_ability.py`'s own receipt named for its script. The generator's correctness evidence
  this cycle is the fixture check above (`corpus_literal_sweep` CLEAN, byte-for-byte re-derivation
  by an independent binary) rather than a unit-test RED→GREEN — no defect was found and fixed
  during this cycle's build (unlike `ingest_ability.py`'s soft-hyphen incident), so there is no
  RED→GREEN narrative to report beyond the fixture check itself.

## Identifier / wired-integration audit (this cycle's own diff)

```bash
git diff --unified=0 857eb85d0370adce3bd113c0cbda4e755b631a0a -- scripts/ingest_companion.py \
  | grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})'
```

Hits are literal references to the ALREADY-EXISTING modules
`scripts/sd32_t9_pi_exposure_audit.py` and
`scripts/sd32_t9_pi_review_companion_monsterability.py` this cycle imports (their own real
filenames, not new identifiers this cycle invented) — not a new bundle-tag leak.

```bash
git diff --unified=0 857eb85d0370adce3bd113c0cbda4e755b631a0a -- scripts/ingest_companion.py \
  | grep -nE '\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b' || echo 'OK_NO_TOKENS'
```

`OK_NO_TOKENS`.

## Files touched

- `scripts/ingest_companion.py` — new.
- `data/corpus/<book>/companion/*.json` — 552 new records across 9 books (never hand-edited,
  written only by the generator above; 0 existing files modified, verified via `git status`).
- `docs/release/SD-32-compute-library-and-cause-closure/artifacts/gate-3-closure-invariant/epic-2-companion-ingest_cycle-1_cycle_receipt_pi-skipped.json`
  — the 217 named §15/§18 stops (committed so the list is not scratch-only).
- `docs/release/SD-32-compute-library-and-cause-closure/artifacts/gate-3-closure-invariant/epic-2-companion-ingest_cycle-1_cycle_receipt.md`
  — this receipt.
- `docs/release/SD-32-compute-library-and-cause-closure/kanban.md` — row 11 note prepended, status
  left `in-progress`.
- `docs/retro/events/sd31-transcribe.jsonl` — one append-only line from this cycle's own
  `scripts/verify.sh --only preflight-oracle` bootstrap run (auto-appended by the harness, not
  hand-edited).

## What remains

1. **217 `still_undecidable` records** need either an operator PI ruling or a further,
   operator-named allowlist widening (`decisions.md §19c`'s own standing constraint) before they
   can transcribe — named, not silently deferred, not attempted under this cycle's own authority.
2. **The 1 `pseudodragon_tail` `engine_book` gap** — a separate, small rendering-side defect
   (`v06_work_inventory.rs`'s `Kind::Companion` engine-book resolution), named not fixed.
3. **The `BONUS:ABILITYPOOL` 7th ownership shape** a prior sibling cycle traced concretely for
   `transcribe_companion_tables.py` remains real, unattempted Gate-2 (reachability) work — this
   cycle's 552 new records are measurable but not attached to it.
4. **No unit tests exist yet for `scripts/ingest_companion.py`** — a real gap, named above.
5. Gate 2 (engines) has not been attempted for these 552 units — measurable (Gate 1), not reachable
   (Gate 2); a follow-up cycle is needed before any reach claim is made.

## Disk

`df -h /`: (pasted after this cycle's writes, below).
