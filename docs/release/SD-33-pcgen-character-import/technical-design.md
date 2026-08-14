---
canonical: true
owner: god-emporer
status: planning-ready (operator directive 2026-08-11)
date: 2026-08-11
canonical_branch: tranche/11
companion_to: ./technical-requirements.md
---

# SD-31 Technical Design

## 1. The format, read off a real file

Every structural claim below is read from the vendored fixture
`docs/release/GE-05-.../artifacts/pf1-crb-human-fighter-level1-provisional-ge05-e1-f2.pcg`
(sha256 pinned by `tests/ge05_vendored_pcg_fixtures.rs`). It is **not** inferred from PCGen
documentation.

`.pcg` is a line-oriented text format with four nesting levels:

```
PCGVERSION:2.0                          # 1. KEY:value
GAMEMODE:Pathfinder_RPG
STAT:STR|SCORE:16                       # 2. pipe-delimited subtokens
SKILL:Climb|OUTPUTORDER:1|CLASSBOUGHT:[CLASS:Fighter|RANKS:1.0|COST:1|CLASSSKILL:Y]
                                        # 3. bracketed group holding its own subtokens
ABILITY:FEAT|TYPE:NORMAL|CATEGORY:FEAT|KEY:Weapon Focus|APPLIEDTO:Longsword|TYPE:Combat.WeaponFocus
                                        # 4. repeated keys (TYPE twice), order-significant
EQUIPSET:Armor|ID:0.1.1|VALUE:Chain Shirt|QUANTITY:1.0
                                        # dotted ID encodes a tree
# Character Feats                       # comments and blank sections are structural
```

Four properties the parser must respect, each of which would break a naive `split(':')`:

- **Keys repeat within a line** (`TYPE` appears twice above). The model keeps an ordered
  multimap, not a `HashMap`.
- **Keys repeat across lines** (`SKILL`, `ABILITY`, `EQUIPNAME`, `EQUIPSET`). These are
  collections, not scalars.
- **Bracketed groups nest subtokens** and may themselves contain `|` and `:`.
- **`EQUIPSET` dotted IDs (`0.1`, `0.1.1`, `0.1.2`) encode a tree** — which item is worn where.
  `CALCEQUIPSET:0.1` names the active set.

## 2. Three layers, one direction

```
 .pcg bytes
     │
     ├─ Layer 1  src/pcgen_character/parser/     → PcgDocument   (syntax only, no game meaning)
     │
     ├─ Layer 2  src/pcgen_character/model/      → PcgCharacter  (typed records, still PCGen's vocabulary)
     │
     ├─ Layer 3  src/pcgen_character/resolve/    → ResolvedImport { CreateCharacterRequest, FidelityReport }
     │                                             (Codex's vocabulary — corpus keys)
     │
     └─ existing character_hub::import path      → mint id, recompute via engine, Saved | Blocked
```

The layering rule: **each layer may fail only in its own vocabulary.** Layer 1 reports
malformed syntax. Layer 2 reports structurally-valid-but-nonsensical records. Layer 3 reports
unresolvable game content. A parse error must never be reported as a missing feat, and a
missing feat must never be reported as a parse error — that conflation is what makes import
bugs unfixable by users.

### Why Layer 3 stops where it does

Layer 3 produces a `CreateCharacterRequest` — the **same struct the existing
`create_character` and `import_character` paths already consume**
(`character_hub.rs:386`). It does not compute anything. Ability modifiers, saves, AC, BAB, hit
points are all recomputed by the real engine downstream.

This is deliberate and load-bearing. The `.pcg` *contains* PCGen's computed values
(`HITPOINTS:10`, `SKILLSGAINED:3`). Copying them would produce a character whose numbers came
from PCGen and whose future edits come from Codex — two engines silently disagreeing inside
one sheet. **SD-31 imports inputs, never outputs.** PCGen's computed values are read for one
purpose only: parity verification (`acceptance-and-verification.md §2`).

## 3. Reference resolution

Layer 3's whole job is turning PCGen's names into corpus keys. Each token kind has a
resolution rule and a declared failure mode.

| `.pcg` token | Resolves against | On failure |
|---|---|---|
| `RACE:Human` | race creation roster (`list_race_creation_roster`) | mechanically significant → **Blocked** |
| `CLASS:Fighter\|LEVEL:1` | class roster + `list_class_spell_levels` | mechanically significant → **Blocked** |
| `STAT:STR\|SCORE:16` | none — direct input | malformed → **Blocked** |
| `ABILITY:FEAT\|KEY:Power Attack` | feat catalog | mechanically significant → **Blocked** |
| `…\|APPLIEDTO:Longsword` | the feat's parameter domain | unresolved parameter → **Blocked** |
| `SKILL:Climb\|CLASSBOUGHT:[…RANKS:1.0…]` | skill catalog | mechanically significant → **Blocked** |
| `EQUIPNAME:Chain Shirt` + `EQMOD=STEEL` | equipment catalog + equipmod tables | mechanically significant → **Blocked** |
| `TEMPLATESAPPLIED:[NAME:…]` | *not modelled* | declared unsupported → **Blocked**, named |
| `LANGUAGE:Common` | language list | cosmetic-adjacent → report, do not block |
| `HEIGHT`, `EYECOLOR`, `PHOBIAS`, … | none | exempt, carried verbatim (`TR-31-004`) |

**`APPLIEDTO` is the subtle one.** `Weapon Focus|APPLIEDTO:Longsword` is not the same feat as
`Weapon Focus|APPLIEDTO:Greataxe`, and importing the feat while dropping its parameter yields a
character that looks right and computes wrong. Parameterized feats resolve as a **pair**, or
they do not resolve.

**`TEMPLATESAPPLIED` is deliberately unsupported in v1.** The fixture carries
`[NAME:Creature with Class Levels]` and `[NAME:Human]`, which are PCGen bookkeeping rather than
player-visible content — but templates in general can carry real mechanics, and Codex has no
template model. Rather than special-casing the two benign values and silently accepting a
future dangerous one, v1 declares the whole token kind unsupported and names it. The
allowlist for known-inert templates is a forward-scope entry.

## 4. The fidelity report

```rust
pub struct FidelityReport {
    pub resolved: Vec<ResolvedRef>,      // token → corpus key
    pub unresolved: Vec<UnresolvedRef>,  // token → reason, raw text preserved
    pub exempt: Vec<ExemptField>,        // cosmetic, carried verbatim
}

pub enum UnresolvedReason {
    UnknownBook { book: String },
    RecordNotIngested { kind: String, name: String },
    UnsupportedTokenKind { token: String },
    ParameterUnresolved { feat: String, applied_to: String },
    GameModeUnsupported { mode: String },
}
```

Two consumers, one structure:

- **The user** sees it as the mapping-review screen — what came across, what did not, and why,
  in the source file's own words so it can be matched against PCGen line by line.
- **`TR-31-005`** reads `unresolved` to decide `Saved` vs `Blocked`.

`RecordNotIngested` is the interesting variant: it is the honest answer to "why is my feat
missing" and it points at SD-29/SD-30's ingest lanes rather than at a bug. The report SHOULD
name the book so the answer is actionable.

## 5. The player surface

Three additions to `apps/desktop/src/`, all inside SD-31's partition:

1. **Import affordance** — extend the existing import entry point in
   `characterHub/LoadCharacterScreen.tsx` (which already wires the JSON import via
   `boundary/loadImportCharacter.ts`) with a `.pcg` file filter.
2. **`boundary/loadImportPcgenCharacter.ts`** — mirrors `loadImportCharacter.ts` exactly,
   invoking the new `import_pcgen_character` command. Same tagged `CreateCharacterOutcome`
   union every other character mutation returns.
3. **Mapping-review screen** — renders the `FidelityReport` *before* anything is persisted, and
   is the only place `TR-31-006`'s acknowledgement can be given.

The review screen is not optional polish. `TR-31-005` refuses lossy imports; without a surface
that explains *what* was lost, the refusal is a dead end the user cannot act on.

## 6. Where the new code lives

```
src/pcgen_character/
  mod.rs
  parser/        tokenizer.rs, document.rs      — Layer 1
  model/         character.rs, ability.rs,
                 equipment.rs, skill.rs         — Layer 2
  resolve/       resolver.rs, fidelity.rs,
                 request_builder.rs             — Layer 3
```

A new top-level module rather than a `src/pcgen_import/` submodule, for two reasons: it keeps
SD-31 off a tree SD-29 and SD-30 read (`TR-31-001`), and the two concerns are genuinely
different — `pcgen_import` ingests *rules data* at build time, `pcgen_character` reads *a
user's file* at runtime, with untrusted input and user-facing errors.

Reuse across that boundary is by **idiom, not linkage**: the tokenizer follows
`pcgen_import/lst_parser`'s established shape without depending on it. A shared tokenizer
would put SD-31 edits inside SD-29's read surface for a modest saving.
