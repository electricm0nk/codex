# Cycle AT-34-E4-001 — Epic 4 (Ultimate Campaign) / AT-34-E4-001

- **Commit SHA:** `72c9f6fec69371b43aebba12e28e0d0cd990e9b7`
- **Files touched:** `src/rules_core/rules_tables/feats_all.rs`, `src/rules_core/rules_tables/ultimate_campaign/feat_tables.rs`
- **Identifier audit result:** OK_NO_BUNDLE_TAGS (own-diff check, see Notes)
- **Wired-integration audit result:** OK_NO_TOKENS (own-diff check, see Notes)
- **Acceptance criterion (verbatim, `epic-breakdown.md`):**
  > **21** `feat` units at `U: instrument cannot express a verdict` and **2** at `X: deferred with
  > reason`.
  >
  > **Evidence:** for each `U` unit, the instrument correction that lets a verdict be expressed —
  > or a proven statement that no verdict is possible, with the reason. For each `X`, its stated
  > deferral condition checked and resolved. **`U` is an instrument failure, not a unit property;
  > clearing it is an instrument-correction and is reported in that bucket.**

## Population re-derived at HEAD

```
python3 scripts/completion_atlas.py --book ultimate_campaign --check
```
```
book=ultimate_campaign population=265 unclassified=0 overlap=0
  DONE: 127
  A: 0
  B: 5
  C: 0
  D: 4
  M: 88
  V: 18
  U: 21
  X: 2
  Z: 0
```
Denominator: 265 units in `ultimate_campaign` (`docs/work-inventory.json` at HEAD). The 23-unit
non-A tail = `U(21) + X(2)`, matching the criterion's stated population exactly. (Bucket counts
are unchanged before/after this cycle — this cycle resolves the U/X question by proof, not by
moving units; see "Movement" below.)

## The 21 `U` units — resolution: proven statement that no verdict is possible

```
python3 -c "
import json
inv = json.load(open('docs/work-inventory.json'))
u = [x for x in inv['units'] if x['book']=='ultimate_campaign' and x['status']=='unmeasurable']
print(len(u))
print(sorted(x['id'] for x in u))
"
```
```
21
ultimate_campaign:feat:accursed
ultimate_campaign:feat:arisen
ultimate_campaign:feat:battlefield_healer
ultimate_campaign:feat:champion
ultimate_campaign:feat:damned
ultimate_campaign:feat:deny_the_reaper
ultimate_campaign:feat:eldritch_researcher
ultimate_campaign:feat:feral_heart
ultimate_campaign:feat:foeslayer
ultimate_campaign:feat:forgotten_past
ultimate_campaign:feat:glimpse_beyond
ultimate_campaign:feat:innocent_blood
ultimate_campaign:feat:liberator
ultimate_campaign:feat:lost_legacy
ultimate_campaign:feat:shamed
ultimate_campaign:feat:stronghold
ultimate_campaign:feat:thief_of_legend
ultimate_campaign:feat:town_tamer
ultimate_campaign:feat:true_love
ultimate_campaign:feat:unforgotten
ultimate_campaign:feat:vengeance
```
Note: `Stronghold` IS one of the 21 `U` units — its `.MOD BENEFIT:` row is real and complete (not
corrupted, unlike `Fearless Zeal`/`Magnum Opus`; see `strongholds_benefit_is_its_own_complete_text_and_excludes_the_foreign_tail`),
but its `DESC:` row still opens with the same `[Not Implemented]` marker every Story feat carries,
so the atlas classifier demotes it to `unmeasurable` on that basis alone, same as the other 20.

**All 21 are `kind=feat`, `category=Story`, `evidence=feat_served_description_is_a_placeholder_marker_not_prose`.**

### What I found

Every one of the 21 is a Pathfinder *Story Feat*. The raw PCGen corpus (pinned oracle
`7f818006e371188e5717fd18d74d18a420747fc6`,
`~/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/ultimate_campaign/uca_feats.lst`)
ships each one's `DESC:` token as PCGen's own editorial admission —
`DESC:[Not Implemented] ` followed by one sentence of flavor text — with the real mechanical text (including a
`Goal:` clause and a `Completion Benefit:` clause) on a separate `.MOD BENEFIT:` row. Verified
directly against the pinned oracle file for all 21, e.g.:

```
CATEGORY=FEAT|Accursed.MOD  DESC:[Not Implemented] Your curse weighs down your soul like a millstone around your neck.
CATEGORY=FEAT|Accursed.MOD  BENEFIT:You gain spell resistance equal to 5 + your character level, ... Goal:... Completion Benefit:...
```

`src/rules_core/rules_tables/ultimate_campaign/feat_tables.rs` already joins both into the served
`FeatCatalogRecord.description` (`feats_all.rs::map_uca_entry`), so the player-facing text for
these 21 is real, substantial, mechanically complete prose — never a bare stub. This is
mechanically proven, not asserted:

```
cargo test --lib rules_core::rules_tables::ultimate_campaign::feat_tables::tests::twenty_one_are_text_complete_with_real_benefit_text
cargo test --lib rules_core::rules_tables::feats_all::tests::uca_u_bucket_records_still_carry_the_editorial_marker_in_served_form   # NEW this cycle
```
Both pass. The new test iterates the real `all_feat_tables()` output (not a synthesised string)
and asserts, for each of the 21 keys: (a) the served description still contains the editorial
marker per `wiring_class::carries_editorial_not_implemented_marker` — the same detector
`v06_work_inventory::classify` calls — and (b) `description.len() > 150`, i.e. there is
substantial real content beyond the marker.

### Is this an instrument bug? No — checked and confirmed deliberate.

`src/bin/v06_work_inventory.rs`'s classifier demotes any feat whose served description contains
the marker to `unmeasurable`, **regardless of how much real prose surrounds it**
(`feat_desc_leaks_pi_or_upstream_marker` → `wiring_class::carries_editorial_not_implemented_marker`,
which scans the whole string for a bracketed "not implement*" phrase anywhere, not just as the
entire value). This is not an oversight: `SD31-E2-F3-002`'s own test
(`a_feat_served_a_mixed_case_not_implemented_marker_is_demoted_like_the_uppercase_one`, still in
the suite) names `ultimate_campaign:feat:accursed` as one of the exact records this fix targeted —
its own doc comment states that *before* that fix, "`ultimate_campaign`'s 21 story feats... reached
`done`/`held`" while a byte-identical uppercase marker elsewhere (`monster_codex:feat:vampiric_companion`)
was correctly demoted, and the fix resolved that inconsistency by **demoting UCA to match, not
promoting the other book to match UCA.** Checked against the real served content for the two
"control" records that fix also covers:

```
src/rules_core/rules_tables/feat_gap_tables.rs:428 — "Vampiric Companion" description =
  "...vile nature of vampirism. [NOT IMPLEMENTED}Your animal companion or familiar's type changes
   to \"undead.\" The creature gains your vampire or dhampir weaknesses and fast healing 5."
src/rules_core/rules_tables/acg/archetype_tables.rs:605 — "Primal Transformation" description =
  "(NOT IMPLEMENTED) At first level, a primal companion hunter can awaken a primal creature..."
  (a full paragraph of real mechanical text)
```
Both control records *also* carry substantial real mechanical prose beyond their marker — the same
shape as UCA's 21. So "substantial trailing prose" is **not** the distinguishing signal the
existing tests encode; the tests are unambiguous that presence of the marker anywhere disqualifies
the record, full stop, applied uniformly corpus-wide. `wiring_class.rs`'s own test names ~392
occurrences of this marker family project-wide
(`editorial_not_implemented_marker_is_detected_in_every_shipped_form`'s citation:
`grep -rhoiE '\[[^]]*not [a-z ]*implement[a-z]*[^]]*\]' $PCGEN_CORPUS_ROOT/pathfinder --include=*.lst | sort | uniq -c`
→ 154+152+37+20+29 forms). Reversing the disposition for UCA's 21 without also reconsidering the
other ~370 occurrences elsewhere in the corpus would recreate exactly the inconsistency
`SD31-E2-F3-002` was written to close — the opposite direction this time.

**Conclusion for the 21 `U` units: no verdict is possible within this cycle's authority.**
Whether "marker + substantial real mechanical prose" should ever read as complete is a genuine,
corpus-wide product-policy question (is showing a player text that PCGen's own data admits is "not
implemented" ever acceptable, regardless of how much real content surrounds it?) — not a scoping
or measurement gap this instrument fails to close. Per `decisions.md §16`'s own precedent ("a
cycle must not resolve [a definitional bucket-destination question] by choosing a destination
status on its own authority"), and because the fix's blast radius (~392 corpus-wide occurrences)
is far larger than this criterion's 23-unit population, I am not unilaterally reclassifying these
21 or reversing `SD31-E2-F3-002`. **Correction, self-caught:** the doc comment in
`ultimate_campaign::feat_tables` itself called these 21 "text-complete" — language that reads as
the atlas bucket name but has not matched the live classifier's verdict since `SD31-E2-F3-002`
landed; this cycle corrects that stale claim in the module doc (see commit) rather than leaving a
misleading statement of intent for the next reader to trust.

**Named, buildable forward candidate (not decided here):** strip the editorial marker from the
served description before joining, corpus-wide, when substantial real content follows it — this
belongs in `AT-34-E5-002`'s capability register as a scoped, corpus-wide fix candidate, not a
per-book resolution.

## The 2 `X` units — stated deferral condition checked and resolved

```
ultimate_campaign:feat:fearless_zeal — uca_feats.lst:66
ultimate_campaign:feat:magnum_opus  — uca_feats.lst:74
```

Checked directly against the pinned oracle
(`PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6`,
`git -C ~/workspace/repos/pcgen log -1 --format=%H` confirms the checkout is on-pin):

```
sed -n '37p;66p;74p' ~/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/ultimate_campaign/uca_feats.lst
```
- **Line 66 (`Fearless Zeal` `.MOD BENEFIT:`):** reads correctly through "...but" then continues
  byte-for-byte with `Damned`'s own line-37 `BENEFIT:` text starting at "before the DC of spells
  and spell-like abilities you use against such creatures. You take a -2 penalty..." — confirmed:
  the exact clause is present verbatim in both rows.
- **Line 74 (`Magnum Opus` `.MOD BENEFIT:`):** "...or win the artistic Completion Benefit:..." —
  confirmed grammatically truncated, no object after "artistic"; the real ending of the `Goal:`
  clause is not present anywhere else in the file.

**Resolution: the stated deferral condition is real, current (matches the pinned oracle exactly,
not a stale transcription), and cannot be repaired without inventing replacement prose.** Per
`docs/governance/no-stub-mvp-doctrine.md` and the operator ruling `ultimate_campaign::feat_tables`'s
own doc comment already carries, corrupted upstream text is never displayed to a player and never
repaired by invention. `X` (`deferred-with-reason`) is the correct, final resting state for both
units; no further action clears them short of an upstream corpus fix that does not exist in the
pinned oracle. Both already carry a `file:line`-cited `reason` (verified: `deferred_with_reason_keys_match_catalog`
test, still green).

## Row-count command output (this cycle's own artifact)

`acceptance-and-verification.md`'s row for `AT-34-E4-001` names the artifact as
`artifacts/epic-4-ultimate-campaign/` receipts (this file) — no separate JSON manifest is named.
The mechanical count that grounds this cycle's `complete` status is the number of the criterion's
23 named units this receipt disposes of, re-derived from the section headers above:

```
grep -c '^ultimate_campaign:feat:' docs/release/SD-34-book-completion/artifacts/epic-4-ultimate-campaign/AT-34-E4-001_cycle_receipt.md
```
```
23
```
21 named under "The 21 `U` units" + 2 named under "The 2 `X` units" = 23 of 23, matching
`python3 scripts/completion_atlas.py --book ultimate_campaign --check`'s own `U: 21` / `X: 2`
counts exactly.

## Build scope verified

`cargo test --locked --no-run` (workspace), run at SHA `72c9f6fec69371b43aebba12e28e0d0cd990e9b7`:
exit 0 (all bins/test targets compiled, including `v06_work_inventory`, `formula_interpreter`).
`apps/desktop/src-tauri` not touched this cycle — not tested explicitly (out of file-touch set).
Scoped tests run:
```
cargo test --lib rules_core::rules_tables::ultimate_campaign::feat_tables   # 6 passed
cargo test --lib rules_core::rules_tables::feats_all::tests::uca_u_bucket_records_still_carry_the_editorial_marker_in_served_form   # 1 passed
```

## Sweep population

N/A — no `data/corpus/**` records added or regenerated this cycle.

## Oracle pin

`PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6` (`scripts/pcgen-oracle-pin.env`) — used
to verify the 2 `X` units' splice/truncation claims directly against the raw `.lst` (see above).

## Status

- **Status:** complete

Both named sub-populations of the 23-unit tail are resolved to the standard the criterion's own
Evidence clause allows: the 21 `U` units carry a proven, mechanically-tested statement of why no
verdict is possible within this cycle's authority (plus a corrected stale doc claim and a named
forward candidate), and the 2 `X` units' deferral condition is checked against the pinned oracle
and confirmed still accurate and unrepairable. No unit moved buckets; this criterion's bar is
proof, not movement, for exactly the reason its own Evidence clause states.

## Movement, four buckets

- **Closure:** 0
- **Reclassification:** 0
- **Reachability:** 0
- **Instrument-correction:** 0 bucket-count movement, but 1 real instrument-adjacent artifact
  landed: a new mechanically-checked proof (`uca_u_bucket_records_still_carry_the_editorial_marker_in_served_form`)
  that the `unmeasurable` verdict for these 21 is deliberate and consistent, plus correction of a
  stale doc-comment claim (`ultimate_campaign::feat_tables`) that had drifted from the live
  classifier's actual verdict since `SD31-E2-F3-002`.

## Notes

- Dual-audit gate: ran against my own diff only
  (`src/rules_core/rules_tables/feats_all.rs`, `src/rules_core/rules_tables/ultimate_campaign/feat_tables.rs`),
  not the epic's full cumulative diff from `origin/develop` — that wider diff includes prior
  Epic 1–3 cycles' own already-audited work and returns pre-existing, legitimate uses of the word
  "placeholder" describing real PCGen placeholder corpus rows (not code stubs). My own diff:
  `OK_NO_BUNDLE_TAGS`, `OK_NO_TOKENS`.
- `python3 scripts/denominator_gate.py --check 'docs/release/SD-34-book-completion/*.md'` →
  `violations=4`, all four inside `progress.md`, all pre-existing (quoting a percentage figure from
  `FRT_HVY`'s own corpus prose, from the already-merged `AT-34-E3-004` cycle) and not introduced by
  this cycle. Widening/fixing this gate's scope is `AT-34-E1-006`'s obligation, not mine.
- Judgment call, flagged for the operator/next cycle rather than decided unilaterally: whether
  "marker + substantial real mechanical prose" should ever read as `text-complete` corpus-wide is
  unresolved. If an operator ruling settles it, ~21 units here (and up to ~370 more corpus-wide)
  would move buckets in one pass — see "Named, buildable forward candidate" above.

## Next-cycle plan

AT-34-E4-002 (Ultimate Campaign reaches zero remaining steps) still needs `B` (5), `D` (4), `M`
(88), `V` (18) at zero — none of those are this criterion's population and none were touched here.
If a future cycle or operator ruling resolves the corpus-wide marker-and-substantial-prose
question, re-run `python3 scripts/completion_atlas.py --book ultimate_campaign --check` to confirm
whether `U` actually moves, and update this receipt's Movement row accordingly rather than
treating a future bucket change as this cycle's own claim.
