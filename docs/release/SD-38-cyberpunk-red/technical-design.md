---
canonical: true
owner: operator
bundle_id: SD-38
slug: cyberpunk-red
canonical_branch: tranche-18
target_version: 0.18.0
date: 2026-10-13
status: draft — authored under operator-away defaults; operator rulings on licence and scope are on record
---

# SD-38 Technical Design

The architectural surface SD-38 changes. Every repo claim cites a path and line read on 2026-10-13
in `/home/ubuntu/workspace/repos/codex` (branch `tranche-17-ui` @ `bb6107aee0`; `tranche-18` will be
cut from `develop` after PR #398, so line numbers may shift by that merge — each owning card
re-reads its lines). Names marked *(new)* are design choices the owning card confirms.

## 1. What carries over from SD-37 and what does not

| Layer | Reused? | SD-38 treatment |
|---|---|---|
| `GameSystem` / `PerSystem` seam (`src/rules_core/game_system.rs:17–20` enum, `:48` `ALL`, `:51–56` `id`, `:59–64` `from_id`, `:67–72` `index`, `:75–80` `sheet_rules_relative`, `:83–88` `corpus_relative`, `:143–155` `PerSystem`) | yes, widened | third variant `CyberpunkRed` (E0.1) |
| Per-system package loader (`src/rules_core/corpus_loader.rs:368–385`: `live_sheet_rules_for`, one `OnceLock` per `GameSystem::ALL` entry, root from `runtime_repo_root()`) | yes | CPR has no sheet-rule package: `None` |
| PCGen converter (`crates/codex-ingest`) | **no** | Not used for CPR (`decisions.md §23`). Its `match`es refuse `CyberpunkRed` by name |
| Adapter seam (`apps/desktop/src-tauri/src/rule_system_adapter.rs:148–157` resolver; `:175–182` envelope router) | yes | `"cyberpunk-red"` → `CyberpunkAdapter` (E0.4) |
| SF module set (`apps/desktop/src-tauri/src/main.rs:42–49` `mod sf_*`; `:240` `invoke_handler`; `:250–257` SF commands) | pattern only | `cpr_adapter`, `cpr_catalog`, `cpr_creation`, `cpr_sheet_print`, `cpr_improve` *(new)* |
| PF/SF compute (`pilot_compute/**`, `sf_*` readers) | **no** | CPR's own engine `src/rules_core/cpr/**` *(new)*; PF/SF untouched |
| Save envelope (`src/saved_character/mod.rs:14` struct, `:34` `game_system`, `:40` `character_input: CharacterInput`) | yes, additive | optional CPR payload field (A-11) |
| Landing + hub (`LandingScreen.tsx:14` `RuleSetId`, `:30` `RULE_SETS`, `:62` `'cyberpunk'`; `characterHubRuntime.ts:33` `resolveRuleSystemId`, `:69` `form` union, `:79` `characterCreationGate`; `CharacterHubPage.tsx:11–13, 24, 153–155, 223–226, 306–307` Starfinder branches) | yes | CPR branches beside the Starfinder ones |
| Layout registry (`apps/desktop/src/characterHub/sheetLayout.ts:14` `LayoutBox`, `:43` `SHEET_LAYOUT`, `:91` `CREATE_LAYOUT`, `:108`/`:116` helpers) | pattern | `cyberpunkSheetLayout.ts`, `cyberpunkCreateLayout.ts` *(new)* in the same shape |
| Restore-starters (`character_hub.rs:2103` `StarterSeed`, `:2214` `starter_seeds()`, `:2471` `restore_starter_characters_at`; `main.rs:278`) | yes, generalised | system-aware starter request (E7.1) |
| Packaging (`tauri.conf.json:34` `bundle.resources`; `tests/packaged_resources.rs:73–85` path list, `:113–119` per-system load) | yes | CPR corpus entry + assertions (E0.3, E7.2) |
| ui-smoke (`apps/desktop/scripts/ui-smoke/spec.json`, 104 rows, `{id, screen, steps, marker, expect, notes}`; `npm run ui-smoke` / `npm run ui-smoke:doc`, `apps/desktop/package.json:12–13`) | yes | CPR rows; inventory doc regenerated |

## 2. The seam (E0)

- **Enum.** `GameSystem::CyberpunkRed`, `id() = "cyberpunk-red"`, `corpus_relative() =
  "data/cyberpunk-red/corpus"`. `ALL: [GameSystem; 3]`. `PerSystem<T>` gains `cyberpunk_red: T`;
  every literal (24 sites, `decisions.md §8`) adds it; `pathfinder_only` (`game_system.rs:164`) sets it
  to `&[]`.
- **No sheet-rule package (decided, `decisions.md §8`).** `sheet_rules_relative(self) -> Option<&'static str>`;
  `PackageRoots.sheet_rules: Option<PathBuf>` (`game_system.rs:39`); `live_sheet_rules_for(CyberpunkRed)`
  → `None`. The `tauri-resources-tracked` stage (`scripts/verify.sh:1493–1576`) reads
  `sheet_rules_relative`'s arms by regex (`:1538–1539`); E0.1 changes the regex to read `Some("…")`
  arms and to skip `None`. The test at `game_system.rs:186` (distinct roots per system) compares
  `Option`s.
- **Licence file** `data/cyberpunk-red/corpus/LICENSE.json` (shape of
  `data/starfinder-1e/corpus/core/LICENSE.json`: `book`, `license_declaration`, `operator_sign_off`,
  `records_processed`, `screening_method_note`; values per `decisions.md §4`).
- **Adapter.** `CyberpunkAdapter: RuleSystemAdapter` *(new `cpr_adapter.rs`)*. The trait
  (`rule_system_adapter.rs:72–133`) is Pathfinder-typed (`chassis_resolve(&CharacterInput) ->
  PilotBaseChassisComputation`, `level_up(&CharacterInput, …) -> LevelUpPlan`). SD-37's
  `StarfinderAdapter` reused the envelope's `CharacterInput` and returned `LevelUpPlan::default()` for
  `level_up` because its grants cannot cite a PF table (`sf_adapter.rs:636–645`). `CyberpunkAdapter`
  does the same for the two PF-typed methods and serves CPR through `load_saved_character` /
  `list_saved_characters` / `save_character` / `recompute` over the CPR payload (A-11); CPR-specific
  actions are their own commands (`cpr_creation`, `cpr_improve`). E0.4's receipt records the choice.
- **Envelope (A-11).** `SavedCharacterEnvelope` gains
  `#[serde(default, skip_serializing_if = "Option::is_none")] cpr_input: Option<CprCharacterInput>`;
  `game_system = "cyberpunk-red"`. PF/SF envelopes serialise byte-identically (E0.4 test, E0.6 gate).
- **Landing.** `'cyberpunk'` → `'cyberpunk-red'` (the only id change); `available: true` at E0.5 with
  New Character closed by `characterCreationGate`'s existing hint until E3.2 (SD-37 E6.1 precedent:
  selectable chip, real Load list, creation closed with its reason).

## 3. Data (E1)

```
/home/ubuntu/workspace/inbox/cyberpunk-red/pages/pNNN.txt  (+ page images via pdftoppm; never in git)
  └─ 8 Sonnet lanes, file-disjoint per kind (CUI §3), cargo-free
       → data/cyberpunk-red/corpus/<kind>.json   one JSON array per kind
       → data/cyberpunk-red/corpus/MANIFEST.json  (E1.0 writes; lanes never edit; E1.V pins)
       → data/cyberpunk-red/corpus/LICENSE.json   (E0.2)
  └─ checks: scripts/cpr_manifest_check.py (E1.0) + tests/cpr_corpus_manifest.rs (E1.V): counts,
     source_page ^p\d{3}$ in p001–p459, no missing/extra kind, no null count under --all
  └─ oracle: E1.V 10% sample per kind vs page image
```

Record shape (every kind): `source_page`, `name`, `kind`, the inventory's fields for that kind, `text`
(verbatim rule text where the book has prose), `transcription_note` (when a CUI §5 ruling or an image
check applied). Nested sub-tables (Role Ability rank bands, Exec team stat rows, Nomad upgrades) live
inside their parent record; MANIFEST's `nested` field pins their counts.

## 4. Engine (E2): `src/rules_core/cpr/` *(new)*

`decisions.md §10` lists each file's computed values. Composition in `sheet.rs` (E2.10):

```
input (CprCharacterInput: STATs, method, Role(s) + ranks + sub-choices, skills, kit, cyberware, deck,
       Lifepath answers, reputation + events, IP, recorded criticals)
  → cyberware.rs : HL sum, slot check (D-11), BODY override (Grafted/Linear Frame), SP sources, +Initiative
  → stats.rs     : effective BODY → HP, SW, Death Save; Humanity max/current, EMP (A-4); MOVE; LUCK
  → armor.rs     : SP per location = max(worn, cyberware SP); worst penalty once → REF/DEX/MOVE
  → skills.rs    : 66 bases with penalised REF/DEX
  → critical.rs  : thresholds; Base Death Save Penalty from recorded criticals
  → roles.rs     : rank-derived values per Role held
  → netrunning.rs: deck slots and NET Actions (Netrunner)
  → reputation.rs, improvement.rs : Facedown line; IP current/total
  → Vec<Explanation> with ids "cpr.<area>.<field>"
```

Lanes own their files; `mod.rs` holds three anchors (A-10). Fixtures come from the page or the
inventories, never from the corpus JSON the engine reads.

## 5. Desktop (E3–E6)

- **Commands** (registered in `main.rs` in the writer order of `epic-breakdown.md §0`):
  `preview_cyberpunk_character`, `create_cyberpunk_character` (E3.1, `cpr_creation.rs`);
  `list_cyberpunk_catalog`, `compute_cyberpunk_deck` (E6.1, `cpr_catalog.rs`);
  `spend_cyberpunk_ip`, `switch_cyberpunk_role`, `install_cyberpunk_cyberware`,
  `apply_cyberpunk_therapy`, `set_cyberpunk_reputation` (E5, `cpr_improve.rs`). Names are proposals; the
  owning card records the final names.
- **Roller.** One seedable table roller (Streetrat row, Edgerunner per-STAT, Lifepath, post-creation HL
  and therapy dice) in `cpr_creation.rs`; tests inject the seed. In-play dice are not built (DEF-2).
- **Components.** `CyberpunkCreateForm.tsx`, `CyberpunkCharacterSheet.tsx`, `CyberpunkImproveDialog.tsx`,
  `apps/desktop/src/cyberpunkCatalog/**` (catalog, `deck/`, `gm/`), boundaries
  `apps/desktop/src/boundary/{cprCreation,cprImprove,cprDeck,loadCyberpunkCatalog}.ts`.
- **Sheet** reads only engine rows (R2): a test maps every rendered digit to one `cpr.` row and shifts
  the rows by +1000 to prove it.
- **Print path** `cpr_sheet_print.rs` prints rule lines with resolved numbers (E4.6).

## 6. Packaging (E0.3, E7.2)

`tauri.conf.json` gains `"../../../data/cyberpunk-red/corpus/": "data/cyberpunk-red/corpus/"` (the
Starfinder entry's form). `packaged_resources.rs` adds `"data/cyberpunk-red/corpus"` to its list
(E0.3) and, at E7.2, loads the CPR corpus and computes a CPR seed under the packaged root. Production
reads go through `GameSystem::package_roots(&runtime_repo_root())` (`game_system.rs:91`, `:116`),
never `env!("CARGO_MANIFEST_DIR")` (`tests/no_baked_manifest_dir_in_production.rs`).

## 7. Verification surfaces

- `verify.sh` has 53 stages (`bash scripts/verify.sh --list | tail -n +2 | wc -l` → 53, 2026-10-13).
  SD-38 changes one stage (`tauri-resources-tracked` parser, E0.1). `pi-sweep` does not read CPR data
  (stage header `scripts/verify.sh:2249`).
- G-1 uses the SD-37 harnesses unchanged (`pf_seed_render_hash.rs`; `SF_SEED_LINES_OUT`).
- E7.4 re-derives `scripts/verify-baselines.env` (a third system moves suite counts).
