# Epic F artifacts — permanent class-census instrument (F0)

This directory holds committed, point-in-time snapshots of the
`class_census` bin's `--json` output, plus the analysis scripts that fed
the original planning pass. **None of these files is a live source of
truth** — `docs/architecture/status.md`'s "Class/level compute coverage —
corpus-wide" table (between the `<!-- class-census:begin -->` /
`<!-- class-census:end -->` markers) is generated fresh from the census
every time `scripts/gen_class_status_table.py` runs, and is what actually
gates `scripts/verify.sh`'s `class-census` stage. The files here are
dated evidence of what the instrument measured at a given commit, kept for
the epic's own review trail (`epic-f-class-completion.md` §2 cites each
one by name).

## Files

- `census-f0b.json` — F0b's first sweep of the merged registry (base +
  untabled + CRB-NPC/Ex classes, alone). No prestige/mix-panel fields yet.
- `census-f0c.json` — F0c's sweep, adds the deterministic prestige
  carrier-mix build (`prestige`, `prestige_swept`, `prestige_alone_blocked`,
  `prestige_mix_computed`).
- `census-f0d.json` — F0d's sweep, adds the multiclass mix panel
  (`mix_panel`, `mix_panel_swept`, `mix_panel_computed`,
  `mix_panel_blocked`, `mix_panel_blocking_histogram`).
- `census-f0.json` — **F0e's final copy**, the full document (every field
  above) as measured at F0's closing commit. This is the file
  `docs/architecture/status.md`'s generated table was last produced from.
- `census-f2a.json` — SD-36 Epic F2a's AFTER sweep (generic class-family gate
  arm; `core_rulebook`/`advanced_players_guide` appended to
  `CLASS_FAMILY_BOOKS`): ids 137, 63 of 63 non-prestige Computed. Before/after
  write-up: `stage-f2-f3/f2a-census-before-after.md`.
- `census-f2b.json` — SD-36 Epic F2b's sweep (prestige-alone game rule):
  counts identical to `census-f2a.json`; every one of the 74 prestige rows'
  `alone_blocking_diagnostics` now carries
  `prestige_class.requires_base_class_levels` at every level, and none carries
  `class_chassis.unsupported`. Write-up: `stage-f2-f3/f2b-prestige-alone.md`.
- `census-f3b.json` — SD-36 Epic F3b's sweep (multiclass gate + fold for every
  class with a chassis): prestige carrier mixes 0 -> 56 of 74 `Computed` (11
  Unknown, 7 Blocked by named mechanism); every other count identical to
  `census-f2b.json`. Write-up: `stage-f2-f3/f3b-multiclass-fold.md`;
  hand-worked oracle: `stage-f2-f3/f3b-hand-worked.md`.
- `census-f3b3.json` — SD-36 Epic F3b3's sweep (class skills read from the
  converted record; single-class skill points): non-prestige 61 of 63
  `Computed` (Expert and Psion refused by name,
  `skill.selected_modifier.class_skill_unknown`); prestige carrier mixes 59 of
  74 (unchanged since F3b2b; 4 Blocked on an oracle save-formula defect, 11
  Unknown); mix panel 185 of 185. Write-up: `stage-f2-f3/f3b3-receipt.md`.
- `census-f3c.json` — SD-36 Epic F3c's sweep. The carrier chooser now walks
  `AtLeast` branches in oracle order (`carrier_reason` on every carried row).
  Prestige carrier mixes: 67 of 74 `Computed`, up from 59. Of the other 7, 6 are
  Blocked on an oracle save-formula defect and 1 is Unknown (dragon_disciple).
  Every other count is identical to `census-f3b3.json`. Histogram and movement:
  `stage-f2-f3/f3c-blocker-histogram.md`. Remainder: `stage-f2-f3/f3c-remainder.md`.
- `census-f3c5.json` — SD-36 Epic F3c5's sweep (Internal natural-attack helper rows convert as
  `Fact::NaturalAttack` facts). Prestige carrier mixes: 68 of 74 `Computed`, up from 67.
  dragon_disciple is the one row that moves: its `Internal|Bite` closure defect resolves. The other 6
  are Blocked on the oracle save-formula defect (FS-15). Every other count is identical to
  `census-f3c4.json`. Write-up: `stage-f2-f3/f3c5-receipt.md`; hand-worked oracle:
  `stage-f2-f3/f3c5-hand-worked.md`.
- `mix-panel-histogram.md` — F0d's human-readable write-up of the
  claim-blocking-diagnostic histogram across the 185-row mix panel.
- `scripts/` — `unres2.py`/`closure.py`, the read-only planning-pass
  analysis scripts `epic-f-class-completion.md` §13 documents (outside
  every `pcgen_residue_gate.py` live root; write nothing to this repo).

## Re-deriving `census-f0.json`

```
cargo run --locked -j 2 --bin class_census -- --json \
  docs/release/SD-36-consolidation/artifacts/epic-f/census-f0.json
```

Prints `ids=135 computed=42 blocked=93` /
`prestige_swept=74 prestige_alone_blocked=74 prestige_mix_computed=0` /
`mix_panel_swept=185 mix_panel_computed=185 mix_panel_blocked=0` to
stdout — these are the same numbers `docs/architecture/status.md`'s
generated table states, field for field (`ids`, `computed`, `blocked`,
`prestige_swept`, `prestige_alone_blocked`, `prestige_mix_computed`,
`mix_panel_swept`, `mix_panel_computed`, `mix_panel_blocked`).

## Regenerating `status.md`'s class-coverage table from this file

```
python3 scripts/gen_class_status_table.py \
  --json docs/release/SD-36-consolidation/artifacts/epic-f/census-f0.json
```

Or, to check for drift without writing (what `scripts/verify.sh`'s
`class-census` stage runs, against a *fresh* sweep rather than this
committed snapshot):

```
python3 scripts/gen_class_status_table.py --check
```

Both commands are pure Python (no `cargo` build) once a census JSON
exists — see `scripts/gen_class_status_table.py`'s own module docstring
for the generator's scope and doctrine (no chassis column, no fabricated
figures, loud failure on an unrecognized family or a partition that does
not sum to the census's own `ids` total).
