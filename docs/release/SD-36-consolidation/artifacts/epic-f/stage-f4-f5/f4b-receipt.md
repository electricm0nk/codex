# SD-36 Epic F4b: desktop class roster and level-up class options (receipt)

Scope: the desktop backend half of F4 (`epic-f-class-completion.md` §6; acceptance F4.1 in
`epic-breakdown.md`). The frontend (`classRoster.ts`, the `CLASS_OPTIONS` fallback,
`LevelUpDialog.tsx`, the ui-smoke rows) is F4c and later. No `data/**` change.

- Branch `sd36/epic-f4-f5`, worktree `/home/ubuntu/workspace/worktrees/codex-epic-f4`, on top of
  F4a (`a1795f6081`).
- Logs in this directory: `f4b-red.log`, `f4b-green.log`, `f4b-verify.log`,
  `f4b-timing-sequential.log`.

## 1. What changed

**`list_class_creation_roster() -> Result<ClassCreationRosterResponse, String>`**
(`apps/desktop/src-tauri/src/character_hub.rs`, registered in `main.rs`).

- The command reads `class_census::class_creation_roster()` (F4a). It returns `classes` (the
  `offered` rows, grouped by census family, in registry order within a family), `withheld`
  (every other census id, each with its named `RosterReason`), and `diagnostics`.
- `diagnostics` has one line per class withheld for a gap (`not_computed` or
  `hit_die_absent`). A healthy checkout has none. Prestige and Ex-* rows are withheld by rule,
  not as a gap, so they appear only in `withheld`.
- There are two `Err` cases, and neither returns an empty picker: the census cannot be swept
  (the sweep fixture fails to load, and the error gives its path), or the sweep offers no class
  at all.
- It differs from `list_race_creation_roster`, which returns an empty list plus diagnostics.
  This command returns `Err` instead, because the step brief asks for that shape.

**`list_level_up_class_options({characterId}) -> Result<LevelUpClassOptionsResponse, String>`**
(registered in `main.rs`). The body is `build_level_up_class_options(&CharacterInput)`.

- **`advance`**: each held class below its own census `max_level`. A held class that is at its
  max, or that is not a census id, gets a named line in `diagnostics`.
- **`add_base`**: each creation-roster class the character does not hold.
- **`add_prestige`**: each census prestige class the character does not hold.
  - Every one is offered, whether or not its requirements are met (ruling §9.2).
  - Each carries `entry_requirements`: one line per top-level term of its converted gate, as
    `{text, status: met|unmet|situational, condition}`, plus `requirements_all_met`. That field
    is a note, never a gate.
- **Level cap**: at character level 20 (`CHARACTER_LEVEL_CAP`), all three lists are empty,
  `at_level_cap` is true, and a diagnostic says why.

**Engine side (`src/rules_core/class_census.rs`).**

- **`prestige_entry_requirements(package, entry, held, facts)`** reuses the census translator:
  - The term split is `top_level_terms`.
  - The bookkeeping filter is `classify_term`. The converter's own-level ceiling
    `ClassLevel(self) <= max` and the `Var == 0` archetype switch are not PF1 requirements, so
    they are not printed.
  - Each term is judged against the real character by the engine's own `evaluate_applies`, and
    worded by `level_up_option_filter::describe_gate`. The feat option filter and the sheet use
    the same evaluator, the same held set and the same facts.
  - The census judges the same gate against a synthetic carrier. The desktop judges it against
    the build the player actually has.
- `prestige_entry_gate_in(package, …)`: `prestige_applies_gate` now delegates to this function.
  One lookup serves both paths.
- `class_roster_reasons()`: a per-process cache of `(entry, RosterReason)`. It is the one sweep
  that both `class_creation_roster` and the desktop's `withheld` list read.
- `roster_display_name` is now `pub`.

## 2. Roster cost, measured, and the choice (F4a §6 carry)

Command (both rows): `cargo test --locked -j 8 --manifest-path
apps/desktop/src-tauri/Cargo.toml list_ -- --test-threads=8 --nocapture`, debug build. Time is
the first `build_class_creation_roster()` call in the process: 63 non-prestige classes × their
levels, 1,197 engine computations.

| sweep | wall time | log |
|---|---|---|
| sequential (F4a's `roster_reasons`) | 463.1 s | `f4b-timing-sequential.log` |
| scoped threads, `min(available_parallelism, 8)` (this commit) | 83.1 s (79.97 s on an earlier run) | `f4b-green.log` |

The Create picker cannot wait 463 s. The sweep stays live and engine-derived. A generated
roster artifact would be a second copy that can drift. Three changes make the live sweep usable:

1. **Parallel sweep.** `roster_reasons` sweeps on up to `ROSTER_SWEEP_THREADS` (8, the memory
   guard) scoped threads. Each class's sweep is independent, and the output order is the census
   id order either way. The census tests (35 of 35) and the roster-equals-census test below
   show that the output is unchanged.
2. **Startup warm-up.** `main.rs`'s `setup` starts `class_creation_roster()` on a background
   thread, so the picker normally reads a warm cache. A failed sweep is not swallowed: the
   command returns the same `Err` when it is called.
3. **`#[tauri::command(async)]`** on both commands. A cold first call runs off the UI thread.

Not measured: release-build wall time (no release build was run in this step).

## 3. RED → GREEN

| test | RED (`f4b-red.log`) | GREEN (`f4b-green.log`) |
|---|---|---|
| `list_class_creation_roster_offers_exactly_the_census_roster` | compile failure: `build_class_creation_roster` absent | ok |
| `list_level_up_class_options_offers_prestige_with_printed_requirements` | compile failure: `build_level_up_class_options` absent | ok |
| `list_level_up_class_options_advances_a_held_prestige_class_to_its_own_max_and_keeps_the_cap` | same | ok |

What the roster test asserts. Denominator: the 137 census ids.

- The set of roster ids equals the set of `census-f4a.json` rows with `in_desktop_roster: true`
  (compared by id, not just by count). The census's own `roster_offered` equals its
  `in_desktop_roster` rows. Printed: **59 offered, 78 withheld, 0 diagnostics; census
  in_desktop_roster 59**. 59 + 78 = 137.
- No offered id is a census prestige id or `class:ex_*`. Every offered class prints a hit die,
  a label and a family label.
- Families are contiguous: no family appears in two runs.
- Every withheld reason is one of `prestige | ex_state | not_computed | hit_die_absent`.
- **Every offered class is Computed through the desktop's own create path**
  (`compose_character_input` → `build_pilot_headless_receipt`) at level 1 and at its own
  `max_level`: 59 × 2 = 118 builds, 0 Blocked.

What the level-up test asserts, for a Human Fighter 6 (the desktop fixture):

- advance 1 (Fighter → 7); add_base 58 (the 59 roster classes minus the held Fighter); add_prestige
  **74 of 74** census prestige classes; 0 diagnostics.
- The Arcane Archer's printed entry lines, with their notes (from `f4b-green.log`):

| requirement (rule's words) | note |
|---|---|
| at least 2 of: requires Point-Blank Shot (attack), requires Precise Shot | unmet |
| at least 1 of: requires Weapon Focus and requires longbow chosen for …weapon focus, requires Weapon Focus and requires shortbow chosen for …weapon focus | unmet |
| highest arcane spell level at least 1 | unmet |
| base attack bonus at least 6 | met |

- The Arcane Archer is offered even though three of its four lines are unmet, and
  `requirements_all_met` is false. The `arcane_archer level at most 10` bookkeeping row is not
  printed. Every line in all 74 options is non-empty and is met, unmet or situational.
- For Fighter 6 / Arcane Archer 1, advance is `[fighter → 7, arcane_archer → 2]`, and Arcane
  Archer is no longer in `add_prestige`. For Fighter 20, `at_level_cap` is true and all three
  lists are empty.

**The F2b guard is still non-vacuous.** `create_character_at_root_grants_no_wealth_when_the_build_is_blocked`
passes in the full desktop run. Its prestige-alone Eldritch Knight fixture is still Blocked by
`prestige_class.requires_base_class_levels`, and the test asserts that.

## 4. Verification (`f4b-verify.log`)

| gate | command | result |
|---|---|---|
| desktop clippy | `cargo clippy --locked --tests -j 8 --manifest-path apps/desktop/src-tauri/Cargo.toml -- -D warnings` | exit 0 |
| root clippy | `cargo clippy --locked --tests -j 8 -- -D warnings` | exit 0 |
| census tests | `cargo test --locked -j 8 --lib class_census -- --test-threads=8` | 35 passed, 0 failed |
| full desktop | `cargo test --locked -j 8 --no-fail-fast --manifest-path apps/desktop/src-tauri/Cargo.toml -- --test-threads=8` | 620 passed, 0 failed (F4a's 617 + these 3) |

## 5. Carried forward (named, not done here)

- **Frontend (F4c):** `classRoster.ts` + test, `CLASS_OPTIONS` reduced to a visible-notice
  fallback, `CreateCharacterForm.tsx`, `LevelUpDialog.tsx` (the three option groups; the
  requirement lines with their notes), and the ui-smoke mocks in
  `apps/desktop/scripts/ui-smoke/run.mjs`, which today mock `list_race_creation_roster` only.
- **Packaged build, by mechanism.** `class_census` reads `data/sheet_rules` through
  `live_sheet_rules()` (a baked `CARGO_MANIFEST_DIR` path) and reads its sweep fixture through
  `repo_root()`. In an installed app with no source checkout, both commands return the `Err`
  naming the missing path. They do not return an empty or fabricated roster. That is the
  existing census loader's posture, not a new one. Pointing it at the Tauri resource directory
  (as `codex_repo_root()` already does for `data/corpus`) is a separate change.
- The entry-requirement wording is `describe_gate`'s. It prints a `Chosen` option's choice id
  as words (`core rulebook:feat:weapon focus`), because the renderer has no label lookup for
  a choice. Fixing that would improve the feat option filter as well.
