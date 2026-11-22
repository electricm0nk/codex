#!/usr/bin/env python3
"""Extract the multiclass negative-control tests' own inputs, mechanically.

Why this exists
----------------
SD-36 Epic F, batch F0, step F0d (`docs/release/SD-36-consolidation/
epic-f-class-completion.md` §2, "Mix panel"; §5, review finding 8's "Fix
(adopted)"). The permanent class census (`src/rules_core/class_census.rs`)
needs a mix panel: every EXISTING multiclass negative-control test's own
(class, level) input pair, re-used as a census row, so the census can sweep
the exact same mixes those tests already exercise and report each one's
real, engine-derived status plus a histogram of claim-blocking diagnostic
ids. §5's own review finding 8 (CONFIRMED) is explicit that this input
count must be MEASURED, never asserted from prose -- no file in the repo
states "183", and the real denominator is "somewhere between" a loose
string-literal count (235) and a hand count, "and must be measured, not
asserted." This script is that measurement.

The three sources, and the two suite-level counts that pin them
------------------------------------------------------------------
1. `tests/sd18_widening/rows.rs`'s `MULTICLASS_NEG_ROWS` array (64 rows,
   `sd18_multiclass_neg_control_test!`-driven) PLUS the hand-written
   `multiclass_<class>_level<N>_is_not_promoted_by_this_slice` tests that
   live directly in `tests/sd18_widening/*.rs` files but are NOT part of
   that array (24 today: Fighter levels 12-20, Wizard levels 11-20 --
   these widen into a genuinely SUPPORTED Fighter/Rogue or Wizard/Rogue
   multiclass mix per the v0.6 swarm update, so their own body now asserts
   Computed, not Blocked -- still a real multiclass mix input, so still a
   census row). Together this is exactly what
   `cargo test --locked --test sd18_widening -- --list | grep -c multiclass`
   measures (88, verified: no non-negative-control test name contains the
   substring "multiclass" in this suite).
2. `tests/sd13_progression/rows.rs`'s `multiclass_negative_controls!` macro,
   invoked once per file across `tests/sd13_progression/*.rs` (59 today),
   PLUS the files in the same directory that inline the identical
   `.replace(...)` shape directly in a hand-written fn instead of calling
   the shared macro (25 today -- mostly Druid/Wizard files with bespoke
   extra assertions, e.g. the animal-companion stat block). Together this
   is exactly what
   `cargo test --locked --test sd13_progression -- --list | grep -c multiclass`
   measures (84, verified: no non-negative-control test name contains the
   substring "multiclass" in this suite either).
3. Every hand-written file OUTSIDE both of the above directories that
   carries its own `fn multiclass_..._is_not_promoted_by_this_slice` --
   the "explicit, enumerated list, not a guess" §5's review finding 8
   calls for. Measured by:

       find tests -maxdepth 1 -name '*.rs' -print0 \\
         | xargs -0 grep -l 'fn multiclass_.*_is_not_promoted_by_this_slice'

   13 files today (4 sd13_*, 1 v06_*, 8 sd18_*), each carrying exactly one
   such fn (`grep -c` on the same pattern, per file, is 1 in every case --
   asserted below, not assumed).

Total measured denominator: 88 + 84 + 13 = 185
(`BASELINE_CENSUS_MIX_COMPUTED`'s own provenance count -- see
`scripts/verify-baselines.env`). This is neither the document's original,
unsourced "183" nor the looser 235-occurrence count of the bare string
"stay claim-blocked" across all of `tests/` (which also catches the
`recognition_negative_controls!` family, an unrelated shape) -- it is the
real, reproducible count of multiclass-shaped tests, unioned from all three
sources above with none double-counted (verified: every source is a
disjoint set of file paths / macro-invocation sites).

Extraction, per source
-----------------------
Every one of the 185 tests either:
  (a) substitutes a fixture string via a literal
      `.replace("class_level=class:<c1>:<n1>", "class_level=class:<c1>:<n1>
      \\nclass_level=class:<c2>:<n2>")` call (or the identical `new_sub`
      field in `MULTICLASS_NEG_ROWS`, or the identical `$to` argument to
      `multiclass_negative_controls!`) -- 184 of 185; or
  (b) appends a second `class_level=` line via `format!("{FIXTURE}\\n
      class_level=class:<c2>:<n2>\\n")`, where `FIXTURE`'s own single
      `class_level=` line (read from its own `include_str!`-referenced
      fixture file) supplies `(<c1>, <n1>)` -- exactly 1 of 185, Ranger's
      `multiclass_ranger_is_not_promoted_by_this_slice`
      (`tests/sd13_ranger_level1_chassis_and_class_feature_separation.rs`).

Both shapes are read MECHANICALLY here -- never retyped -- via regexes over
the actual source text, never re-derived from this docstring's own prose.

Output
------
Writes `tests/fixtures/rules_core/multiclass_census_panel.json`: one row
per test, `{key, source_file, test_fn, classes: [[class, level], ...]}`,
sorted by `key`. `src/rules_core/class_census.rs` reads this file at
runtime (mirroring `load_sweep_fixture`'s own repo-root-relative read) to
build the mix panel.

Usage
-----
    python3 scripts/extract_multiclass_census_panel.py [--check]

`--check` (used by CI / `scripts/verify.sh`) re-runs the extraction and
diffs it against the committed file instead of overwriting it, exiting
non-zero on any difference -- the same "generated, verified never re-typed"
contract `scripts/gen_class_status_table.py --check` uses elsewhere in this
batch.
"""

from __future__ import annotations

import argparse
import json
import re
import subprocess
import sys
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parent.parent
OUTPUT_PATH = REPO_ROOT / "tests/fixtures/rules_core/multiclass_census_panel.json"

PAIR_RE = re.compile(r"class_level=class:([a-z_]+):(\d+)")
FN_DEF_RE = re.compile(r"fn (multiclass_\w*_is_not_promoted_by_this_slice)\s*\(\s*\)\s*\{")
REPLACE_CALL_RE = re.compile(
    r'\.replace\(\s*"((?:[^"\\]|\\.)*)"\s*,\s*"((?:[^"\\]|\\.)*)"\s*,?\s*\)', re.S
)
FORMAT_FALLBACK_RE = re.compile(
    r'format!\(\s*"\{(\w+)\}((?:[^"\\]|\\.)*)"\s*\)', re.S
)


class ExtractionError(RuntimeError):
    pass


def extract_pairs(text: str) -> list[tuple[str, int]]:
    """Every distinct `class_level=class:<c>:<n>` pair in `text`, in
    first-seen order (a `.replace()` call's "from" argument legitimately
    echoes one pair the "to" argument also carries; this dedupes that
    echo rather than double-counting it)."""
    seen: list[tuple[str, int]] = []
    for m in PAIR_RE.finditer(text):
        pair = (m.group(1), int(m.group(2)))
        if pair not in seen:
            seen.append(pair)
    return seen


TOP_LEVEL_CLOSE_BRACE_RE = re.compile(r"^\}$", re.M)


def function_body(text: str, start_index: int) -> str:
    """The fn body from `start_index` (the position right after the fn
    signature's opening `{`, i.e. `FN_DEF_RE.end()`) up to the next line
    that is a bare `}` at column 0.

    NOT brace-counting: several of these fns hold format strings like
    `"{:?}"`, whose literal `{`/`}` characters are not code braces, so a
    naive depth counter over the raw text mis-closes the body early (proven
    the hard way: it did, on the first real file this script ran against).
    rustfmt's own house style closes every top-level item's brace alone on
    its own line at zero indentation, which every fn in this repo's test
    suites already follows (verified against the full set this script
    walks); that convention is the terminator used here instead.
    """
    end_m = TOP_LEVEL_CLOSE_BRACE_RE.search(text, start_index)
    if not end_m:
        raise ExtractionError(f"no top-level closing brace found after position {start_index}")
    return text[start_index : end_m.end()]


def resolve_include_str_pairs(source_file: Path, const_name: str) -> list[tuple[str, int]]:
    """Resolve `const <const_name>: &str = include_str!("<path>");` in
    `source_file`'s own text to the referenced fixture file, and return
    that fixture's own `class_level=class:<c>:<n>` pairs."""
    text = source_file.read_text()
    const_re = re.compile(
        re.escape(const_name) + r'\s*:\s*&str\s*=\s*\n?\s*include_str!\(\s*"([^"]+)"\s*\)'
    )
    m = const_re.search(text)
    if not m:
        raise ExtractionError(f"{source_file}: no include_str! definition found for {const_name}")
    fixture_path = (source_file.parent / m.group(1)).resolve()
    if not fixture_path.is_file():
        raise ExtractionError(f"{source_file}: resolved fixture path does not exist: {fixture_path}")
    pairs = extract_pairs(fixture_path.read_text())
    if not pairs:
        raise ExtractionError(f"{fixture_path}: no class_level= line found")
    return pairs


def rows_from_sd18_multiclass_neg_rows() -> list[dict]:
    """`MULTICLASS_NEG_ROWS`' own row keys never equal the generated test's
    fn name in every case (`"bard_level11_inspire"`'s row is invoked as
    `multiclass_bard_level11_is_not_promoted_by_this_slice`, not
    `multiclass_bard_level11_inspire_is_not_promoted_by_this_slice` --
    caught only by cross-checking against the real `cargo test -- --list`
    output, not assumed from the naming convention every other row
    happens to follow). So the real fn name is read from each macro
    invocation call site (`crate::sd18_multiclass_neg_control_test!($fn_name,
    "$module", $fixture)`, `rows.rs`'s own macro signature) across
    `tests/sd18_widening/*.rs`, keyed by `$module` back to its
    `MULTICLASS_NEG_ROWS` row -- never re-derived from the row key alone."""
    rows_path = REPO_ROOT / "tests/sd18_widening/rows.rs"
    rows_text = rows_path.read_text()
    start = rows_text.index("pub const MULTICLASS_NEG_ROWS")
    end = rows_text.index("\n];", start)
    body = rows_text[start:end]
    row_re = re.compile(
        r'\(\s*"([a-z0-9_]+)"\s*,\s*MulticlassNegControlRow\s*\{(.*?)\}\s*\),', re.S
    )
    new_sub_by_key: dict[str, list[tuple[str, int]]] = {}
    for m in row_re.finditer(body):
        key, fields = m.group(1), m.group(2)
        new_sub_m = re.search(r'new_sub:\s*"((?:[^"\\]|\\.)*)"', fields)
        if not new_sub_m:
            raise ExtractionError(f"{rows_path}: row {key!r} has no new_sub field")
        pairs = extract_pairs(new_sub_m.group(1))
        if len(pairs) != 2:
            raise ExtractionError(f"{rows_path}: row {key!r} new_sub yielded {pairs}, expected exactly 2 pairs")
        new_sub_by_key[key] = pairs
    if len(new_sub_by_key) != 64:
        raise ExtractionError(f"{rows_path}: measured {len(new_sub_by_key)} MULTICLASS_NEG_ROWS rows, expected 64")

    invocation_re = re.compile(
        r'sd18_multiclass_neg_control_test!\(\s*(\w+)\s*,\s*"([a-z0-9_]+)"\s*,\s*\w+\s*\)'
    )
    rows: list[dict] = []
    seen_keys: set[str] = set()
    for source_file in sorted((REPO_ROOT / "tests/sd18_widening").glob("*.rs")):
        if source_file.name == "rows.rs":
            continue
        text = source_file.read_text()
        for m in invocation_re.finditer(text):
            fn_name, key = m.group(1), m.group(2)
            if key not in new_sub_by_key:
                raise ExtractionError(f"{source_file}: invocation names module {key!r}, absent from MULTICLASS_NEG_ROWS")
            rows.append(
                {
                    "key": f"sd18_widening::{fn_name}",
                    "source_file": str(source_file.relative_to(REPO_ROOT)),
                    "test_fn": fn_name,
                    "classes": new_sub_by_key[key],
                }
            )
            seen_keys.add(key)

    missing = set(new_sub_by_key) - seen_keys
    if missing:
        raise ExtractionError(f"MULTICLASS_NEG_ROWS keys with no invocation site found: {sorted(missing)}")
    if len(rows) != 64:
        raise ExtractionError(f"measured {len(rows)} sd18_multiclass_neg_control_test! invocations, expected 64")
    return rows


def rows_from_hand_written_fns_in_dir(dir_path: Path, dir_label: str, exclude_files: set[str]) -> list[dict]:
    """Every `fn multiclass_..._is_not_promoted_by_this_slice` defined
    directly (not macro-generated) in `dir_path`'s own `*.rs` files, minus
    `exclude_files` (files this caller has already covered another way)."""
    rows = []
    for source_file in sorted(dir_path.glob("*.rs")):
        if source_file.name in exclude_files:
            continue
        text = source_file.read_text()
        for m in FN_DEF_RE.finditer(text):
            fn_name = m.group(1)
            body = function_body(text, m.end())
            replace_matches = REPLACE_CALL_RE.findall(body)
            if len(replace_matches) == 1:
                pairs = extract_pairs(replace_matches[0][1])
            elif len(replace_matches) == 0:
                fmt_m = FORMAT_FALLBACK_RE.search(body)
                if not fmt_m:
                    raise ExtractionError(
                        f"{source_file}:{fn_name}: no .replace() call and no format!(\"{{X}}...\") "
                        "fallback shape found"
                    )
                base_const, tail = fmt_m.group(1), fmt_m.group(2)
                base_pairs = resolve_include_str_pairs(source_file, base_const)
                tail_pairs = extract_pairs(tail)
                pairs = base_pairs + tail_pairs
            else:
                raise ExtractionError(
                    f"{source_file}:{fn_name}: {len(replace_matches)} .replace() calls found, "
                    "extractor only handles exactly 0 or 1"
                )
            if len(pairs) != 2:
                raise ExtractionError(f"{source_file}:{fn_name}: extracted {pairs}, expected exactly 2 pairs")
            rows.append(
                {
                    "key": f"{dir_label}::{fn_name}",
                    "source_file": str(source_file.relative_to(REPO_ROOT)),
                    "test_fn": fn_name,
                    "classes": pairs,
                }
            )
    return rows


def rows_from_sd13_progression_macro() -> list[dict]:
    """Every `multiclass_negative_controls!` macro invocation across
    `tests/sd13_progression/*.rs` (the macro itself lives in
    `rows.rs`, but is invoked per-file, not data-tabled -- see
    `rows.rs`'s own design note)."""
    invocation_re = re.compile(
        r'(multiclass_\w*_is_not_promoted_by_this_slice)\s*\(\s*\w+\s*,\s*'
        r'"((?:[^"\\]|\\.)*)"\s*=>\s*"((?:[^"\\]|\\.)*)"\s*\)\s*\{',
        re.S,
    )
    rows = []
    covered_fn_names: set[str] = set()
    for source_file in sorted((REPO_ROOT / "tests/sd13_progression").glob("*.rs")):
        if source_file.name == "rows.rs":
            continue
        text = source_file.read_text()
        for m in invocation_re.finditer(text):
            fn_name, to = m.group(1), m.group(3)
            pairs = extract_pairs(to)
            if len(pairs) != 2:
                raise ExtractionError(f"{source_file}:{fn_name}: macro $to yielded {pairs}, expected exactly 2 pairs")
            rows.append(
                {
                    "key": f"sd13_progression::{fn_name}",
                    "source_file": str(source_file.relative_to(REPO_ROOT)),
                    "test_fn": fn_name,
                    "classes": pairs,
                }
            )
            covered_fn_names.add(fn_name)
    return rows, covered_fn_names


def rows_from_sd13_progression_bespoke(covered_fn_names: set[str]) -> list[dict]:
    """The `tests/sd13_progression/*.rs` files whose multiclass negative
    control is a hand-written fn (not a `multiclass_negative_controls!`
    invocation) -- same shape `rows_from_hand_written_fns_in_dir` reads
    elsewhere, filtered to skip fns the macro pass above already covered."""
    rows = []
    for source_file in sorted((REPO_ROOT / "tests/sd13_progression").glob("*.rs")):
        if source_file.name == "rows.rs":
            continue
        text = source_file.read_text()
        for m in FN_DEF_RE.finditer(text):
            fn_name = m.group(1)
            if fn_name in covered_fn_names:
                continue
            body = function_body(text, m.end())
            replace_matches = REPLACE_CALL_RE.findall(body)
            if len(replace_matches) != 1:
                raise ExtractionError(
                    f"{source_file}:{fn_name}: {len(replace_matches)} .replace() calls found, "
                    "extractor only handles exactly 1 for this source"
                )
            pairs = extract_pairs(replace_matches[0][1])
            if len(pairs) != 2:
                raise ExtractionError(f"{source_file}:{fn_name}: extracted {pairs}, expected exactly 2 pairs")
            rows.append(
                {
                    "key": f"sd13_progression::{fn_name}",
                    "source_file": str(source_file.relative_to(REPO_ROOT)),
                    "test_fn": fn_name,
                    "classes": pairs,
                }
            )
    return rows


def measured_hand_written_top_level_files() -> list[Path]:
    """`find tests -maxdepth 1 -name '*.rs' | xargs grep -l 'fn
    multiclass_.*_is_not_promoted_by_this_slice'` -- run for real (not
    re-derived) so this list can never silently drift from what the
    committed sync test also greps for."""
    find = subprocess.run(
        ["find", "tests", "-maxdepth", "1", "-name", "*.rs", "-print0"],
        cwd=REPO_ROOT,
        capture_output=True,
        check=True,
    )
    names = [n for n in find.stdout.decode().split("\0") if n]
    grep = subprocess.run(
        ["xargs", "-0", "grep", "-l", "fn multiclass_.*_is_not_promoted_by_this_slice"],
        input=find.stdout,
        cwd=REPO_ROOT,
        capture_output=True,
    )
    if grep.returncode not in (0, 1):
        raise ExtractionError(f"grep failed: {grep.stderr.decode()}")
    paths = [REPO_ROOT / p for p in grep.stdout.decode().split("\n") if p]
    return sorted(paths)


def build_panel() -> list[dict]:
    rows: list[dict] = []
    rows.extend(rows_from_sd18_multiclass_neg_rows())
    rows.extend(
        rows_from_hand_written_fns_in_dir(
            REPO_ROOT / "tests/sd18_widening", "sd18_widening", exclude_files={"rows.rs"}
        )
    )
    macro_rows, covered_fn_names = rows_from_sd13_progression_macro()
    rows.extend(macro_rows)
    rows.extend(rows_from_sd13_progression_bespoke(covered_fn_names))

    top_level_files = measured_hand_written_top_level_files()
    for source_file in top_level_files:
        text = source_file.read_text()
        matches = list(FN_DEF_RE.finditer(text))
        if len(matches) != 1:
            raise ExtractionError(
                f"{source_file}: {len(matches)} multiclass-negative-control fns found, expected exactly 1"
            )
        m = matches[0]
        fn_name = m.group(1)
        body = function_body(text, m.end())
        replace_matches = REPLACE_CALL_RE.findall(body)
        if len(replace_matches) == 1:
            pairs = extract_pairs(replace_matches[0][1])
        elif len(replace_matches) == 0:
            fmt_m = FORMAT_FALLBACK_RE.search(body)
            if not fmt_m:
                raise ExtractionError(f"{source_file}:{fn_name}: no .replace() and no format!() fallback shape")
            base_const, tail = fmt_m.group(1), fmt_m.group(2)
            base_pairs = resolve_include_str_pairs(source_file, base_const)
            tail_pairs = extract_pairs(tail)
            pairs = base_pairs + tail_pairs
        else:
            raise ExtractionError(f"{source_file}:{fn_name}: {len(replace_matches)} .replace() calls found")
        if len(pairs) != 2:
            raise ExtractionError(f"{source_file}:{fn_name}: extracted {pairs}, expected exactly 2 pairs")
        rows.append(
            {
                "key": f"top_level::{fn_name}",
                "source_file": str(source_file.relative_to(REPO_ROOT)),
                "test_fn": fn_name,
                "classes": pairs,
            }
        )

    keys = [r["key"] for r in rows]
    if len(keys) != len(set(keys)):
        dupes = sorted({k for k in keys if keys.count(k) > 1})
        raise ExtractionError(f"duplicate panel keys: {dupes}")

    rows.sort(key=lambda r: r["key"])
    return rows


def render(rows: list[dict]) -> str:
    document = {
        "generated_by": "scripts/extract_multiclass_census_panel.py",
        "source_of_truth": (
            "docs/release/SD-36-consolidation/epic-f-class-completion.md §2 (F0), "
            "review finding 8 (§5) -- every row is one EXISTING multiclass "
            "negative-control test's own (class, level) input pair, extracted "
            "mechanically from its source, never retyped."
        ),
        "denominator_commands": [
            "cargo test --locked --test sd18_widening -- --list | grep -c multiclass",
            "cargo test --locked --test sd13_progression -- --list | grep -c multiclass",
            "find tests -maxdepth 1 -name '*.rs' -print0 | xargs -0 grep -l "
            "'fn multiclass_.*_is_not_promoted_by_this_slice' | wc -l",
        ],
        "row_count": len(rows),
        "rows": rows,
    }
    return json.dumps(document, indent=2, sort_keys=False) + "\n"


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--check", action="store_true", help="diff against the committed file instead of writing it")
    args = parser.parse_args()

    try:
        rows = build_panel()
    except ExtractionError as e:
        print(f"extract_multiclass_census_panel: {e}", file=sys.stderr)
        return 1

    rendered = render(rows)

    if args.check:
        if not OUTPUT_PATH.is_file():
            print(f"extract_multiclass_census_panel: {OUTPUT_PATH} does not exist", file=sys.stderr)
            return 1
        committed = OUTPUT_PATH.read_text()
        if committed != rendered:
            print(
                f"extract_multiclass_census_panel: {OUTPUT_PATH} is stale -- "
                "re-run `python3 scripts/extract_multiclass_census_panel.py` and commit the result",
                file=sys.stderr,
            )
            return 1
        print(f"extract_multiclass_census_panel: {OUTPUT_PATH} matches the measured extraction ({len(rows)} rows)")
        return 0

    OUTPUT_PATH.parent.mkdir(parents=True, exist_ok=True)
    OUTPUT_PATH.write_text(rendered)
    print(f"extract_multiclass_census_panel: wrote {len(rows)} rows to {OUTPUT_PATH}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
