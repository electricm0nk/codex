# Cycle t9-onboarding/corpus-literal-sweep-pi-exemption-gap — repo-wide `corpus_literal_sweep` blocker (Gate 3 closure invariant precondition)

- **Card ID:** none (precondition fix, not itself an Epic card; unblocks `epic-6-kind-trait`
  row 16 and every other lane's `docs/work-inventory.json` regen)
- **Actor:** `t9-onboarding`
- **Base:** `3c7834101cf152cc86e016513a4e382248c833f5` (pinned `PIN`)
- **Commit SHA:** (this cycle's own commit, see push log)
- **Files touched:**
  - `scripts/pi_scrub.py` — root-cause fix: `_normalize_haystack` (new), used by
    `blacklist_term_hit_including_concatenated` (check 4) and `scrub_name_pi_tokens`'s check 3,
    preserves real whitespace as a match boundary instead of deleting it.
  - `scripts/tests/test_pi_scrub.py` — new `ConcatenatedCheckDoesNotSpanRealWhitespaceTests` class,
    3 tests (RED->GREEN proved, plus a mutation proof the fix is load-bearing).
  - `data/corpus/inner_sea_magic/ability/hidden_wand.json`,
    `data/corpus/advanced_players_guide/ability/favored_son_daughter_belor_hemlock_town_sheriff.json`,
    `data/corpus/inner_sea_gods/ability/codex_named_unit_ability_inner_sea_gods_isg_abilities_faith_lst_98.json`
    — regenerated through the guarded generator path (`python3 scripts/ingest_ability.py`, no hand
    edits), all 3 un-redacted (false positives, confirmed clean against the pinned oracle).
  - `docs/work-inventory.json` — regenerated through the guarded path (below).

## Root cause (`§17a` — reproduce before trusting a lead; do not guess)

The dispatch brief named `data/corpus/inner_sea_magic/ability/hidden_wand.json` as the sweep's one
`clean:false` finding and asked whether the exemption is too narrow (a) or the record is genuinely
inconsistent (b). **Reproduced first.** `record.codex_generated_name` is `false` on this record — it
is NOT a `§24`-renamed record at all, so `compare_tokens`'s third (`§24`) exemption never applies to
it; that branch of the brief's hypothesis (a) does not describe this record.

The real defect: `ingest_ability.py`'s blacklist screen (`scrub_blacklist_pi_tokens` ->
`pi_scrub.blacklist_term_hit_including_concatenated`, check 4, "concatenated-form" scan for a term
joined PascalCase-style into a token value with no separator) false-positived on ordinary prose. The
DESC text "...you activate a wand (or any similar spell trigger item..." contains the three separate,
real, whitespace-separated English words "wand", "or", "any" — but `_normalize`'s
strip-every-non-alphanumeric-character normalization DELETES the real spaces between them, manufacturing
the run-on substring formed by "wand"+"or"+"any", which happens to contain one of
`pi_scrub.PI_BLACKLIST_TERMS`'s place-name entries (see that list for the literal value — not
repeated here per PI discipline: never write a blacklist term into a receipt) even though no genuine
no-separator concatenation exists anywhere in the source. Confirmed directly:
`python3 -c "...; from pi_scrub import blacklist_term_hit_including_concatenated; print(blacklist_term_hit_including_concatenated(<hidden_wand DESC text>))"` -> a non-`None` hit, before the fix; `None` after.

Cross-checked against the pinned oracle row itself
(`pathfinder/paizo/campaign_setting/inner_sea_magic/ism_abilities_other.lst:120`): the real PCGen row
carries **no** `NAMEISPI:YES`/`DESCISPI:YES` declaration, and its sibling row at line 119
(`Lingering Illusions`, same `PREABILITY:...,White Grotto ~ Guild` reference, same structure,
ingested 22 minutes earlier) shipped correctly un-redacted (`license: "OGL"`, `pi_field: null`) —
proving the correct disposition for this shape is clean, and `hidden_wand.json`'s redaction was the
outlier, not the rule. This settles candidate (b) as a real defect, but NOT in the record's own data —
in the SCREEN that redacted it (a genuine false positive), matching neither (a) nor (b) exactly as the
brief framed them: the exemption in `compare_tokens` is correctly scoped (this was never a `§24`
record); the corpus row itself is not "inconsistent" upstream data either. The defect is a
false-positive in `ingest_ability.py`'s own PI screen.

## Fix (`§1a` — a weakened detector is worse than a noisy one; §22-adjacent — this is OUR bug, fixed at
the source, not an upstream inconsistency)

`pi_scrub._normalize` (strip-everything, used to build short known-needle/term forms — correct and
unchanged there, since a multi-word deity/place-name term must still normalize to its own no-separator
form to be found embedded in a genuinely-concatenated identifier) is now used ONLY for
needle/term-side normalization. A new `pi_scrub._normalize_haystack` (strips punctuation, PRESERVES
real whitespace) is used for the VALUE side in both check 3 (`scrub_name_pi_tokens`'s own-identity
concatenation check) and check 4 (`blacklist_term_hit_including_concatenated`). Real whitespace in the
source text now acts as a hard separator, the same way `.`/`,`/`|`/`~` already did — which costs the
checks' actual designed purpose nothing, because a genuine PCGen `BONUS`/`DEFINE`/`TYPE` identifier
concatenation (`CoordinatedeityAspectChoice`, the shape check 4 exists for) never contains whitespace
to begin with.

RED->GREEN proved (`scripts/tests/test_pi_scrub.py::ConcatenatedCheckDoesNotSpanRealWhitespaceTests`,
synthetic term "Testcase"/"test case", never a real blacklist term, matching this file's own stated
convention):
- RED before the fix: `test_ordinary_prose_whose_words_concatenate_across_real_whitespace_is_not_a_hit`
  failed (`'Testcase' is not None`) — confirmed for the intended reason.
- GREEN after: same test passes; `test_genuine_no_separator_concatenation_of_the_same_term_is_still_caught`
  (no real whitespace in the value at all) still passes unchanged — the mechanism check 4 exists for
  is not weakened.
- Mutation proof: `test_mutation_proof_a_naive_strip_all_normalize_reopens_the_false_positive` proves
  the OLD strip-everything normalization DOES reproduce the collision, so the RED case means something.

Full suite: `python3 -m unittest scripts.tests.test_pi_scrub` 10/10 green (was 7/7 pre-cycle, +3 new).
Wider regression sweep, no failures: `scripts.tests.test_pi_scrub
scripts.tests.test_sd32_t9_pi_normalization_and_inheritance
scripts.tests.test_sd32_companion_allowlist_widening scripts.tests.test_pi_key_rawtokens_audit
scripts.tests.test_pi_key_rawtokens_defect1_regen scripts.tests.test_ingest_ability_raw_tokens_pi_screen
scripts.tests.test_generic_ingest_remediation scripts.tests.test_race_trait_remediation` — 60/60 green,
plus `scripts.tests.test_declared_pi_shipping_defect2_regen scripts.tests.test_ingest_ability_pi_rename
scripts.tests.test_pi_redaction scripts.tests.test_site_dashboard_pi_gate
scripts.tests.test_site_public_status_pi_gate` — 82/82 green (1 skip, pre-existing).

## Guarded corpus regen (`§17a`/`AGENTS.md` rule 5/8 — never `--allow-stamp-loss` without fresh reports)

`python3 scripts/ingest_ability.py` re-run over the full 4,824-unit `ability` population (unchanged;
`decisions.md §20` population, re-derived): `written 4824, changed 3, unchanged 4821`. All 3 changed
files un-redacted a false-positive DESC (confirmed clean against the pinned oracle for each: no
`DESCISPI:YES`/`NAMEISPI:YES` declared on the source row, `normalized_term_hit`/
`blacklist_term_hit_including_concatenated` both return `None` on the real description post-fix).
The other two (`favored_son_daughter_belor_hemlock_town_sheriff.json`,
`codex_named_unit_ability_inner_sea_gods_isg_abilities_faith_lst_98.json`) were ingested at
`2026-08-23T14:17:00Z` — BEFORE `decisions.md §26`'s rn->m OCR-fold exemption landed
(`fc06c4e4f1`, committed 2026-08-23T16:40:48Z UTC) — and were simply never re-run since; this regen
is the first re-run of `ingest_ability.py` since that fix, so it also picks up that already-approved
correction as a side effect of the SAME generator over the SAME kind, not new scope.

`corpus_literal_sweep --json-out` (built `CARGO_TARGET_DIR=/home/ubuntu/.cache/codex-targets/sd32-t9-onboarding`,
against the pinned oracle `7f818006e371188e5717fd18d74d18a420747fc6`):
```
corpus-literal-sweep: 46334 records examined of 50615 read, 381495 tokens compared (9 synthesized), 50602 digests checked, 0 findings
corpus-literal-sweep: 2370 tokens exempted under decisions.md §24 redaction across 726 codex_generated_name records
corpus-literal-sweep: CLEAN
```
JSON report: `clean: true, records_examined: 46334, verified: 46267`.

`derived_evaluator_fixture_check --json-out`:
```
derived-evaluator-fixture-check: 1836 unit(s) cleared over 2577 fixture row(s); 0 failed; 0 not ingested
```

`docs/work-inventory.json` regen, guarded (`CORPUS_LITERAL_SWEEP_REPORT`/`DERIVED_FIXTURE_CHECK_REPORT`
set from the two live runs above, no `--allow-stamp-loss`):

Status distribution, before -> after (`python3 -c "... Counter(u['status'] for u in units) ..."`
over the committed `docs/work-inventory.json` before vs. the freshly-regenerated file):

| status | before | after |
|---|---:|---:|
| `not-ingested` | 28,060 | 27,058 |
| `literal-verified` | **6,506** | **6,506** |
| `text-complete` | 4,435 | 5,021 |
| `unknown` | 4,347 | 4,286 |
| `grounded` | 2,724 | 3,224 |
| `fixture-verified` | **1,741** | **1,741** |
| `ingested-magnitude` | 1,612 | 1,612 |
| `deferred-with-reason` | 46 | 46 |
| `not-started` | 19 | 19 |
| **TOTAL** | 49,490 | 49,513 |

**The 8,247 pre-existing `literal-verified`+`fixture-verified` stamps are exactly preserved — verified
by ID set, not just by count** (`python3 -c "... b_stamped={u['id'] for u in before if status in
(...)}; a_stamped={...}; print(len(b_stamped-a_stamped), len(a_stamped-b_stamped))"` -> `0 lost, 0
gained`; the two sets are identical). The other deltas (`not-ingested`/`text-complete`/`unknown`/
`grounded`, net `+23` total units) are ordinary population churn from a fresh, accurate walk — not
this cycle's scope to explain further; no stamp-bearing status lost a single unit.

- **Identifier audit result:** `OK_NO_BUNDLE_TAGS` (own diff, `git diff -- scripts/pi_scrub.py
  scripts/tests/test_pi_scrub.py data/corpus`, scoped per §6 step 2's own instruction to scope to the
  cycle's own diff, not the full `BASE_BRANCH...HEAD` form)
- **Wired-integration audit result:** `OK_NO_TOKENS` (same scope)
- **Acceptance criterion:** `corpus_literal_sweep` reports `clean:true`; `docs/work-inventory.json`
  regen through the guarded path preserves all 8,247 pre-existing `literal-verified`/
  `fixture-verified` stamps (dispatch brief's own stated bar).
- **Corpus SHA:** `7f818006e371188e5717fd18d74d18a420747fc6` (`scripts/pcgen-oracle-pin.env`)
- **Status:** complete
- **Notes:** PI discipline (`§15`): no record in this cycle's scope was judged to carry genuine
  Product Identity requiring a stop — the 3 changed records were confirmed CLEAN (false-positive
  redactions reversed), not PI content redacted or transcribed. Territory: `ability` kind,
  `scripts/pi_scrub.py`, `scripts/ingest_ability.py`, `src/bin/corpus_literal_sweep.rs` — none of
  these are in the sibling lanes' forbidden list (spell/equipment/equipment_modifier/companion/
  monster_ability/bestiary_4/template). Kanban rows 11 and 15 left untouched, `in-progress`, per
  dispatch brief.
- **Discovery forwards:** none opened.
- **Next-cycle plan:** `epic-6-kind-trait`'s own remaining scope (BookSource additions,
  `ingest_generic_kind.py --kind trait` run, picker/reach-gate build) is unblocked but out of this
  cycle's scope — the next `epic-6-kind-trait` cycle picks it up.
