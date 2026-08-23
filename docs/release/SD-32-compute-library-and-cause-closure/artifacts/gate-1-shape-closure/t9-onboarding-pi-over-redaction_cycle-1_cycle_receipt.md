# Cycle t9-onboarding-pi-over-redaction — Gate 1 (shape closure) / row 17's PI-over-redaction defect

- **Card ID:** `epic-7-shape-categorization-100` (kanban.md row 17) — note updated, status left `backlog` (correctly blocked on `no_record == 0`, a sibling lane's ingest work; this cycle made zero `no_record`-closing writes).
- **Commit SHA:** (this cycle's own commit, see push log)
- **Files touched:**
  - `scripts/pi_scrub.py` (the fix)
  - `scripts/tests/test_pi_scrub.py` (3 new tests: RED→GREEN + genuine-self-reference-preserved + mutation-proof)
  - `scripts/regen_row17_pi_over_redaction.py` (new — scoped guarded-path regenerator)
  - 68 files under `data/corpus/**` (regenerated `data.raw_tokens`/`data.description`/`data.pi_field` in place, same path/slug/key/name)
  - `docs/release/SD-32-compute-library-and-cause-closure/kanban.md` (row 17 note)
  - `docs/release/SD-32-compute-library-and-cause-closure/artifacts/gate-1-shape-closure/row17-census.json` (re-derived output)
  - `docs/retro/events/t9-onboarding.jsonl` (near-miss + verify.sh preflight events)
- **Identifier audit result:** one grep hit, explained: `scripts/regen_row17_pi_over_redaction.py`'s `from sd32_t9_pi_review_feat_equipment import (...)` — a legitimate import of a pre-existing, already-shipped repo module (the same import `ingest_generic_kind.py` already uses), not an ephemeral bundle-tag leak. No other hits.
- **Wired-integration audit result:** `OK_NO_TOKENS_CODE` on the code diff. The whole-diff grep hits one pre-existing occurrence of the word "placeholder" inside an unchanged sentence of the kanban.md row-17 prose (kanban rows are single long lines, so editing the row re-diffs the whole line) — describing the *defect itself* ("F0 reached by fallthrough... a placeholder wearing a family label"), not a stub/mock in shipping code. Self-healed by inspection per §6 step 4; no shipping-code stub/mock/no-op present.
- **Acceptance criterion:** Dispatch brief — "83 units whose mechanical formula values were destroyed by over-redaction... find the over-redacting write path and fix it... regenerate the affected records through the guarded path... prove the mechanical values are restored... prove you did not re-leak PI... verify the 84th unit."
- **Corpus SHA:** PCGen oracle bootstrapped via `scripts/fetch-pcgen-oracle.sh` this cycle: `7f818006e371188e5717fd18d74d18a420747fc6`.
- **Status:** complete
- **Notes:**

## Re-derivation (`§17a`)

`python3 scripts/row17_census.py` (fresh run, not trusted from the brief): population 34,631, F0-fallthrough **84** (83 `pi_redacted_formula`), matching the brief's inherited figure exactly this time — but re-derived, not assumed.

## Root cause

`scripts/pi_scrub.py::scrub_name_pi_tokens`'s needle generator split every `~`-delimited `KEY` segment into individual WORDS with no length floor on the plain (space-preserving) needle set. PCGen's `KEY` schema is frequently `<Category-or-Group> ~ <Specific>` (real shape: `"Trait ~ <PI-bearing trait name>"`, `"Temp Bonus ~ <PI-bearing role/deity name>"` — coordinates only, never the PI names themselves, per `§24b`-2). The per-word split turned ordinary PCGen/Pathfinder rules vocabulary making up the group half — "Trait", "Temp", "Bonus", "Evangelist", "Sentinel", "Exalted" — into standalone redaction needles (none of these six words is itself a PI item; they are PCGen category/role vocabulary shared across many records, blacklisted or not). Any unrelated, genuinely PI-free `BONUS`/`DEFINE` value that happened to also contain the same common word (worst example: **every** `Trait ~ <Name>` record's own `BONUS:...TYPE=Trait` token, present by PCGen convention on every trait) got wiped to `[redacted PI]` for a reason unrelated to its actual content.

## Fix

`scripts/pi_scrub.py::scrub_name_pi_tokens`:
1. Gate the plain (space-preserving) `needles` set on the SAME `_MIN_NORMALIZED_NEEDLE_LEN` (6) floor `norm_needles` already used alone.
2. Remove the per-WORD split loop entirely — needles are now name, key, and whole `~`-segments only, never a segment's individual constituent words.

Genuine self-reference (the record's own FULL name/key or a full `~`-segment appearing verbatim in a token value — e.g. an `ABILITYPOOL` value restating the record's own pool name) is still caught, unaffected by removing the word-level loop. Every blacklist-term catch (checks 1 + 4, `blacklist_term_hit_including_concatenated`) is untouched — it derives its needles from `PI_BLACKLIST_TERMS`, never from `name`/`key`.

## Tests (RED → GREEN, `§1a`)

`scripts/tests/test_pi_scrub.py`, new class `GenericCategoryWordIsNotAStandaloneNeedleTests`:
- `test_generic_group_word_from_the_key_does_not_redact_an_unrelated_token` — the fixed case.
- `test_genuine_self_reference_of_the_full_segment_is_still_redacted` — proves the fix does not weaken real coverage.
- `test_mutation_proof_reintroducing_the_per_word_split_reopens_the_over_redaction` — proves the pre-fix per-word split reproduces the exact over-redaction, so the first test means something.

**RED proved live**: copied the pre-fix module over the fixed one (`git show HEAD:scripts/pi_scrub.py`), ran the suite — `test_generic_group_word_from_the_key_does_not_redact_an_unrelated_token` FAILs (`AssertionError: True is not false`) for the intended reason; all 12 other tests still pass. Restored the fix, re-ran — 13/13 GREEN.

Full dependent-suite run after the fix: `python3 -m unittest scripts.tests.test_pi_scrub scripts.tests.test_sd32_t9_pi_normalization_and_inheritance scripts.tests.test_sd32_companion_allowlist_widening scripts.tests.test_pi_key_rawtokens_audit scripts.tests.test_ingest_ability_raw_tokens_pi_screen scripts.tests.test_ingest_generic_kind scripts.tests.test_ingest_ability_pi_rename scripts.tests.test_ingest_simple_filename_kinds scripts.tests.test_codex_neutral_name` — **101 tests, all green**.

## Regen through the guarded path

Considered the existing corpus-wide `regen_all_renamed_pi_scrub.py` (856 records changed on a dry run) but rejected it: it reprocesses every `codex_generated_name: true` record regardless of kind/book, and most of the 856 are `equipment`/`ability`/`deity`/`spell`/`monster` records this cycle's territory names as live sibling-lane ground — an unnecessary collision risk for a fix scoped to 83 units.

Wrote `scripts/regen_row17_pi_over_redaction.py`: re-derives the SAME `row17_census`/`shape_ledger` fallthrough+`pi_redacted_formula` population directly (not a static list, so a re-run always re-derives current scope), scoped strictly to those units, `codex_generated_name: true` only (the only records `scrub_name_pi_tokens` ever ran on).

### Near-miss caught before commit (`§1a` / declared_pi_shipping_audit)

First draft re-derived `data.raw_tokens` via `scrub_name_pi_tokens` alone (mirroring `regen_all_renamed_pi_scrub.py`'s own shape). `cargo run --bin declared_pi_shipping_audit` against that output found **3 NEW `DESC-PI-SHIPPED-IN-RAW-TOKENS` violations**: `adventurers_guide/ability/...ag_abilities_lst_25.json`, `adventurers_guide/feat_generic/...ag_feats_lst_37.json`, `inner_sea_world_guide/feat_generic/...iswg_feats_lst_30.json` — records whose `DESC` prose does not happen to literally contain the record's own PI name as a substring, so the identity/blacklist checks never redacted that DESC token, and the freshly re-parsed oracle line's full, un-redacted description was about to ship in `raw_tokens` even though `data.description` itself stayed correctly redacted.

Reverted `data/corpus/` to HEAD (`git checkout HEAD -- data/corpus`, confirmed 0 modified corpus files afterward) before any commit. Confirmed the SAME gap reproduces directly against `regen_all_renamed_pi_scrub.py`'s own logic (its `raw_tokens` re-derivation also omits `ingest_generic_kind.py::remediate`'s DESC-blanking step) — a latent, pre-existing defect in that established driver too, logged as a discovery-forward below rather than silently patched (out of this cycle's territory; that driver's blast radius is corpus-wide).

Rewrote `regen_row17_pi_over_redaction.py` to reproduce `ingest_generic_kind.py::remediate`'s FULL pipeline: declared-PI detection (`declared_pi`) → DESC blanking → blacklist scan (`blacklist_term_hit_including_concatenated`, skipping the already-blanked DESC) → identity/blacklist scan (`scrub_name_pi_tokens`). Re-ran; the 3 records now redact correctly; `declared_pi_shipping_audit` shows zero violations in any of the 68 touched files.

Logged: `scripts/retro.py near-miss` (`RETRO_ACTOR=t9-onboarding`, `docs/retro/events/t9-onboarding.jsonl`, id `1787518468793-t9-onboarding-802448`, `recurrence-key`-equivalent tag `pi-leak-near-miss`).

### Regen run

```
PCGEN_CORPUS_ROOT=<oracle>/data python3 scripts/regen_row17_pi_over_redaction.py
```
Result: `row17_pi_redacted_fallthrough_population: 83`, `not_codex_generated_skipped: 9`, `changed: 68`, `unchanged: 6`, `unresolved_path: []`. `git status --porcelain` confirmed exactly 68 modified files under `data/corpus/**`, nothing else, before every subsequent step.

## Result split (`§16` — a unit moved out of a shape is not a unit closed; name the split)

Of the 83 `pi_redacted_formula` units:
- **63 recovered** — genuinely PI-free; formula restored to real content.
- **7 correctly remain redacted** — the record's own FULL name/key `~`-segment appears verbatim in the token value (e.g. an `ABILITYPOOL` value restating the record's own key's second `~`-segment). Genuine self-reference, correctly caught, unaffected by the fix. Unit coordinates only (never the PI names) available in `artifacts/gate-1-shape-closure/row17-census.json`'s `per_kind_book` breakdown and this cycle's own analysis, not transcribed here.
- **13 correctly remain redacted** — a real blacklist term (6 distinct terms across the 13 records, per `docs/governance/ogl-pi-blacklist.md` / `scripts/pi_scrub.py::PI_BLACKLIST_TERMS`, never named here per `§24b`-2) is concatenated into the token's value (a class/archetype name built into a `MASTERVAR`/`DEFINE` variable identifier with no separator), caught by the independent, unaffected blacklist scan.

`63 + 7 + 13 = 83`. Both the 7 and the 13 are named individually in this cycle's own analysis (unit ids + matched terms), not asserted.

## The 84th unit (non-PI)

`ultimate_campaign:trait:trait_harvester` (`data/corpus/ultimate_campaign/trait_generic/trait_harvester.json`, `data/corpus/ultimate_campaign/ability/harvester.json`) — `license: "OGL"`, `pi_marker: null`, never redacted. Its `BONUS:SKILL|%LIST` token has only 2 pipe-parts; `shape_ledger.py::extract_formula_segment`'s `BONUS` handling requires `len(parts) >= 3` to extract a magnitude segment, so this token falls through to F0 with no segment to classify — a genuine classifier gap, not a PI issue. `%LIST` is PCGen's own CHOOSE-substitution shorthand and carries no explicit numeric value in the LST row itself (verified: only 2 corpus files corpus-wide use this exact `"SKILL|%LIST"` 2-part shape, both this same unit's dual `ability`/`trait` ingest — not a recurring pattern). Named here rather than guessed at with an invented default magnitude, per `§1a` (fabricating a value the corpus does not state is worse than leaving it named and open).

## PI audits, before vs. after (`§12c`)

`python3 scripts/pi_key_rawtokens_audit.py` (population: `data/corpus/**`, all kinds, `scanned=27523`):

| | scanned | confirmed | candidate (unratified, informational) |
|---|---:|---:|---:|
| Before | 27523 | 0 | 25540 |
| After | 27523 | 0 | 25540 |

Confirmed unchanged at 0 both runs. "Before" obtained by temporarily restoring the 68 touched files to their `HEAD` content (`git checkout HEAD -- data/corpus`, backed up the fixed versions first), running the audit, then restoring the fixed versions from the backup and re-running to confirm byte-identical output to the pre-revert run (confirmed: `diff` clean). The candidate bucket's per-term counts shift (e.g. "Trait" 1806→1843, "Knowledge" 691→699) — expected, since restored formula text now genuinely contains more instances of ordinary words; candidates are informational/unratified only, no action taken on them per standing policy.

`cargo run --locked --bin declared_pi_shipping_audit` (Rust; scans `data/corpus/**` at runtime, source code untouched by this diff):

| | violations | population |
|---|---:|---|
| After (this cycle) | 65 | all `DESC-PI-SHIPPED` in `bestiary_4/monster_ability` — same population `decisions.md §26` named as the sibling lane's own known, unchanged baseline |

Confirmed **zero** violations in any of the 68 files this cycle touched, both immediately after the near-miss fix and in this final run.

## Row 17 honest size

`python3 scripts/row17_census.py --output artifacts/gate-1-shape-closure/row17-census.json`:

| | before | after |
|---|---:|---:|
| `derived` (real family) | 11,341 | 11,404 (+63) |
| `measured_empty` (real F0) | 23,083 | 23,083 |
| `fallthrough` | 84 | **21** |
| `fallthrough` × `pi_redacted_formula` | 83 | **20** (7 + 13 above) |
| `row17_honest_size` | 84 | **21** |

`python3 scripts/row17_census.py --check` — exit 0, no malformed-marker violations.

## Discovery forwards

- **`regen_all_renamed_pi_scrub.py` has the same latent DESC-blanking gap** this cycle's own first draft hit (confirmed by direct reproduction against its exact logic on `adventurers_guide/ability/...lst_25.json` — its `scrub_name_pi_tokens`-only re-derivation leaves the un-redacted DESC token unchanged). Not fixed here (corpus-wide blast radius, sibling territory); named for whichever lane next runs that driver, or for a dedicated follow-up.
- **`ultimate_campaign:trait:trait_harvester`'s `BONUS:SKILL|%LIST` 2-part shape** is an open, single-unit, non-PI classifier gap in `shape_ledger.py::extract_formula_segment`. Needs PCGen-semantics confirmation of what magnitude (if any) `%LIST` alone implies before any fix — do not guess a default.

## Discoveries

- 4th duplication-drift-adjacent finding this bundle: an over-broad word split, not a blacklist-term/OCR-fold issue like `decisions.md §26`'s single-term fold-collision fix — a different failure mode of the same "needle generation is more subtle than it looks" class.
- `regen_all_renamed_pi_scrub.py`'s own 856-record corpus-wide dry-run blast radius is a useful signal for how much sibling-lane surface a blanket `codex_generated_name`-scoped regen touches; scoping to a specific defect's actual population (this cycle's approach) avoids that entirely.

## Next-cycle plan

1. Row 17 stays `backlog` until `no_record` (currently 123) reaches 0 (sibling lane).
2. Once row 17's categorization pass genuinely starts, its residual 21 (`fallthrough`, 20 PI-genuine + 1 `trait_harvester` classifier gap) join whatever generic classification work that pass does — the 20 genuine-PI ones are not further reducible without violating `§24b`; `trait_harvester` needs the `%LIST` semantics question answered.
3. `regen_all_renamed_pi_scrub.py`'s DESC-blanking gap: escalate/fix in a dedicated cycle before its next corpus-wide invocation.

## Disk

`df -h /`: see final report below.
