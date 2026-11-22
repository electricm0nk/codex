# Cycle t9-onboarding-row17-remaining-21 — Gate 1 (shape closure) / row 17's remaining 21

- **Card ID:** `epic-7-shape-categorization-100` (kanban.md row 17) — note appended, status left `backlog` (correctly blocked on `no_record == 0`, a sibling lane's ingest work; this cycle made zero `no_record`-closing writes).
- **Commit SHA:** (this cycle's own commit, see push log)
- **Files touched:**
  - `scripts/pi_scrub.py` — `scrub_name_pi_tokens` gained an opt-in `neutral_name` parameter: narrower (self-reference-only) redaction instead of a full wipe, byte-identical default behavior when omitted.
  - `scripts/tests/test_pi_scrub.py` — 4 new tests (narrowing, backward-compat default, blacklist-blocks-narrowing, invariant guard).
  - `scripts/shape_ledger.py` — `classify_unit` emits `f0_reached_by == "measured_pi_redacted"` (a real, measured answer) instead of `fallthrough` for a genuinely PI-redacted formula value; `extract_formula_segment` extended for the `BONUS:SKILL|%LIST` 2-field shorthand (implicit magnitude `1`), scoped narrowly (never a bare `SKILL|<name>` with no magnitude).
  - `scripts/tests/test_shape_ledger.py` — 2 pre-existing tests updated to the new bucket semantics (the old assertions encoded the behavior this cycle deliberately changed, per `§27a`'s own instruction), 4 new tests (classification split, non-PI-fallthrough guard, `%LIST` shorthand, sibling guard against over-widening to a bare `SKILL|<name>`).
  - `scripts/row17_census.py` — reports `measured_pi_redacted` as its own line item, excluded from `row17_honest_size`.
  - `scripts/tests/test_row17_census.py` — fixture extended with a 5th (genuine non-PI fallthrough) unit; 2 pre-existing tests updated to the new split; 1 new mutation-proof test (`measured_pi_redacted` moves, `row17_honest_size` does not).
  - `scripts/regen_row17_pi_over_redaction.py` — now threads each record's own `codex_neutral_name` into `scrub_name_pi_tokens`, so a self-reference-only redaction narrows instead of wiping.
  - `scripts/regen_all_renamed_pi_scrub.py`, `scripts/regen_generic_kind_pi_scrub.py` — **discovery-forward, fixed in-cycle**: both had the SAME DESC-blanking gap `regen_row17_pi_over_redaction.py`'s own module docstring already named as latent in its two siblings (re-deriving `raw_tokens` via `scrub_name_pi_tokens` alone, omitting `remediate()`'s DESC-blanking/blacklist-scan steps first — would ship un-redacted `DESCISPI:YES` prose whenever it doesn't restate the record's own name). Fixed by importing and calling the SAME `regen_row17_pi_over_redaction.redact_tokens`, not a fourth divergent copy. **Neither script was RUN against the live corpus** (both `--dry-run`-verified clean; their populations are sibling-lane territory per this cycle's own scope note — `equipment`/`ability`/`deity`/`monster`/... — only the code fix + tests land here).
  - `scripts/tests/test_regen_scripts_desc_blanking_parity.py` — new (4 tests: DESC-blanking behavior, mutation-proof reproducing the old leak against the OLD two-step shape, two import-identity checks so a future regression to a local copy fails at import time).
  - 6 files under `data/corpus/**` (regenerated `data.raw_tokens` in place via the narrower redaction — same path/slug/key/name/description; `data.description` unchanged in all 6, none had DESC-declared PI):
    - `data/corpus/adventurers_guide/class_feature/fighter/codex_named_unit_class_feature_adventurers_guide_ag_abilities_class_lst_846.json`
    - `data/corpus/adventurers_guide/class_feature/shaman/codex_named_unit_class_feature_adventurers_guide_ag_abilities_class_lst_1041.json`
    - `data/corpus/adventurers_guide/feat_generic/codex_named_unit_feat_adventurers_guide_ag_feats_lst_37.json`
    - `data/corpus/inner_sea_gods/trait_generic/codex_named_unit_trait_inner_sea_gods_isg_abilities_lst_145.json`
    - `data/corpus/inner_sea_magic/class_feature/bard/codex_named_unit_class_feature_inner_sea_magic_ism_abilities_class_lst_160.json`
    - `data/corpus/inner_sea_world_guide/feat_generic/codex_named_unit_feat_inner_sea_world_guide_iswg_feats_lst_30.json`
  - `docs/release/SD-32-compute-library-and-cause-closure/kanban.md` (row 17 note appended)
  - `docs/release/SD-32-compute-library-and-cause-closure/artifacts/gate-1-shape-closure/row17-census.json` (re-derived output, `--check` exit 0)
  - `docs/retro/events/t9-onboarding.jsonl` (verify.sh preflight-oracle events, auto-appended)
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS` (zero hits, scoped to this cycle's own diff).
- **Wired-integration audit result:** scoped diff shows 9 hits of the bare word "placeholder" — all inside comments/docstrings describing the F0/row-17 CONCEPT ("a genuinely-derived answer... not a placeholder", "row 17's placeholder population"), the same domain vocabulary `decisions.md §27a` itself uses repeatedly and that `shape_ledger.py` already carried 4 pre-existing occurrences of before this cycle (`git show HEAD:scripts/shape_ledger.py | grep -c placeholder` → 4). Self-healed by inspection per §6 step 4 — no stub/mock/no-op/would-string in shipping code.
- **Acceptance criterion:** Dispatch brief §1/§2/§3 — re-derive row 17's remaining 21 (`row17_census.py`), decide each of the 20 PI-redacted-formula units (narrow where safe, correctly reclassify where genuinely PI), extend the classifier for the named 21st unit's `BONUS:SKILL|%LIST` shorthand after confirming semantics against the pinned oracle and checking corpus-wide reuse, and fix the named latent DESC-blanking gap in `regen_all_renamed_pi_scrub.py` (then check other regen scripts for the same shape).
- **Corpus SHA:** PCGen oracle bootstrapped this cycle (fresh worktree, empty git-ignored slot): `7f818006e371188e5717fd18d74d18a420747fc6`.
- **Status:** complete
- **Notes:**

## Re-derivation (`§17a`)

Inherited brief figures (`no_record` 106, row 17 honest size 21 = 20 PI-redacted + 1 named gap)
re-derived and CONFIRMED via `python3 scripts/row17_census.py`, and via
`python3 -c "..."` enumerating the 21 `f0_reached_by=="fallthrough"` ledger rows by coordinate
(id/kind/book/source_file/source_line/corpus_key) before touching anything — see the ids listed
in step 1 below. All 21 confirmed present and correctly attributed before any fix.

## Step 1 — the 20 PI-redacted-formula units

Split confirmed by direct inspection against the pinned oracle, per record:

- **7 self-reference-only** (own name/key segment appears verbatim in the value; the record's
  own name/key is itself PI, `NAMEISPI:YES` in the oracle, already `§24`-neutral-named):
  `armiger_hellknight_order`, `name_keeper_pathfinders_past`, `gray_maiden_initiate`,
  `inner_sea_gods:ability:norgorber_aspect`, `inner_sea_gods:trait:trait_arcane_depth`,
  `bard_archetype_chelish_diva`, `inner_sea_world_guide:feat:hermean_blood`,
  `inner_sea_world_guide:ability:shoanti` (8, not 7 — the brief's inherited 7/13 split was off by
  one; re-derived and corrected here, `§17a`).
- **13 blacklist-term-driven** (a real `PI_BLACKLIST_TERMS` entry — `Aldori`/`Qadira`/`Razmiran`/
  `Lamashtu`/`Varisia` — concatenated into a `DEFINE`/`BONUS` variable identifier or substring):
  the `aldori_*` (5), `qadiran_horselord_*` (2 + the `default` MOD row referencing
  `MASTERVAR("QadiranHorselordMountBonusSpeed")`), `razmiran_priest_false_piety`,
  `teratoma_lamashtu`, `tattooed_sorcerer_varisian_tattoo`, `inner_sea_world_guide:feat:varisian_tattoo`,
  `inner_sea_world_guide:feat:aldori_dueling_mastery` (13).

**Decision, applied generically via `pi_scrub.py::scrub_name_pi_tokens`'s new `neutral_name`
parameter, not per-record:** a value whose ONLY hit is the self-reference case (never a blacklist
hit) is narrowed — the matched span replaced with the record's own `§24` coordinate-derived
neutral name — preserving the surrounding mechanical structure (magnitude, `TYPE=`, `PRECLASS=`,
sub-`ABILITY` references). A value that also hits the blacklist is never narrowed (that PI is a
different entity's identity — a deity/nation/organization name — not just this record's own, and
cannot be safely substituted without inventing a cross-record-consistent renamed identifier, out
of this cycle's scope).

**Applied via `regen_row17_pi_over_redaction.py --dry-run` then (unchanged) real run:**
`row17_pi_redacted_fallthrough_population: 20, changed: 6, unchanged: 5, not_codex_generated_skipped: 9`.
6 of the 8 self-reference-only records had a narrowable value (2 — `norgorber_aspect`,
`shoanti` — carried NO separately-redacted formula token beyond `NAMEISPI`/name fields, so nothing
to narrow); all 3 records that ALSO carry a blacklist hit alongside self-reference
(`tattooed_sorcerer_varisian_tattoo`, `inner_sea_world_guide:feat:aldori_dueling_mastery`,
`inner_sea_world_guide:feat:varisian_tattoo`) stayed correctly, fully redacted, as designed.

**Result: 6 units recovered real formula content and moved OUT of F0 into a real family**
(`derived` 11402 → 11408, confirmed via `row17_census.py` before/after) — a genuine closure per
`§16`, not a relabeling. Sample diff (full mechanical structure preserved, only the PI span
replaced):

```diff
-        "value": "[redacted PI]"
+        "value": "ABILITYPOOL|Codex-Named Unit (class_feature_adventurers_guide_ag_abilities_class_lst_846)|1|TYPE=Base"
```

**Arithmetic: 20 PI-redacted formula units − 6 narrowed (moved to a real family) = 14 remain
fully redacted** (2 self-reference-only records with nothing separately narrowable beyond
name/key fields, plus the 13 blacklist-driven records, which are never narrowed by design — see
above). These 14 genuinely still carry PI and correctly stay redacted. Reclassified:
`shape_ledger.py::classify_unit` now emits `f0_reached_by == "measured_pi_redacted"` (not
`fallthrough`) for any F0 unit whose formula tokens are ALL the PI-redaction marker — per `§27a`,
*"if the value genuinely carries PI, it stays redacted — but then it is not a fallthrough
placeholder, it is a correctly-measured redacted value."* `row17_census.py` now reports
`measured_pi_redacted: 14` as its own line, excluded from `row17_honest_size` the same way
`measured_empty` already is.

**No re-leak, before vs. after (`§12c`):**

| | `pi_key_rawtokens_audit.py` (scanned/confirmed/candidate) | `declared_pi_shipping_audit` |
|---|---|---|
| Before this cycle's writes | 27,538 / **0** / 25,540 | 65 (all pre-existing `bestiary_4/monster_ability`) |
| After this cycle's writes | 27,538 / **0** / 25,540 | 65 (unchanged; zero in any of the 6 touched files) |

Additionally diff-scanned the 6 changed files' added lines against all 61 `PI_BLACKLIST_TERMS`
and each record's own original PI name string ("Hellknight", "Pathfinders Past", "Gray Maiden",
"Arcane Depth", "Nethys", "Chelish Diva", "Hermean") — zero matches. `git status --porcelain`
confirmed zero changes under `data/corpus/**` during both audit runs.

## Step 2 — the 21st unit: `ultimate_campaign:trait:trait_harvester`

**Semantics confirmed against the pinned oracle**, not guessed: `uca_abilities_traits.lst:198`
carries `BONUS:SKILL|%LIST` with no third (magnitude) field at all — genuinely, byte-for-byte, in
the pinned source. The record's own `ASPECT:SkillBonus|+1 trait bonus on Profession (tanner) or
Profession (trapper) checks...` and matching `DESC` text state the omitted magnitude is a flat
+1 — Pathfinder's universal trait skill-bonus convention, read from the record's own declared
text, never invented.

**Corpus-wide reuse checked before extending the classifier** (`§17`: a rule serving one record
is per-object work): scanned every `.lst` file in the pinned oracle for the exact bare, tab-
terminated `BONUS:SKILL|%LIST` shape. Result: **exactly 1 occurrence anywhere in the whole
corpus** — this record. (The general `BONUS:SKILL|%LIST|<magnitude>|<type>` 3-4-field shape is
common — 70+ occurrences — and already correctly classified by the pre-existing `parts[2]`
extraction; only the bare 2-field omission is unique.) The fix is implemented as a GENERIC
extraction rule in `shape_ledger.py::extract_formula_segment` (any future `BONUS:SKILL|%LIST`
2-field token gets the same treatment), not a per-record special case, even though today's
population is 1 — satisfying `§17`'s "generic pass, not per-object work" by construction rather
than by current population size.

**Scoped precisely** to avoid over-widening: a bare `SKILL|<real skill name>` (2 fields, 2nd
field NOT `%LIST`) still returns `None` — the genuine parse-failure case
`test_bonus_too_short_returns_none` already covered — proven by a new sibling guard test.

**Result: the 21st unit closes for real** — `trait_harvester` now classifies as family `F1`
(flat-constant magnitude, formula `"1"`), confirmed via `row17_census.py`
(`fallthrough: 1 → 0` after this fix alone, before the classification-bucket fix above).

## Step 3 — the discovery-forward: `regen_all_renamed_pi_scrub.py`'s latent DESC-blanking gap

Confirmed present, exactly as flagged: `regen_all_renamed_pi_scrub.py` re-derived
`data.raw_tokens` via `scrub_name_pi_tokens` alone (`tokens = row_tokens(raw_line); scrubbed,
extra = scrub_name_pi_tokens(tokens, orig_name, orig_key)`), never touching `data.description`
and never running the declared-PI DESC-blanking/blacklist-scan steps first — the identical shape
`regen_row17_pi_over_redaction.py`'s own module docstring records catching in its own first draft.

**Checked whether any OTHER regen script has the same gap, per the brief's own instruction ("this
is the fourth... so assume there are more")**: `ls scripts/regen_*.py` → 3 scripts total. The
third, `regen_generic_kind_pi_scrub.py`, was found to have the IDENTICAL gap (same two-step
shape, same missing DESC-blanking step). Both fixed.

**Fix:** both scripts now import and call `regen_row17_pi_over_redaction.redact_tokens` — the
ONE canonical, already-mutation-proved pipeline (declared-PI detection → DESC blanking →
blacklist scan → identity/blacklist scan) — rather than each carrying its own divergent
two-step re-implementation. This closes the exact duplication-drift shape `decisions.md §17`
names, the same way `pi_scrub.py`'s extraction already unified three drifted PI-scan copies.
Both scripts now also thread the record's own `§24` neutral name for the narrower-redaction
benefit from Step 1.

**Test added** (`scripts/tests/test_regen_scripts_desc_blanking_parity.py`, 4 tests):
1. `redact_tokens` blanks a declared-PI `DESC` whose prose does not match any identity/blacklist
   needle (the exact leak shape), while leaving an unrelated mechanical `BONUS` value untouched.
2. Mutation-proof reproducing the OLD two-step shape directly (`row_tokens` +
   `scrub_name_pi_tokens` alone, no DESC pre-blanking) against the SAME synthetic fixture,
   confirming it really does leak the full DESC prose — proving the fix's extra steps are
   load-bearing, not decorative.
3/4. Import-identity checks (`assertIs`) that both sibling scripts' `redact_tokens` name is the
   SAME function object `regen_row17_pi_over_redaction` defines — a future regression back to a
   local copy fails at import/test time, not by re-discovering the leak via a corpus-wide audit.

**Neither corpus-wide script was RUN against the live corpus in this cycle.**
`regen_all_renamed_pi_scrub.py --dry-run` (856-record `codex_generated_name: true` population,
spanning `equipment`/`ability`/`deity`/`monster`/`spell`/... across dozens of books) and
`regen_generic_kind_pi_scrub.py --dry-run` (`{race,monster,class,race_trait}_generic/`, 46
records) both ran clean with zero errors, confirming the fix imports and executes correctly —
but their populations are explicitly named sibling-lane territory in this cycle's own dispatch
brief (`equipment_modifier`/`equipment`/`ability`/`monster_ability`). Applying either corpus-wide
is out of this cycle's granted write scope; `git status --porcelain` confirms zero corpus writes
from either `--dry-run` invocation.

## Verification summary

- `python3 -m unittest scripts.tests.test_pi_scrub scripts.tests.test_shape_ledger
  scripts.tests.test_row17_census scripts.tests.test_regen_scripts_desc_blanking_parity
  scripts.tests.test_shape_provisional_marker scripts.tests.test_pi_key_rawtokens_audit
  scripts.tests.test_pi_key_rawtokens_defect1_regen scripts.tests.test_declared_pi_shipping_defect2_regen
  scripts.tests.test_generic_ingest_remediation scripts.tests.test_ingest_generic_kind
  scripts.tests.test_ingest_ability_pi_rename scripts.tests.test_ingest_ability_raw_tokens_pi_screen
  scripts.tests.test_sd32_t9_pi_normalization_and_inheritance scripts.tests.test_sd32_companion_allowlist_widening`
  — **173 tests, all green** (post-refactor total from the earlier combined run; the newly-added
  `test_regen_scripts_desc_blanking_parity.py` and the row17/shape-ledger updates bring the
  touched-suite total to 89 directly-authored/modified tests across `test_pi_scrub.py` (17),
  `test_shape_ledger.py` (62), `test_row17_census.py` (6), `test_regen_scripts_desc_blanking_parity.py` (4)).
- RED→GREEN mutation proofs, all confirmed by actually reverting a temporary code mutation and
  re-running (never asserted from memory): `pi_scrub.py`'s narrowing branch (full-file mutation
  of the `if` guard to `False and ...`, confirmed RED, reverted, confirmed GREEN);
  `shape_ledger.py`/`row17_census.py`'s classification split (both pre-existing and new tests
  exercise the RED shape directly via `assertEqual`/mutation of the fixture's on-disk record,
  per `test_pi_redacted_mutation_moves_measured_pi_redacted_not_honest_size`); the `%LIST`
  extraction rule (new test failed with `None != '1'` before the fix, passed after);
  `regen_scripts_desc_blanking_parity`'s own reproduction of the OLD leak shape.
- `python3 scripts/row17_census.py --check` → exit 0 (no malformed provisional-default markers).
- `bash scripts/verify.sh --only preflight-oracle` → PASS (oracle bootstrapped fresh this cycle,
  a fresh worktree's git-ignored slot, pin `7f818006e371188e5717fd18d74d18a420747fc6`).
- **Row 17 honest size: 21 → 0.** `not_ingested` (106) unchanged — sibling-lane territory,
  outside this cycle's write scope, confirmed via `git status --porcelain` throughout (never
  touched `monster_ability`/`equipment_modifier`/`equipment`/`ability` beyond the 6 named
  `class_feature`/`feat`/`trait`/`ability` files this cycle's own narrowing fix legitimately
  touched — all 6 are `adventurers_guide`/`inner_sea_gods`/`inner_sea_magic`/`inner_sea_world_guide`
  units this cycle's dispatch brief explicitly names as its own scope, not the sibling
  `monster_ability`/`equipment_modifier` territory).
- **Discovery forward, closed in-cycle:** the DESC-blanking gap named as latent in
  `regen_all_renamed_pi_scrub.py` is fixed and tested; the SAME gap, also found in
  `regen_generic_kind_pi_scrub.py`, is fixed and tested too — the fifth "one path screens,
  another doesn't" instance in this bundle is now the fifth one closed, not deferred.
- `df -h /` at cycle end: reported in the final turn summary.

## Discovery forwards

None new. The two latent DESC-blanking gaps this cycle's brief named/implied were both found
and fixed in this cycle, not forwarded.

## Next-cycle plan

Row 17 (`epic-7-shape-categorization-100`) stays `backlog`: it cannot genuinely start —
i.e., begin the full "categorize every provisionally-defaulted or F0-fallthrough shape" pass —
while `no_record` (106, `monster_ability` 78 / `equipment_modifier` 19 / `equipment` 8 /
`ability` 1) is nonzero. That is entirely sibling-lane ingest work per this cycle's own
territory note. Once `no_record` reaches zero, row 17's own honest-size instrument now correctly
reports 0 pre-existing placeholder units — the sibling lanes' newly-ingested units will need
their own fresh `row17_census.py` run to measure whatever NEW fallthrough/provisional population
their ingestion introduces, if any.
