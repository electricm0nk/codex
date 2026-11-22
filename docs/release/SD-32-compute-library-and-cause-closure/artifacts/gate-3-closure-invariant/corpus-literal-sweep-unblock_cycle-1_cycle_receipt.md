# Cycle corpus-literal-sweep-unblock — Gate 3 / repo-wide blocker clearance

- **Card ID:** 9 (`gate-3-closure-invariant`)
- **Commit SHA:** (this cycle's commit, see push log)
- **Files touched:** `src/rules_core/pi_screening.rs`, `src/rules_core/corpus_literal_sweep.rs`,
  `scripts/pi_scrub.py`, `scripts/tests/test_sd32_t9_pi_normalization_and_inheritance.py`,
  `data/corpus/advanced_players_guide/class_feature/shifter_s_blessing/form_of_the_cat.json`,
  `data/corpus/advanced_race_guide/class_feature/buccaneer/seadog_s_gait.json`,
  `data/corpus/advanced_race_guide/class_feature/gunslinger_archetype/buccaneer.json`,
  `data/corpus/horror_adventures/class_feature/barbarian_archetype/dreadnought.json`,
  `data/corpus/horror_adventures/class_feature/dreadnought/steady_gait.json`,
  `data/corpus/ultimate_wilderness/class_feature/commando/ranger_trap.json`,
  `docs/work-inventory.json` (regenerated through the guarded path)
- **Identifier audit result:** OK_NO_BUNDLE_TAGS
- **Wired-integration audit result:** OK_NO_TOKENS
- **Acceptance criterion:** repo-wide blocker — `corpus_literal_sweep --json-out` must report
  `clean:true` before `docs/work-inventory.json` can be regenerated through the guarded path
  (`v06_work_inventory`'s own stamp-loss guard requires a fresh `CORPUS_LITERAL_SWEEP_REPORT`, and
  the sweep's own doc comment: units are "only verified here if the WHOLE sweep came back CLEAN").
- **Corpus SHA:** `7f818006e371188e5717fd18d74d18a420747fc6` (pinned PCGen oracle)
- **Status:** complete
- **Notes:** see full re-derivation and disposition below.

## Re-derivation (`§17a`)

Reproduced the blocker with `cargo run --locked --bin corpus_literal_sweep --release --
--json-out`: **8 findings across 7 records**, all `class_feature`, all `TokenNotInClosure` on a
non-`DESC` token reading `[redacted PI]` — matching the escalation exactly.

## Disposition — split by root cause, not a single carve-out

Per-record inspection (`license`, `pi_field`, the real corpus row at the cited `(source_file,
source_line)`) found **two independent, unrelated defects**, not one shape:

**(a) Six records — genuine false-positive PI redactions (the records are wrong; fixed via the
guarded generator).** `cache_gen::class_feature::redact_concatenated_blacklist_tokens` correctly
redacted these using `pi_screening::blacklist_term_hit_including_concatenated`, but that function
itself has two bugs, both now fixed in `src/rules_core/pi_screening.rs` (and the equivalent,
independently-drifted `scripts/pi_scrub.py`, which shares the same defect):

1. **The "Galt"/"gait" OCR-fold collision** (same false-positive class `decisions.md §26` already
   fixed for "Jarn"/"jam", but on the *character* fold table `l`/`1`/`!`→`i` rather than the `rn`→`m`
   fold): "Galt" (a Golarion nation, the only blacklist term containing `l`) canonicalizes to
   "gait" — an ordinary English word ("his gait more deliberate...", "Seadog's Gait", "Steady
   Gait"). Fixed with a term-specific fold exemption (`_CHAR_FOLD_EXEMPT_TERMS_CASEFOLD` /
   `term_needs_char_fold`), mirroring `§26`'s `_RN_FOLD_EXEMPT_TERMS_CASEFOLD` mechanism exactly.
   Affects: `form_of_the_cat.json` (DESC), `seadog_s_gait.json` (KEY), `gunslinger_archetype/
   buccaneer.json` (ABILITY, restates "Seadog's Gait"), `barbarian_archetype/dreadnought.json`
   (ABILITY, restates "Steady Gait"), `dreadnought/steady_gait.json` (KEY).
2. **The `alnum_normalize`/whitespace-stripping collision** — the Rust concatenated-scan (`checks
   3/4`) stripped real whitespace before the substring match, an already-known and already-fixed-in-
   Python bug (`pi_scrub.py::_normalize_haystack`, the `hidden_wand.json`/"Andoran" incident) that
   was never ported to the Rust mirror. `"Commando ~ Ranger Trap"` collapses to
   `"commandorangertrap"`, which contains "andoran" purely from whitespace deletion. Fixed by adding
   `alnum_normalize_haystack` (whitespace-preserving) to `pi_screening.rs`, mirroring the Python
   fix exactly. Affects: `commando/ranger_trap.json` (KEY).

Both fixes are **narrowing false positives only** — proven not to weaken real detection by a
mutation-proof test per fix (`concatenated_scan_galt_still_catches_a_literal_plain_spelling`,
`concatenated_scan_still_catches_a_genuinely_no_separator_andoran_identifier`,
`test_mutation_proof_removing_the_char_fold_exemption_reopens_the_false_positive`), matching `§1a`.

The 6 affected records were regenerated through the guarded path (`cargo run --locked --release
--bin gen_cache_class_feature`), which rewrote all 17,979 `class_feature` records (their
`ingested_at` stamp is real system time, not content); every file whose content besides
`ingested_at` was byte-identical to its pre-image was reverted (`git checkout --`) before staging,
leaving exactly the 6 records with a genuine content diff. Diffed and confirmed: each now ships
`license: "OGL"`, `pi_field: null`, and its real, un-redacted token value — no other field moved.

**(b) One record — a genuine PI redaction the sweep's own exemption logic was too narrow to
recognise (the sweep was wrong; fixed there, record untouched).**
`inner_sea_combat/class_feature/ranger_combat_style/cayden_callean.json`'s `KEY`/`PREDEITY` tokens
are genuinely redacted: `"Cayden Callean"` is a double-`L` misspelling of the blacklisted
`"Cayden Cailean"` that only the word-bounded, OCR-normalized scan
(`blacklist_term_hit_including_concatenated`) can see (both spellings fold to the same canonical
form under the `l`→`i` table) — the older, bare-substring `classify_field` the sweep's second
exemption re-derived against cannot. Widened `corpus_literal_sweep.rs::compare_tokens`'s non-`DESC`
re-screen to accept **either** scan (purely additive — `classify_field`'s existing disjunct is
unchanged), proven with a mutation-style negative test
(`the_concatenated_scan_disjunct_does_not_wave_through_a_genuinely_clean_value`) that a token backed
by neither scan is still reported, per `§1a`.

## Verification

- `corpus_literal_sweep --json-out`: **0 findings, `clean: true`** (was 8/`clean: false`).
- `cargo test --locked --lib rules_core::pi_screening`: 33/33 pass (7 new: Galt/Andoran
  false-positive-refused + mutation-proof + genuine-catch-survives tests).
- `cargo test --locked --lib rules_core::corpus_literal_sweep`: 38/38 pass (2 new: the widened
  exemption's positive case + its negative/mutation case).
- `cargo test --locked --lib rules_core::cache_gen::class_feature`: 70/70 pass (10 ignored,
  oracle-gated, unaffected).
- `python3 -m unittest scripts.tests.test_pi_scrub scripts.tests.test_sd32_t9_pi_normalization_and_inheritance scripts.tests.test_sd32_companion_allowlist_widening scripts.tests.test_pi_key_rawtokens_audit scripts.tests.test_pi_key_rawtokens_defect1_regen scripts.tests.test_ingest_ability_raw_tokens_pi_screen`:
  57/57 pass (8 new Galt tests).
- `cargo test --locked --lib rules_core::pilot_compute`: 893/893 pass (broad engine sweep,
  unaffected by the un-redaction — confirms no downstream consumer regressed).

## Inventory regen (unblocked by the above)

Guarded path: `CORPUS_LITERAL_SWEEP_REPORT`/`DERIVED_FIXTURE_CHECK_REPORT` set to fresh
`--json-out` reports from `corpus_literal_sweep` (clean) and `derived_evaluator_fixture_check`
(1,836 units cleared over 2,577 fixture rows, 0 failed); **no `--allow-stamp-loss`**. Exit code 0,
no stamp-loss refusal.

**Stamps preserved BY ID** (not by count): before 8,247 (`literal-verified` 6,506 +
`fixture-verified` 1,741), after 8,308 (`literal-verified` 6,567 + `fixture-verified` 1,741).
**0 lost, 61 gained, 0 changed-verdict** — every one of the 8,247 pre-regen ids is present in the
post-regen stamped set with the same status.

**Full status-distribution diff** (`total_units` 49,513 → 49,511):

| status | before | after | Δ |
|---|---:|---:|---:|
| `not-ingested` | 27,058 | 27,035 | -23 |
| `literal-verified` | 6,506 | 6,567 | +61 |
| `text-complete` | 5,021 | 5,070 | +49 |
| `unknown` | 4,286 | 4,257 | -29 |
| `grounded` | 3,224 | 3,232 | +8 |
| `fixture-verified` | 1,741 | 1,741 | 0 |
| `ingested-magnitude` | 1,612 | 1,544 | -68 |
| `deferred-with-reason` | 46 | 46 | 0 |
| `not-started` | 19 | 19 | 0 |

**Per-kind (only kinds with a movement):**

- `equipment`: `ingested-magnitude` 446→394 (-52), `literal-verified` 5099→5149 (+50), `grounded`
  168→170 (+2), `unknown` 227→225 (-2).
- `equipment_modifier`: `ingested-magnitude` 467→451 (-16), `literal-verified` 35→46 (+11),
  `text-complete` 509→536 (+27), `unknown` 403→376 (-27), `grounded` 165→170 (+5).
- `monster_ability`: `not-ingested` 121→98 (-23), `text-complete` 2044→2066 (+22), `grounded`
  1393→1394 (+1).

**Classification of every movement above: instrument correction, not closure** (`§16`). None of
this cycle's own diff touches `equipment`, `equipment_modifier`, or `monster_ability` territory
(the sibling lanes' data) — these movements are latent HEAD-source fixes from those lanes that
could not previously *materialize* into `docs/work-inventory.json` because the sweep-dirty state
blocked every regen since the last committed one. This cycle only unblocks the instrument; it does
not claim credit for the underlying content work.

**2 units disappeared entirely** (`total_units` -2, not a status movement):
`advanced_class_guide:equipment:dust_knuckles_forget`,
`advanced_class_guide:equipment:false_face_forget` — both `.FORGET` rows, correctly stopping being
enumerated as their own unit (a `.FORGET` row is a removal directive, not an item) now that the
regen can run; the code fix for this was already landed in HEAD source but unmaterialized in the
inventory, per the brief.

## `no_record` re-derivation (`§20`)

`python3 scripts/shape_ledger.py --inventory docs/work-inventory.json`: population 34,521,
`no_record` **128** (was **130** in the brief — the 2 `.FORGET` removals above account for the
delta), by kind: `monster_ability` 98, `equipment_modifier` 19, `equipment` 8 (was 10, -2 for the
`.FORGET` fix), `companion` 2, `ability` 1. `scripts/shape_coverage_standing_gate.py`: budget
128/34,521 vs. baseline 21,521/36,028 — **not exceeded**. `NO_RECORD_BUDGET_COUNT`/`POPULATION`
constants in `scripts/verify.sh`/`shape_coverage_standing_gate.py`: **untouched**.

## Discovery forwards

1. **This cycle's own blocker: closed, no new blocker opened.** The 128 remaining `no_record` units
   are sibling-lane territory (`monster_ability`/`equipment_modifier`/`equipment`/`companion`/
   `ability`), unchanged in disposition by this cycle beyond the 2 `.FORGET` count correction.
2. **A pre-existing, already-named `class_feature` PI-redaction gap, confirmed still open, NOT
   fixed by this cycle (different audit surface, larger scope than the dispatched blocker).**
   `artifacts/gate-3-closure-invariant/sd32-pi-leak-screening-path-inner-sea-combat-feat_cycle-1_cycle_receipt.md`
   §5 (a prior cycle, generic-field blacklist scan across every `data.*` field, all kinds) found
   **31 `class_feature` hits, 28 confirmed real** (`data.class`/`data.name`/`data.description`
   carrying an unredacted blacklist term verbatim), explicitly handed off "by coordinate" to the
   `class_feature` lane. **This cycle independently fixed the 3 it also separately named as
   false-positive** (the Galt/gait collision — `steady_gait.json`, `seadog_s_gait.json`,
   `form_of_the_cat.json` — all now clean per `corpus_literal_sweep`) **but did not touch the 28
   real ones**, which is orthogonal to `corpus_literal_sweep` (raw_tokens byte-match only; does not
   read `data.class`/`data.description` at all) and requires its own root-cause fix in
   `cache_gen::class_feature.rs` (screen those two typed fields against the blacklist the way
   `raw_tokens` already is) plus a guarded regen — out of this cycle's dispatched scope (the sweep
   blocker), logged here so it is not silently dropped: `adventurers_guide/class_feature/
   aldori_swordlord/*` (12), `aldori_defender/*` (4), `magaambyan_arcanist/*` (11),
   `magaambyan_initiate/*` (2), `inner_sea_combat/class_feature/ranger/
   codex_named_unit_class_feature_inner_sea_combat_isc_abilities_class_lst_256.json` (1). (The 31st,
   `cayden_callean.json`'s `data.name` hit, is the SAME record whose `raw_tokens` this cycle's `§17a`
   re-derivation independently confirmed genuinely PI — its `data.name` field likely needs the same
   disposition, unverified by this cycle.)

## Next-cycle plan

Sibling lanes resume regenerating `docs/work-inventory.json` freely — the sweep is clean and stays
clean unless a future ingest path introduces a genuinely new PI-redaction shape the two
`compare_tokens` exemptions still cannot recognise (in which case: re-derive per `§17a`, do not
widen an exemption without a mutation-proof, per `§1a`).
