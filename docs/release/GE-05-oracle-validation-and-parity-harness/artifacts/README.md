---
title: GE-05 — Oracle Validation and Parity Harness (vendored build artifacts)
status: repo-resident build fixtures
purpose: "The two PCGen `.pcg` pilot-case character saves that the GE-05 parity harness compares against. They are build inputs for `tests/sd26_pilot_case_verification.rs` and `tests/v06_wizard_pilot_case_verification.rs`, so they live in this build's own artifact folder rather than being referenced from an external source."
date: 2026-08-01
canonical_branch: tranche/7-1
mirror_of: ~/workspace/programs/codex/requirements/GE-05-oracle-validation-and-parity-harness/artifacts/
---

# GE-05 — Vendored build artifacts

## 1. Why these files are committed

Both GE-05 parity suites previously reached their `.pcg` fixtures through a
hardcoded absolute path into another machine's *planning* tree
(`/home/ubuntu/workspace/programs/codex/requirements/GE-05-.../artifacts/`),
with **no environment override of any kind**. Neither suite could run
anywhere except that one box, and both failed with a bare "expected the real
… fixture at …" panic everywhere else.

Per operator ruling:

> "This is why we always include artifacts needed for the build in the
> artifact folder for the build instead of referring to an external source."

The fixtures are now committed here, in the GE-05 build's own artifact
folder, alongside the runtime-evidence receipts that document the engine runs
which produced and validated them. Both suites resolve them repo-relative.

This matches the precedent already set for the SD-27 book-parity fixtures
(`data/corpus/<book>/_parity/*.pcg`), which are likewise repo-resident.

## 2. Inventory

| File | Size | sha256 |
|---|---|---|
| `pf1-crb-human-fighter-level1-provisional-ge05-e1-f2.pcg` | 3242 B | `d0c6b2a2e9c190d0be97c20caf247b96108299340331d044547d9a57bdb64f4f` |
| `pf1-crb-human-wizard-level1-v06-alpha-swarm.pcg` | 3554 B | `e2bcdae8cfccecbf871f7c587d4af577a86b5466747e41b329d0e36ee777330b` |

Both digests are pinned in three places, all of which must agree:

- `tests/ge05_vendored_pcg_fixtures.rs` — the integrity and provenance suite
- `tests/sd26_pilot_case_verification.rs` — Fighter, checked before the PCGen run
- `tests/v06_wizard_pilot_case_verification.rs` — Wizard, checked before the PCGen run

A swapped, regenerated, or truncated fixture therefore fails loudly rather
than quietly shifting the parity numbers these suites publish as evidence.
**Changing a digest requires re-verifying the fixture against the real PCGen
engine first** — the digest is a record of verified content, not a checksum to
be refreshed.

## 3. Provenance

Both files are complete PCGen 2.0-format character saves
(`PCGVERSION:2.0` header through the `SUPPRESSBIOFIELDS:` trailer), written
against **PCGen v6.09.08.RC1**, `GAMEMODE:Pathfinder_RPG`,
`CAMPAIGN:Core Rulebook`.

### `pf1-crb-human-fighter-level1-provisional-ge05-e1-f2.pcg`

The GE-05 E1-F2 pilot case. Proven to load in the real PCGen engine by
`./ge05-e1-f2-runtime-output-attempt-3-2026-06-20.md` (`Starting PCGen
v6.09.08.RC1` … `BUILD SUCCESSFUL`). Completed during SD-26 Epic 2 to the
exact GE-06 deterministic input contract and renamed to the pilot's own
`case_id` so the `character.identity` parity dimension carries a genuine
same-character signal.

Human Fighter 1, `TN`. STR 16 (including the `+2 Strength` human ability
bonus) / DEX 14 / CON 14 / INT 10 / WIS 12 / CHA 8; 10 hp; Power Attack,
Dodge, Weapon Focus (Longsword); Climb, Intimidate and Swim at rank 1 as real
**class** skills (`CLASSSKILL:Y`); Longsword and Chain Shirt equipped, no
shield.

### `pf1-crb-human-wizard-level1-v06-alpha-swarm.pcg`

The v0.6 alpha-swarm Wizard pilot case, mirroring the Fighter proof. Built and
PCGen-verified against the real headless Gradle pipeline (BUILD SUCCESSFUL
twice) to match `compose_character_input`'s fixed Wizard-1 loadout
field-for-field.

Human Wizard 1 (`SUBCLASS:Evoker`, `SPELLBASE:INT`,
`PROHIBITED:Necromancy,Transmutation`), `TN`. Same ability array and same
Longsword + Chain Shirt kit as the Fighter — the two pilot cases differ only
by class posture. 6 hp; same three feats; Climb, Intimidate and Swim at rank 1
correctly marked **not** class skills (`CLASSSKILL:N`), which is what makes
the skill-posture comparison honest; the seeded level-0 `Light` cantrip
present in both the `Known Spells` and `Prepared Spells` books.

## 4. Path resolution

Both suites anchor the repo-relative path at the codex repo root via
`CARGO_MANIFEST_DIR`, following `ge08_workbench::resolve_package_path`'s
contract ("repo-relative paths anchor at the codex repo root"). That helper
lives in the separate `codex-desktop` crate and is not reachable from a
`codex` root-crate integration test, so the suites use the identical anchor
its fallback uses — the same in-crate pattern
`tests/sd27_advanced_race_guide_parity.rs` already established for its own
vendored `.pcg` fixture.

## 5. Running the parity suites

The `.pcg` fixtures resolve with no environment configuration. The suites
additionally invoke the real PCGen engine, which needs a PCGen checkout:

```
PCGEN_REPO_DIR=$HOME/workspace/repos/pcgen \
  cargo test --test sd26_pilot_case_verification \
             --test v06_wizard_pilot_case_verification
```

`tests/ge05_vendored_pcg_fixtures.rs` verifies the fixtures themselves and
needs no environment configuration and no PCGen checkout.
