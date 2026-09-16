---
title: Engine handoff — class skill points per level never reached the corpus
status: open
severity: feature gap (UI cannot honestly show skill points; a TS twin fills the hole today)
found_by: backend teammate, v0.8 UI sprint, during ticket B-10
found_on: 2026-09-01
owner: the concurrent session that owns repo-root src/
scope_note: repo-root src/ (including src/pcgen_import) is out of scope for the v0.8 UI sprint
  (agent-team-ui.md §4.2). Nothing was built for B-10; this is the write-up only.
---

# Class skill points per level never reached the corpus

## Impact

The character sheet shows "Skill points: N" from a **hand-authored TypeScript table**
(`apps/desktop/src/characterHub/characterProgression.ts:11`, `CLASS_SKILL_POINTS`). That is a
PF1e rule living in TypeScript, which brief §3.3 forbids — but it cannot be removed, because the
value does not exist anywhere on the engine side to replace it with.

It also blocks the class-preview ticket (F-11): a player comparing Fighter against Rogue cannot see
skill points per level, which is one of the more decision-relevant numbers in the comparison.

## Why it is an ingestion gap, not an engine gap

`backend` traced this to the PCGen source rather than stopping at "the engine doesn't have it":

- **Engine:** `grep -rn "skill_point\|STARTSKILLPTS\|SKILLPTS" src --include=*.rs` finds only
  `HUMAN_EXTRA_SKILL_POINTS_AT_LEVEL_1` / `HUMAN_EXTRA_SKILL_RANKS_PER_LEVEL` in `pilot_compute`.
  No per-class table exists. `ClassTableRow` (CRB), the PU chassis row, and the generic class
  progression all carry only level / BAB / three saves.
- **Corpus:** the ingested class records carry no skill-points token at all. `raw_tokens` keys for
  Fighter are BONUS / DEFINE / FACT / HD / MAXLEVEL / ROLE / SOURCEPAGE / TYPE.
- **Root cause, verified against the pinned PCGen oracle:** skill points live on a **second**
  `CLASS:Fighter` line in `cr_classes.lst`. Line 139 is the one the ingestion captured; line 141 is:

      CLASS:Fighter	STARTSKILLPTS:FighterSkillPoints	DEFINE:FighterSkillPoints|0	BONUS:VAR|FighterSkillPoints|2

  The LST format continues a class definition across multiple `CLASS:<Name>` lines, and the
  importer keeps only the first. `grep -c STARTSKILLPTS data/corpus/.../core_rulebook/cr_classes.lst`
  → **28** such lines, **zero** of which reach the corpus.

So the data was never missing from the source. It was dropped in translation.

## Recommended fix (not applied)

1. Extend the class LST ingestion in repo-root `src/pcgen_import` to merge continuation
   `CLASS:<Name>` lines into one record — or, minimally, capture `STARTSKILLPTS` and the
   `BONUS:VAR|<Name>SkillPoints|N` pair.
2. Re-ingest.
3. Expose `skill_ranks_per_level` on the class chassis rows.

After that, B-10 becomes a one-line bridge ticket in `apps/desktop/src-tauri/`, and F-11 can show
the real value while `CLASS_SKILL_POINTS` is deleted from TypeScript.

## Note on scope

Worth checking whether the same continuation-line truncation dropped **other** tokens across the
28 affected classes — skill points are simply the one this sprint happened to need. That is a
question for whoever re-ingests, not a claim this audit verified.
