# GE-01 Oracle Surface Inventory

## Purpose
This file records grounded and candidate PCGen oracle surfaces for later parity and validation work.

Unknown capability remains explicit. Absence of an entry does not imply absence of an oracle; it means the oracle has not yet been grounded.

## Grounded oracle surfaces

| Surface | Path | Oracle role | Trust posture | Notes |
|---|---|---|---|---|
| Campaign root corpus | `/home/ubuntu/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/core_rulebook/core_rulebook.pcc` | Structural oracle for campaign composition, include edges, and top-level object declaration files. | High for file-layout truth; not itself a semantic execution oracle. | Grounded directly from `core_rulebook.pcc`, especially lines 42-96. |
| Human race subtree | `/home/ubuntu/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/core_essentials/races/human/_race.pcc` and `human_races.lst` | Pilot race oracle for Human-specific declarations and supporting race files. | High for pilot race source truth. | Grounded from `_race.pcc` lines 16-24 and `human_races.lst` line 6. |
| Human racial-trait carrier bundle | `/home/ubuntu/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/core_rulebook/cr_abilities_race.lst` | Pilot race-trait oracle for how Human semantics are decomposed into named ability carriers. | High for Human trait-bundle source truth. | Grounded from `cr_abilities_race.lst` lines 129 and 134-139. |
| Proficiency feat surface | `/home/ubuntu/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/core_rulebook/cr_feats.lst` | Structural oracle for automatic proficiency grants, prerequisite guards, and choice-enabled proficiency feats. | High for pilot-adjacent grant semantics; medium for final canonical interpretation. | Grounded from `cr_feats.lst` lines 19-21, 120, 150, 155, and 175. |
| Fighter class-skill and proficiency carriers | `/home/ubuntu/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/core_rulebook/cr_abilities_class.lst` | Pilot class-feature oracle for Fighter grants, class-skill relations, proficiency carriers, and feature-level gates. | High for Fighter source truth; medium for final canonical interpretation. | Grounded from `cr_abilities_class.lst` lines 236-262 and 2797-2835. |
| Core skills surface | `/home/ubuntu/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/core_rulebook/cr_skills.lst` | Pilot skill-object oracle for KEYSTAT, ACHECK, TYPE, class-skill bonus predicates, and representative Fighter skill targets. | High for skill source truth. | Grounded from `cr_skills.lst` lines 6-10 and 40-45. |
| Base stats and saves surfaces | `/home/ubuntu/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/core_rulebook/cr__stats.lst` and `cr__saves.lst` | Foundation oracle for base ability-score formulae and save-to-stat bindings. | High for current-state source truth; high semantic risk for canonical model decisions. | Grounded from `cr__stats.lst` lines 3-9 and `cr__saves.lst` lines 3-6. |
| Human trait grant/removal surfaces | `/home/ubuntu/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/core_essentials/races/human/human_abilities_race.lst` and `human_abilities_globalvar.lst` | Pilot Human oracle for default trait definitions, automatic grants, and replacement gates. | High for Human trait source truth. | Grounded from `human_abilities_race.lst` lines 10 and 18-23 plus `human_abilities_globalvar.lst` lines 10-24. |
| Listfile token documentation slices | `docs/listfilepages/systemfilestagpages/gamemodestatsandcheckslist.html`, `docs/listfilepages/lstfileclass/lfc_lesson03_race1.html`, `docs/listfilepages/lstfileclass/lfc_lesson16_domains2.html` | Candidate documentation oracle for STATMOD, RACETYPE/RACESUBTYPE, and CSKILL semantics. | Medium; cite exact sections before treating docs as semantic authority. | Grounded by search hits around STATMOD lines 500-562, RACETYPE lesson lines 473-519, and CSKILL lesson lines 336-399. |
| Listfile documentation | `/home/ubuntu/workspace/repos/pcgen/docs/listfilepages/listfileimportanttoknow.html` | Candidate semantic oracle for legacy token meaning and authoring intent. | Medium until specific token sections are cited into later artifacts. | File existence verified; referenced already by GE-01 technical requirements. |
| Generic loader implementation | `/home/ubuntu/workspace/repos/pcgen/code/src/java/pcgen/persistence/lst/GenericLoader.java` | Grounded parser-behavior oracle for how PCGen binds object names, source campaign, source URI, and token processing in code. | Medium-high for loader mechanics; not automatically authoritative for desired Codex semantics. | Grounded from `GenericLoader.java` lines 31 and 58-99; especially useful for provenance and token-processing expectations. |
| Campaign source entry implementation | `/home/ubuntu/workspace/repos/pcgen/code/src/java/pcgen/persistence/lst/CampaignSourceEntry.java` | Grounded PCC/campaign-loading oracle for campaign binding and include-item retention semantics. | Medium-high for campaign-entry mechanics. | Grounded from `CampaignSourceEntry.java` lines 45-49, 61-71, 117, and 150; useful when package/include ingest is formalized later. |

## Not yet grounded

| Candidate surface | Why it matters | Current status |
|---|---|---|
| PCGen runtime / character-generation execution path | Would provide behavioral parity evidence beyond static files. | Not grounded in this pass. |
| Exported character sheet or debug output for a pilot Human Fighter | Could serve as comparison evidence for later GE-05 oracle work. | Not grounded in this pass. |
| Automated parser test fixtures from the PCGen repo | Could become source-span and regression anchors for GE-03. | Not grounded in this pass. |

## Usage rule
GE-01 may inventory oracle surfaces, but it MUST NOT imply that an oracle has been validated just because a file exists. Later epics must cite which oracle surfaces they trust for structural truth, token semantics, and behavioral comparison.
