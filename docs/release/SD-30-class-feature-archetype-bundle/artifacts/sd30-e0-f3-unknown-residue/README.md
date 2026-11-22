# SD30-E0-F3 — `unknown`-residue characterization, corpus-wide

**Cycle:** `SD30-E0-F3-001`. **Status:** read-only classification pass, complete for the two kinds
that carry a nonzero `unknown` residue. No corpus data, no engine code, no `data/corpus/` file
touched — the PI gate is not engaged, per the card's own framing.

## 1. Which kinds have a nonzero `unknown` residue — re-derived fresh, not assumed

```
python3 -c "
import json, collections
d = json.load(open('docs/work-inventory.json'))
c = collections.Counter()
for u in d['units']:
    if u['status'] == 'unknown':
        c[u['kind']] += 1
print(c)
"
```
Against `docs/work-inventory.json` `generated_at: 2026-08-14T20:03:13Z` (the current committed
inventory, SD30-E0-F1-001's regen — unchanged by this cycle):

| kind | `unknown` units |
|---|---|
| `class_feature` | 3,622 |
| `feat` | 367 |
| every other kind (`class`, `race`, `race_trait`, `spell`, `equipment`, `equipment_modifier`, `monster`, `monster_ability`, `companion`) | **0** |

**No kind other than `feat` needed this cycle's treatment.** `class_feature` already has a method
and a disposition (`decisions.md #38`, owned by the former Epic 4, moved to `SD-31-corpus-closure-
grind/kanban.md epic-1-measurement` under `decisions.md §51`) — this card's scope is the `feat`
residue specifically, plus checking (not re-executing) whether any *other* kind newly needs the
treatment. None does.

**Correction, both of this bundle's own inherited figures, re-derived not transcribed:**
- `epic-breakdown.md` SD30-E0-F3's own acceptance text claims feat's residue is "329 units". The
  live re-derivation above is **367**, not 329. `retro.py correction` event
  `sd30-e0-f3-feat-residue-count` (see progress.md receipt).
- `decisions.md §38` characterized class_feature's residue at "3,218" (later corpus snapshot, per
  that decision's own text, superseding an earlier 2,958). The live re-derivation above is
  **3,622**. This is **not** re-characterized by this cycle (out of scope — owned by the class_
  feature measurement chain, now at `SD-31`), but the drift is recorded here so `SD-31` inherits the
  correct current count rather than a three-bundles-stale one. `retro.py correction` event
  `sd30-e0-f3-classfeature-residue-drift` (see progress.md receipt).

## 2. Method — `decisions.md #38`'s own three buckets, applied to `feat`

`decisions.md #38` characterized `class_feature`'s `unknown` residue into three buckets:
**option-pool sub-choice content** (mechanism real and wired; the *specific named option*'s own
magnitude is what's uncomputed — not an ingest gap), **genuinely-unreachable** (needs new engine
code, no chooser code at all), and **unclustered remainder** (left open, uncharacterized).

`feat`'s residue has a materially different starting shape than `class_feature`'s did: every one of
the 367 units carries the *same* `evidence` value —
`in_catalog_with_corpus_magnitude_but_no_observed_consumer` — meaning the feat-effect probe
(`probe_feat_effect_wiring`, `src/bin/v06_work_inventory.rs:1574`, sweeping `PROBE_CLASSES = {fighter,
barbarian, monk, wizard, swashbuckler}` × `PROBE_LEVELS = {1, 12}` × 4 generic
`PROBE_SELECTIONS`) found the feat in the engine's catalog with a real corpus magnitude, but never
observed a computed delta on any swept posture. That single evidence shape is not, by itself, a
characterization — the same three buckets apply, translated into what "mechanism real but
unexercised" vs. "structurally can't be exercised by this probe" vs. "no signal" mean for a feat
record specifically. Applied per-unit by reading each unit's own PCGen `.lst` source line (not the
stored `reason` text alone) for a structural signal:

- **Option-pool (mechanism real, specific pool-slot ungrounded)** — the feat record itself is a
  named sub-choice of a parent class-native chooser feat (`KEY` containing PCGen's own `" ~ "`
  sub-choice marker, e.g. `KEY:Angelic Flesh ~ Brazen`), grants a resource-pool slot via
  `BONUS:ABILITYPOOL` (the `Extra <X>` family — Extra Rage Power, Extra Discovery, Extra Hex, 34
  distinct pool names), or carries its own inline `CHOOSE:` the probe's fixed, generic
  `PROBE_SELECTIONS` roster does not exercise. **100 units.**
- **Genuinely-unreachable (needs new probe-fixture capability)** — a positive `PREABILITY` names a
  prior chooser selection (a specific Rage Power/Discovery already picked) the probe's synthetic
  character never makes (194 units), or a `PRESTAT`/`PRESKILL` prerequisite floor the probe's fixed
  per-class stat block was not built to satisfy per-feat (23 units, e.g. `Battle Cry`'s Perform 5
  ranks). Both are a **probe-fixture** gap, not a corpus or engine-mechanism gap — the feat's own
  mechanism may already be wired; the fixture never puts a swept character in the state that would
  exercise it. **217 units.**
- **Unclustered remainder** — no `ABILITYPOOL`/`CHOOSE`/positive-`PREABILITY`/`PRESTAT`/`PRESKILL`
  signal on the record's own `.lst` line. Left open for individual characterization, the same honest
  disposition `decisions.md #38` gave its own 1,772-unit (at that snapshot) unclustered bucket — not
  silently dropped, not guessed at. **50 units.**

**One `!PREABILITY` (negated — "you must NOT already have X") near-miss caught live**: the first
classifier pass matched `PREABILITY` as a substring without checking polarity and wrongly routed
`Amateur Investigator` (`!PREABILITY:1,CATEGORY=Special Ability,Investigator ~ Inspiration`) into the
chooser-pre-selection-gap bucket. `!PREABILITY` is trivially satisfied by the probe's synthetic
characters (none of them are Investigators) and is not the same shape at all. Fixed to a
polarity-aware regex (`(?<!!)PREABILITY:`) before the count above was taken. `retro.py near-miss`
event `sd30-e0-f3-negated-preability` (see progress.md receipt).

**One honest residual flagged, not smoothed over**: 4 of the 68 `resource-pool-expansion` units
(`Extra Rage Power`, `Extra Arcana` ×2, `Extra Cantrips or Orisons`) name an owning class already
*inside* `PROBE_CLASSES` (barbarian, wizard) — for these four specifically, "the fixture doesn't
cover the owning class" is not the actual explanation, since the fixture does sweep that class. Their
true cause (pool-capacity grant with no visible consumer in the swept postures even though the class
is probed) is un-diagnosed by this pass and is called out in the JSON artifact (`sub_reason` for
those four IDs) rather than folded silently into the other 64 units' explanation.

## 3. Result

| top bucket (decisions.md #38 taxonomy) | units | share |
|---|---:|---:|
| genuinely-unreachable (needs new probe-fixture capability) | 217 | 59.1% |
| option-pool (mechanism real, specific pool-slot ungrounded) | 100 | 27.2% |
| unclustered-remainder | 50 | 13.6% |
| **total** | **367** | 100% |

| shape sub-count | units |
|---|---:|
| chooser-pre-selection-gap (positive `PREABILITY`) | 194 |
| resource-pool-expansion (`BONUS:ABILITYPOOL`/`Extra <X>`) | 68 |
| no-structural-signal (unclustered) | 50 |
| prereq-stat-or-skill-gap (`PRESTAT`/`PRESKILL`) | 23 |
| inline-choose (feat's own `CHOOSE:`) | 16 |
| named-sub-choice-key (PCGen `KEY:... ~ ...`) | 16 |

Per-unit detail (id, name, book, source file/line, bucket, shape, sub-reason) for all 367 units is in
`feat_unknown_characterization.json` alongside this file. The classifier script,
`characterize_feat_unknown.py`, is included for reproducibility — it is read-only over
`docs/work-inventory.json` and the PCGen `.lst` source tree under
`~/workspace/repos/pcgen/data/pathfinder/`, writes no file outside its own JSON output, and can be
re-run against a future `work-inventory.json` regen to see how the residue has moved.

## 4. Invocation contract for SD-31 (`decisions.md §51` split)

This card's acceptance criteria are satisfied at SD-30; the disposition of the 217+100+50 units is a
**mechanism/engine-capability question** that is now SD-31's / SD-32's to consume, per the
2026-08-14 split (`decisions.md §51`, this package's SCOPE NOTE):

- The **217-unit genuinely-unreachable bucket** needs a **probe-fixture capability expansion**, not
  new production/ingest code against a book. Concretely: (a) the `chooser-pre-selection-gap` 194
  units need the feat probe fixture (`probe_feat_effect_wiring`) to grow the ability to pre-select a
  representative chooser option (a Rage Power, a Discovery, …) per swept class before checking for a
  feat's computed delta — mirrors the existing per-class-feature "ground one representative option"
  design intent `decisions.md #38` already ratified for `class_feature`, applied to the probe rather
  than the corpus; (b) the 23 `prereq-stat-or-skill-gap` units need either a richer per-feat stat
  floor in the fixture or an honest acknowledgment that some feats structurally require a
  purpose-built character, not a generic swept one. This is engine-capability-shaped work — it
  belongs with `SD-32-engine-capability-builds` under its verdict-path-capability epic, or a
  successor probe-capability card SD-31/SD-32 mints; SD-30 does not claim it.
- The **100-unit option-pool bucket** needs no new ingest — its disposition mirrors `decisions.md
  #38`'s standing ruling for `class_feature`'s dominant shape: ground one representative option per
  chooser family, defer the rest with a named diagnostic, do not attempt per-option computation. No
  further SD-31 action is owed unless the operator wants the option-pool status renamed from
  `unknown` to a status meaning "known, deliberately deferred" (the same open classifier-taxonomy
  question `decisions.md #38` left open for `class_feature`, now also true for `feat`).
- The **50-unit unclustered remainder** is inherited unfinished characterization work, same standing
  as `decisions.md #38`'s own residual bucket — a future cycle (SD-31 scope, since it is
  measurement-shaped, not ingest-shaped) should read each unit's full `.lst` record individually; no
  further pattern was found by this pass's structural signals.

No PI-gate interaction: this pass read `docs/work-inventory.json` and the already-checked-out PCGen
source tree only; no `data/corpus/` write, no declared-PI field read or written.
