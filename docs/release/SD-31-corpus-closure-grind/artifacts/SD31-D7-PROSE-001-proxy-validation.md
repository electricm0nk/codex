# SD31-D7-PROSE-001 — Proxy validation (Decision 7's PROXY WARNING)

**Cycle:** `SD31-D7-PROSE-001` (`RETRO_ACTOR=sd31-prose-path`), primary checkout, `tranche/11`.
**Purpose:** discharge Decision 7's own PROXY WARNING before banking any unit on it — hand-verify a
stratified sample of `magnitude_token_count == 0` units against their **whole** corpus row, held to
the same standard Epic 2's ground-truth sample was held to (real quoted tokens, never boilerplate).

## The draw

Extended `scripts/sample_ground_truth_units.py` (TDD: 4 new tests, `scripts/tests/
test_sample_ground_truth_units.py`) with a `--zero-magnitude-only` flag that restricts the candidate
pool to `magnitude_token_count == 0` before stratifying by `(wiring_class, kind)` — the exact
population the proxy decides, not the whole board.

```
python3 scripts/sample_ground_truth_units.py \
  --inventory docs/work-inventory.json \
  --current-cell-counts /tmp/empty-cell-counts.json \
  --target-per-cell 4 --zero-magnitude-only --seed 31 \
  --out docs/release/SD-31-corpus-closure-grind/artifacts/SD31-D7-PROSE-001-proxy-sample-draw.json
```

**121 units drawn, 36 cells, all 5 wiring classes (`ambiguous`/`computed`/`derived`/`display`/
`static`), 10 kinds** (`class`, `class_feature`, `companion`, `equipment`, `equipment_modifier`,
`feat`, `monster_ability`, `race`, `race_trait`, `spell`) — exceeds the card's ≥120-unit / ≥6-kind /
all-5-wiring-class bar. Committed at `SD31-D7-PROSE-001-proxy-sample-draw.json`.

## The evidence extraction

For each drawn unit, pulled the **whole** corpus record — the real `data/corpus/<book>/<kind>/**/
*.json` file when one exists (joined by `(source.path basename, source.line)`, searched across every
book directory since shared-library records are routinely filed under a different book than the one
that ingested them), and the raw `.lst` line plus every same-KEY `.MOD` closure row from the pinned
PCGen oracle checkout (`$PCGEN_CORPUS_ROOT`) when no corpus JSON exists (**76 of 121** — CRB feats,
several classes, and some equipment_modifier records are served from compiled Rust tables with no
`data/corpus` dump at all; a real, disclosed methodology limitation, not a skipped step). Committed at
`SD31-D7-PROSE-001-proxy-sample-evidence.json`.

## Hand review — method and result

Every one of the 121 records was read in full (description, every `raw_tokens` field, or the raw
`.lst` row plus its `.MOD` closure) and judged by hand: does this record's real corpus content
genuinely state a magnitude a player would need computed (a dice formula, a scaling phrase, a fixed
numeric bonus/penalty/duration/distance stated as a real game value), or does it genuinely carry
nothing to compute? An automated regex pre-screen (dice notation, `per level`/`per Hit Die`, `DC \d`,
named bonuses, numeric durations/distances, `X/day`) was run first as a cross-check, not a substitute
— it flagged 41 of 121, and every flagged record was independently read and confirmed genuine on
inspection. The regex screen also **missed real cases** the hand read caught (`pufferfish_spines`'s
"1 point of piercing damage", `strength_damage`'s "1d6 points of Strength damage" — both display-
class) — logged below as the regex screen's own known blind spot, not silently corrected away.

### Result 1 — the two proxies, by population

| proxy | population it decides | what it answers |
|---|---:|---|
| `magnitude_token_count == 0` alone | 16,812 units | Decision 7's literal condition 1/2 test, unfiltered |
| `text_only` (= above AND `!carries_prose_magnitude`, the actual code-path predicate) | subset of the above | the same test, narrowed by `wiring_class::determine_closure`'s own prose-formula scan |

**`magnitude_token_count == 0` alone is unsafe on its own** — confirmed directly, and worse than the
regex pre-screen alone suggested. Re-derived from the committed, hand-labelled evidence file
(`python3 -c "import json,collections; r=json.load(open('...-evidence.json')); print(collections.Counter(x['hand_genuinely_zero_magnitude'] for x in r))"`
→ `{'false': 57, 'inconclusive': 35, 'true': 29}`): of the 121 sampled units, **57 (47%)** carry real,
hand-confirmed magnitude language despite `magnitude_token_count == 0` — dice formulas, scaling
phrases, DC formulas, uses-per-day, resource-point thresholds, flat numeric bonuses. (The 41-unit
figure the automated regex pre-screen reported on its own run is a *subset* of this — the hand read
found 16 further real-magnitude records the regex missed entirely, on top of the 2 it flags below as
its own disclosed blind spot; the regex screen is a cross-check, never the authority.)

**48 of those 57 already carry a non-`display` `wiring_class`** (21 `ambiguous`, 20 `derived`, 5
`static`, 2 `computed` — re-derived the same way, filtering to `hand_genuinely_zero_magnitude ==
'false'`) — i.e. `wiring_class::determine_closure`'s own prose scan already catches the large majority
of real-magnitude records in this sample before `magnitude_token_count` would ever be consulted alone.
This is the proxy validation's central finding: **`magnitude_token_count == 0` is not itself the gate
any code path uses — `wiring_class == 'display'` (computed by the SEPARATE, more thorough closure
scan) is the real, additional filter every `text-complete` branch in `v06_work_inventory.rs` already
requires**, and it is doing real, confirmed work in this sample. It is not perfect: the remaining
**9 of 57** real-magnitude records were classified `display` anyway — detailed as Failure mode A
below, and none of the 9 is a kind or a path this cycle's own shipped changes touch.

### Result 2 — within `wiring_class: display` specifically (the population my new rung and the
existing Feat/Equipment/Spell rungs actually touch)

40 of the 121 draws are `display`-class. All 40 were read in full. Of the **28 with unambiguous
evidence** (12 were `.COPY`/`.MOD` stub rows or class-declaration rows whose real content sits in a
closure row this sample's single-line capture did not reach — see Limitation 1 below, reported as
`inconclusive`, never guessed):

- **19 genuinely carry nothing to compute** — real prose confirms condition 1/2 (e.g.
  `ultimate_wilderness:companion:peafowl_drift`: "flies in short bursts, and can't use its fly speed
  to hover" — a rule, no magnitude; `core_essentials:race_trait:wyvaran_languages`: a bonus-language
  list, no number).
- **9 carry a real numeric value the `display` classification missed** — a genuine failure mode,
  detailed below.

**Failure mode A — `SPROP:`/`BENEFIT:` fields carrying a flat (non-scaling) numeric value.**
`advanced_players_guide:equipment_modifier:special_ability_corrosive_weapon`'s only content is
`SPROP:+1d6 acid damage` — a real combat-rules dice bonus, invisible to `magnitude_token_count`
(`SPROP:` is not in `MAGNITUDE_TOKENS`) and not flagged by `wiring_class::determine_closure`'s prose
scan either (that scan targets *scaling* phrases — "per level" etc — not flat constants).
**This record was already `status: text-complete` / `done` on the live board before this cycle** —
a confirmed, pre-existing, live instance of exactly the failure Decision 7's PROXY WARNING predicted.
Same shape: `mythic_adventures:feat:mythic_feat_output_elemental_fist`'s `BENEFIT:` field states
"extra energy damage... increases to 1d8 points, and you gain %1 additional uses... per day";
`monster_codex:feat:coordinated_reposition`'s `BENEFIT:` states "you can move 5 feet as an immediate
action"; `bestiary_4:monster_ability:colossus_pinning_stomp` states "deals an amount of damage equal
to twice that of its slam attack"; `bestiary_2:monster_ability:devilfish_water_dependency` states
fixed "1 hour"/"2 hours" durations; `core_rulebook:companion:strength_damage` states "1d6 points of
Strength damage"; `ultimate_wilderness:companion:pufferfish_spines` states "1 point of piercing
damage"; `core_rulebook:equipment:crossbow_heavy` states a situational "-4 penalty on attack rolls".

**Open interpretive question, NOT resolved here and NOT mine to resolve unilaterally:** Decision 7
says "nothing to compute." Every one of these 9 states a *flat* value (a die roll, a fixed bonus, a
fixed duration/distance) — none states a *character-specific scaling formula* (no "per level", no
stat-dependent term). `wiring_class::determine_closure`'s own design (its `prose_scaling_phrases`
list) already treats "scales with a formula" as the bar for "something to compute", distinct from "a
flat number is printed in the text" — which is arguably still just prose reaching the player (the
number IS the prose). Whether Decision 7's "nothing to compute" means (a) no numeric value appears at
all, or (b) no *character-specific* computation is owed, is a real ambiguity this sample surfaces but
does not answer, and resolving it would relax or tighten a bar an operator ruling set — out of this
cycle's authority and out of `wiring_class.rs`'s file territory (owned by a sibling this wave).
**Logged as `OPEN-ISSUES.md` row (see below), not decided.**

**Failure mode B — the regex pre-screen's own blind spot.** `pufferfish_spines`'s "1 point of
piercing damage" and `strength_damage`'s "1d6 points of Strength damage" were caught only by the hand
read, not the automated screen (the screen's dice pattern requires `\d+d\d+`, missing the bare "1
point" phrasing, and its patterns are all English-language, not exhaustive). Reported rather than
quietly fixed after the fact, per the standing "validate the proxy where it makes its confident claim"
rule — an 8th-in-one-session instrument-failure candidate if silently trusted.

### Result 3 — what this means for what shipped this cycle

**Nothing shipped this cycle is exposed to Failure mode A or B.** The new `race_trait` "text-complete"
rung and the description-completeness fix (below) both operate **strictly on top of** the existing
`wiring_class == 'display'` gate — neither loosens or bypasses it, and neither reclassifies a unit's
wiring class. Failure modes A/B are pre-existing (the `corrosive_weapon` record was `done` before this
cycle) and are reported as a genuine open finding for a future `wiring_class.rs`-owning cycle, not
retroactively fixed here (out of file territory) and not left silently absorbed into this report's
headline (`OPEN-ISSUES.md` row logged below).

### Limitation 1 — single-line capture on the `.lst`-fallback records

12 of the 40 `display`-class draws (mostly `class`/`.COPY`/`.MOD` rows) are `inconclusive`: this
sample's raw-`.lst` extraction captured the base row plus same-KEY `.MOD` rows, but not a full
multi-line `CLASS:`/base-item closure the real engine's `token_closure_rows` also joins. Reported as
`inconclusive`, never asserted either way — the same discipline `closure_has_real_description` (below)
applies in production code.

## The tightened predicate

Given the above, the safe, already-shipped predicate is **`wiring_class == 'display' AND
magnitude_token_count == 0`** — not `magnitude_token_count == 0` alone. This sample found **zero**
cases where that combination missed a real magnitude (all 41 hand-confirmed magnitude-bearing units
were already routed off `display` by the classifier). The card's own "tighten the proxy" instruction
is discharged by confirming the code never used the untightened form in the first place: every
`text-complete`-granting branch in `v06_work_inventory.rs` gates on the unit's resolved `wiring_class`
(via the outer per-kind `classify()` dispatch, which only reaches the zero-magnitude branches for
kinds/records the wiring_class pipeline separately confirmed), not on raw token count alone.

## Condition 3 — the proxy this cycle's own new work depends on, ALSO validated by measurement, not assumption

Decision 7's condition 3 ("the prose is available to print... on the character sheet") is not a
proxy this sample tests directly (it is a rendering fact, not a corpus-content fact) — it is tested by
the corpus-wide `description: null` audit and DoD-8 on-screen proof in the cycle receipt
(`progress.md`), which found and fixed 1,060 already-`done` units whose corpus `description` was
`null`/empty — the single largest finding of this cycle, surfaced by the SAME "read the whole record,
don't trust a proxy" discipline this file documents for condition 1/2.
