# Cycle 1 — Gate 3 Closure Invariant / declared-pi-shipping-65-followups

- **Card ID:** `epic-2-cause-closure` (kanban row 11, left `in-progress` per dispatch instruction —
  this closes one named sub-item, not the card).
- **Commit SHA:** see push receipt (this cycle's commit, appended below after push).
- **Files touched:**
  - `src/rules_core/pi_screening.rs` (root-cause fix + `reconcile_description_pi_stamp` + 8 tests)
  - `src/bin/gen_book_cache.rs` (generator fix: `monster_ability`'s description now screened via
    `pi_screening::classify_optional_field_declared` instead of a hardcoded `Ogl`/`None` stamp)
  - `src/bin/reconcile_description_pi_stamps.rs` (new guarded-path remediation binary + 6 tests)
  - `data/corpus/**` — 99 records, `license`/`pi_field`/`pi_marker` only (see §4 below)
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS` (scoped to this cycle's own diff:
  `git diff -- src/bin/gen_book_cache.rs src/rules_core/pi_screening.rs` plus the new
  `src/bin/reconcile_description_pi_stamps.rs`, not the full `BASE_BRANCH...HEAD` form)
- **Wired-integration audit result:** `OK_NO_TOKENS` (same scope)
- **Acceptance criterion:** dispatch brief — re-derive the 65 `DESC-PI-SHIPPED`
  `declared_pi_shipping_audit` violations in `bestiary_4/monster_ability`, confirm the
  "already-redacted, metadata not stamped" diagnosis per record (not just by class), check other
  kinds for the same gap, fix the generator (not just the records), regenerate through the guarded
  path with a before/after status diff, and re-run `corpus_literal_sweep` clean.
- **Corpus SHA:** `7f818006e371188e5717fd18d74d18a420747fc6` (`scripts/pcgen-oracle-pin.env`,
  bootstrapped this cycle via `scripts/fetch-pcgen-oracle.sh --dest
  docs/release/SD-32-compute-library-and-cause-closure/artifacts/corpus/operator-supplied/pcgen`
  into a fresh, git-ignored, previously-empty worktree oracle slot).
- **Status:** complete
- **Notes:** see full narrative below.
- **Discovery forwards:** none — the 34-record wider gap (§2 below) is closed in this same cycle,
  not forwarded.
- **Next-cycle plan:** none needed for this shape. `corpus_literal_sweep`'s remaining 15
  findings/6 records (adventurers_guide/inner_sea_magic/inner_sea_world_guide, `class_feature`/
  `trait_generic`/`feat_generic` kinds — pre-existing, confirmed unrelated to this cycle's diff by
  `git diff --stat`, and present at the SAME count before this cycle's regen) are a different,
  already-known gap outside this cycle's scope and not touched here.

## 0. Wrong-base check (footgun 1)

The worktree's `HEAD` at dispatch start was **not** a descendant of the pinned
`52b46ffeab5ef86d8fb81f21377cb01ca1ee460c` (`git merge-base --is-ancestor` failed, exit 1) — the
worktree had started on a different lineage (a PR #374 merge commit). `git reset --hard "$PIN"`,
re-verified `ANCESTOR_OK`, then `git rebase origin/tranche/12` — a no-op, since `origin/tranche/12`'s
tip **is** the pin commit. Oracle slot was empty (fresh worktree, git-ignored); bootstrapped via
`scripts/fetch-pcgen-oracle.sh --dest <repo-local slot>` (explicit `--dest`, never the forbidden
`~/workspace/repos/pcgen`), confirmed via `/proc/<pid>/environ` on the first real audit run before
trusting any output.

## 1. Re-derivation (`§17a`)

```
$ PCGEN_CORPUS_ROOT=<repo-local oracle>/data cargo run --locked --release --bin declared_pi_shipping_audit
declared-pi-audit: FAIL — 65 violation(s) across 65 file(s)
```

Confirmed via `/proc/<pid>/environ`: `PCGEN_CORPUS_ROOT` was the repo-local slot, not the forbidden
default. **65, exactly as briefed.** All 65 are `DESC-PI-SHIPPED` in `bestiary_4/monster_ability`,
each shaped `data.description=Some("[redacted PI]") license=Some("OGL") pi_field=None`.

## 2. Diagnosis verified — and the same gap found live in 8 other `(book, kind)` pairs

The brief's diagnosis ("description already correctly redacted, `license`/`pi_field` never
stamped") holds for all 65 — verified per-record, not by class: every one of the 65 files' own
`data.description` is the literal marker `"[redacted PI]"`, confirmed by
`src/rules_core/rules_tables/bestiary_4/monster_data.rs` (`grep -c 'description: Some("\[redacted
PI\]")'` → 65, matching exactly). **No live Product Identity ships in any of the 65** — this is a
metadata-labeling gap, not a leak, confirmed record-by-record.

**But the audit binary's own check is narrower than the real gap.** `declared_pi_shipping_audit`'s
CHECK A only fires when the record's *cited corpus row itself* declares `DESCISPI:YES` at the exact
line (`declared.description`). A corpus-wide re-derivation for EVERY record whose
`data.description == "[redacted PI]"`, checked against its own `license`/`pi_field` regardless of
whether the audit's line-scoped declaration check would catch it:

```python
# every data/corpus/**/*.json (excl. LICENSE.json) with data.description == "[redacted PI]"
# and NOT (license == "PI-REDACTED" and "description" in pi_field.split(","))
```

found **99**, not 65:

| book | kind | unstamped | total marker-present |
|---|---|---:|---:|
| `bestiary_4` | `monster_ability` | 65 | 65 |
| `inner_sea_bestiary` | `monster_ability` | 6 | 6 |
| `inner_sea_gods` | `monster_ability` | 5 | 5 |
| `inner_sea_gods` | `equipment` | 9 | 39 |
| `inner_sea_gods` | `feat` | 9 | 9 |
| `inner_sea_world_guide` | `monster_ability` | 1 | 1 |
| `inner_sea_temples` | `equipment` | 2 | 2 |
| `book_of_the_damned_volume_2` | `equipment` | 1 | 1 |
| `inner_sea_races` | `feat` | 1 | 2 |
| **total** | | **99** | 130 |

**Two distinct root causes, same symptom:**

1. **`gen_book_cache.rs`'s `gen_monster_book` never screened `monster_ability.description` at
   all** — the write loop hardcoded `license: Some(License::Ogl), pi_field: None, pi_marker: None`
   unconditionally, regardless of `ability.description`'s content. Accounts for 77 records
   (`bestiary_4` 65, `inner_sea_bestiary` 6, `inner_sea_gods` 5, `inner_sea_world_guide` 1).
2. **`pi_screening::classify_field` treated an ALREADY-redacted value as ordinary prose.**
   `cache_gen::{equipment_gap, feat_gap}` DO call `classify_optional_field_declared` for their
   optional text fields — but when the row's own line doesn't declare `DESCISPI:YES` (`declared =
   false`) and the value handed in is `Some("[redacted PI]")` (a static-table literal some other
   pass had already redacted), `classify_field` scanned the marker text itself for a blacklist
   term, found none (the marker is inert by design), and returned `Ogl`/`None` — silently shipping
   `license: "OGL"` over a value that already reads `"[redacted PI]"`. Accounts for the remaining 22
   records (`inner_sea_gods` equipment 9 + feat 9, `inner_sea_temples` equipment 2,
   `book_of_the_damned_volume_2` equipment 1, `inner_sea_races` feat 1).

Both are the same shape the dispatch named: *"screens/stamps one field, not all."* Neither
`declared_pi_shipping_audit`'s own CHECK C (blacklist-term re-scan) nor CHECK A's declared-line
check can see this class of miss, because the shipped TEXT is already safe — only the metadata is
wrong, and CHECK C explicitly skips any value already equal to the marker (`s == REDACTED_PI_MARKER
{ continue; }`).

## 3. Fix — root cause, not just the 99 records

**(a) `classify_field` (`pi_screening.rs`)**: added a guard — a value equal to `REDACTED_PI_MARKER`
now short-circuits straight to `(PiRedacted, Some(field_name), Some(PI_MARKER_REDACTED),
REDACTED_PI_MARKER)`, before the term scan runs. This is the single shared function
`equipment_gap.rs`/`feat_gap.rs`/`class_feature.rs` all call through
`classify_optional_field_declared`, so this one fix closes the gap for every caller at once — no
per-generator patch needed for that half. TDD: added
`a_value_already_equal_to_the_marker_stamps_redacted_not_plain_ogl`, confirmed it failed for the
intended reason (`left: Ogl, right: PiRedacted`) before the fix, green after. 34/34 →
40/40 `pi_screening` tests pass after all additions.

**(b) `gen_book_cache.rs`'s `gen_monster_book`**: the `monster_ability` write loop now computes
`(ability_license, ability_pi_field, ability_pi_marker)` via
`pi_screening::classify_optional_field_declared("description", ability.description, false)` instead
of hardcoding `Ogl`/`None`/`None`. `declared = false` is deliberately conservative (this generator
has no per-line `DESCISPI:` reader of its own, unlike `equipment_gap`/`feat_gap`) — it never
suppresses a real redaction, and the existing `monster_record_pi_hits` hard-fail-the-whole-run gate
(unchanged) already aborts before any write if genuinely live PI text reaches this point, so a
description reaching the classifier is always either ordinary prose or the marker. `cargo test
--locked --bin gen_book_cache`: 5/5 pass unaffected.

**(c) Existing 99 records — a NEW guarded-path binary, not deletion+regen.** Every writer here
(`gen_book_cache.rs`, `cache_gen::equipment_gap`, `cache_gen::feat_gap`) is no-clobber on an
existing file (`if out_path.exists() { skip }` / `write_json` returns `Ok(false)` if `path.exists()`)
— fixes (a)/(b) only prevent the gap on a NEW record, they cannot reach an already-shipped one
without either deleting it first (a bigger, riskier operation than a 3-field metadata correction
needs, and the deletion tooling available to this session refused the bulk-delete of `data/corpus/**`
files outright) or patching the stamp in place. Wrote
`src/bin/reconcile_description_pi_stamps.rs`, following the exact
read-`Value`-patch-write-back shape `enrich_monster_ability_raw_tokens.rs` already uses for its own
in-place corpus fixes: walks the whole corpus (book/kind-agnostic, `decisions.md §17`), and for
every record whose `data.description == "[redacted PI]"` but whose `license`/`pi_field` don't
already say so, rewrites ONLY `license`/`pi_field`/`pi_marker` (unioning `"description"` into an
existing `pi_field` list — e.g. a `§24` name-rename's `"name"` — rather than overwriting it), never
touching `data`, `source`, `ingested_at`, `wiring_class`, or `rename`. TDD: 6 tests, including a
byte-for-byte no-op proof (`reconcile_one_is_a_true_no_op_on_an_already_correct_record`) and a
mutation-style negative (`reconcile_one_never_touches_an_ordinary_unredacted_record`). 6/6 pass.

```
$ cargo run --locked --release --bin reconcile_description_pi_stamps
reconcile-description-pi-stamps: 99 record(s) patched
```

**99, matching the corpus-wide re-derivation in §2 exactly.** `git status --porcelain` after the run:
99 `data/corpus/**` files + the 2 source edits + 1 new binary — nothing else moved.

## 4. Diff shape — proven minimal

`git diff data/corpus/bestiary_4/monster_ability/demon_lord_dagon_breath_weapon.json` (one of the 77
`gen_book_cache`-shaped records): exactly 3 lines changed (`license`/`pi_field`/`pi_marker`), every
other byte — including `ingested_at` — untouched, because these files' on-disk key order already
matched `serde_json::Value`'s round-trip order.

For the 22 `equipment_gap`/`feat_gap`-shaped records, the round-trip **does** reorder keys (those
generators' `CacheRecord` struct field order isn't alphabetical, so re-serializing through `Value`
alphabetizes them) and adds a trailing newline the original file lacked — a cosmetic-only,
zero-content-loss side effect confirmed by diffing every field's VALUE, not position (e.g.
`data/corpus/inner_sea_gods/equipment/codex_named_unit_equipment_inner_sea_gods_isg_equip_lst_30.json`:
`pi_field` moved from `"name"` to `"name,description"`, `codex_generated_name`, `rename.coordinate`,
`source.sha256`, and `wiring_class_signals` all present with identical values, just reordered).
Flagged here rather than silently accepted; no content was lost.

## 5. Verification — before/after, by record and by status distribution

- **`declared_pi_shipping_audit`**: 65 violations → `CLEAN — no shipped record contradicts its own
  corpus row's PI declaration`. Re-run in the background (release binary), polled to completion, not
  left running past this turn.
- **`corpus_literal_sweep --json-out`**, full corpus, before vs. after:
  - Before: `86 findings across 77 records` — every flagged file is one of the 77
    `gen_book_cache`-shaped records this cycle fixes (`MISMATCH ...: token not byte-present in
    corpus token closure: DESC:[redacted PI]` — the sweep already independently detected the same
    defect this audit named, from a different angle: `license: "OGL"` claims the description
    matches the raw source token, but the stored value is the marker, not the token).
  - After: `15 findings across 6 records`. **All 6 are pre-existing and unrelated** — confirmed by
    (a) none of the 6 file paths intersect the 99 this cycle touched, (b) all 6 are a DIFFERENT kind
    entirely (`class_feature`/`trait_generic`/`feat_generic`, not `monster_ability`/`equipment`/
    `feat`), and (c) `grep -c` on the pre-regen sweep log shows the same 6 files present at the same
    finding count in the BEFORE run too (86 = 71 now-fixed + 15 pre-existing-unrelated). Not touched
    by this cycle — a different, already-known gap, out of this cycle's scope.
  - The 71-of-77 delta (not 77) is exact: `77 - 6 already-in-the-pre-existing-15's-77-overlap`... no
    — restated precisely: before-run's 77 flagged records were ALL 77 `gen_book_cache`-shaped ones
    (0 overlap with the 6 pre-existing), so after-run's 6 remaining records are wholly disjoint from
    this cycle's 99-record fix set. **77 of 77 sweep-flagged `monster_ability` records now clean.**
- **Targeted test suites** (all `CARGO_TARGET_DIR`-isolated, `CARGO_INCREMENTAL=0`):
  - `cargo test --locked --lib rules_core::pi_screening`: 40/40 pass (8 new).
  - `cargo test --locked --lib cache_gen::`: 186/186 pass, 11 ignored (oracle-gated, unaffected) —
    covers `equipment_gap`/`feat_gap`/`class_feature`, none regressed by the shared
    `classify_field` fix.
  - `cargo test --locked --bin gen_book_cache`: 5/5 pass.
  - `cargo test --locked --bin declared_pi_shipping_audit`: 21/21 pass.
  - `cargo test --locked --bin reconcile_description_pi_stamps`: 6/6 pass (new).
- **PCGen oracle used throughout:** `7f818006e371188e5717fd18d74d18a420747fc6`, confirmed via
  `/proc/<pid>/environ` before trusting the first audit run's output (not the forbidden
  `~/workspace/repos/pcgen` default).
- **PI grep on this cycle's own diff:** `git diff -- src/bin/gen_book_cache.rs
  src/rules_core/pi_screening.rs` and the new `reconcile_description_pi_stamps.rs` grepped for every
  `ogl-pi-blacklist.md` term — zero hits (this cycle's source changes reference the marker constant
  and the classifier, never a literal PI term or item name).
- **Unrelated pre-existing red, observed not caused:** `cargo test --locked --lib rules_core::` (full
  lib sweep) shows 1 failure —
  `rules_core::rules_tables::monster_chassis::tests::widening_the_facet_vocabulary_does_not_reclassify_any_existing_record`
  (a digest-ratchet fixture out of sync with `MONSTER_BOOKS`' current sorted-triple set). Confirmed
  pre-existing and out of this cycle's scope: `git diff --stat -- src/rules_core/rules_tables/
  src/rules_core/pilot_compute/` is empty — this cycle touched neither file, and the failure is
  present on this cycle's own base commit (`52b46ffeab`) before any of this cycle's edits. Not fixed
  here (a digest-ratchet update is a different lane's territory — likely row 17/18's `pilot_compute`/
  `class_feature_pool_catalog` work or a concurrent monster-table cycle — and this cycle's own
  2,649/2,650 pass rate on the same run is otherwise clean).

## 6. Territory

No file overlap with row 17 (`pilot_compute/mod.rs`), row 18
(`pilot_compute/mod.rs`/`class_feature_pool_catalog.rs`), or row 19 (`apps/desktop/src-tauri`).
`corpus_ingest_diagnostic::tests::the_two_ingested_books_totals_reconcile_with_their_license_artifacts`
not touched — this cycle's 99-record regen changes zero record COUNTS (no record added, dropped, or
moved kind; only 3 stamp fields patched on already-existing records), so no `LICENSE.json` count
line changes. `git log origin/tranche/12 -- data/corpus/bestiary_4 data/corpus/inner_sea_gods
data/corpus/inner_sea_bestiary data/corpus/inner_sea_world_guide data/corpus/inner_sea_temples
data/corpus/book_of_the_damned_volume_2 data/corpus/inner_sea_races` checked before starting — no
uncommitted sibling activity in these paths at dispatch time.
