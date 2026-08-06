# Forward-Scope Register — SD-28

This register captures work downstream of SD-28 that either depends on
SD-28's ingest outputs or revisits SD-28's contracts as a future bundle.
SD-28's successor bundles (SD-29 / SD-30) are recorded here as **Class 1**
(predecessor-deferred, in-scope for named successor). Bundles that depend
on SD-28's book-list completion but are not yet named land in **Class 2**
(future-acquired, deferred). SD-28-specific retrofits land in **Class 3**
(if/when operator requests).

## Class 0 — Doctrinal anchors (always-on)

| Anchor | Path | Note |
|--------|------|------|
| Per-book ingest pipeline | `docs/governance/book-ingestion-playbook.md` | Doctrine-of-record per the playbook; pre-cycle trap-report is mandatory |
| Reach gate | `apps/desktop/src-tauri/src/reach_gate.rs` | Definition-of-done per `decisions.md §18`; gate's `OPEN_FINDINGS` carries the APG/ACG equipment-surface prerequisite |
| Identifier discipline | `~/workspace/governance/identifier-discipline.md` | SD-28 inherits; Epic 1 enforces |
| Build-version scheme | `<major>.<tranche-base>.<build>` (2026-07-17 amendment) | SD-28 first concrete value `0.8.<build>` |
| Source STC chassis | `spec-domain-bundle-authoring` skill | 12-file shape |
| Move-not-copy publish | `release-package-promotion` skill | Workspace tree removed on publish commit |

## Class 1 — Predecessor-deferred (named successor owns)

### C1.1 — Bestiary 2-5 cycle pattern

**Owner:** SD-29 (`./../SD-29-bestiary-2-3-4-5-content-ingestion/` — repo-resident canonical home; workspace source-of-record removed on publish per move-not-copy doctrine).

**What depends on SD-28:** SD-29 inherits SD-28's per-book-ingest pipeline shape.
The cycle pattern (per-monster-block, reach-gate, trap-report) is established
by SD-22's Bestiary 1 ingest and refined by SD-28's per-class / per-equipment
cycles. No API changes — SD-29 reads SD-28's pipeline surface as the
documented shape, not as a code dependency.

**Cross-bundle doc:** SD-29's `loop-instruction.md` cites SD-28's
`loop-instruction.md` for the per-cycle base procedure.

### C1.2 — Cross-bundle class overlap (SD-30)

**Owner:** SD-30 (`./../SD-30-occult-and-companion-content-ingestion/` — repo-resident canonical home; workspace source-of-record removed on publish per move-not-copy doctrine).

**What depends on SD-28:** Classes shared between Ultimate Intrigue and
Occult Adventures (Occultist, Spiritualist, Medium, Mesmerist) have their
canonical class definition owned by SD-30 per `decisions.md §5`. SD-28
references the canonical class id from SD-30's progress; SD-28 does not
redefine.

**Cross-book conflict rule applies after both bundles land:** when SD-30's
Occult Adventures definition contradicts SD-28's Ultimate Intrigue
definition on the same class, `decisions.md §16` resolves it (newer book =
doctrine, older book = errata). The class-grant case is the only exception
to §16 (preserved from SD-22's doctrine).

## Class 2 — Future-acquired (deferred)

### C2.1 — Dreamscarred Press corpus expansion

If operator acquires additional Dreamscarred Press books beyond
`ultimate_psionics` (e.g., `psionics_unleashed`, `psionics_expanded` — both
confirmed in the corpus per `decisions.md §17`), a future bundle (or an
in-cycle retrofit) covers their ingest. SD-28 does not lock these in.

### C2.2 — Bulk-modification retrofit

If operator requests a bulk-modification pass across all ingested U-line
records (per `decisions.md §17a` — "bulk modifications deferred"), that
pass is a separate bundle. SD-28 preserves the per-cycle one-record-at-a-
time discipline and does not bulk-edit.

### C2.3 — Post-tranche consumer

The next tranche after `tranche/8` (whatever it becomes — `tranche/9`,
`tranche/9-1`, etc.) is out of scope here; it inherits the post-`develop`
merge of SD-28's closure work.

## Class 3 — Retrofit (operator-on-request)

### C3.1 — UE equipment catalog widening (CLOSED — stale, corrected 2026-08-06 by `epic-14-harness`)

**Stale as written.** `apps/desktop/src-tauri/src/equipment_catalog.rs` no
longer "reads CRB alone" — it was widened to all 6 ingested books
(`BOOK_CRB/APG/ACG/B1/ARG/PU`, one `map_<book>_entry` per book feeding a
book-tagged DTO) in `a92ae066` ("widen equipment catalog from CRB-only to
all 6 ingested books"), refined in SD-27's `d44ea892`. Confirmed by reading
the file directly, not inherited from this entry.

Separately, and more directly relevant to SD28-E14 (which cited this entry
as a dependency before verifying it): the real rules-core consumer,
`equipment_effects::compute_equipment_effects`, was **already book-agnostic**
before any widening work — it resolves against whatever `SourcePackageContent`
it is given, and every per-category resolver reads tokens directly off the
resolved record rather than a CRB-only compiled table. So neither the
desktop catalog nor the rules-core consumer had the CRB-only shape this
entry describes by the time E14 needed to depend on it.

**Operator decision this surfaced is moot.** The widening already shipped as
a retrofit (SD-27/SD-28, prior to this correction) — there is no remaining
"precycle prerequisite outside SD-28 or SD-28-owned retrofit" question to
answer. See `docs/release/SD-28-ultimate-book-content-ingestion/artifacts/e14-harness-widening.md`
for the full correction and its verification.

### C3.2 — Ultimate Psionics third-party tier license retro-fit

The pre-cycle verification per `decisions.md §17` validates licensing at
upsionics ingest start. If the verification surfaces records whose
licensing annotations don't match open-content tier (e.g., a record
annotated `OGL` but matching PSPF PI patterns), the affected records
drop from the per-cycle scope. A retrofit bundle may revisit; SD-28
records the dropped records as cycle findings, not blockers.

## Class 4 — Measured inheritance from tranche/7 (SD-28-specific, derived 2026-08-01)

Three findings from the tranche/7 retrospective that are **about this bundle specifically**, each
derived by command rather than routed by assumption. Sources: `docs/retro/tranche-7-retrospective.md`,
`../SD-29-bestiary-line-book-ingestion/forward-scope-register.md §7`.

### C4.1 — SD-28 is likely LARGER than SD-27, not smaller

The bundle's own doctrine says *"size alone is never a stop reason"* (`decisions.md §24`,
`loop-instruction.md`). That rule is right, and it has never been given the size.

Per-book real (non-comment, non-blank) `.lst` row counts show a **12× spread** across the six books —
`ultimate_magic` ~4,756 rows against `ultimate_campaign` ~397 — and roughly **5,450 `abilities_class`
rows**, which is §24-shaped hand-model content, one pure function and one test each.

**Consequence for planning:** SD-27's per-book cycle shape was sized against ARG (479 records) and PU
(59). Applying that cadence unchanged to `ultimate_magic` will under-provision it by roughly an order
of magnitude. **Re-derive the counts at dispatch** (`v06_work_inventory`, `v06_corpus_trap_report`) and
size cycles per book, not per bundle.

### C4.2 — The 46-spell Unchained Summoner gap is entirely SD-28's to close

SD-27 left the Unchained Summoner unable to cast: its 202-entry spell list has a **46-spell gap**,
per-level 12/35/39/39/27/23/27 with the gap at zero for level 0.

**All 46 come from `ultimate_combat` (26) or `ultimate_magic` (18)** — two of them also in Mythic
Adventures. **None requires a Bestiary-line book**, so this does **not** wait on SD-29 and SD-30 is not
a prerequisite either. The other four Ultimate books are not prerequisites for it.

**Readiness:** whichever cycle lands `ultimate_magic` + `ultimate_combat` spells should close this in
the same pass and re-run SD-27's Unchained Summoner surface, rather than leaving it to a follow-up.

### C4.3 — Settle the magnitude predicate BEFORE publishing any coverage ratio

There is no stable predicate for *"this record carries a computed magnitude"*. Magnitude rows carry no
corpus key, only prose that usually repeats the record's name, so four reasonable variants of the
name-substring test returned **48 / 49 / 51 / 52 on one unchanged tree** during SD-27.

SD-28, SD-29 and SD-30 will each want to publish a "% of records reaching a player" figure. Without a
shared predicate, **all three will publish defensible, non-comparable numbers** — and so will any
comparison against SD-27's.

The fix is small: an optional `source_record` on `ComputationExplanation`, so a magnitude row names the
corpus record it came from instead of being matched by prose. **Whichever bundle dispatches first
should land it** — it is cheaper than the argument the three bundles will otherwise have about their
own numbers.

This is SD-27 `decisions.md §27.1` recurring one layer up: *625 mentions vs 271 settings — the
arithmetic was never the defect, the label was.*

### C4.4 — The ingest pipeline is four binaries with three private copies of one treatment

There is no single ingestion pipeline. `src/bin/ingest_races.rs`, `ingest_race_traits_arg.rs`,
`ingest_pu_classes.rs` and `cache_gen/apg.rs` each carry their own partial copy of the PCGen
description treatment; only `codex::rules_core::pcgen_desc::render_pcgen_desc` is sanctioned. SD-27
paid this defect three times in three places.

**Readiness, and it is cheap if done once before book #1:** route every ingest binary's description
path through `render_pcgen_desc` and give each a `leaked_pcgen_syntax` production guard. Six books ×
four binaries is where SD-28 pays it repeatedly instead.

Ownership is shared with SD-29 (`forward-scope-register.md §7.4`) and SD-30: **whichever bundle
dispatches first pays it; the others re-verify rather than re-implement.**

### C4.5 — The `.MOD` schema question is resolved; SD-28 inherits Ruling B

`../SD-30-occult-and-companion-content-ingestion/decisions.md §29` resolves the `.MOD` schema question
pre-dispatch, and it binds SD-28 because **Ultimate Magic carries 538 `.MOD` spell rows and Ultimate
Combat 159**.

Measured across the whole PCGen tree: **5371 `.MOD` spell rows — 1642 DESC-only, 3729
mechanics-bearing.** The two shapes get different treatment:

- **DESC-only** (Ultimate Combat 144, Ultimate Intrigue 101, Ultimate Wilderness 50) → **§29.2**: an
  additive `variant_descriptions` field on the existing spell payload carrying its own book and page.
  **Not a new record** — a `.MOD` row has no school, level or class list, so minting one puts an
  uncastable row in the catalog (a dead affordance) and double-counts it in every coverage ratio.
- **Mechanics-bearing** (Ultimate Magic 538 — `ITEM` 231, `CLASSES` 163, `DESCRIPTOR` 139; Ultimate
  Combat 15) → **§29.3**: a `CLASSES:`-bearing `.MOD` is a per-class spell level, and the pattern
  already exists — a per-book supplement chained into `rules_core::rules_tables::class_spell_levels`,
  as `advanced_race_guide::class_spell_levels` does (commit `f4dcb522`).

**Sizing consequence, and it cuts SD-28's estimate:** Ultimate Magic's 538 `.MOD` rows are **not** 538
new spell records. They are supplements against spells other books already define. Count new
declarations separately from `.MOD` supplements when sizing cycles — conflating them is the same
measurement-shape error `§C4.3` warns about.

## Class 4 — Epic 12 code-review deferrals (owned, per `decisions.md §26`)

Landed by cycle `SD28-E12-F1-001` (`epic-12-code-review`, 2026-08-02).
Decision 26 requires every `deferred` finding to name an owner — an unowned
deferral is not a valid disposition. Full evidence for each is in
`progress.md` under the cited finding id.

### C4.6 — Re-key the nine ACG Naturalist spell records (`progress.md` F3)

**Severity:** high — it is the sole cause of definition-of-done item 3
(`v06_corpus_trap_report -- --audit` exits 0) being red repo-wide, and all
seven SD-28 book cycles cited it.

**Owner:** the next bundle holding write authority over SD-22 corpus
content. Not SD-29 or SD-30 by default — neither's scope includes
`data/corpus/advanced_class_guide/`; the operator assigns this on return.

**What:** `src/rules_core/cache_gen/acg.rs::generate_spells` writes
`record_key: entry.key` (the display name) for every ACG spell, including
the nine whose declared corpus `KEY:` is `Naturalist Summon Nature's Ally
I..IX` (`acg_spells.lst:785-793`). The generated records therefore claim the
base spell's identity, which already exists as a separate CRB record
(`data/corpus/core_rulebook/spell/level_1/summon_nature_s_ally_i.json`).
Remedy: re-key the nine `acg::spell_list` entries to their declared `KEY:`
and regenerate via `gen_cache_acg`.

**Explicitly not acceptable as a fix:** correcting `record_key` alone while
leaving `data.key` as the base spell's name. That turns the audit green
without fixing the defect.

### C4.7 — Re-dispatch Epics 3-9 with the audit red reclassified (`progress.md` F1/F2)

**Severity:** critical — the bundle's entire stated purpose is undelivered.

**Owner:** the SD-28 supervisor session, before `epic-10-closure` fires.

**What:** all seven book epics recorded `decision-blocked` citing C4.6's
audit failure as a precondition for writing ingest code. It is a closure
condition (DoD item 3), not an entry gate, and its cause is a different,
closed book. Re-dispatch with that reclassification stated in the brief, so
the audit red is carried as a DoD item 6 shortfall rather than as a reason
to write nothing. All seven cycles' step 0/0b/1b shape findings are already
recorded in `progress.md` and are reusable as the starting shape.

### C4.8 — Epic 11's lockfile sweep (`progress.md` F7)

**Severity:** low — a refresh, not a defect.

**Owner:** `epic-10-closure`, at PR-authoring time.

**What:** commit `27dbbdea` moved 79 transitive crate versions and added one
package (`syn`) to `apps/desktop/src-tauri/Cargo.lock` inside a
version-numbering commit. Not reverted (an unreviewed revert at closure time
carries more risk than the sweep). The tranche-promotion PR body must
disclose it rather than let it arrive unannounced.

## Review trigger

Reopen SD-28's forward-scope register when:

- A successor bundle (named or un-named) reaches into the seven books' ingest outputs.
- A new U-line book arrives in the corpus.
- The bulk-modification retrofit is operator-authorized.
- The post-`tranche/8` consumer is operator-named.
- Operator requests Class 3.x retrofits.

Closed-form: the bundle closes when Epic 10 (Closure Epilogue) fires.
