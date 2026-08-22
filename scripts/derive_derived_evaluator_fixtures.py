#!/usr/bin/env python3
"""Derive the `derived` evaluator-vs-fixture expectations FROM THE CORPUS.

This is the documented derivation behind `tests/fixtures/rules_core/
derived-evaluator-fixtures.json`, the fixture the evaluator-vs-fixture check
(`tests/derived_evaluator_fixture_check.rs`) tests the engine against.

WHY THE PROVENANCE RULE MATTERS
-------------------------------
The former `SD-32-instrument-coverage-and-consumer-wiring/epic-breakdown.md` E6-F1 (absorbed into
`docs/release/SD-30-class-feature-archetype-bundle/`, package deleted per `decisions.md §50`; text
retrievable from git history at commit `b88b18fa3700125f992e67b0ae29e1d5b70de3c0`):
"A fixture generated from the evaluator's own output is worthless."
This repo has a live finding of exactly that shape (SD-29 Epic 10 F3 /
register C1.4a): frontend preview fixtures were hand-authored rules data with
nothing pinning them to the corpus, and one shipped wrong rules content to a
player.

So this script is constrained by construction:

* It reads the UPSTREAM PCGen `.lst` bytes (`$PCGEN_CORPUS_ROOT`, default
  `$HOME/workspace/repos/pcgen/data`) and nothing else for every VALUE it
  emits. No engine module is imported; no engine binary is run; no file under
  `data/corpus/` (this repo's ingested JSON, which is what the engine actually
  reads) is opened.
* It reads `docs/work-inventory.json` for IDENTITY ONLY -- which units are
  `derived` and held, and which `.lst` file and line each one came from.
  No magnitude, no status-derived number, and no engine-computed value is
  copied out of it. That file is engine-produced, so using any VALUE from it
  would break the independence this script exists to guarantee.

The two sources the check compares are therefore genuinely different artifacts
on genuinely different trees: the fixture is anchored to the upstream PCGen
`.lst`, while the engine evaluates this repo's own `data/corpus/**.json`
ingest of the same rows. A drift between the two makes the check RED, which
is the whole point -- `docs/retro` memory "generated artifacts mutated
post-hoc" is a recorded hazard with no test covering it until now.

WHAT `derived` MEANS, AND WHAT ITS BAR IS
-----------------------------------------
`src/rules_core/wiring_class.rs`: a row is `derived` when a magnitude-bearing
field's value is a function of a character/item scalar rather than a bare
literal. The dashboard's own `doneness_meaning` states the bar for a `derived`
unit: "needs an evaluator-vs-fixture check" -- the engine's evaluator output
must equal an independently stated expected value.

TOKEN FAMILY COVERED
--------------------
`BONUS:STAT|<ability list>|<value>[|<extra>...]` -- PCGen's ability-score
bonus token. Chosen because it is the only derived-bearing token family in the
corpus for which this engine has a real evaluator that produces a NUMBER
(`rules_core::equipment_effects::magic_items::compute_magic_items_effect`,
reached through the shipping `compute_equipment_effects` seam). Every other
`derived` token family found on held units is reported by `--report` with the
reason it is not covered; leaving a family uncovered is an honest outcome and
is recorded rather than papered over.

Derivation rule for this family, from PCGen's token grammar:

    BONUS:STAT|STR|2|TYPE=Enhancement
              ^^^ ^ ^^^^^^^^^^^^^^^^
              |   |  stacking type, does not change the magnitude
              |   value: the integer added to each named ability
              comma-separated ability list

    expected := { abilities: ["STR"], bonus: 2 }

A row carrying several `BONUS:STAT` fields contributes its FIRST one, which is
the field PCGen itself applies first and the field the engine's evaluator
reads; rows like that are flagged `multi_stat_row` in the fixture so the check
can be widened deliberately rather than by accident.

RUN
---
    python3 scripts/derive_derived_evaluator_fixtures.py            # write
    python3 scripts/derive_derived_evaluator_fixtures.py --report   # survey only
"""

import argparse
import collections
import hashlib
import json
import os
import re
import sys

REPO = os.path.abspath(os.path.join(os.path.dirname(__file__), ".."))
INVENTORY = os.path.join(REPO, "docs", "work-inventory.json")
FIXTURE = os.path.join(
    REPO, "tests", "fixtures", "rules_core", "derived-evaluator-fixtures.json"
)
CORPUS_RELATIVE_ROOT = "pathfinder/paizo/roleplaying_game"

# Transcribed from `src/rules_core/wiring_class.rs` (MAGNITUDE_TOKENS,
# SCALARS_SUBSTRING, SCALARS_WORD, has_arith). Used ONLY to find which field of
# a row carries the derived magnitude -- never to decide a unit's class, which
# `docs/work-inventory.json` already carries. Reproducing the field-level scan
# here is what lets the fixture name the exact corpus field it was derived from.
MAGNITUDE_TOKENS = (
    "BONUS:", "TEMPBONUS:", "DEFINE:", "COST:", "WT:", "CR:", "AC:", "ACCHECK:",
    "DAMAGE:", "CRITMULT:", "CRITRANGE:", "RANGE:", "REACH:", "MOVE:", "HITDIE:",
    "LEVELADJUSTMENT:", "SR:", "DR:", "SPELLFAILURE:", "STAT:",
)
SCALARS_SUBSTRING = ("CASTERLEVEL", "CLASSLEVEL", "TOTALLEVELS", "TOTALLEVEL",
                     "PLUSTOTAL", "SPELLLEVEL")
SCALARS_WORD = ("BAB", "HD", "STR", "DEX", "CON", "INT", "WIS", "CHA", "TL",
                "CL", "RACESIZE")
# SD31-W29-LANE1 (corrected by wave 29 INTEGRATION after adversarial
# review -- see `todo/defects.md` D7 and `todo/sweeps.md` S6 for the full
# correction record): the committed fixture's own 94 `entries` reproducibly
# drop to 0 on a fresh run from the committed state. The dominant cause was
# the write step rebuilding the whole document from scratch and only
# preserving a hardcoded `"monster_"` prefix (fixed below, in `main()`,
# by preserving every key BY EXCLUSION instead) -- that alone accounts for
# all 2,110 lost rows across all 8 families, INCLUDING every one of this
# family's own 94 `entries` (83 of which are carried forward rather than
# freshly re-derived; see `main()`'s carry-forward-merge comment for why).
#
# `fixture-verified` (the stamp `apply_done_rung_stamps()`,
# `src/bin/v06_work_inventory.rs`, writes onto a `derived` unit once THIS
# fixture already verified it) being missing from this tuple is a SEPARATE,
# real, independently-necessary bug: without it, a `derived` unit newly
# verified since the fixture was last regenerated stays permanently
# invisible to this generator, one that the carry-forward merge alone
# cannot mask forever (a unit dropped from the LIVE selection because its
# own status is never recognised is never carried forward either, since
# carry-forward only preserves rows the PREVIOUS run already wrote). Fixed
# by adding it back.
#
# `literal-verified` was ALSO added here by the original lane 1 fix, on the
# (wrong) theory that it restored the 83 reclassified-to-`static` `entries`
# rows. It cannot: `apply_done_rung_stamps()` stamps `literal-verified`
# ONLY inside its `WiringClass::Static` arm, and this generator's own
# selection below is gated on `wiring_class == "derived"` -- a
# `derived`+`literal-verified` unit is therefore structurally impossible
# (confirmed directly against the live inventory: 0 such units exist), so
# `literal-verified` in this tuple was unreachable dead configuration.
# Removed.
#
# Reproduced by running this script ONCE from the committed state (the
# destruction is complete on the very first run -- a SECOND run then diffs
# clean against the first, which is exactly why a twice-run-diff test
# cannot catch this class of bug).
# `scripts/tests/test_derive_derived_evaluator_fixtures.py` pins this as a
# run-once-vs-committed regression instead; see that file's module
# docstring for the full twice-run-diff-is-blind argument. It also pins
# (`at_least_one_entry_is_freshly_derived_this_run_not_merely_carried_
# forward`) that `fixture-verified` specifically stays in this tuple: the
# carry-forward merge in `main()` makes a REGRESSION here invisible to
# every other test in this file (reverting to the original 3-status tuple
# leaves the committed fixture byte-identical AND every other test green,
# because the generator derives zero fresh rows and the merge silently
# absorbs that) -- so this is the one guard load-bearing enough that it is
# also asserted directly, immediately below.
HELD_STATUSES = (
    "ingested-magnitude", "grounded", "text-complete", "fixture-verified",
)
assert "fixture-verified" in HELD_STATUSES, (
    "fixture-verified must stay in HELD_STATUSES -- its absence is silently "
    "masked by the carry-forward merge in main() and caught by nothing else "
    "(todo/defects.md D7, todo/sweeps.md S6)"
)

# The engine reads its equipment records from `data/corpus/<book>/equipment/`.
# Only `equipment`-family kinds reach `compute_equipment_effects` at all.
EVALUATED_KINDS = ("equipment", "equipment_modifier")


def family_entry_counts(doc):
    """`{top-level list-valued key: row count}` for a fixture document.

    Every `*_entries` family this script owns or preserves is a JSON list at
    the top level; every sidecar (`*_token_family`, `*_derivation`, ...) is a
    string or dict and is ignored here on purpose -- this function answers
    exactly one question, "how many rows does each family carry," nothing
    about their content.
    """
    return {k: len(v) for k, v in doc.items() if isinstance(v, list)}


def shrunk_families(before, after):
    """Families present in `before` whose row count went DOWN in `after`.

    Returns `{family: (before_count, after_count)}`, empty if none shrank.
    This is the single invariant the whole self-erasure bug (THE-BOX.md wave
    28 S3 item #1 / `defects.md` D7) violated: a generated fixture may grow
    freely across a run, it may never silently shrink. `main()` calls this on
    every write, unconditionally, over EVERY family (not just the one this
    script derives) -- so a future bug that corrupts a PRESERVED sibling
    family some other way is caught by the same gate, not just a repeat of
    this exact bug shape. `scripts/tests/test_derive_derived_evaluator_
    fixtures.py` imports this function directly and also reproduces the
    original defect's exact output shape against it, so the check itself is
    proven able to fail, not just proven able to pass.
    """
    before_counts = family_entry_counts(before)
    after_counts = family_entry_counts(after)
    return {
        k: (n, after_counts.get(k, 0))
        for k, n in before_counts.items()
        if after_counts.get(k, 0) < n
    }


def own_document_fields(entries):
    """This script's own top-level fixture keys, and ONLY those keys.

    SD31-W29-INTEGRATE (adversarial-review CONFIRMED finding, MEDIUM):
    `main()` used to keep a hand-maintained `OWN_KEYS` set that had to be
    kept in sync BY EYE with the literal keys in the `document = {...}`
    dict built further down -- proven able to desync by mutation (removing
    one key from that set left a stale `preserved` value silently
    overriding a freshly-derived one, with zero test catching it). Making
    this function the SINGLE SOURCE for both the exclusion set (`main()`
    computes `OWN_KEYS = frozenset(own_document_fields([]))`, needing only
    the key NAMES, not real values) and the actual written document
    (`main()` builds `{**own_document_fields(entries), **preserved}`)
    makes a desync structurally impossible: there is only one place the
    key list is written, ever.
    """
    return {
        "schema": 1,
        "generated_by": "scripts/derive_derived_evaluator_fixtures.py",
        "pcgen_corpus_relative_root": CORPUS_RELATIVE_ROOT,
        "token_family": "BONUS:STAT",
        "derivation": (
            "PCGen `BONUS:STAT|<comma-separated ability list>|<integer value>"
            "[|<extra qualifiers>]`. `expected.abilities` is the ability list "
            "split on commas; `expected.bonus` is the integer value. Trailing "
            "qualifiers (`TYPE=Enhancement`, ...) select a stacking type and do "
            "not change the magnitude. Read straight out of the upstream PCGen "
            "`.lst` bytes named by `upstream_lst`/`upstream_line`."
        ),
        "independence": (
            "Every value here is a function of the upstream PCGen `.lst` bytes "
            "alone. The generator imports no engine module, runs no engine "
            "binary, and opens no file under `data/corpus/` -- which is the "
            "ingest the engine actually evaluates, and therefore the artifact "
            "this fixture must stay independent of. `docs/work-inventory.json` "
            "is read for unit identity and source-line provenance only. This "
            "fixture is committed before the check that consumes it is written "
            "(SD-32 E6-F1)."
        ),
        "entries": entries,
    }


def pcgen_data_root():
    root = os.environ.get("PCGEN_CORPUS_ROOT")
    if root:
        return root
    home = os.environ.get("HOME")
    if not home:
        sys.exit("HOME must be set (or PCGEN_CORPUS_ROOT given) to locate the PCGen corpus")
    return os.path.join(home, "workspace", "repos", "pcgen", "data")


def has_scalar(value):
    if any(s in value for s in SCALARS_SUBSTRING):
        return True
    return any(
        re.search(r"(?<![A-Za-z0-9_])" + re.escape(s) + r"(?![A-Za-z0-9_])", value)
        for s in SCALARS_WORD
    )


def has_arith(value):
    if "*" in value or "/" in value:
        return True
    if "min(" in value.lower() or "max(" in value.lower():
        return True
    return re.search(r"\+\s*\w*[A-Z]{2,}", value) is not None


def derived_fields(line):
    """The magnitude-bearing fields of one raw `.lst` row that are derived."""
    fields = [f.strip() for f in line.rstrip("\r\n").split("\t")]
    out = []
    for field in fields:
        if not any(field.startswith(t) for t in MAGNITUDE_TOKENS):
            continue
        value = field.split(":", 1)[1] if ":" in field else ""
        if has_scalar(value) or has_arith(value):
            out.append(field)
    return fields, out


def record_key_of(fields):
    """The row's corpus identity: its `KEY:` token, else its first column.

    Exactly what PCGen itself does, and exactly the identity
    `equipment_resolver::equipment_id_resolve` matches on.
    """
    for field in fields:
        if field.startswith("KEY:"):
            return field[len("KEY:"):]
    return fields[0] if fields else ""


def parse_bonus_stat(field):
    """`BONUS:STAT|<abilities>|<value>[|...]` -> (abilities, value) or None.

    The whole derivation. It is deliberately tiny and deliberately total: a
    field it cannot parse yields None and the unit is reported uncovered
    rather than given a guessed expectation.
    """
    if not field.startswith("BONUS:STAT|"):
        return None
    parts = field[len("BONUS:"):].split("|")
    if len(parts) < 3 or parts[0] != "STAT":
        return None
    abilities = [a.strip() for a in parts[1].split(",") if a.strip()]
    try:
        value = int(parts[2])
    except ValueError:
        return None
    if not abilities:
        return None
    return abilities, value


def sha256_file(path):
    h = hashlib.sha256()
    with open(path, "rb") as fh:
        for chunk in iter(lambda: fh.read(1 << 20), b""):
            h.update(chunk)
    return h.hexdigest()


def index_corpus(root):
    """basename -> [absolute paths], over the PCGen roleplaying_game tree."""
    index = collections.defaultdict(list)
    for dirpath, _dirs, files in os.walk(root):
        for name in files:
            index[name].append(os.path.join(dirpath, name))
    return index


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--report", action="store_true",
                    help="print the coverage survey and write nothing")
    args = ap.parse_args()

    data_root = pcgen_data_root()
    corpus_root = os.path.join(data_root, CORPUS_RELATIVE_ROOT)
    if not os.path.isdir(corpus_root):
        sys.exit(f"PCGen corpus not found at {corpus_root}; set PCGEN_CORPUS_ROOT")

    with open(INVENTORY, encoding="utf-8") as fh:
        inventory = json.load(fh)

    index = index_corpus(corpus_root)
    sha_cache = {}
    text_cache = {}

    entries = []
    uncovered = collections.Counter()
    multi_stat = 0

    held_derived = [
        u for u in inventory["units"]
        if u.get("wiring_class") == "derived" and u["status"] in HELD_STATUSES
    ]

    for unit in held_derived:
        if unit["kind"] not in EVALUATED_KINDS:
            uncovered[f"kind={unit['kind']}: no equipment-effect evaluator"] += 1
            continue
        candidates = index.get(unit["source_file"], [])
        scoped = [p for p in candidates if f"/{unit['book']}/" in p] or candidates
        if not scoped:
            uncovered["source .lst not found under the PCGen corpus root"] += 1
            continue
        path = scoped[0]
        if path not in text_cache:
            with open(path, encoding="utf-8", errors="replace") as fh:
                text_cache[path] = fh.read().split("\n")
            sha_cache[path] = sha256_file(path)
        lines = text_cache[path]
        if unit["source_line"] < 1 or unit["source_line"] > len(lines):
            uncovered["cited source_line is outside the .lst"] += 1
            continue
        line = lines[unit["source_line"] - 1]
        fields, derived = derived_fields(line)
        stat_fields = [f for f in derived if f.startswith("BONUS:STAT|")]
        if not stat_fields:
            families = ",".join(sorted({f.split("|")[0] for f in derived})) or "(none)"
            uncovered[f"derived token family with no numeric evaluator: {families}"] += 1
            continue
        parsed = parse_bonus_stat(stat_fields[0])
        if parsed is None:
            uncovered["BONUS:STAT field the derivation cannot parse"] += 1
            continue
        abilities, value = parsed
        if len(stat_fields) > 1:
            multi_stat += 1
        entries.append({
            "unit_id": unit["id"],
            "book": unit["book"],
            "kind": unit["kind"],
            "record_key": record_key_of(fields),
            "upstream_lst": os.path.relpath(path, data_root),
            "upstream_lst_sha256": sha_cache[path],
            "upstream_line": unit["source_line"],
            "corpus_field": stat_fields[0],
            "token_family": "BONUS:STAT",
            "multi_stat_row": len(stat_fields) > 1,
            "expected": {"abilities": abilities, "bonus": value},
        })

    entries.sort(key=lambda e: e["unit_id"])

    print(f"held `derived` units in the inventory: {len(held_derived)}")
    print(f"fixture entries derived from the corpus: {len(entries)}")
    print(f"  of which rows carrying >1 BONUS:STAT field: {multi_stat}")
    print("uncovered, by reason:")
    for reason, n in uncovered.most_common():
        print(f"  {n:6}  {reason}")
    print(f"  {sum(uncovered.values()):6}  TOTAL uncovered")

    if args.report:
        return

    # SD31-W29-LANE1 (was SD31-E6-F11-002): this script only ever derives the
    # `kind=equipment` `BONUS:STAT` family below -- every OTHER top-level
    # family in the committed fixture (`monster_entries`, `spell_entries`,
    # `spell_range_entries`, `companion_entries`, `companion_skill_entries`,
    # `companion_save_dc_entries`, `class_feature_entries`,
    # `class_feature_description_entries`, `monster_sla_entries`,
    # `monster_ability_entries`, `monster_ability_formula_entries`, and their
    # `*_token_family`/`*_derivation`/`*_independence`/`*_coverage` sidecar
    # keys) is generated by a SIBLING script and hand-appended or merged
    # directly into this same committed JSON.
    #
    # The previous fix here (`**monster_keys`) special-cased ONLY the
    # `monster_` prefix and silently dropped every other sibling family on
    # every write -- 2,015 entries across 7 more families, confirmed by
    # running this script twice in an isolated worktree and diffing against
    # the committed baseline (THE-BOX.md wave 28 S3 item #1). The root cause
    # was never the missing prefix; it was that this is the ONLY one of the
    # 10 `scripts/derive_*_fixtures.py` generators that reconstructs the
    # WHOLE fixture document from scratch on every run. Every sibling
    # generator instead loads the existing committed document and mutates
    # ONLY the one top-level key it owns (e.g.
    # `derive_spell_range_fixtures.py`: `doc["spell_range_entries"] =
    # entries`), which preserves every other family for free with no
    # allowlist at all. Fixed by adopting that same shape: preserve by
    # EXCLUDING this script's own keys, not by including a hardcoded prefix,
    # so a brand-new sibling family added later needs no matching edit here.
    # `OWN_KEYS` is derived from `own_document_fields()` itself (not a
    # separately hand-maintained set) so it can never desync from the keys
    # the write step actually emits -- see that function's own docstring.
    OWN_KEYS = frozenset(own_document_fields([]))
    existing = {}
    if os.path.exists(FIXTURE):
        with open(FIXTURE, encoding="utf-8") as fh:
            existing = json.load(fh)
    preserved = {k: v for k, v in existing.items() if k not in OWN_KEYS}
    if preserved:
        family_keys = sorted(
            k for k in preserved if k.endswith("_entries")
        )
        print(
            f"preserving {len(family_keys)} sibling *_entries famil"
            f"{'y' if len(family_keys) == 1 else 'ies'} untouched: "
            + ", ".join(f"{k}={len(preserved[k])}" for k in family_keys)
        )

    # Never silently shrink this script's own family -- same "a generated
    # artifact may grow, it may never silently shrink" posture as
    # `derive_spell_range_fixtures.py`'s "FATAL: this run would drop N
    # already-covered unit(s)" guard, but MERGED rather than aborted.
    #
    # A pure abort is wrong here for a reason specific to this generator: its
    # live selection (`held_derived`) is filtered on the unit's CURRENT
    # `wiring_class` in `docs/work-inventory.json`, a field this script does
    # not own and cannot control -- 83 of the 94 committed `entries` rows are
    # for units whose `wiring_class` has since moved from `derived` to
    # `static` (a real, correct reclassification; each now carries its OWN
    # `static` done-rung stamp, `literal-verified`, from the entirely
    # independent `corpus_literal_sweep` -- confirmed by joining every
    # `entries` unit_id against `docs/work-inventory.json`:
    # 83 static + 11 derived/fixture-verified = 94 exactly -- the 83 carry
    # their OWN independent `literal-verified` stamp from `corpus_literal_
    # sweep`, but that stamp plays no role in THIS generator selecting or
    # preserving them; they survive purely because this merge carries
    # forward any `entries` row the live `derived`-only selection no
    # longer covers, regardless of what status the unit now carries).
    # An abort-on-shrink guard would therefore refuse to ever run again,
    # permanently, for a condition outside this script's control -- worse
    # than the bug it replaces. Merging instead keeps every previously
    # verified row (each is still a true, re-checkable fact about the pinned
    # PCGen bytes, never a fabricated one) while letting freshly-derived rows
    # for units still genuinely `derived`+held overwrite their own entry with
    # up-to-date provenance. The result can only ever grow or refresh, never
    # shrink -- satisfying THE-BOX.md wave 28 S3 item #1's `len(after) >=
    # len(before)` invariant by construction, not by refusal.
    existing_by_id = {e["unit_id"]: e for e in existing.get("entries", [])}
    fresh_by_id = {e["unit_id"]: e for e in entries}
    carried_over = sorted(existing_by_id.keys() - fresh_by_id.keys())
    if carried_over:
        print(
            f"carrying forward {len(carried_over)} previously-derived `entries` "
            "row(s) this run's live selection no longer covers (their unit's "
            "wiring_class has moved on since) -- never deleted, only ever grown; "
            f"first 5: {carried_over[:5]}"
        )
    entries = sorted(
        (fresh_by_id.get(uid, existing_by_id.get(uid))
         for uid in existing_by_id.keys() | fresh_by_id.keys()),
        key=lambda e: e["unit_id"],
    )

    # Wave 29 integration: `shrunk_families` (below) cannot see this class
    # of regression at all -- a `HELD_STATUSES` bug that makes `held_derived`
    # (and therefore `entries`, the freshly-derived set) empty produces
    # `fresh_by_id == {}`, which the carry-forward merge above silently
    # backfills from `existing`, so `entries` (the merged, WRITTEN list)
    # never shrinks even though the generator has stopped doing its job
    # entirely. Adversarial review proved this exact blind spot by
    # reverting `HELD_STATUSES` to its pre-fix 3-status tuple: all 5
    # pre-existing tests stayed green and the written fixture stayed
    # byte-identical. Guard the thing `shrunk_families` cannot: if there
    # was ANY previously-committed `entries` row to derive from, at least
    # one row must be FRESH this run, not merely carried forward.
    if existing_by_id and not fresh_by_id:
        sys.exit(
            "FATAL: this run derived ZERO fresh `entries` rows even though "
            f"{len(existing_by_id)} were previously committed -- HELD_STATUSES "
            "likely regressed (see todo/defects.md D7, todo/sweeps.md S6); "
            "refusing to write a fixture the carry-forward merge would "
            "otherwise silently paper over"
        )

    # `own_document_fields()` is the SAME function `OWN_KEYS` above was
    # derived from -- the write step and the exclusion set can no longer
    # desync, by construction (see that function's own docstring).
    document = {**own_document_fields(entries), **preserved}

    # Final, unconditional safety net -- over EVERY family, not just the one
    # this run touched. The merge above already makes `entries` itself
    # monotonic by construction; this catches any OTHER way a write could
    # shrink something (e.g. a future edit to the preserve-by-exclusion logic
    # above that regresses back toward an allowlist). See `shrunk_families`'s
    # own docstring; `scripts/tests/test_derive_derived_evaluator_fixtures.py`
    # proves this exact check is not vacuous by running it against the real
    # pre-fix commit's output.
    violations = shrunk_families(existing, document)
    if violations:
        lines = "\n".join(
            f"  {k}: {before_n} -> {after_n}"
            for k, (before_n, after_n) in sorted(violations.items())
        )
        sys.exit(
            f"FATAL: this run would shrink {len(violations)} famil"
            f"{'y' if len(violations) == 1 else 'ies'} relative to the "
            f"committed fixture -- refusing to write:\n{lines}"
        )

    os.makedirs(os.path.dirname(FIXTURE), exist_ok=True)
    with open(FIXTURE, "w", encoding="utf-8") as fh:
        json.dump(document, fh, indent=2, sort_keys=False)
        fh.write("\n")
    print(f"\nwrote {os.path.relpath(FIXTURE, REPO)}")


if __name__ == "__main__":
    main()
