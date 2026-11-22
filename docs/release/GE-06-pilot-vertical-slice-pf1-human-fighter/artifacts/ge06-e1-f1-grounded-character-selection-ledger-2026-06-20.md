---
title: GE06-E1-F1 Grounded Character Selection Ledger
artifact_type: research-receipt
stc_id: STC-CODEX-GE-06
source_handoff: ../research-handoff.md
selected_slice: GE06-E1-F1 — Grounded character selection ledger
status: draft
created_at: 2026-06-20
code_authority: false
scope: program
owner: Todd Hintzmann
---

# GE06-E1-F1 Grounded Character Selection Ledger

## Objective
Recover the narrowest honest PF1 Core Rulebook Human Fighter level 1 input contract that GE-06 can currently claim from source evidence alone.

This receipt does **not** compute outputs, claim parity, or authorize code. It answers a smaller question: which parts of the pilot input set are truly selected today, which are only grounded anchors, and which remain blocked because the authority surfaces do not close them.

## Sources Read
- `programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/research-handoff.md`
- `programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/README.md`
- `programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/epic-breakdown.md`
- `programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/risks-and-open-questions.md`
- `programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/pilot-charter-alignment.md`
- `programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/pilot-character-fixture-requirements.md`
- `programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/required-token-family-list-requirements.md`
- `programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/required-canonical-object-list-requirements.md`
- `programs/codex/plans/pilot-slices/pf1-crb-human-fighter-level1-charter.md`
- `programs/codex/requirements/GE-01-legacy-corpus-and-conversion-matrix/artifacts/pilot-corpus-inventory.csv`
- `programs/codex/requirements/GE-01-legacy-corpus-and-conversion-matrix/artifacts/pilot-token-taxonomy.csv`
- `programs/codex/requirements/GE-01-legacy-corpus-and-conversion-matrix/artifacts/conversion-matrix.csv`
- `programs/codex/requirements/GE-01-legacy-corpus-and-conversion-matrix/artifacts/unsupported-token-ledger.csv`
- `/home/ubuntu/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/core_rulebook/core_rulebook.pcc`
- `/home/ubuntu/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/core_essentials/races/human/_race.pcc`
- `/home/ubuntu/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/core_essentials/races/human/human_races.lst`
- `/home/ubuntu/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/core_essentials/races/human/human_abilities_race.lst`
- `/home/ubuntu/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/core_essentials/races/human/human_abilities_globalvar.lst`
- `/home/ubuntu/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/core_rulebook/cr_classes.lst`
- `/home/ubuntu/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/core_rulebook/cr_abilities_class.lst`
- `/home/ubuntu/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/core_rulebook/cr_abilitycategories.lst`
- `/home/ubuntu/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/core_rulebook/cr_skills.lst`
- `/home/ubuntu/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/core_rulebook/cr_feats.lst`
- `/home/ubuntu/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/core_rulebook/cr_equip_arms_armor.lst`
- `/home/ubuntu/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/core_rulebook/cr_profs_weapon.lst`
- `/home/ubuntu/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/core_rulebook/cr_profs_armor.lst`

## Commands Run
All work in this receipt used read-only Hermes discovery operations. No mutating shell commands were run.

Key commands/tool calls executed:
- `read_file(path=".../research-handoff.md")`
- `read_file(path=".../README.md")`
- `read_file(path=".../epic-breakdown.md")`
- `read_file(path=".../risks-and-open-questions.md")`
- `read_file(path=".../pilot-charter-alignment.md")`
- `read_file(path=".../pilot-character-fixture-requirements.md")`
- `read_file(path=".../required-token-family-list-requirements.md")`
- `read_file(path=".../required-canonical-object-list-requirements.md")`
- `read_file(path=".../pf1-crb-human-fighter-level1-charter.md")`
- `search_files(path=".../pilot-slices", file_glob="pf1-crb-human-fighter-level1-charter.md", pattern="16/14/14/10/12/8|STR 16|DEX 14|power_attack|Power Attack|ability scores")`
- `search_files(path=".../GE-01-legacy-corpus-and-conversion-matrix/artifacts", file_glob="pilot-token-taxonomy.csv", pattern="STARTFEATS|ABILITYPOOL|CHOOSE|CSKILL|STARTSKILLPTS|PRE\\*|PREVAREQ|PREVARGTEQ|KEYSTAT")`
- `search_files(path=".../GE-01-legacy-corpus-and-conversion-matrix/artifacts", file_glob="conversion-matrix.csv", pattern="Human|Fighter|Power Attack|Chain Shirt|Longsword|CSKILL|STARTSKILLPTS|ABILITYPOOL|STARTFEATS")`
- `search_files(path=".../GE-01-legacy-corpus-and-conversion-matrix/artifacts", file_glob="unsupported-token-ledger.csv", pattern="Human|Fighter|ABILITYPOOL|STARTFEATS|STARTSKILLPTS|CSKILL|PRE|Choice|selector")`
- `read_file(path=".../core_rulebook.pcc")`
- `read_file(path=".../_race.pcc")`
- `read_file(path=".../human_races.lst")`
- `read_file(path=".../human_abilities_race.lst")`
- `read_file(path=".../human_abilities_globalvar.lst")`
- `read_file(path=".../cr_classes.lst")`
- `read_file(path=".../cr_abilities_class.lst")`
- `read_file(path=".../cr_abilitycategories.lst")`
- `read_file(path=".../cr_skills.lst")`
- `read_file(path=".../cr_feats.lst")`
- `read_file(path=".../cr_equip_arms_armor.lst")`
- `read_file(path=".../cr_profs_weapon.lst")`
- `read_file(path=".../cr_profs_armor.lst")`

## Grounded Inputs Recovered
- The pilot identity is fixed as `pf1-crb-human-fighter-level1`, with Human, Fighter, level 1, a final ability-score vector, and a named `power_attack` seed already grounded in the charter and mirrored by GE-06 fixture artifacts (`programs/codex/plans/pilot-slices/pf1-crb-human-fighter-level1-charter.md:28,124-143`; `programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/pilot-character-fixture-requirements.md:22-35`).
- GE-06 already records that Chain Shirt and Longsword are candidate anchors rather than final selections, and that exact skill allocation / equipment loadout / additional feat closure remain unresolved (`.../pilot-character-fixture-requirements.md:37-52`; `.../risks-and-open-questions.md:22-47`).
- GE-01 token-taxonomy doctrine classifies `STARTFEATS`, `ABILITYPOOL`, `CSKILL`, `KEYSTAT`, `STARTSKILLPTS`, and `PRE*` as pilot-critical token families rather than optional cleanup (`.../pilot-token-taxonomy.csv:11-17,20,22-27`).
- GE-01 conversion posture already marks Human/Fighter choice, equipment, class-skill, skill-budget, and predicate surfaces as deferred or partial canonicalization work, which is why this receipt must stop at `Parsed` instead of pretending the choices are computed (`.../conversion-matrix.csv:3-9,11-12,14-16,18-20,23-30`).
- The campaign root and Human subtree include graph directly bind the relevant local files into the pilot source universe (`/home/ubuntu/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/core_rulebook/core_rulebook.pcc:58-80`; `/home/ubuntu/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/core_essentials/races/human/_race.pcc:19-24`).
- The Human row and Human trait rows ground three distinct entitlement surfaces that matter here: `STARTFEATS:1`, a selectable ability-score bonus pool, and a selectable Human bonus-feat pool, plus the `Skilled` rank bonus (`.../human_races.lst:6`; `.../human_abilities_race.lst:18,21-22`; `.../human_abilities_globalvar.lst:10-20`).
- The Fighter row and Fighter class-feature rows ground progression identity, skill-budget surfaces, blanket martial/light/medium/heavy/shield proficiencies, and the Fighter bonus-feat carrier/category (`.../cr_classes.lst:139-143`; `.../cr_abilities_class.lst:236-257,2835`; `.../cr_abilitycategories.lst:86-94`; `.../cr_abilities_class.lst:893`).
- Longsword and Chain Shirt are both real grounded source rows with matching proficiency concepts; compatibility is grounded, but final selection is not (`.../cr_equip_arms_armor.lst:40,165`; `.../cr_profs_weapon.lst:57`; `.../cr_profs_armor.lst:7-13`).

## Selection Ledger
| Ledger ID | Domain | Candidate / decision | Exact evidence | Evidence class | Claim ceiling | Status | Charter impact / blocker |
|---|---|---|---|---|---|---|---|
| GE06-SL-001 | race / class / level | Selected case identity remains `pf1-crb-human-fighter-level1` with race `Human`, class `Fighter`, level `1`. | `programs/codex/plans/pilot-slices/pf1-crb-human-fighter-level1-charter.md:28`; `programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/pilot-character-fixture-requirements.md:22-35`; `/home/ubuntu/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/core_essentials/races/human/_race.pcc:19-24`; `/home/ubuntu/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/core_essentials/races/human/human_races.lst:6`; `/home/ubuntu/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/core_rulebook/cr_classes.lst:139-143` | charter-grounded | Parsed | selected | no-change |
| GE06-SL-002 | ability scores | Selected final score vector is `STR 16 / DEX 14 / CON 14 / INT 10 / WIS 12 / CHA 8`. | `programs/codex/plans/pilot-slices/pf1-crb-human-fighter-level1-charter.md:125-133`; `programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/pilot-character-fixture-requirements.md:29-35,57-74`; `programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/pilot-charter-alignment.md:24` | charter-grounded | Parsed | selected | no-change |
| GE06-SL-003 | feat | `power_attack` is the only explicitly named selected feat today, and it is a grounded seed rather than proof that feat closure is complete. | `programs/codex/plans/pilot-slices/pf1-crb-human-fighter-level1-charter.md:132-133`; `programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/pilot-character-fixture-requirements.md:33-35,71-74`; `programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/pilot-charter-alignment.md:24`; `/home/ubuntu/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/core_rulebook/cr_feats.lst:134` | charter-grounded | Parsed | selected | no-change |
| GE06-SL-004 | bonus-feat entitlement | Additional feat-surface reconciliation is mandatory. Current sources ground `STARTFEATS:1`, Human bonus-feat entitlement, and Fighter bonus-feat entitlement, but they do **not** map all resulting feat-slot debt to named selected feats. | `/home/ubuntu/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/core_essentials/races/human/human_races.lst:6`; `/home/ubuntu/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/core_essentials/races/human/human_abilities_race.lst:21`; `/home/ubuntu/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/core_rulebook/cr_abilities_class.lst:257,893`; `/home/ubuntu/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/core_rulebook/cr_abilitycategories.lst:86-89`; `programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/pilot-character-fixture-requirements.md:46-52`; `programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/risks-and-open-questions.md:22-32` | legacy-source-grounded | Parsed | blocked | explicit blocker: unmapped Human/Fighter feat-slot debt beyond the named `power_attack` seed |
| GE06-SL-005 | selector | Human ability-score choice remains only partially closed: the final score vector is grounded, but the separate Human `+2 to One Ability Score` pool is not independently resolved to a named selected bonus target by current authority surfaces. | `programs/codex/plans/pilot-slices/pf1-crb-human-fighter-level1-charter.md:125-131`; `/home/ubuntu/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/core_essentials/races/human/human_abilities_race.lst:18`; `programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/pilot-charter-alignment.md:24`; `programs/codex/requirements/GE-01-legacy-corpus-and-conversion-matrix/artifacts/unsupported-token-ledger.csv:14` | blocked | Parsed | blocked | explicit blocker: Human ABILITYPOOL choice exists, but current authority names only the final vector, not the internal choice decomposition |
| GE06-SL-006 | skill allocation | No exact skill-rank allocation is selected. Skill behavior is mandatory, but current authority surfaces ground only the budget/carrier context, not a final rank distribution. | `programs/codex/plans/pilot-slices/pf1-crb-human-fighter-level1-charter.md:67-69,142`; `programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/pilot-character-fixture-requirements.md:46-52,75-78`; `/home/ubuntu/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/core_rulebook/cr_classes.lst:141`; `/home/ubuntu/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/core_essentials/races/human/human_abilities_race.lst:22`; `/home/ubuntu/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/core_rulebook/cr_abilities_class.lst:2835`; `/home/ubuntu/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/core_rulebook/cr_skills.lst:10,40,42,44-45`; `programs/codex/requirements/GE-01-legacy-corpus-and-conversion-matrix/artifacts/unsupported-token-ledger.csv:9,13` | blocked | Parsed | blocked | explicit blocker: no authority surface names the deterministic level-1 rank allocation |
| GE06-SL-007 | equipment | `Chain Shirt + Longsword` are grounded representative anchors, not a final mandatory loadout. | `programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/pilot-character-fixture-requirements.md:37-52`; `programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/risks-and-open-questions.md:41-48`; `programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/research-handoff.md:267-269`; `/home/ubuntu/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/core_rulebook/cr_equip_arms_armor.lst:40,165` | artifact-grounded | Parsed | anchored-but-not-final | explicit blocker: representative anchors exist, but no authority selects the final equipped loadout or active-state assumptions |
| GE06-SL-008 | selector / proficiency | Longsword does **not** force a separate weapon-proficiency selector if it is later chosen, because standard Fighter grants blanket martial proficiency and Longsword is a martial proficiency concept. | `/home/ubuntu/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/core_rulebook/cr_abilities_class.lst:237`; `/home/ubuntu/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/core_rulebook/cr_profs_weapon.lst:57`; `/home/ubuntu/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/core_rulebook/cr_equip_arms_armor.lst:165`; `programs/codex/requirements/GE-01-legacy-corpus-and-conversion-matrix/artifacts/pilot-token-taxonomy.csv:15,20` | legacy-source-grounded | Parsed | selected | no-change |
| GE06-SL-009 | equipment / proficiency | Chain Shirt is proficiency-compatible with standard Fighter via granted light-armor proficiency, but compatibility alone does not promote it from anchor to final loadout. | `/home/ubuntu/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/core_rulebook/cr_abilities_class.lst:238-240`; `/home/ubuntu/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/core_rulebook/cr_equip_arms_armor.lst:40`; `/home/ubuntu/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/core_rulebook/cr_profs_armor.lst:7-13`; `programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/pilot-character-fixture-requirements.md:37-44` | legacy-source-grounded | Parsed | anchored-but-not-final | explicit blocker: compatibility is grounded, selection is not |
| GE06-SL-010 | class-skill / selector | Fighter class-skill closure is not fully normalized because the carrier row mixes explicit skills with `TYPE=Craft` and `TYPE=Profession` selectors. That upstream ambiguity blocks a stronger deterministic skill-closure claim. | `/home/ubuntu/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/core_rulebook/cr_abilities_class.lst:2835`; `programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/required-token-family-list-requirements.md:35-41`; `programs/codex/requirements/GE-01-legacy-corpus-and-conversion-matrix/artifacts/unsupported-token-ledger.csv:9` | blocked | Parsed | deferred-with-owner | explicit blocker: GE-02 skill relation model + GE-03 parser still own class-skill TYPE-selector normalization |

## Entitlement and Gate Reconciliation
1. **Is `power_attack` the only grounded mandatory feat selection?**
   - No. It is the only **named selected** feat.
   - Human and Fighter surfaces still force additional feat-surface reconciliation through `STARTFEATS:1`, Human bonus-feat `ABILITYPOOL`, and Fighter bonus-feat carrier/category rows (`human_races.lst:6`; `human_abilities_race.lst:21`; `cr_abilities_class.lst:257,893`; `cr_abilitycategories.lst:88`).
   - The honest result is therefore: `power_attack` is selected, but feat closure is still blocked.

2. **Does the first deterministic loadout close cleanly as `Chain Shirt + Longsword`?**
   - No.
   - Those rows are grounded and proficiency-compatible (`cr_equip_arms_armor.lst:40,165`; `cr_abilities_class.lst:237-240`; `cr_profs_weapon.lst:57`; `cr_profs_armor.lst:7-13`).
   - But GE-06 already states they are candidate anchors rather than final selections (`pilot-character-fixture-requirements.md:37-52`; `risks-and-open-questions.md:41-48`).
   - The honest result is therefore: anchored, not closed.

3. **Which exact skill allocations are mandatory for the first deterministic pilot case?**
   - None are presently named by authority.
   - Skill behavior is mandatory, and the Fighter/Human surfaces clearly force a skill-budget and class-skill context (`cr_classes.lst:141`; `human_abilities_race.lst:22`; `cr_abilities_class.lst:2835`; `cr_skills.lst:10,40,42,44-45`).
   - But the exact rank allocation is not chosen anywhere in the charter or GE-06 artifacts.
   - The honest result is therefore: blocked.

4. **Do Human or Fighter `ABILITYPOOL`, `CHOOSE`, `PRE*`, or class-skill surfaces force additional selector closure?**
   - Yes.
   - Human ability-score and bonus-feat pools force selector closure (`human_abilities_race.lst:18,21`; `pilot-token-taxonomy.csv:25-26`).
   - Fighter bonus-feat carriers force selector closure for the Fighter bonus-feat pool (`cr_abilities_class.lst:257,893`; `cr_abilitycategories.lst:88`).
   - Fighter class-skill carriers force selector/normalization discipline because `TYPE=Craft` and `TYPE=Profession` are still present (`cr_abilities_class.lst:2835`; `unsupported-token-ledger.csv:9`).
   - By contrast, Longsword does **not** add a separate weapon-proficiency choice obligation if the standard Fighter blanket martial proficiency is used (`cr_abilities_class.lst:237`; `cr_profs_weapon.lst:57`).

5. **Does the resolved selection set stay inside the pilot charter as written?**
   - Yes, at the current claim level.
   - The present honest classification is `no-change`, not because everything is closed, but because the unresolveds are still being carried as unresolved rather than hidden as fake closure (`pilot-charter-alignment.md:21-29`; `research-handoff.md:267-271`).
   - A later attempt to silently broaden equipment breadth, feat scenarios, or multiple skill/loadout cases would become `charter-patch` or `ADR-trigger` territory.

## Charter Boundary Check
The charter boundary remains intact.

Why:
- GE-06 still preserves the first case as PF1 Core Rulebook Human Fighter level 1 (`pilot-charter-alignment.md:21-23`).
- GE-06 still treats the initial score vector and named `power_attack` feat as grounded defaults rather than as permission to fabricate the rest of the choice set (`pilot-charter-alignment.md:24`).
- GE-06 still routes any scope growth into explicit escalation rather than silent implementation drift (`pilot-charter-alignment.md:26-29`).

Current classification: **no-change**.

## Remaining Blockers
- **Unmapped feat-slot debt.** The charter names `power_attack`, but the receipt cannot yet map all Human/Fighter feat entitlements to named selected feats.
- **Human ability-score choice decomposition.** The final six-score vector is grounded, but the underlying Human `+2 to One Ability Score` choice is not independently named.
- **Exact skill allocation.** No authority surface names a deterministic level-1 rank distribution.
- **Class-skill TYPE-selector normalization.** `TYPE=Craft` and `TYPE=Profession` remain upstream modeling debt rather than closed concrete skill lists.
- **Final equipment loadout and active-state assumptions.** Chain Shirt and Longsword are grounded anchors only; equipped-state closure is still missing.

## Proposed Upstream Deltas
- **No immediate charter patch is required.** The correct current move is to preserve `no-change` and keep unresolveds explicit.
- **Next GE-06 documentary closure should name the final deterministic input set.** That follow-on should map the feat-slot debt, the exact skill ranks, and the exact equipped loadout without crossing into computed outputs.
- **GE-02 / GE-03 / GE-04 should continue to own the modeling debt already visible here.** The receipt surfaces the need for first-class handling of `ABILITYPOOL`, `STARTSKILLPTS`, class-skill TYPE selectors, and equipment/proficiency references; it does not authorize local invention around those gaps.

## Verification
Verification was performed against the handoff contract in `programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/research-handoff.md:284-292`.

Checks satisfied:
1. Required GE-06, GE-01, charter, and PCGen source files were read and cited in this receipt.
2. This receipt exists at the exact required path: `programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/ge06-e1-f1-grounded-character-selection-ledger-2026-06-20.md`.
3. All required sections are present.
4. Every `Selection Ledger` row populates `Exact evidence`, `Evidence class`, `Claim ceiling`, and `Charter impact / blocker`.
5. Every row caps the claim ceiling at `Parsed`.
6. In this run, the only write action was creation of this receipt; all other tool calls were read/search operations.

## Verdict
The honest GE-06 pilot input contract is narrower than the optimistic version.

What is actually selected now:
- case identity: Human / Fighter / level 1
- final ability-score vector: `16 / 14 / 14 / 10 / 12 / 8`
- named feat seed: `power_attack`

What is **not** yet honestly selected:
- the full feat mapping implied by Human and Fighter entitlements
- the explicit Human ability-score bonus target
- the exact skill allocation
- the final equipped loadout

What `Chain Shirt + Longsword` really are:
- grounded, compatible anchors
- **not** final loadout closure

Charter result:
- **no-change**

The lesser models would have called this “close enough” and moved to implementation. That would be counterfeit. The current selection set is only partially closed, and the unresolved parts are real.