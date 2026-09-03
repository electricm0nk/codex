# Cycle — wave 36, lane B (mine bucket D, `mine-bucket-d` kanban row 37) — Shape 2's 25-unit zero-magnitude sub-split, `DESC:`/`universal_sheet_modifier` cross-reference — complete, zero promotable

- **Commit SHA:** `7c5261836c` (primary content — analysis, `progress.md`, `kanban.md`), with
  two small trailing docs-only commits on this same worktree branch:
  `101c6f5438` (`completion-atlas.json` freshness stamp) and this file's own final polish
  commit (base-branch hazard + audit-window notes, committed immediately after this line was
  written). Full range: `4379c9be05..HEAD` on `worktree-wf_2d4a97a1-eaf-2`.
- **Files touched:**
  - `docs/release/SD-34-book-completion/artifacts/bucket-d-mining/wave36_laneB_shape2_zero_magnitude_subsplit_cycle_receipt.md` (new, this file)
  - `docs/release/SD-34-book-completion/progress.md` (prepended entry, one merge conflict resolved — see Notes)
  - `docs/release/SD-34-book-completion/kanban.md` (row 37 `mine-bucket-d` Notes cell appended)
  - `docs/release/SD-34-book-completion/artifacts/epic-1-atlas/completion-atlas.json` (trailing `derived_at` freshness stamp, from the mandated `completion_atlas.py --check` re-run — no bucket count changed)
- **Identifier audit result:** OK_NO_BUNDLE_TAGS for this cycle's own diff (`git diff --unified=0
  4379c9be05...HEAD -- <the 4 files above>`, this cycle's true starting point — zero matches).
  The protocol's own wider window (`git merge-base HEAD origin/develop` = `ea2b3396f2`, spanning
  every wave since) finds 48 matches, all pre-existing `sd13_`/`sd18_`/`sd27_`-shaped
  cross-bundle test-name citations from wave 33's own baseline-count receipt (already landed,
  not introduced by this cycle) — confirmed by diffing this cycle's own two commits in
  isolation, which come back clean.
- **Wired-integration audit result:** OK_NO_TOKENS for this cycle's own diff (isolated window,
  zero matches). The wide protocol window finds 19 matches, all the literal word
  `placeholder` inside legitimate, already-landed prose from earlier waves (e.g.
  `vacuous_placeholder_reason`, "3 PCGen 'no selection' CHOOSE-menu placeholder rows" —
  real, named, non-stub code/corpus concepts, not a stub marker) — none introduced by this
  cycle's own two commits.
- **Acceptance criterion:** No `AT-34-E#` card exists yet for this investigation (kanban row 37,
  `mine-bucket-d`, tracked under Epic 3, status `partial` since wave 32). This cycle's own
  assignment (wave 35 lane C's next-cycle plan item 2, `wave35_laneC_reconnaissance_cycle_receipt.md`
  §"Next-cycle plan", item 2): run wave 32's own proven `DESC:`-token /
  `universal_sheet_modifier` cross-reference method against the 17 `display`-wiring-class,
  zero-magnitude units inside Shape 2's 25-unit sub-split.

## What this cycle did

Docs-only reconnaissance/mining cycle, no code or corpus touched, matching wave 35 lane C's own
precedent shape. Read the assignment section in full
(`wave35_laneC_reconnaissance_cycle_receipt.md` lines 297–315, "A cheap sub-split exists inside
these 179...") and wave 32's own proven method
(`wave32_laneC_reconnaissance_cycle_receipt.md` lines 95–124, the 1,727-unit shape's own
`DESC:`/`wiring_class`/`universal_sheet_modifier` cross-reference).

**Re-derived the population fresh, first.** This cycle's worktree initially rebased onto
`origin/tranche/14`, which turned out to be **stale** (pinned at wave 33's `7ea9651b87`, three
commits behind the real, unpushed-to-origin local `tranche/14` branch tip at wave 35's own
`4379c9be05`) — caught before committing (no figure below had actually gone stale, since Shape 2
was unchanged either way, but the base itself was wrong). Re-rebased onto the local `tranche/14`
ref directly; all figures in this receipt are derived at that corrected tip, `4379c9be05`
(logged in `progress.md`'s own entry for this cycle, "Note on this cycle's own base-branch
hazard").

```
python3 scripts/completion_atlas.py --check
# population=49438 buckets=10 unclassified=0 overlap=0
#   D: 2891   (down from wave 32/35's own 2955 — wave 33 landed real closures in the interim;
#              re-confirmed as a real, expected population shift, not a defect — see Figures)
```

Shape 2 (`class_feature_no_dedicated_magnitude_id_matched_the_record_slug`) itself is
**unchanged at 179** (wave 33's closures came from other named D mechanisms, not this one —
re-derived below), so the 25-unit zero-magnitude sub-split and its 17-unit `display`-wiring-class
subset are exactly as wave 35 lane C named them.

```python
import json
d = json.load(open('docs/work-inventory.json'))
units = d['units']
shape2 = [u for u in units if u.get('evidence') ==
          'class_feature_no_dedicated_magnitude_id_matched_the_record_slug']
zero_mag = [u for u in shape2 if u['magnitude_token_count'] == 0]
display17 = [u for u in zero_mag if u['wiring_class'] == 'display']
print(len(shape2), len(zero_mag), len(display17))
# 179 25 17 — all three figures reproduce wave 35 lane C's own count exactly
```

**Cross-referenced all 17 against real corpus `DESC:` tokens**, following wave 32's own method
exactly, but joining on `(source.path, source.line)` against the real `data/corpus/<book>/
class_feature/**/*.json` records (glob, then filter to the exact `source_file`/`source_line`
pair each unit's `docs/work-inventory.json` row already carries) rather than trusting a
directory-name guess — every one of the 17 resolved to exactly one real corpus JSON file:

```python
import json, glob
for u in display17:
    pattern = f"data/corpus/{u['book']}/{u['kind']}/**/*.json"
    for m in glob.glob(pattern, recursive=True):
        rec = json.load(open(m))
        src = rec.get('source', {})
        if src.get('path', '').endswith(u['source_file']) and src.get('line') == u['source_line']:
            data = rec.get('data', {})
            raw = {t['key']: t['value'] for t in data.get('raw_tokens', [])}
            has_real_desc = bool(raw.get('DESC')) or bool(data.get('description')) \
                or bool(raw.get('ASPECT')) or bool(raw.get('SPROP')) or bool(raw.get('BENEFIT'))
            print(u['id'], m, has_real_desc)
            break
```

**Result: 0 of 17 carry a real description under any of the three sources
`has_real_description` actually checks** (`src/bin/v06_work_inventory.rs:15972-15979`:
`closure_has_real_description` — a `DESC:` token anywhere in the record's own `.MOD` closure;
`corpus_json_has_real_description` — a `.COPY=`-inherited description recovered at ingest time,
read from `facts.corpus_json_descriptions`; `closure_has_real_aspect_description` — a real-prose
`ASPECT:` tooltip segment). Checked directly against each of the 17 real corpus JSON records'
own `data.raw_tokens` (no `DESC`/`ASPECT`/`SPROP`/`BENEFIT` token on any of them) and each
record's own resolved `data.description` field (`null` on all 17, confirming no `.COPY`
inheritance recovered one either) — every source the gate checks, not just the raw `.lst`
token, comes back empty on all 17.

Two structural shapes explain why, both traced by direct read of the matched corpus JSON:

1. **7 "Class Skills" header records** (Kineticist/Medium/Mesmerist/Occultist/Psychic/
   Spiritualist class skills, Unchained Rogue/Unchained Summoner skills, Vigilante class
   skills, Shifter class skills) carry only a `CSKILL:` token (a machine-readable skill-list,
   not player-facing prose) and no `DESC:` at all — there is no prose to promote; the record is
   correctly a mechanical list, not missing text.
2. **10 internal-chassis records** (5 Magus records — Spell Combat, Improved/Greater Spell
   Combat, Spell Recall/Improved Spell Recall — plus Unchained Rogue Evasion and Shifter
   Timeless Body) carry `VISIBLE:NO` and/or an `ABILITY:`/`SERVESAS:` token pointing at a
   SIBLING corpus record that holds the real prose (confirmed for one: `Magus ~ Spell Combat`'s
   `ABILITY:` token names `Spell Combat Output`, a separate, real corpus record at
   `data/corpus/ultimate_magic/class_feature/spell_combat/spell_combat.json` that DOES carry a
   full `DESC:` paragraph). These are deliberately-empty pointer/chassis rows, not
   description-less content gaps — the real text lives on a different corpus_key the engine
   would need to be told to follow, which `has_real_description`'s three sources (by design)
   do not do: they check the unit's OWN closure/coordinate, never a sibling record reached via
   an `ABILITY:`/`SERVESAS:` link.

Since `has_real_description` is the FIRST of the promotion gate's three conjuncts
(`is_display_wiring_class_for_promotion(wc_class) && has_real_description &&
!universal_sheet_modifier`, `v06_work_inventory.rs:10180-10183`) and it fails on all 17, the
chain terminates there — checking `universal_sheet_modifier` on these 17 would not change the
outcome (a unit already excluded by the first conjunct cannot be rescued by the third), so it
was not run as a separate step (unlike wave 32's 1,727-shape, where 3 units DID pass
`has_real_description` and needed the `universal_sheet_modifier` check to settle their fate).

**This is the same honest outcome wave 35 lane C flagged as one of two possibilities** ("this
could promote a subset to text-complete with zero magnitude-id work — or it could find zero
promotable, the same as wave 32's own result on the larger shape"): **zero promotable**. The
"text-only, zero-magnitude, real-description-shown = COMPLETE" ruling has now been verified
mined to its floor on BOTH known zero-magnitude D shapes (1,727-unit and this 25-unit one).

**The remaining 8 of the 25** (4 `derived`, 4 `ambiguous` wiring class) were not individually
re-checked this cycle — they fail `is_display_wiring_class_for_promotion` categorically
(the gate requires `wc_class == "display"` exactly, `v06_work_inventory.rs:10105-10115`), so no
`DESC:`/`universal_sheet_modifier` cross-reference could promote them regardless of their own
description state. Named here for completeness, not re-derived in depth (out of this cycle's
scope, which the brief named as the 17 `display`-class units specifically).

## Movement (four buckets, this cycle)

- **Closure (bucket → DONE):** 0.
- **Reclassification (bucket → different non-DONE bucket):** 0. No unit's `evidence` or
  `status` changed — `docs/work-inventory.json` untouched this cycle (docs-only diff, confirmed
  by `git status --porcelain` before commit showing only the 3 files named above).
- **Reachability:** 0 units newly reached or lost reachability.
- **Instrument-correction:** 1 — resolves wave 35 lane C's own flagged-but-unverified item
  ("not yet run to completion") to a definitive, verified negative result: 0 of 17 promotable.
  This removes the open question from the next wave's queue (no future lane needs to re-run
  this same 17-unit check) but moves no bucket population. Not a correction of a wrong prior
  claim — wave 35 lane C explicitly declined to claim an outcome and named this cycle's own
  check as the way to resolve it; no `scripts/retro.py correction` event was logged because
  nothing false was asserted and corrected, only an open question was closed.

## Figures (every number, its command, its denominator)

- `population=49438`, `D: 2891` — `python3 scripts/completion_atlas.py --check`, of the full
  corpus. `D` dropped from wave 32/35's `2955` to `2891` (a real, expected `-64` shift from
  wave 33's landed closures on OTHER named D mechanisms between wave 35 and this cycle — not
  a defect in this cycle's own read; re-derivable from wave 33's own three receipts under
  `artifacts/bucket-d-mining/wave33_lane{A,B,C}_*`, none of which touched Shape 2).
- `179` (`class_feature_no_dedicated_magnitude_id_matched_the_record_slug`) — same file,
  exact-string match on `evidence`, of `D: 2891`. Unchanged from wave 32/35 — confirms wave
  33's closures came from other D mechanisms, not this one.
- `25` of `179` are `magnitude_token_count == 0` — same file, field filter, of the 179-unit
  shape. Unchanged from wave 35.
- `17` of `25` are `wiring_class == "display"` — same file, field filter, of the 25-unit
  sub-split. Unchanged from wave 35.
- `0` of `17` carry a real description under any of `has_real_description`'s three sources
  (`DESC:` token, `.COPY`-inherited description, real-prose `ASPECT:` tooltip) — this cycle's
  own direct cross-reference against the 17 matched `data/corpus/<book>/class_feature/**/*.json`
  records (script reproduced in full above), of the 17-unit `display`-class sub-split. This is
  the cycle's own headline finding.
- `7` "Class Skills"-shaped records (no `DESC`, `CSKILL:` only) + `10` internal-chassis records
  (`VISIBLE:NO` and/or `ABILITY:`/`SERVESAS:` pointer to a sibling record) `= 17` — same
  cross-reference, sub-classified by raw-token shape, of the 17.

## Verification

- `python3 scripts/completion_atlas.py --check` → `population=49438 buckets=10 unclassified=0
  overlap=0`, `done_evidence_violations=0`, `missing_clearing_mechanisms=0`,
  `stale_derived_at=False`, `citation_failures=0`, run both before this cycle's own commit and
  confirmed unchanged after (no code/corpus/`docs/work-inventory.json` touched, so identical
  output is guaranteed, not merely observed).
- No `src/`, `Cargo.toml`, `data/corpus/`, or `docs/work-inventory.json` changes this cycle
  (docs-only diff) — a `cargo build`/`cargo test` re-run is not warranted by this cycle's own
  diff and was not run, consistent with the wave's three-lane cargo-concurrency safety ceiling
  (`.cargo/config.toml` `jobs=6`; no cargo process started in this lane at all).
- `git status --porcelain` clean before every write this cycle, in this cycle's own isolated
  worktree; no `git add -A`; `git diff --cached --numstat` read before committing.
- Identifier and wired-integration audits (`workflow-instruction.md §6` step 2) run against
  this cycle's own diff, scoped to the 4 files above, both windows (this cycle's own two
  commits, and the protocol's full `merge-base HEAD origin/develop` window): `OK_NO_BUNDLE_TAGS`,
  `OK_NO_TOKENS` (see the two audit-result rows above for the wide window's own pre-existing,
  not-introduced-here matches).

## Build scope verified

N/A — docs-only cycle, no `src/`, `Cargo.toml`, or `apps/desktop/src-tauri` file touched. Per
`workflow-instruction.md §6` step 3, a `cargo test --locked --no-run` re-run is warranted only
when the diff can move a figure a test asserts; this cycle's diff cannot (no code, no corpus,
no `docs/work-inventory.json` change), so it was not run — the same disposition wave 32 and
wave 35 lane C both recorded for their own docs-only cycles.

## Sweep population

N/A — no corpus record added or regenerated this cycle.

## Oracle pin

N/A — no figure in this receipt was derived from the PCGen oracle corpus; all figures come from
`docs/work-inventory.json` and the committed `data/corpus/` tree already in this worktree.

## Status

complete

## Notes

- **Base-branch hazard caught mid-cycle.** This worktree's first rebase targeted
  `origin/tranche/14`, per `workflow-instruction.md §5`'s own protocol — but that ref turned out
  to be 3 commits **stale** (pinned at wave 33's `7ea9651b87`), three commits behind the real
  local `tranche/14` branch tip (wave 35's own `4379c9be05`, unpushed to origin). No figure in
  this receipt had actually gone stale as a result (Shape 2's own population is unchanged either
  way), but the base itself was wrong, and a later wave working from a genuinely-changed shape
  would have shipped a stale figure without knowing it. Re-rebased onto the local `tranche/14`
  ref directly; this required resolving one real merge conflict in `progress.md` (both this
  cycle's own prepend and the wave-35-wave-end-gate entry wanted the same insertion point —
  resolved by placing this cycle's entry above wave 35's, newest-first, per the file's own
  convention). Logged in `progress.md`'s own entry for the next lane: check both `origin/*` and
  local refs before trusting "fresh", since `workflow-instruction.md §5` names only
  `origin/tranche/14`.
- The brief's own description of where Shape 2's evidence is emitted in
  `v06_work_inventory.rs` ("owner is found... but no record-specific magnitude id matched")
  did not match this cycle's own read of the code at the cited line range (12335-12337 is
  inside the `let Some(owner) = ... else { ... }` fallback's `else` arm — the "owner NOT found"
  path, terminating in `engine_does_not_hold("class_feature_no_dedicated_magnitude_id_matched_
  the_record_slug")`). This did not block or change this cycle's own result: the classification
  used throughout is the ACTUAL `evidence` string recorded live in `docs/work-inventory.json`
  (ground truth regardless of which code branch produces it), not a re-derivation from the
  prose description of the code path. Flagged here as a small prose-precision item for the next
  wave to correct in the standing docs, not re-derived further this cycle (out of scope — this
  cycle's assignment was the DESC/`universal_sheet_modifier` cross-reference, not a code-path
  audit).
- Population went 2955→2891 between wave 35 and this cycle (wave 33's closures). Re-confirmed
  none of that delta came from Shape 2's own 179 (unchanged), so this cycle's own sub-split
  figures are not stale relative to wave 35's.

## Next-cycle plan

1. **Shape 2's remaining 154 magnitude-bearing units, 36 owning classes** — real per-feature
   magnitude-id matching, Epic 3 scope (wave 32/35's own next-cycle item, still open; largest
   groups: Mesmerist 11, Magus 10, Medium/Unchained Monk/Vigilante 8 each).
2. **The 8 remaining zero-magnitude units in the 25-unit sub-split** (4 `derived`, 4
   `ambiguous`) — not checked this cycle (categorically excluded from the promotion gate by
   `wiring_class`, not by description state); worth a brief note in the standing docs that they
   are excluded for a DIFFERENT reason than the 17, but no promotion path exists for them
   either without a `wiring_class` change, which is out of this investigation's scope.
3. **Sub-mechanism 1 from wave 35's own 931-unit split (19 units, `psychic_warrior`
   18 + `rogue` 1)** — still the cheapest named item across both shapes per wave 35's own
   ordering, not picked up by this cycle (assigned scope was the 25-unit sub-split only).
4. Update the standing "6 named D mechanisms" table (wherever it is next restated in a brief or
   receipt) to note Shape 2's 25-unit zero-magnitude sub-split is now FULLY mined (0
   promotable, both the 17 `display`-class and, categorically, the 8 non-`display`-class units)
   — no future lane needs to re-run this check.
