# Release-Swarm Task Table (v0.6 alpha)

> Single source of truth for the operator-facing task table on the dashboard.
> Each row is read 1:1 by the observer at `~/.hermes/profiles/god-emporer/skills/release-swarm-observer/scripts/observer.py`.
>
> Schema (operator-pinned 2026-07-23):
>   task     – ≤ 2-sentence description
>   owner    – one of {UI, Backend, QA}
>   rulebook – one of {CRB, B1, APG, ACG} or `-` (none / cross-cutting)
>   status   – one of {done, in progress, queued, blocked}
>   last_update – free-form (commit hash, date, or short note)
>
> The lead updates this file as work moves through the alpha-bar. The cron
> driver re-renders the dashboard every 60 seconds from this sidecar, so
> keeping it current is the cheapest possible visibility signal.
>
> **Rebuilt from scratch 2026-07-24.** The prior version of this file
> (seeded 2026-07-23) described work that never happened in this swarm —
> "Epic"-numbered rows ("campaign manager + Drive persistence," "Wizard
> single-class completion," etc.) that don't correspond to any real
> commit or task this session produced. Flagged as untrusted for most of
> the session rather than acted on; rebuilt here against the swarm's real
> tracking record (`risks-and-open-questions.md` items 1-27,
> `SWARM_STATUS.md`'s Happened log, and `git log`) instead of guessed or
> carried forward. Owner mapping note: this schema has no "lead" value;
> rows for lead-authored scoping/design docs use `QA` per the nearest
> real-work analog, with the actual author named in the row text.

| task | owner | rulebook | status | last_update |
|------|-------|----------|--------|-------------|
| 4-race ability-adjustment bug (Elf/Dwarf/Gnome/Halfling missing mental-stat component) | Backend | CRB | done | 9ec0e036 + 2f05dee4 |
| Create-character ability-score submission bug (calculated score shown but raw score sent) | UI | CRB | done | f2c616ed |
| Wizard spellbook bootstrap chain (school-choice seed, first-spell deadlock, slot-budget enforcement) | Backend | CRB | done | 3484b5d, b2a5eb6, 365b3a1a |
| Wizard spell-save-DC missing calculation | Backend | CRB | done | 3b39731 |
| Class-skill-modifier bug (Climb/Intimidate/Swim bonus applied regardless of actual class) | Backend | CRB | done | 93a0636d |
| Damage-reduction exposure (DTO + Defense tab render) | Backend/UI | CRB | done | f7ce289d, 26ac0704 |
| Recompute-DTO damage-reduction parity gap | Backend | CRB | done | af2d8b9f |
| Fighter feat-choice legality (race gate, multiclass gate, internal-consistency gate — 3 instances) | Backend | CRB | done | 32289cb4, 0eb9ea65, 68721ca0 |
| Class-dropdown honesty fix (human-diagnostics-only vs partial-human-only mislabeling) | UI | - | done | 34635157 |
| corpus_derived wire-serialization render-staleness bug | Backend | - | done | 498679d1 |
| Feat catalog exposure + Feats-tab picker + full feat-list display | Backend/UI | CRB | done | 89c3710, febf4d80, 1509124c, aa611ce1 |
| Money conversion + Money panel UI | Backend/UI | CRB | done | 67490ac, 59d5bc0a |
| Equipment purchase atomic money coupling | Backend | CRB | done | 29e67515 |
| Skill allocation persistence + SkillAllocationDialog wiring | Backend/UI | CRB | done | e0a0bda4, 75200fcb |
| Level-up HP + choices persistence + LevelUpDialog wiring | Backend/UI | CRB | done | 7694b227, e8e45976 |
| Bio schema + persistence command + Bio editor wiring | Backend/UI | - | done | 0ab784df, 94a3865 |
| Durability calc (max/current/temp HP, dying/unconscious/death thresholds) + Tauri commands | Backend | CRB | done | 0aeed25 |
| Durability/HP tab (Defense tab damage/heal UI) | UI | CRB | done | 75b083bd |
| Carry-capacity + encumbrance calculation | Backend | CRB | done | d475097 |
| Multiclass BAB/save widening (Fighter/Wizard/Rogue) | Backend | CRB | done | d20a5b9 + 8d814e8 |
| Multiclass classSummary parsing bug (comma vs slash separator, silent corruption) | Backend/UI | CRB | done | d03bc89d |
| Skill-allocation Wizard/Rogue class-skill grounding + cross-class rank-cap enforcement | Backend | CRB | done | 21f815c1 |
| characterProgression.ts test coverage (was zero, including for the classSummary bug) | UI | - | done | ca25bfef |
| LevelUpDialog feat-pick at feat-gaining levels (was silently absent) | UI | CRB | done | ddfc66bb |
| PCGen parity comparator: new combat.base_attack_bonus dimension | Backend | CRB | done | cda3bf1c + b8eff433 |
| Frontend test-coverage backlog (skillsModel, characterBio, spell-routing extraction, abilityScoreMethods, portraitImageProcessing) | UI | - | done | f905d656 |
| Load-Character list stale cached row after a sheet mutation | UI | - | done | 2b13a23c |
| ItemPickerModal duplicate-catalog-key React reconciliation bug | UI | - | done | e50d7762 |
| Comprehensive live-UI smoke test across all 3 working classes | UI | - | done | e50d7762 (fix landed as part of this pass) |
| driver.sh cross-agent GUI display/state-file collision fix | QA/Backend (infra) | - | done | f6fe0df2 |
| Interim four-check wired-integration audit (doctrine compliance) | QA | - | done | c3b5fba8, lead re-verified f3676470 |
| Starting wealth by class (operator-provided table, 11 CRB classes) | Backend | CRB | done | 0dbf67ad |
| Feat-effects engine, Toughness +3 HP slice (first real feat mechanical effect) | Backend | CRB | done | 53ddd1ce |
| Item 1 architecture-wall design/scoping pass (headless/corpus-aware split) | Backend | - | done | 4dadad51 (`item-1-architecture-wall-design.md`) |
| Item 1 shape (c): corpus-derived AC/ACP display section (non-claim-gated) | Backend | CRB | done | 08a829a1, verified 236/236 lib + 202/202 desktop + 4253/0 full workspace |
| Item 1 attack-bonus, bounded single-weapon slice (no schema change, honest-absent for 0/2+ weapons) | Backend | CRB | done | 845f860c, verified 242/242 lib + 202/202 desktop |
| Bundled desktop corpus fixture caps corpus_derived — backend half (surface unresolved_spell_ids/unresolved_equipment_item_ids instead of silent drop) | Backend | CRB | done | 647e52aa, verified 245/245 lib + 203/203 desktop |
| Render an honest "not shown — outside demo corpus" indicator for the new unresolved-selection lists | UI | CRB | done | 5406e335, live-verified against the exact original smoke-test finding |
| Wire shape (c)'s equipmentEffects (AC/ACP) into the frontend TS type and Defense tab | UI | CRB | done | d8528ce2, live-verified against a real Chain-Shirt-equipped Fighter and cross-checked on a Wizard |
| Defense tab doesn't refetch durability after a feat pick (render-staleness, same shape as corpus_derived bug) | UI | - | done | 7360fe4a, live-verified both feat-grant paths |
| Spells tab corpusDerived inconsistency for non-Human Wizards (found in smoke test) | Backend | CRB | done | root cause identified -- NOT a Wizard/race issue, see the bundled-corpus-fixture row above; superseded, not itself a separate task |
| Item 1 attack-bonus enhancement math for 2+ weapons (needs equipment-attachment data-model decision) | Backend | CRB | blocked | the single-weapon case is done (see above); multi-weapon attachment still needs a schema decision, see item 1 in risks-and-open-questions.md |
| Item 1 posture-gate widening (would `Computed` ever accept non-hardcoded equipment?) | - | CRB | blocked | operator decision, risks-and-open-questions.md item 27 |
| Feat-effects engine widened to 3 more feats (Great Fortitude/Iron Will/Lightning Reflexes) | Backend | CRB | done | f38e9f33, verified 254/254 lib + 203/203 desktop |
| Wizard non-Human spell-math (spell-save-DC + spellbook-ceiling absent for non-Human) | Backend | CRB | in progress | documented completeness gap, risks item 18; dispatched to backend 2026-07-24 ~14:20 ET |
| Live verification of the EquipmentEffectsDto null-serialization fix against real 0-weapon and 2-weapon builds | QA | CRB | in progress | dispatched 2026-07-24 ~14:20 ET |
| Class/multiclass breadth: 8 of 11 CRB classes have no working chassis at all (Cleric/Druid/Bard/Sorcerer/Barbarian/Monk/Paladin/Ranger) | Backend | CRB | blocked | multi-cycle engine work, future epic |
| Starting wealth for the 12 non-CRB-recognized classes in the operator's table | Backend | APG/ACG | blocked | no `class:<name>` id recognized anywhere in this crate yet for those classes |
| EquipmentEffectsDto's 3 optional fields serialize None as literal `null`, not an omitted key -- renders as garbled "+null"/"null%" for the common case (0 or 2+ weapons) | Backend | CRB | done | 874df6db, lead-verified 204/204 desktop; frontend needed no change (TS type was already `?:`, not `\| null`) |
