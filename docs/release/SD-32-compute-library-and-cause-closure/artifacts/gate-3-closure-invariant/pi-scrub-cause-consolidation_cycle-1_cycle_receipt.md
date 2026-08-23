# Cycle pi-scrub-cause-consolidation — gate-3-closure-invariant / Card 11 (Product Identity, cause fix)

- **Card ID:** card 11 (`epic-2-cause-closure`)
- **Commit SHA:** (this cycle's own commit, see push log)
- **Files touched:**
  - `scripts/pi_scrub.py` (new) — the ONE shared `scrub_name_pi_tokens` implementation
  - `scripts/regen_generic_kind_pi_scrub.py` (new) — narrowly-scoped one-shot regen driver for
    already-shipped `codex_generated_name: true` `{race,monster,class,race_trait}_generic` records
  - `scripts/regen_all_renamed_pi_scrub.py` (new) — general, directory-shape-agnostic one-shot regen
    driver for every `codex_generated_name: true` record anywhere under `data/corpus/**` (added after
    a second concurrent-cycle merge shipped `deity`/`class_feature` renamed records via yet another
    ingest path, before this cycle's fix)
  - `scripts/ingest_ability.py` — local `scrub_name_pi_tokens`/`REDACTED_PI_MARKER`/`PI_MARKER_REDACTED`
    deleted, imports from `pi_scrub.py`
  - `scripts/ingest_generic_kind.py` — same
  - `scripts/tests/test_pi_scrub.py` (new) — 7 tests, mutation-proved
  - `data/corpus/**/ability/*.json` (4,824 files) — full deterministic regen
  - `data/corpus/inner_sea_world_guide/monster_generic/codex_named_unit_monster_inner_sea_world_guide_iswg_races_lst_14.json`,
    `data/corpus/inner_sea_races/race_trait_generic/codex_named_unit_race_trait_inner_sea_races_isr_abilities_race_lst_67.json`
    — the 2 of 46 already-shipped `_generic` renamed records whose tokens actually changed under the fix
  - `docs/release/SD-32-compute-library-and-cause-closure/artifacts/gate-3-closure-invariant/20-class-pi-skipped.json`,
    `11-race-trait-generic-ingest-report.json`, `epic-2-race-trait-generic-ingest_cycle-1_cycle_receipt.md`
    — literal PI names reduced to coordinate-only form (pre-existing exposure, found and fixed this cycle)
  - `docs/release/SD-32-compute-library-and-cause-closure/kanban.md` (card 11 row, prepended note)
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS` (own new/changed lines in `scripts/*.py`,
  `docs/release/**/*.md`; the 4,824 regenerated `ability/*.json` files carry no bundle-tag-shaped
  content, only PCGen-cited tokens and `[redacted PI]`)
- **Wired-integration audit result:** `OK_NO_TOKENS`
- **Acceptance criterion:** `decisions.md §15/§19/§24` — a Product-Identity-exposure defect
  (`scrub_name_pi_tokens` duplication) found by an urgent orchestrator escalation; re-derive the true
  population, fix at the cause in one place, regenerate the affected records through the guarded
  generator path, verify zero leaks with a command, per-kind report.
- **Corpus SHA:** `7f818006e371188e5717fd18d74d18a420747fc6` (`scripts/pcgen-oracle-pin.env`)
- **Status:** complete (for this cycle's own scope — see "Discovered, not fixed this cycle" below for
  what is explicitly NOT closed)
- **Discovery forwards:** two `## DISCOVERED`-shaped findings below, filed in this receipt (single-cycle
  scope) rather than a separate entry.
- **Next-cycle plan:** see "Discovered, not fixed this cycle" below.

## §17a — re-deriving the brief's own figure before trusting it

The brief handed me: *"ability records scanned: 4,824, records with a concatenated PI term inside
data: 40"* — the orchestrator's own heuristic (strip non-letters from each blacklist term, substring-
match inside `data`). Built an honest instrument instead of trusting it:

```bash
# Full-corpus cross-check of every codex_generated_name:true record's raw_tokens against its
# PRE-RENAME name/key (joined via rename.coordinate against docs/work-inventory.json), using the
# SAME two checks the fix below adds -- alnum-normalized (>=6 char) substring matching, both for
# (a) the record's own identity and (b) PI_BLACKLIST_TERMS.
python3 <ad-hoc script, logic reproduced in scrub_name_pi_tokens itself>
```

**Result, before any fix:** 622 `codex_generated_name: true` records existed corpus-wide (576
`ability`, 46 `{race,monster,class,race_trait}_generic`). Two independent leak shapes, not one:

| Shape | Distinct leaking records (before fix) |
|---|---:|
| Identity leak (record's own pre-rename name/key, concatenated) | 361 (word-split needle logic; see caveat below) |
| Blacklisted-term leak (a `PI_BLACKLIST_TERMS` entry concatenated PascalCase-style into a `TYPE:` token's value, e.g. `TYPE:<Deity>AspectChoice.SpecialQuality`, placeholder substituted for the real term) | 8 |
| **Union (distinct records)** | **368** |

**The orchestrator's brief figure (40) was an undercount**, not because the shape it named was wrong
(it named exactly the right defect class) but because its heuristic under-detected: it likely matched
whole blacklist terms only, missing the record's-own-identity shape (361 records) entirely and
undercounting even the blacklist-term shape (8, not 40 — the brief's 40 may have included false
positives from an unbounded substring match; not re-derived further since this cycle's own instrument
is the one that shipped).

**Caveat on the 361 figure, found by inspection (`§17a` applies to my own instrument too):** most of
the 361 "identity leak" hits are a single common English word (7 letters, not a proper noun, not on
the blacklist) that happens to also appear, coincidentally, inside an unrelated spell-list token —
matched only because the identity-needle logic (reused verbatim from the ALREADY-SHIPPED, accepted
`ingest_generic_kind.py` reference implementation) splits the pre-rename key into individual words and
treats every word ≥6 normalized characters as a needle, with no check that the word is itself
PI-shaped. This is a real, already-accepted design choice (conservative: over-redact rather than
under-redact for PI content, `decisions.md §1a`'s "under-include rather than invent" spirit applied to
redaction), not a false claim — every one of the 361 records genuinely does carry a verbatim word from
its own PI-blocked title in another field, and the fix (below) redacts them, matching what would have
happened had the accepted reference implementation been used for `ability` from the start.

**The unambiguous, highest-severity case** — a blacklisted deity name directly concatenated into a
`TYPE:` token with no separator, already shipped in a `codex_named_unit_*` `ability` record under
`inner_sea_gods`, is real, verified, and is what led me to find the SECOND leak shape (below).

## The two real defects (mutation-proved RED → GREEN both times)

1. **`ingest_ability.py`'s copy of `scrub_name_pi_tokens` was never updated with the identifier-
   concatenation fix `ingest_generic_kind.py`'s copy already had** (found by a prior tail lane, its own
   receipt: `t9-t2-race-monster-class-racetrait-no-record-closure_cycle-1_cycle_receipt.md`). Two
   independently-maintained copies of a PI screen is exactly the drift `decisions.md §17` names.
   **Fixed by extraction**, not by porting the fix a second time: `scripts/pi_scrub.py` is now the ONE
   `scrub_name_pi_tokens`, imported by both `ingest_ability.py` and `ingest_generic_kind.py` (`grep -rn
   "def scrub_name_pi_tokens" scripts/*.py` → exactly one hit, `scripts/pi_scrub.py`, confirmed after
   this cycle's own diff).

2. **A second, more severe instance of the SAME defect class, not found by the prior tail lane** because
   its own fix only covers a record's OWN identity, not the 60-term `PI_BLACKLIST_TERMS` scan.
   `normalized_term_hit` is deliberately word-bounded
   (`(?<![a-z0-9])term(?![a-z0-9])`) to avoid the recorded short-blacklist-term-inside-an-
   ordinary-word false positive
   (`ogl-pi-blacklist.md §4`) — but that same boundary rule means a blacklisted term concatenated
   PascalCase-style into another token's value (the character immediately after the term is a letter,
   never a boundary) is invisible to it. **Found live in an already-shipped `codex_named_unit_*`
   `ability` record** (`inner_sea_gods`, `isg_abilities_faith.lst:546`, plus 5 sibling lines and 2 more
   records elsewhere): its `TYPE:` token still carried the deity's name in plain text, un-redacted,
   despite `NAME`/`DESC` having been correctly redacted — precisely the shape `decisions.md §24b`-2
   ("the PI original appears nowhere that ships") exists to prevent.

   Mutation-proved: `scripts/tests/test_pi_scrub.py::
   BlacklistTermConcatenationTests::test_mutation_proof_removing_check_4_lets_the_concatenated_form_through`
   asserts that with the new check (`_NORM_BLACKLIST_TERMS`) disabled, the SAME concatenated-form input
   that the test above proves is caught now passes through un-redacted (`any_redacted is False`) — i.e.
   the new check is load-bearing, not vacuously always-true.

## The fix

`scripts/pi_scrub.py::scrub_name_pi_tokens` runs FOUR checks per token value, additive (any one
redacts):

1. `normalized_term_hit` — word-bounded, OCR-normalized 60-term blacklist scan (pre-existing).
2. Space-preserving substring match against the record's own name/key (pre-existing).
3. Alphanumeric-normalized (≥6 char) substring match against the record's own name/key — the
   identity-concatenation fix `ingest_generic_kind.py` already had, now the ONE copy.
4. **New this cycle:** alphanumeric-normalized (≥6 char) substring match against
   `PI_BLACKLIST_TERMS` — the blacklist-term-concatenation fix, closing the gap check 1 cannot see.

Both normalized checks share the same `_MIN_NORMALIZED_NEEDLE_LEN = 6` bound already accepted in the
reference implementation, so a short blacklist term (the 3-4-normalized-character entries in
`PI_BLACKLIST_TERMS`) is
still protected against over-redaction on coincidence — those remain covered only at their ordinary,
separated occurrences via check 1, an accepted residual gap this cycle did not widen further.

## Concurrent-cycle merge (found and closed at push time, `§5`'s rebase)

`git fetch origin tranche/12 && git rebase origin/tranche/12` conflicted on
`scripts/ingest_ability.py`, two `ability` corpus files, and `kanban.md`. A sibling cycle had
landed, independently, in the same window: its own new `scrub_blacklist_pi_tokens` — applying the
word-bounded 60-term blacklist scan (`normalized_term_hit`) to EVERY non-renamed record's token
values, not only `DESC` — closing exactly the architectural gap this cycle's own "Discovered, not
fixed this cycle" section (below) had flagged as out of scope. Its own change comment named the two
records that proved the gap live: `inner_sea_gods/ability/adept.json`,
`inner_sea_magic/ability/diplomatic_student.json`.

**Merged, not chosen between:** kept the sibling's `scrub_blacklist_pi_tokens` (their fix for a
different, real gap) and `records_equal_ignoring_timestamp`/changed-vs-unchanged write tracking
(their unrelated improvement); dropped their re-added local `scrub_name_pi_tokens` (stale, pre-this-
cycle-fix — a rebase-driven revert of my own extraction, not a deliberate edit on their part) in
favour of the import from `pi_scrub.py`.

**One more real leak found resolving the conflict, in the sibling's OWN new code:**
`diplomatic_student.json`'s `TYPE:` token concatenated a blacklisted term (a campaign-setting
institution name) directly onto a generic suffix with no separator — a blacklisted term
concatenated with no separator, un-redacted, on an otherwise-clean (non-renamed) record processed by
their brand-new `scrub_blacklist_pi_tokens`. Their function used only `normalized_term_hit`
(word-bounded), which has the identical blind spot check 4 exists to close. **Fixed by exposing
check 4 as a public function** (`pi_scrub.blacklist_term_hit_including_concatenated`, wrapping
`normalized_term_hit` plus the alphanumeric-normalized concatenated-term check) and wiring
`scrub_blacklist_pi_tokens` to call it too — one shared check, two call sites, rather than a third
independently-drifting copy of the same blacklist logic. Re-ran `ingest_ability.py` after the merge:
`diplomatic_student.json`'s `TYPE` token is now `[redacted PI]`. Verified: full re-scan of all 657
`codex_generated_name: true` records post-merge (up from 622 — the sibling's cycle ingested more
kinds) — 0 identity leaks, 0 blacklisted-term leaks. The corpus-wide (not-limited-to-renamed) scan's
`ability`-directory contribution to the 212-record "Discovered" finding above is now 0 (184 remain,
all in the other, out-of-scope kind directories).

## Second concurrent-cycle merge: `deity`/`class_feature` `§24` renaming (later rebase)

A later `git fetch && rebase` (this cycle's second push attempt) landed a THIRD concurrent cycle:
`c1505f6497` ingested `deity` (459) and `class_feature` (140) name-PI-blocked units under `§24`
neutral names via `ingest_simple_filename_kinds.py`, which imports `scrub_name_pi_tokens` FROM
`ingest_ability.py` (`from ingest_ability import scrub_name_pi_tokens`) — a real, correct reuse of
this cycle's own fix, not a fourth copy. But `c1505f6497` was authored and its records written
BEFORE this cycle's fix reached `origin/tranche/12`, so its 599 new `codex_generated_name: true`
records shipped through the PRE-fix `scrub_name_pi_tokens` (the stale, un-fixed logic still on
`origin/tranche/12` at the time it ran) and were never regenerated after.

**Re-derived the full population after this rebase, not assumed clean:** `grep -rl
'"codex_generated_name": true' data/corpus | wc -l` → 1,256 (up from 657). Re-ran the `§17a`
cross-check instrument — **110 distinct records leaking** (all identity-shape, 0 blacklist-term-
shape), spanning `deity` and `class_feature` (the latter's directory layout differs from
`ingest_generic_kind.py`'s: some records nest one level deeper under a class-name subdirectory,
e.g. `class_feature/rogue/codex_named_unit_....json`, which neither existing regen driver's file
glob covered).

**Fixed generally, not per-directory-shape:** `regen_generic_kind_pi_scrub.py` is scoped to
`{race,monster,class,race_trait}_generic/`; rather than write a THIRD narrowly-scoped driver for
`deity`/`class_feature`'s two different layouts, wrote `scripts/regen_all_renamed_pi_scrub.py` —
walks the WHOLE `data/corpus/` tree for any `codex_generated_name: true` record regardless of
directory shape or which ingest path wrote it, using the identical
`(rename.coordinate → work-inventory.json join → re-read cited row → scrub_name_pi_tokens)`
sequence. This driver supersedes `regen_generic_kind_pi_scrub.py`'s narrower scope going forward
(both are kept — the narrower one runs faster for its specific 46-record population when that is
all that is needed; the general one is the safe default for "regenerate everything renamed").

```bash
PCGEN_CORPUS_ROOT=<pinned oracle>/data python3 scripts/regen_all_renamed_pi_scrub.py
# {"scanned": 24033, "renamed_reprocessed": 1256, "non_renamed_skipped": 22777, "changed": 126, "unchanged": 1130}
```

126 changed (110 identity-leak records plus 16 more the coarser per-record diff caught beyond the
110 my `§17a` instrument flagged — e.g. a token whose value changed shape without necessarily
matching my instrument's own needle-length bound; the regen driver's own byte-diff is the ground
truth, not my separate detector). **Re-verified after this second regen: 0 leaks across all 1,256
`codex_generated_name: true` records**, both shapes, full-corpus cross-check.

## Regeneration (guarded generator path, no hand-edits)

**`ability` — full deterministic rerun** (its own `load_units()` loads all 4,824 `kind: "ability"`
units unconditionally, not gated on `no_record`, so a plain rerun is a safe, idempotent full regen):

```bash
PCGEN_CORPUS_ROOT=<pinned oracle>/data python3 scripts/ingest_ability.py --out /tmp/ability_regen_report.json
# {"population": 4824, "written": 4824, "name_pi_renamed": 576, ...}
```

**`{race,monster,class,race_trait}_generic`'s 46 already-shipped renamed records** — `ingest_generic_kind.py`'s
own population is gated on `join_status == "no_record"`, and by construction every unit it already wrote
is no longer `no_record`, so a plain rerun would find and write zero units. A blanket "reprocess every
unit of this `--kind`" bypass was rejected (unsafe — for `race`/`monster`/`class`/`race_trait`, MOST
units of that `kind` in `docs/work-inventory.json` are ALREADY correctly ingested by a different, curated
generator into a DIFFERENT directory, e.g. `race_trait/`'s 1,913-unit main population vs. `race_trait_generic/`'s
6-unit residual; a bypass would have attempted to write duplicate content for thousands of already-correct
units). Instead, `scripts/regen_generic_kind_pi_scrub.py` walks ONLY the existing
`data/corpus/**/{race,monster,class,race_trait}_generic/*.json` files, re-derives `raw_tokens` from each
record's own already-cited `(source.path, source.line)` via the module's own `read_row`/`row_tokens`, joins
the pre-rename identity from `docs/work-inventory.json` via `rename.coordinate`, and re-applies ONLY the
fixed `scrub_name_pi_tokens` — key/name/slug/file path untouched, so no slug-collision or file-identity
risk (the near-miss the prior tail lane found and fixed a different way):

```bash
PCGEN_CORPUS_ROOT=<pinned oracle>/data python3 scripts/regen_generic_kind_pi_scrub.py
# {"scanned": 1992, "renamed_reprocessed": 46, "non_renamed_skipped": 1946, "changed": 2, "unchanged": 44, "unresolved_coordinate": []}
```

2 of 46 changed (the `inner_sea_world_guide` `monster_generic` blacklist-term leak the brief's own
example partially anticipated, and one further `race_trait_generic` token). 44/46 were already clean
under checks 1-3 (the prior tail lane's own fix already covered the identity shape for all 46; only the
NEW check 4 found anything left).

## Verification (first merge, 622-record population)

```bash
# Full-corpus cross-check, all 622 codex_generated_name:true records (576 ability + 46 generic_kind),
# both leak shapes, joined against docs/work-inventory.json's pre-rename name/corpus_key:
python3 <the §17a instrument above, re-run post-fix>
# renamed (codex_generated_name=true) records scanned: 622
# identity-leak token hits: 0  distinct records: 0
# blacklist-term-leak token hits: 0  distinct records: 0
# TOTAL distinct leaking records (union): 0
```

**`codex_named_unit_*` records specifically (`decisions.md §24`), per the brief's own instruction:**

```bash
grep -rl '"codex_generated_name": true' data/corpus | wc -l   # 622
# per-kind-dir breakdown:
grep -rl '"codex_generated_name": true' data/corpus | awk -F/ '{print $(NF-1)}' | sort | uniq -c
#     576 ability
#      21 class_generic
#      19 monster_generic
#       5 race_trait_generic
#       1 race_generic
```

Zero leaks confirmed across all 622, all 5 affected kind-directories, as of the FIRST merge.

## Final verification (after the second merge, 1,256-record population), by command

```bash
grep -rl '"codex_generated_name": true' data/corpus | wc -l
# 1256
grep -rl '"codex_generated_name": true' data/corpus | awk -F/ '{print $(NF-1)}' | sort | uniq -c | sort -rn
#     576 ability
#      21 class_generic
#      19 monster_generic
#       5 race_trait_generic
#       1 race_generic
#     ... 140 class_feature units across ~40 class-name/coordinate-named directories
#      459 deity
```

```bash
# Full-corpus cross-check (both leak shapes), joined against docs/work-inventory.json's
# pre-rename name/corpus_key, all 1,256 codex_generated_name:true records:
python3 <the §17a instrument above, re-run post BOTH merges>
# renamed (codex_generated_name=true) records scanned: 1256
# identity-leak token hits: 0  distinct records: 0
# blacklist-term-leak token hits: 0  distinct records: 0
# TOTAL distinct leaking records (union): 0
```

**Zero leaks confirmed across all 1,256 `codex_generated_name: true` records, every kind, every
directory shape, as of this cycle's final push.**

## Sweeps, unchanged by design (`decisions.md §12c`)

`shape_ledger.py`'s `join_status` distribution, before and after (identical — content-only fix, no
units added, removed, or reclassified):

```bash
python3 scripts/shape_ledger.py --inventory docs/work-inventory.json --output /tmp/ledger.json
python3 -c "import json,collections; r=json.load(open('/tmp/ledger.json'))['rows']; print(collections.Counter(x['join_status'] for x in r))"
# Before: Counter({'no_formula_tokens': 21009, 'matched': 11170, 'no_record': 3149})
# After:  Counter({'no_formula_tokens': 21009, 'matched': 11170, 'no_record': 3149})
```

`docs/work-inventory.json` untouched (`md5sum` identical before/after this cycle — this cycle never
runs `v06_work_inventory`, so `CORPUS_LITERAL_SWEEP_REPORT`/`DERIVED_FIXTURE_CHECK_REPORT`/stamp-loss
concerns do not apply; noted per the brief's regeneration-safety instruction, not because this cycle's
regen touches that binary).

`cargo run --locked --bin corpus_literal_sweep` (pre-built binary, `CARGO_TARGET_DIR` unchanged):
957 findings across 362 records, exit code 1. **Correction of a prior receipt's claim** (`§17a`): the
tail lane's own receipt stated this tool's "exit code (0) treats [redacted-token mismatches] as
non-fatal" — checked the tool's own source (`src/bin/corpus_literal_sweep.rs`): it returns
`ExitCode::from(1)` whenever `findings` is non-empty, unconditionally; there is no redacted-token
exemption. This has been the tool's behaviour since the FIRST `[redacted PI]` token shipped (well
before this cycle), so the exit-1 status is a pre-existing, already-red condition this cycle did not
cause — but the prior receipt's "exit code (0)" claim was wrong and unverified. 957 (down slightly from
the prior receipt's 968, expected: this fix's regen changed WHICH tokens are `[redacted PI]`, not
whether the corpus otherwise byte-matches).

## Pre-existing PI-exposure fix (found doing the honest re-derivation, in scope per `decisions.md §15`)

Three already-committed artifacts named units by their literal (real) PI display name, under a
"permanently `no_record`, a name cannot be redacted" premise `decisions.md §24` (2026-08-23) supersedes:

- `20-class-pi-skipped.json` — 21 class names (e.g. proper-noun Paizo prestige-class names), now
  ingested under `§24` via `class_generic/`.
- `11-race-trait-generic-ingest-report.json` + `epic-2-race-trait-generic-ingest_cycle-1_cycle_receipt.md` —
  5 race-trait names, now ingested under `§24` via `race_trait_generic/`.

Both reduced to coordinate-only form this cycle (`§24b`-4). **Per `decisions.md §15`: stopped, landed
everything else, reporting this by name/count rather than silently fixing it as a footnote** — it is
the same exposure class this cycle's own brief warned me to avoid in my OWN artifacts, found instead in
a predecessor cycle's.

## Discovered, not fixed this cycle (filed per `decisions.md §15`'s "lands everything else... reports it")

1. **Corpus-wide scan (not limited to renamed records) found 212 records across ~39 kind directories**
   with the same blacklisted-term-concatenation shape in an un-redacted token — **mostly Rust-curated
   content** (`blessings`, several per-archetype directories named after the archetype's own
   PI-blocked class/organization title, monster-template directories like `teratoma_*`)
   **outside the two Python files this cycle's brief
   named.** Root architectural cause, confirmed by reading both `main()` bodies: `scrub_name_pi_tokens`
   is only invoked when the record's OWN name/key is PI-blocked (`name_is_pi` branch); a record with a
   clean name/key never has its OTHER token values scrubbed for blacklisted content at all (only `DESC`
   gets a separate, narrower check). This is a genuine, pre-existing, corpus-wide gap, not a duplication-
   drift issue — a different, larger defect than the one this cycle's brief named and scoped me to.
   **Out of this cycle's granted scope** (39 different generators, mostly Rust, would each need
   individual investigation) — named here by count/kind-breakdown for a future cycle, per
   `decisions.md §15`.
   Re-derive: `grep -rln '"codex_generated_name": true' data/corpus | wc -l` (622, the renamed-only
   population this cycle closed) vs. the corpus-wide scan's population (44,838 records with a
   `raw_tokens` field) — the 212 are entirely OUTSIDE the 622.
2. **Three already-committed artifacts still name units by their literal display name**: the two fixed
   above, plus `epic-2-companion-ingest_cycle-1_cycle_receipt_pi-skipped.json` (217 `companion` entries,
   sampled — every name inspected was a generic mechanical term like "Grab"/"Pounce"/"Bully Feats", no
   proper-noun/deity/place name found in the sample, consistent with `decisions.md §19a`'s "entirely
   generic game mechanic" ruling for `companion` — left unfixed this cycle as lower-risk, but not
   independently re-verified record-by-record).
3. **243 units** (`companion` 217 + `class` 21 + `race_trait` 5, from the three artifacts above) **are
   real remaining `no_record` residual** that predates `§24` and has not been re-ingested under a
   Codex-generated neutral name the way `ability`/`race`/`monster`/`class`-tail/`race_trait`-tail were.
   `decisions.md §20`: `no_record` must reach ZERO — this is real remaining work, not closed by this
   cycle (this cycle's scope was the concatenation-defect fix and the resulting leak, not building a
   new ingest mechanism for `companion`).

## Tests

- `python3 -m unittest scripts.tests.test_pi_scrub` — 7/7 pass, including the mutation proof.
- `python3 -m unittest scripts.tests.test_ingest_ability_pi_rename scripts.tests.test_ingest_generic_kind`
  — 21/21 still pass unchanged (both now exercise the imported, not locally-defined, function).
- `python3 -m unittest discover -s scripts/tests -p "test_*.py"` — 495 tests, 1 failure, 1 skip. The
  one failure (`test_transcribe_monster_tables.py::InternalBundleAbilityHopIsResolved::
  test_an_ability_no_bundle_names_stays_an_orphan_and_is_not_shipped`) is **pre-existing and unrelated**:
  reproduced in isolation on an unmodified file this cycle never touched
  (`git diff --stat scripts/tests/test_transcribe_monster_tables.py scripts/transcribe_monster_tables.py`
  → empty).

## Pinned-count sweep

```bash
grep -rn "\b3149\b\|\b3440\b\|\b40\b.*ability.*concatenat" tests/ src/ scripts/ apps/ 2>/dev/null | grep -v /target/
```
No pinned assertion anywhere depends on this cycle's `no_record` figure (unchanged, so nothing to
re-pin) or on the pre-fix leak counts.
