# DoD-8 on-screen verification — companion rung extension

- **Cycle:** SD31-D7-PROSE-003 (`RETRO_ACTOR=sd31-cf-surface`)
- **When:** 2026-08-16T12:38:16Z
- **HEAD at capture:** `b8c36417dd6dff1bad090d65e3b958f8f39177b2`
- **RUN_DESKTOP_AGENT:** `sd31-cf-surface`
- **Method:** `driver.sh` driven directly (Companion Catalog is not one of
  `verify-on-screen.sh`'s four supported families)

## What this proves

From the hub, **Browse Companion Catalog** → searched `Elemental, Air` → the
**Elemental, Air (Small)** entry (Core Essentials p.120, the pre-existing
`companion_catalog::serve_ability_description` render path
`SD31-W6-INTEGRATE-001` already proved live) shows its **Air Mastery**
ability with real rules text:

> *"Airborne creatures take a -1 penalty on attack and damage rolls against
> an air elemental."*

This is `core_essentials:companion:air_elemental_air_mastery` — one of the 165
`companion` units this cycle's rung extension promoted from `held` to `done`
(`companion_held_and_corpus_record_carries_real_description`).

## Byte-match confirmation, by direct file read

`data/corpus/core_essentials/companion/air_elemental_air_mastery.json`:

```
"description": "Airborne creatures take a -1 penalty on attack and damage
rolls against an air elemental."
```

**Byte-for-byte match**, no `%N` substitution present in this row, confirmed
by direct read.

## Artifact

- `companion-air-elemental-air-mastery.png` — full catalog screenshot with the
  search box, the matched creature card, and the Air Mastery ability text all
  visible together.
