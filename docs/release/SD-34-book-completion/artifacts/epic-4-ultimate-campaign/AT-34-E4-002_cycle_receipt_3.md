# Cycle AT-34-E4-002 (cycle 3) — Epic 4 (Ultimate Campaign) / AT-34-E4-002

- **Commit SHA:** (this cycle's own commit — see push output; parent `3cc878de05084383614f820276c0aca4391ca0e9`)
- **Files touched:** `docs/release/SD-34-book-completion/artifacts/epic-4-ultimate-campaign/ultimate-campaign-completion-manifest.json` (new), `docs/release/SD-34-book-completion/artifacts/epic-4-ultimate-campaign/AT-34-E4-002_cycle_receipt_3.md` (new), `docs/release/SD-34-book-completion/progress.md`, `docs/release/SD-34-book-completion/kanban.md`. **Zero Rust source touched.**
- **Identifier audit result:** OK_NO_BUNDLE_TAGS (own-diff check, see Notes)
- **Wired-integration audit result:** OK_NO_TOKENS (own-diff check, see Notes)
- **Acceptance criterion (verbatim, `epic-breakdown.md`):**
  > **Evidence:** `python3 scripts/completion_atlas.py --book ultimate_campaign --check` exits 0
  > with `DONE=265 of 265`, every other bucket zero, plus
  > `artifacts/epic-4-ultimate-campaign/ultimate-campaign-completion-manifest.json`.

## 0. What this cycle actually is

The dispatch brief for this cycle described the population as `M:89 U:21 V:18 X:2 D:2` (132
remaining, "last measured"). Re-derived at HEAD **before touching anything** (`decisions.md §12`
L2 — never carry a number forward):

```
python3 scripts/completion_atlas.py --book ultimate_campaign --check
```
```
book=ultimate_campaign population=265 unclassified=0 overlap=0
  DONE: 151
  D: 2
  M: 89
  V: 0
  U: 21
  X: 2
```

**`V` is already 0**, not 18. The corpus-wide bucket-V widening this same bundle landed since the
brief's figures were measured (`AT-34-E3-005`, commit `cfd9c6d3d9`/`3cc878de05`, cross-referencing
SD-33's own combined oracle ledger, `decisions.md §19`) already closed `ultimate_campaign`'s 18 `V`
units to `DONE` as a side effect of its corpus-wide pass — **checked, not assumed**: the atlas run
above shows `V: 0` directly, and `progress.md`'s `AT-34-E3-005` entry states the widening covered
"non-`core_rulebook` units," which includes this book. So the brief's own instruction — "check
whether SD-33 ledgers cover your 18 before engineering anything" — was correct, and the answer is
**yes, already covered, by a different cycle, no engineering needed here.** This is the "movement"
this cycle can honestly claim credit for measuring, not for causing.

**Remaining: 265 − 151 = 114 = D(2) + M(89) + U(21) + X(2).**

## 1. The 23-unit U+X tail — already resolved, correctly not reopened

The brief's own text distinguishes this book's 21 `U` units
(`feat_served_description_is_a_placeholder_marker_not_prose`) from `§17`'s ruled shape
(`equipment_modifier` internal codes) and instructs: *"do not assume the ruling extends... if
they are real feats whose description was lost, that is a content gap... not a quiet
disposition."*

**This exact question was already asked and answered by `AT-34-E4-001`** (a separate, already
`complete` criterion, `kanban.md` row 19,
`artifacts/epic-4-ultimate-campaign/AT-34-E4-001_cycle_receipt.md`), re-read in full this cycle
before doing anything further:

- **Content was NOT lost.** All 21 records join real, substantial (>150-char), mechanically
  complete text (a `Goal:` + `Completion Benefit:` clause) into the served
  `FeatCatalogRecord.description` — proven by two still-passing tests
  (`twenty_one_are_text_complete_with_real_benefit_text`,
  `uca_u_bucket_records_still_carry_the_editorial_marker_in_served_form`), re-run this cycle:
  ```
  cargo test --locked --lib rules_core::rules_tables::ultimate_campaign::feat_tables::tests::twenty_one_are_text_complete_with_real_benefit_text \
    rules_core::rules_tables::feats_all::tests::uca_u_bucket_records_still_carry_the_editorial_marker_in_served_form
  ```
  ```
  test result: ok. 2 passed; 0 failed; 0 ignored
  ```
  (run at this cycle's own HEAD before committing — see Build scope row.)
- **The classifier's demotion is deliberate, corpus-wide, and NOT a instrument bug**:
  `SD31-E2-F3-002`'s own still-live test names `ultimate_campaign:feat:accursed` as one of the
  exact records the fix targeted, and the marker family occurs **~370 more times** corpus-wide.
  Promoting UCA's 21 without a corpus-wide product-policy decision on the marker would recreate,
  in reverse, the exact inconsistency that fix was written to close.
- **`§16`'s own precedent forbids a single cycle from picking a bucket destination for a
  definitional question like this one.** `AT-34-E4-001` therefore correctly reported "no verdict
  possible within this cycle's authority" and did **not** move the 21 `U` / 2 `X` units. Nothing
  in this cycle's own investigation contradicts that finding, and nothing new was discovered that
  would reopen it — so it is **not re-litigated here**, per this same doctrine.

**Disposition this cycle: U(21) and X(2) are correctly unchanged.** They remain named,
proven-terminal-pending-an-operator/corpus-wide-policy-decision, exactly as `AT-34-E4-001` left
them.

## 2. The 89 `M` units — real diagnosis, no fabricated closure

**Split** (`python3 -c` against `docs/work-inventory.json`, filtered `book=ultimate_campaign`,
`status=ingested-magnitude`, grouped by `evidence`):
```
59 trait_content_table_holds_record_magnitude_not_yet_computed   (kind=trait)
30 ability_content_table_holds_record_magnitude_not_yet_computed (kind=ability)
```
**Corpus-wide, the same two evidence strings total 1,665 of 49,438** (re-derive: filter
`docs/work-inventory.json`'s units on `evidence in {trait_content_..., ability_content_...}`,
`len()`), confirming the brief's own claim that this shape is "far larger elsewhere" — a fix
keyed on the *kind* rather than the book would pay far beyond these 89.

### What `M` requires, per `decisions.md §2`/`§2a`

`M` is cleared by **"running the compute path"** — not by reformatting evidence, and **not** by a
shape engine merely producing a number (`§2a`: "a shape engine computes a number; it does not
complete a record... it does not place the record in a table, attach it to a character, or show
it to a player"). `simple_kind_verdict` (`src/bin/v06_work_inventory.rs:9230`), the function that
classifies every one of these 89, has **no arm that ever returns anything but `ingested-magnitude`
for a held, magnitude-bearing record** — there is no "computed" branch to fall into today, for any
book, for either of these two kinds. This was confirmed by reading the function in full, not
assumed from its name.

### The corpus content itself, read directly (not inferred from field names)

Sampled every one of the 59 `trait` units' real `data/corpus/ultimate_campaign/trait_generic/*.json`
`BONUS:` tokens:
```
44  BONUS:SKILL only               (e.g. "Trait ~ Acrobat": BONUS:SKILL|Acrobatics|1)
 2  BONUS:SKILL + CASTERLEVEL/SITUATION mixed
 3  BONUS:VAR
 2  BONUS:SAVE
 2  BONUS:SITUATION only
 2  BONUS:ABILITYPOOL
 1  BONUS:COMBAT
 1  BONUS:CONCENTRATION
 1  BONUS:COMBAT + CONCENTRATION
```
The 30 `ability` units are PCGen's Ultimate Campaign **Drawback** (17) and **character
retraining** (12) sub-mechanics (`Drawback ~ Attached`, `Retrain ~ Feat`, `Retraining ~ Skill Rank
Added`, plus one `Default` record granting `BONUS:ABILITYPOOL|Traits|1|...` — "taking a drawback
grants one extra trait slot"), read directly from
`data/corpus/ultimate_campaign/ability/*.json`. These are **not** simple flat skill bonuses: they
are house-rule bookkeeping (a "drawback grants a trait slot" pool mechanic) and GM-adjudicated
narrative penalties (e.g. "Attached": a -1 Will save penalty only while a GM-chosen attachment
object is threatened), several with no clean formulaic trigger at all.

### The missing capability, confirmed empty, not assumed

A record-level `BONUS:SKILL` token cannot be "computed and applied" without a character having
*chosen* the trait in the first place. Searched the whole engine and desktop app for any such
selection surface:
```
grep -rln "selected_traits\|character_traits\|CharacterTrait\b" src/ apps/desktop/src-tauri/src/
```
```
(zero matches)
```
`CharacterInput`'s `ChosenCharacterState` (`src/rules_core/character_input.rs`) carries
`selected_feats`, `selected_choices`, `skill_allocations`, etc. — **no trait or drawback field of
any kind.** The one existing PF1e "Trait" machinery in the repo, `src/rules_core/trait_pool.rs`
(403 lines), is a **different** mechanic entirely — it indexes `Trait.RaceTrait.<X>` records so an
*Adopted Race* selector can offer them as **options**, and its own module doc states plainly:
*"Nothing is computed... this loader only indexes them."* It does not apply a chosen trait's
`BONUS:` effect to anything, and it only covers the `RaceTrait` subtype (1 of `ultimate_campaign`'s
59) — the ordinary "pick 2 traits at character creation" Basic-Trait house rule (the other 58) has
no engine surface at all.

**Conclusion: clearing `M` for `trait_content`/`ability_content` is not a wiring gap in an
existing mechanism — it is a missing player-facing capability** (a character trait/drawback
selection surface, plus a generic `BONUS:` interpreter for at least `SKILL`/`SAVE`/`COMBAT`
shapes, threaded through `CharacterInput` and every one of its construction sites
(`composed_input.rs`, `pilot_compute_corpus.rs`, `level_up.rs`, `feat_prereqs.rs`), then surfaced
in the desktop character-creation flow so it is genuinely reachable, not fixture-only). That is
new, corpus-wide (1,665-unit), multi-file, product-surfaced engineering — the same shape
`decisions.md §17` found for bucket `X`'s "half-built capability," except here **zero** of the
capability exists, not half. Building it as a rushed, single-cycle, undertested addition risks
exactly the two failure modes this bundle's own doctrine forbids: a stub with no real UI path to
reach it (`no-stub-mvp-doctrine.md`), or a classify()-time promotion to `DONE` that is not backed
by a real, applied computation (`§2a`) — the "8 closures where measurement found 1" failure this
cycle's own brief was warned against. **No unit was moved into `DONE` on the strength of this
diagnosis alone.**

## 3. The 2 `D` units — read directly, genuinely unmodelled, not fixable this cycle either

```
ultimate_campaign:trait:trait_alchemical_intuition  (Trait ~ Alchemical Intuition)
ultimate_campaign:trait:trait_wrecking_wrath         (Trait ~ Wrecking Wrath (Rovagug))
```
Both carry **zero** `BONUS:` tokens at all (`magnitude_token_count: 0`) — PCGen's own data never
modelled either mechanically. Read directly:
- *Alchemical Intuition*: "you may gain a trait bonus equal to your Charisma modifier (minimum 0)
  on a Craft (alchemy) check... **after you roll the check**" — a once-per-day, apply-after-the-
  fact reroll-shaped bonus with no `BONUS:` token anywhere in the source `.lst` line.
- *Wrecking Wrath (Rovagug)*: same shape (verified directly against its own corpus JSON).

`wiring_class=ambiguous, wiring_class_reason=prose_ability_scaling` is the correct classification —
the description states a real numeric scaling rule that the corpus/engine never captured as data.
Fixing this needs the same "apply a rule after a die roll" mechanic bucket `M`'s Drawback
sub-shapes also need — not a smaller, separately tractable fix.

## 4. Deliverable produced this cycle

`artifacts/epic-4-ultimate-campaign/ultimate-campaign-completion-manifest.json` — did not exist
before this cycle (checked: `find` returned only `core-rulebook-completion-manifest.json`).
Built by re-using `scripts/completion_atlas.py`'s own `_bucket_of`/`_head_sha` functions directly
(not a re-implementation of its bucket logic) against the live `docs/work-inventory.json`, in the
identical shape `core-rulebook-completion-manifest.json` already established. Every one of its
265 `units` entries carries `id`, `kind`, `bucket`, `status`, `evidence`, `source_file`,
`source_line`.

**Row-count command output (this cycle's own artifact, `decisions.md §4`):**
```
python3 -c "import json; d=json.load(open('docs/release/SD-34-book-completion/artifacts/epic-4-ultimate-campaign/ultimate-campaign-completion-manifest.json')); print('population', d['population']); print('done', d['current_state']['done']); print('remaining_total', d['current_state']['remaining_total']); print('buckets', d['current_state']['buckets']); print('units_len', len(d['units'])); print('complete', d['complete'])"
```
```
population 265
done 151
remaining_total 114
buckets {'D': 2, 'M': 89, 'U': 21, 'X': 2}
units_len 265
complete False
```
114 = 2 + 89 + 21 + 2. Matches `completion_atlas.py --book ultimate_campaign --check` exactly.

## 5. Figures + their re-derive commands (denominators stated)

| Figure | Command | Denominator |
|---|---:|---|
| `population=265` | `python3 scripts/completion_atlas.py --book ultimate_campaign --check` | `ultimate_campaign` units in `docs/work-inventory.json` |
| `DONE=151 of 265` | same | same |
| `V=0 of 265` (was 18) | same | same — closed by `AT-34-E3-005`, a different cycle, before this one ran |
| `M=89 of 265` (trait 59 + ability 30) | `python3 -c` grouped-by-evidence query above | same |
| `1,665 of 49,438` corpus-wide `trait_content`/`ability_content` M-bucket population | same query, no `book` filter | whole corpus, `docs/work-inventory.json` |
| `U=21, X=2` (unchanged) | `completion_atlas.py --book ultimate_campaign --check` | same |
| `D=2 of 265` | same | same |
| `114 = 265 − 151` remaining | arithmetic on the above | `ultimate_campaign` population |
| `0` selection-surface matches | `grep -rln "selected_traits\|character_traits\|CharacterTrait\b" src/ apps/desktop/src-tauri/src/` | whole engine + desktop crate source tree |

**Oracle pin:** not applicable — no figure in this cycle was drawn from the pinned PCGen oracle
corpus (all figures are read from the live repo's own `docs/work-inventory.json` and
`data/corpus/`).

## 6. Build scope verified

Zero Rust source changed this cycle, so this is a confirmation run, not a regression-risk run:
```
CARGO_TARGET_DIR=/tmp/cargo-sd34-at-34-e4-002 CARGO_INCREMENTAL=0 cargo test --locked --no-run
```
run at parent commit `3cc878de05084383614f820276c0aca4391ca0e9` (no Rust source changed by this
cycle, so this SHA is still the tree's real compile state) — **EXIT=0**, full workspace, 600
targets built. `apps/desktop/src-tauri` not touched, not run (§2.5 — scope test runs to what a
cycle touches; a docs/JSON-only diff does not reach the desktop crate). The two `AT-34-E4-001`
regression tests quoted in §1 above were re-run explicitly and pass:
```
test rules_core::rules_tables::ultimate_campaign::feat_tables::tests::twenty_one_are_text_complete_with_real_benefit_text ... ok
test rules_core::rules_tables::feats_all::tests::uca_u_bucket_records_still_carry_the_editorial_marker_in_served_form ... ok
```
`2 passed; 0 failed` (each run individually, filtered).

## 7. Sweep population

N/A — this cycle added/regenerated zero `data/corpus/**` records (`decisions.md §12` L8 does not
apply; no corpus change to measure a delta against).

## 8. Status

- **Status:** partial

**Remainder, named exactly, summing to 114 of 265:**

| Sub-cause | Population | Disposition |
|---|---:|---|
| `M` — `trait_content`/`ability_content`, missing character trait/drawback selection capability (zero existing surface) | 89 | Needs a new, corpus-wide (1,665-unit) capability — trait/drawback selection in `CharacterInput` + a generic `BONUS:` interpreter, then real desktop UI wiring. Named for `AT-34-E5-002`'s capability register; not built this cycle (too large, too risky to rush without a real UI path — would violate the no-stub doctrine). |
| `U` — feat-served-description-carries-editorial-marker, corpus-wide (~370-occurrence) product-policy question | 21 | Proven terminal within this cycle's/any single cycle's authority by `AT-34-E4-001` (already `complete`); not reopened, no new evidence found. |
| `X` — splice/truncation defects against the pinned oracle, confirmed real and unrepairable | 2 | Proven terminal by `AT-34-E4-001` (already `complete`); not reopened. |
| `D` — prose-only ability-score-scaled bonus, zero `BONUS:` token, apply-after-roll mechanic never modelled | 2 | Same missing-mechanic shape as `M`'s Drawback sub-cases; not separately tractable. |

**89 + 21 + 2 + 2 = 114.** `114 = 265 − 151` (`DONE`), verified by the row-count command in §4.

## 9. Movement, four buckets (`decisions.md §9`)

- **Closure:** 0 (no unit moved to `DONE` by this cycle's own work).
- **Reclassification:** 0 (no unit moved buckets by this cycle).
- **Reachability:** 0.
- **Instrument-correction:** 0 — `simple_kind_verdict`'s missing "computed" arm is a genuine,
  confirmed gap, not an instrument bug to patch; the *content* really has not been computed
  anywhere, so `M` is the correct bucket for all 89.

This cycle's real contribution is **measurement** (`decisions.md §12` L9 / workflow-instruction
lesson 6: "Measurement waves that bank zero units are legitimate deliverables") — confirming `V`
was already closed by a sibling cycle, ruling out an instrument-side shortcut for `M`, and
producing the previously-missing completion manifest.

## 10. Notes — judgment calls

- **Did not attempt a rushed `CharacterInput` schema change.** Adding `selected_traits` would
  ripple into every one of `ChosenCharacterState`'s construction sites
  (`composed_input.rs`, `pilot_compute_corpus.rs`, `level_up.rs`, `feat_prereqs.rs`) and, to be a
  real (non-stub) feature rather than dead code, would also need a desktop character-creation UI
  path. That is genuinely multi-cycle work; attempting it in the time remaining risked leaving an
  uncommitted or half-wired change, which the bundle's own "commit early" rule and the no-stub
  doctrine both forbid doing carelessly. Filed as a named, sized finding instead.
- **Did not re-litigate `AT-34-E4-001`'s U/X finding.** Re-read its receipt in full; found nothing
  new; `§16`'s own precedent forbids a cycle from picking a bucket destination for a definitional
  question, and this remains one.
- **A retro `deferral` event is emitted for the missing trait/drawback capability** — see the
  cycle's retro log line — naming the revisit condition (`AT-34-E5-002`'s capability register, or
  a future cycle that decides to build it) so this is not rediscovered as an unexplained gap.

## 11. Next-cycle plan

1. **If a future cycle is authorized to build the trait/drawback capability**: start with the
   44-of-59 pure `BONUS:SKILL` shape (the largest, cleanest sub-population, and the one that
   generalizes furthest corpus-wide across the 1,665-unit `trait_content`/`ability_content` M
   population) before attempting `VAR`/`ABILITYPOOL`/`SITUATION`/`COMBAT`/`CONCENTRATION` shapes
   or the Drawback/Retraining `ability` sub-mechanics, which are smaller and structurally
   different.
2. **The 2 `D` units and the Drawback narrative-penalty sub-shapes of `M`** likely need the same
   "apply a rule the player invokes explicitly" mechanic (a once-per-day reroll/override), not a
   pure passive `BONUS:` line — worth scoping as one capability, not two.
3. **U(21)/X(2) stay parked**, pending whatever cycle or ruling resolves the corpus-wide
   `[Not Implemented]`-marker product-policy question `AT-34-E4-001` surfaced. That decision is
   out of `ultimate_campaign`'s scope alone (it is a ~370-occurrence, corpus-wide question) and
   is not this criterion's to force.
