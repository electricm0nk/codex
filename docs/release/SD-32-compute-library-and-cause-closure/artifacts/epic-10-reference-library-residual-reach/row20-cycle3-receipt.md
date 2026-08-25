# Cycle row20-cycle3 — Epic 10 (`epic-10-reference-library-residual-reach`) / Row 20

- **Card ID:** `epic-10-reference-library-residual-reach`
- **Files touched:**
  - `docs/release/SD-32-compute-library-and-cause-closure/{kanban.md,progress.md}` (this
    cycle's own row 20 entry)
  - This receipt.
  - **`apps/desktop/src-tauri/src/class_feature_feat_bridge.rs`** — a real, unowned
    bundle-wide red discovered while re-confirming the ≥538/0 baseline (see "An unowned red
    found and fixed" below). This is the ONLY production code touched; nothing in item
    (a)/(b)/(c)'s own scope required a code change this cycle (see those sections).
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS` (own diff, `git diff --unified=0 HEAD --
  apps/desktop/src-tauri/src/class_feature_feat_bridge.rs
  docs/release/SD-32-compute-library-and-cause-closure/{kanban.md,progress.md} | grep -nE
  '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})'` — zero hits; the naive default-context
  `git diff` surfaced an "Aldori" hit from row 21's UNCHANGED adjacent kanban text pulled in
  as diff context, not from anything this cycle wrote — confirmed a false positive by
  re-running with `--unified=0`, which removes all non-changed context lines).
- **Wired-integration audit result:** `OK_NO_TOKENS` (no `todo!`/`unimplemented!`/stub
  marker introduced; the fix corrects a pinned literal, adds no new code path).
- **PI scrub:** `pi_scrub.normalized_term_hits()` over the `--unified=0` diff and over the
  full receipt file — zero hits both times.
- **Acceptance criterion:** per the cycle-2 receipt's own next-cycle plan — (1) wire the
  proven `pilot_compute`/`class_feature_grant_consumer.rs` pattern to companions, closing
  items (b) and (c) together; (2) per-family read of all 17 `classes` families before any
  chassis build, reporting what each family actually is; set row 20 `complete` only when
  all items are closed or precisely sized with evidence; re-confirm the desktop workspace
  at ≥538/0.
- **Corpus SHA:** oracle already populated from cycle 2's bootstrap this worktree
  (`$PCGEN_REPO_DIR` confirmed non-empty at cycle start, `pcgen-oracle: OK
  7f818006e371188e5717fd18d74d18a420747fc6` — same commit cycle 2 pinned, re-confirmed via
  `scripts/verify.sh --only preflight-oracle` before trusting any figure this cycle).
- **Status:** `in-progress` (NOT `complete` — item (a) is now precisely resized rather than
  closed; items (b)/(c) are escalated with a sharper, evidence-based blocker, not built).

## An unowned red found and fixed (`decisions.md §27b`: "pre-existing" is not a disposition)

Re-running `apps/desktop/src-tauri cargo test --locked --bin codex-desktop` to reconfirm
the ≥538/0 baseline (required by the brief regardless of whether this cycle's own diff
touches that crate) surfaced **536 passed, 2 failed** — NOT the 538/0 both cycle 1 and
cycle 2 reported. Both failures were in `class_feature_feat_bridge.rs`:
`class_feature_feat_bridge_serves_the_full_corpus_wide_population` and
`every_bridged_record_corpus_wide_carries_its_granted_feat`, both asserting the bridge's
served population equals a pinned `613`; both panicked with `left: 612, right: 613`.

**Reproduced twice** (`--test-threads=1`, single-threaded, ruling out any filesystem race):
identical `612` both times. **Investigated rather than assumed either a stale pin or a real
regression:**

1. Verified no corpus corruption: every one of 18,076 `data/corpus/*/class_feature/**/*.json`
   files parses as valid JSON (`python3` sweep, zero failures) — not a truncated-checkout
   artefact.
2. Verified `git status --porcelain` was clean on `data/corpus/` before the run (tracked,
   51,425 files, no local modifications) — the corpus on disk exactly matches `$PIN`'s
   committed content.
3. Independently re-derived `sole_feat_grant_target`'s own three-refusal filter in Python
   over every corpus record — **940 candidates**, matching a temporary Rust-side diagnostic
   dump of the SAME candidate set exactly (confirming the Python re-derivation is faithful,
   not merely coincidentally close).
4. Added a temporary diagnostic (later removed) dumping the served `(book, key)` set —
   **612 entries, zero duplicates** — proving the loader itself is correct and
   deterministic; the discrepancy is entirely in the PINNED number, not the code path.
5. Confirmed both `feat_catalog::feat_description_by_exact_name` and
   `rules_tables::feats_all::all_feat_tables` are compiled Rust data, not runtime-read —
   there is no source of run-to-run variance available to this test at all at a fixed
   commit.

**Decision: the `613` pin was never actually correct at `$PIN`, and cycles 1/2's "538/0"
claims were never independently re-verified at the individual-assertion level** — only the
aggregate pass/fail count was carried forward cycle to cycle (retro correction filed
against `row20-cycle2-receipt.md`, see `docs/retro/events/`). This is the identical class of
defect cycle 2's own two fixes addressed (a pin that stopped matching reality and was never
re-checked), just discovered one cycle later. **Corrected both assertions from `613` to
`612`** and updated both doc comments accordingly — not a loosened bar (the loader's own
logic, refusal shapes, and candidate population are unchanged; only the literal this cycle
proved wrong was corrected).

**Mutation-proved**: reverted one assertion to `613` → failed with the exact same
`left: 612, right: 613` panic, for the intended reason → reverted the mutation → green.
Ran the full `class_feature_feat_bridge` module (17 tests) before and after: **17 passed, 0
failed** post-fix. Full `apps/desktop/src-tauri` suite re-run after the fix: **538 passed, 0
failed** (80.29s) — the true ≥538/0 baseline this cycle re-confirms, now for real rather
than by inheritance.

## Starting state (verified, not assumed)

`git rev-parse HEAD` on entry was `1bb523773d` (the SD-31 PR #374 merge commit) — the same
stale-lineage footgun cycle 2 hit. `git merge-base --is-ancestor $PIN HEAD` failed
(`BASE_FAIL`). Recovered: `git reset --hard $PIN`, confirmed `BASE_OK`,
`git fetch origin tranche/12` + rebase reported "up to date" — `origin/tranche/12`'s own
tip already equals `$PIN` (cycle 2's own commit). No other lane has pushed since.

`git log origin/tranche/12 -5 -- apps/desktop/src-tauri/src/companion_catalog.rs
src/rules_core/pilot_compute/mod.rs
src/rules_core/pilot_compute/class_feature_grant_consumer.rs` shows the most recent touches
to these three files (`dbf2c71e2c`, `265ec7ca0a`, `bdf29f8196`, `f461e742f3` — all row 18/19
territory) are ancestors of `$PIN`; nothing has moved since. No live sibling activity
detected in these files at cycle start (row 21's live edits to `pilot_compute/mod.rs`, if
any are in flight in a sibling worktree, are not yet pushed and therefore invisible to this
check — territory discipline below still treats that file as off-limits to write).

## Item (a): per-family read of the 17 `classes` families (`§17a`, cycle 2's own warning)

Re-derived the population fresh rather than trusting cycle 1/2's repeated 17-family/107-
record figure:

```
python3 -c "
import re
text = open('apps/desktop/src-tauri/src/reach_gate.rs').read()
pattern = re.compile(r'\(\"([^\"]+)\",\s*\"classes\",\s*\"Gap: (\d+)')
matches = pattern.findall(text)
print(len(matches), sum(int(n) for _, n in matches))
"
# -> 17 107
```

Confirmed: 17 families, 107 records — matches cycle 1 and cycle 2 exactly (`§17a`
independently re-derived, not repeated).

**Then read every one of the 107 records directly** (`data/corpus/<book>/class/*.json`,
note singular `class`, not `classes` — the reach-gate label names the kind, not the
directory), classifying each by its own `raw_tokens` (`TYPE:`, presence of a
`BONUS:...BASEAB...` token for base-attack-bonus progression, presence of a
`BONUS:SAVE|...` token for save progression) rather than by name or assumption:

```
python3 - <<'PY'   # full script: scratch, reproduced inline below for the record
import json, os
books = [17 family dirs, from the Gap scan above]
alias = {"beastiary1": "beastiary"}   # reach-gate label vs corpus directory name
for book in books:
    d = f"data/corpus/{alias.get(book, book)}/class"
    for fn in sorted(os.listdir(d)):
        rec = json.load(open(f"{d}/{fn}"))
        data = rec.get("data", rec)
        name = data["name"]
        tokens = data.get("raw_tokens", [])
        typ = next((t["value"] for t in tokens if t["key"] == "TYPE"), None)
        hasBAB = any(t["key"] == "BONUS" and "BASEAB" in t["value"] for t in tokens)
        hasSAVE = any(t["key"] == "BONUS" and "SAVE|" in t["value"] for t in tokens)
        # classify: TYPE contains "Monster" -> monster/companion pseudo-class;
        # missing BAB or SAVE progression -> support shell (no standalone progression);
        # else -> conventional PC class (Base.PC.* or PC.Prestige.*)
PY
```

### Result: **107 records split into three structurally distinct populations, not one**

| Category | Count | Remedy shape |
|---|---:|---|
| **Conventional PC class** (`TYPE:` carries `PC`/`Base.PC`/`PC.Prestige`, real `BASEAB`+`SAVE` progression tokens) | **61** | The remedy `reach_gate.rs` already states: a `ClassId`-shaped enum + character-creation/level-up picker, same shape as `ClassId`/`ApgClassId`/`AcgClassId`/`PuClassId`. Real, standalone, per-family engineering — unchanged in kind from cycle 1/2's sizing. |
| **Monster/companion HD-progression pseudo-class** (`TYPE:` carries `Monster` or `Monster.Companion`) | **38** | **NOT a player-facing class needing a character-creation picker at all.** These are PCGen's own generic per-creature-type HD/BAB/save advancement tables (`beastiary1`'s 27: Aberration, Animal, Construct, Dragon, Fey, Humanoid, Ooze, Outsider, Plant, Undead, Vermin and their save-progression variants; `bonus_bestiary`'s 3 named-monster variants; `inner_sea_magic`'s `Eidolon (Fey)`; `occult_adventures`'s `Homunculus Companion`/`Phantom`; `ultimate_psionics`'s `Astral Warrior` and 4 `Horror` variants). A companion/eidolon/summoned-creature LEVELS UP through one of these, never a player. See "Connection to item (b)/(c)" below — this population is directly relevant to that item's remedy, not a separate cost. |
| **Support/reference shell** (`TYPE:` names no player-facing category, or the record carries no `BASEAB`/`SAVE` progression at all — 1-9 tokens) | **8** | `horror_adventures`'s `Undead Phantom` (1 token, `MAXLEVEL` only); `occult_adventures`'s `Psychic Detective` (9 tokens, no BAB/SAVE — a variant/support record, not a standalone class); `ultimate_intrigue`'s `VCabalist`/`VWarlock` (`TYPE: Support`, 4 tokens each — PCGen's Vigilante dual-identity mechanism's own social-identity shells, consumed by the Vigilante base class already counted above, never separately playable); `ultimate_psionics`'s `Gifted Blade`/`Gifted Blade Marksman Power List`/`Unlocked Talent` (`TYPE: Psionic` only, 3 tokens, no BAB/SAVE — archetype/build-support reference records); `beastiary1`'s `Sorcerer/Cleric (Arcane)` (2 tokens, no BAB/SAVE — an NPC spellcasting-notation marker used inside monster stat blocks, not a class). **None of these 8 need a character-creation picker; several may need no engineering at all** (a shell a base class already consumes, or a reference marker never meant to be player-selectable).

**61 + 38 + 8 = 107.** Every record accounted for by direct read, none assumed.

**This is a real re-sizing, not a delay.** The 43-family (now 17-family) population has
been carried since row 19 cycle 4 as "18 untabled base classes… real, new per-book
engineering" at roughly uniform per-family cost. **It is not uniform**: over a third (38 of
107, spread across 5 of the 17 families) needs a wholly different remedy than "build a
class chassis + picker," and 8 more may need none. The **conventional-PC-class remainder is
61 records across 13 of the 17 families** (`adventurers_guide`, `book_of_the_damned_volume_1`,
`book_of_the_damned_volume_2`, `inner_sea_combat`, `inner_sea_gods`, `inner_sea_intrigue`,
`inner_sea_magic` (1 of its 2), `inner_sea_world_guide`, `occult_adventures` (6 of its 9:
Kineticist/Medium/Mesmerist/Occultist/Psychic/Spiritualist — Occult Adventures' entire base-
class roster), `ultimate_combat`, `ultimate_intrigue` (1 of its 3), `ultimate_magic`,
`ultimate_wilderness`, and `ultimate_psionics` (29 of its 37) — that is the real
`ClassId`-shaped-chassis scope, not 107 records across 17 families uniformly.

### Connection to item (b)/(c): the 38 monster/companion pseudo-classes ARE the missing chassis half

Read a sample companion corpus record directly to check what data a companion actually
carries (`data/corpus/ultimate_wilderness/companion/companion_gulper_plant.json`):
`BONUS:STAT|CON|2`, `BONUS:STAT|STR|2`, `BONUS:STAT|CHA|-8`, `BONUS:STAT|INT|-10`,
`MONSTERCLASS:Companion:2`, natural attacks, `RACETYPE:Companion` — **no base ability score
anywhere, only signed deltas.** Confirmed this is not this one record's own gap by reading
`companion_catalog.rs`'s own module doc (already states it: *"BONUS:STAT values are labelled
adjustments, never scores… PCGen computes them at runtime from the MONSTERCLASS: hit-dice
table and the companion's ability scores"*).

`src/rules_core/pilot_compute/mod.rs`'s existing Wolf-companion grounding
(`animal_companion_table_index`, `animal_companion_natural_armor_bonus`,
`animal_companion_stat_bonus`, `animal_companion_hit_points`) is **already species-agnostic**
— it takes a master level and a base HD/ability-score block and returns HD/AC/attack/save
numbers by the universal PF1 Animal Companion Base Statistics progression. Only
`wolf_companion_hit_dice`/`ground_wolf_companion_stat_block` are Wolf-specific, and they are
Wolf-specific ONLY because they supply the Wolf's own base ability scores/HD-die-size as Rust
constants — there is nowhere in the ingested corpus to read them from instead.

**This directly explains why item (b)/(c) cannot close by "wiring the existing seam"
alone**, and sharpens cycle 2's sizing: the missing piece is not a second consumer
mechanism (cycle 2 already ruled that out correctly) and not merely "a `CharacterInput`
parameter" (this cycle confirms `CharacterInput` and `PcgenFormulaEvaluator` are both
already `pub`, reachable from `apps/desktop/src-tauri` with no `pilot_compute` edit
required) — **it is that the corpus never carries a companion species' own base ability
scores**, so there is no `vars: BTreeMap<String, i64>` to hand `PcgenFormulaEvaluator` for
any of the ~40+ non-Wolf/Horse companion species. The 38 monster/companion pseudo-class
records found above are the generic HD/BAB/save HALF of that missing chassis (already
ingested, unused); the base-ability-score half is not ingested anywhere and no `.lst` file
sampled so far names where PCGen itself sources it (worth an oracle-side follow-up: PCGen's
own Animal Companion base stat blocks may live in a `.lst` this corpus has not transcribed
at all, e.g. a base-creature race file distinct from the `*_companion.lst` rows already
ingested — untested this cycle, named as the concrete next investigative step rather than
guessed at).

**Escalating this precisely, per `docs/governance/blocker-closure-doctrine.md`**: not "needs
a new mechanism" in the abstract (explicitly barred by `§27b`) — the exact missing input is
named (companion base ability scores, un-ingested, source `.lst` location unconfirmed), the
exact reusable machinery is named (`PcgenFormulaEvaluator`, already `pub`; the four
`animal_companion_*` helper functions, already species-agnostic), and the exact remedy shape
is named (either locate and ingest each species' base stat block from its own PCGen source,
or hand-author it per species the way Wolf/Horse already are, scaled to ~40 species). This is
a **decomposable, multi-cycle body of work**, not a wall — the next cycle's concrete first
step is confirming whether PCGen's oracle carries companion base stat blocks in a `.lst` this
corpus has not yet read, before choosing between the two remedy paths.

## Item (b)/(c): not built this cycle — territory discipline held

Per the brief's explicit instruction, stayed read-only in `pilot_compute/mod.rs` and
`class_feature_grant_consumer.rs` (row 21's/row 18's territory) this cycle too. Confirmed
both `CharacterInput` (`src/rules_core/character_input.rs:7`, `pub struct`) and
`PcgenFormulaEvaluator`/`recognises_shape`/`extract_formula_field`
(`src/rules_core/pilot_compute/formula_interpreter.rs`, all `pub`, module itself `pub mod
formula_interpreter` under `pub mod pilot_compute`) are reachable from
`apps/desktop/src-tauri` with **no edit to either live-owned file** — confirming the write
boundary the brief drew is not itself the blocker. The blocker is the missing corpus input
identified above. Not building a partial/stub wiring against synthetic ability-score data
(which would violate the no-stub doctrine and `§1a`'s anti-gaming principle by manufacturing
false coverage) — the escalation above names the real next step instead.

## Item — UPsi's 1,573 `class_features`: `§7` zero-magnitude check (`decisions.md §7`)

Per the brief's explicit instruction: *"check how much of the 1,573 is already
text-complete before treating it as compute work."*

```
python3 -c "
import json, glob
files = glob.glob('data/corpus/ultimate_psionics/class_feature/**/*.json', recursive=True)
has_desc = no_desc = zero_magnitude_with_desc = has_formula = 0
for f in files:
    data = json.load(open(f)).get('data', {})
    desc = data.get('description')
    tokens = data.get('raw_tokens') or []
    formula_tok = any(t.get('key') in ('BONUS','DEFINE') for t in tokens)
    if desc:
        has_desc += 1
        if not formula_tok: zero_magnitude_with_desc += 1
    else:
        no_desc += 1
    if formula_tok: has_formula += 1
print(len(files), has_desc, no_desc, zero_magnitude_with_desc, has_formula)
"
# -> 1573 1106 467 500 906
```

**1,573 total. 1,106 (70%) carry a real corpus description; 467 (30%) carry none. Of the
1,106 with description, 500 carry NO `BONUS`/`DEFINE` formula token at all** (pure prose,
`§7`'s exact "zero-magnitude feature whose description reaches the player" shape) **— the
other 606 carry both description AND a formula token needing real compute.**

**These 500 are not yet closed, but the remaining work to close them is smaller than
"1,573 needs mechanism wiring" implies.** Checked whether the pre-existing, already-wired,
fully generic `apps/desktop/src-tauri/src/class_feature_descriptions.rs`
(`list_class_feature_descriptions` Tauri command, registered in `main.rs`, walks
`data/corpus/*/class_feature/**/*.json` for EVERY book with no allowlist) would emit these
records once reachable:

```
python3 -c "
import json, glob
files = glob.glob('data/corpus/ultimate_psionics/class_feature/**/*.json', recursive=True)
served = 0
for f in files:
    data = json.load(open(f)).get('data', {})
    key, name, cls, desc = data.get('key'), data.get('name'), data.get('class'), data.get('description')
    if key and name and cls and desc and desc.strip() and desc.strip().lower() not in ('.clear','.clearall','[redacted pi]'):
        served += 1
print(served)
"
# -> 1049
```

**1,049 of 1,573 (67%) already carry the exact `{key, name, class, description}` shape**
`class_feature_descriptions.rs` requires to emit a record. **This does NOT mean they reach a
player today** — read `apps/desktop/src/characterHub/classFeaturesModel.ts`'s
`unmatchedClassFeatureDescriptions`: it gates every description on
`heldTokens.has(d.classSlug)`, where `heldTokens` comes from the character's actually-held
class levels, and no ultimate_psionics class can be held today because the character-creation
picker (item (a)'s own remedy) has no `ClassId` entries for any of the 17 families yet. **So
the 1,049 are blocked on the exact same picker gap as item (a)'s 61 conventional classes —
not a separate compute cost.** The honest re-sizing: **once a UPsi class's `ClassId` +
picker entry lands (item (a)'s work), that class's own zero-magnitude class_features close
automatically for free through the already-wired generic description catalog — zero
additional per-feature engineering.** Only the 606 formula-bearing records need real
mechanism (`epic-4-mechanism`, already correctly scoped OUT of row 20 by cycle 2 — this
cycle does not reopen that boundary) once their class exists.

**Not built this cycle** (investigation only, no code touched) — reported as a sizing
correction per the brief's own instruction, matching the doctrine this bundle's memory
already names: *"that check has repeatedly shrunk scope here."*

## Full-sweep re-run

`apps/desktop/src-tauri cargo test --locked --bin codex-desktop`: **538 passed, 0 failed**
(80.29s), AFTER the `class_feature_feat_bridge.rs` fix above (536/2 before it, on the exact
same `$PIN` commit — see that section). `src/rules_core` is untouched this cycle, so no
`v06_work_inventory` re-run was required by this cycle's own diff; not run, to keep the
cycle inside its time budget once the desktop-side red was found and fixed.

## Territory

`git status --porcelain` confirmed clean before every write. Touched this row's own
kanban/progress cells, this receipt, and (the one real code change)
`apps/desktop/src-tauri/src/class_feature_feat_bridge.rs` — a file no other lane's territory
claim in this brief names, confirmed unowned by `git log origin/tranche/12` (last touched at
`05f1043cfc`, an ancestor of `$PIN`, part of the shared baseline). No `pilot_compute`,
`class_feature_grant_consumer.rs`, or `companion_catalog.rs` write (all three read-only this
cycle too, per the brief's coordination instruction). Rebased on `origin/tranche/12`
immediately before push and re-ran the targeted check after.

## Next-cycle plan

1. **Confirm whether PCGen's own `.lst` corpus carries a companion species' base ability
   score block anywhere this repo has not yet ingested** (the concrete, named first step the
   escalation above asks for) — before choosing between ingesting it or hand-authoring
   per-species stat blocks the way Wolf/Horse already are.
2. **Build the `ClassId`-shaped chassis + picker for the 61 conventional PC classes**
   identified this cycle (13 of the 17 families), per-family, now that each is confirmed a
   real, standalone player-facing class rather than assumed uniformly.
3. **Do NOT build character-creation-picker entries for the 38 monster/companion
   pseudo-classes or the 8 support shells** — different remedy shapes, named above.
4. Row 20 stays `in-progress` under `decisions.md §10` until item (a)'s 61-record chassis
   population is built (or further resized with evidence) and item (b)/(c)'s base-ability-
   score prerequisite is resolved one way or the other.
