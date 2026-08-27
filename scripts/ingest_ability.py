#!/usr/bin/env python3
"""Generic ingest of `Kind::Ability`'s enumerated-but-engine-does-not-hold units into
`data/corpus/<book>/ability/*.json`.

SD-32 `decisions.md §20`: `ability` is one of the 18 kinds whose 4,824-unit
population reached `docs/work-inventory.json` (`kind: "ability"`,
`15-ability_cycle_receipt.md`) but has NO corpus record at all, so
`scripts/shape_ledger.py` reports every one of them `join_status: no_record` --
their shape cannot be measured. This transcribes them.

**Generic, not per-book** (`decisions.md §17`): a unit's own
`(book, source_file, source_line)` citation, already established by
`v06_work_inventory`'s enumeration, is resolved against the pinned corpus by
searching for a directory whose basename equals the unit's `book` field
(falling back to `core_essentials` for the `decisions.md §9`-reattributed
units whose physical file lives there), then searching that directory for a
file named `source_file`. No book is named in this file's own source; adding
a 29th book costs nothing here. Verified against the current 28-book/102-file
population before this cycle built anything:
`docs/release/SD-32-compute-library-and-cause-closure/artifacts/gate-0-census-closure/17-ability-ingest-receipt.md`
records the resolution check.

**Nothing is computed.** Every emitted `raw_tokens` entry is a verbatim
substring of the cited row (skip the identity column, split each remaining
tab field on its first `:`). `corpus_literal_sweep` independently re-derives
the same tokens from the same citation to confirm the copy byte for byte.

**Product Identity.** Two independent screens, matching `class_feature`'s own
generator and `decisions.md §15/§19`'s standing rule:

1. The row's own declaration (`NAMEISPI:YES`/`DESCISPI:YES`).
2. The amended, word-boundary, OCR-normalized 60-term blacklist scan
   (`scripts/sd32_t9_pi_review_feat_equipment.py`'s own `normalized_term_hit`
   -- imported, not re-typed, so this cycle's ingest inherits the exact
   correction `decisions.md §19a` approved rather than forking a fourth copy
   of the stale 57-term substring scan `src/rules_core/pi_screening.rs` still
   carries, which `docs/governance/ogl-pi-blacklist.md`'s own frontmatter
   says the next transcribing cycle -- this one -- must apply).

**No soft-hyphen or other byte substitution is applied to the cited row.**
`transcribe_monster_tables.py::read_row` replaces PCGen's PDF-extraction
soft-hyphen artifact (U+00AD) with a plain `-` because its consumer is a
*compiled Rust source table* (`clippy::invisible_characters` is deny-by-
default there). This generator's consumer is `corpus_literal_sweep`, which
independently re-derives `raw_tokens` by re-reading the SAME cited bytes and
comparing byte-for-byte -- any substitution here would desync from it. Found
live this cycle (`isg_abilities_faith.lst:53`'s "soul<U+00AD>scouring"):
an early draft that copied the substitution produced exactly one
`corpus_literal_sweep` MISMATCH, caught and reverted before landing.

A `NAMEISPI:YES` declaration OR a name-blacklist hit (checked against BOTH
the unit's bare `name` and its full `key`, since a key can carry a term the
bare name does not -- `isg_abilities_faith.lst`'s own
`"Exalted Boon ~ Asmodeus ~ Hellfire Blast"` is exactly this shape) used to
skip the whole record (a name cannot be redacted). **SD-32 `decisions.md
§24` changes that**: such a record is now INGESTED, under a Codex-generated
neutral name derived only from `(kind, book, source_file, source_line)`
(`scripts/codex_neutral_name.py`; see that module's own docstring and
`scripts/tests/test_codex_neutral_name.py` for the `§24b`-1 proof that its
output cannot be influenced by the PI name). `data.name` and `data.key`
become the neutral name; `data.raw_tokens` has every token whose VALUE hits
the blacklist scan OR restates the record's own original name/key redacted
(not just `DESC` -- a `KEY:` field can carry the row's own PI name a second
time, and `§24b`-2 requires the PI original appear nowhere that ships, not
only in the fields the pre-`§24` screen checked -- see
`scrub_name_pi_tokens`'s own docstring for the live example that found this,
described there by shape rather than by naming the deity). `data.codex_generated_name` is `True` and
`data.rename` records the coordinate and reason (`§24b`-3/4) -- never the
original string. A `DESCISPI:YES` declaration OR a description-blacklist hit
still redacts only the `DESC` field to `shape_b_v1::REDACTED_PI_MARKER`,
matching every other generator's `redact_desc_token_if_pi` precedent.

A record whose PI-ness this screen cannot confidently resolve (declared
neither way, blacklist scan inconclusive) is still never transcribed and
never silently skipped -- `§24c` only licenses renaming a record whose name
IS the PI content; it does not touch `decisions.md §15`'s standing rule for
anything the screen cannot itself decide.

Run: `python3 scripts/ingest_ability.py [--dry-run] [--out <report.json>]`
`PCGEN_CORPUS_ROOT` must point at a pinned PCGen `data/` checkout.
"""
from __future__ import annotations

import hashlib
import json
import os
import re
import subprocess
import sys
from collections import defaultdict

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from sd32_t9_pi_review_feat_equipment import (  # noqa: E402
    PI_BLACKLIST_TERMS,
    extract_free_text,
    normalized_term_hit,
)
from codex_neutral_name import (  # noqa: E402
    divergence_entry,
    neutral_key,
    neutral_name,
)
from pi_scrub import (  # noqa: E402
    PI_MARKER_REDACTED,
    REDACTED_PI_MARKER,
    blacklist_term_hit_including_concatenated,
    scrub_name_pi_tokens,
)

REPO_ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
INVENTORY_PATH = os.path.join(REPO_ROOT, "docs/work-inventory.json")
SOFT_HYPHEN = "­"
CORE_ESSENTIALS_BASENAME = "core_essentials"

# `scripts/shape_ledger.py::BOOK_CORPUS_DIR_ALIASES` -- the corpus-record
# writer must agree with the READER's directory choice, not just the
# resolver's. `bestiary` (the inventory's own book spelling) walks
# `data/corpus/beastiary/` (the historical directory spelling) for every
# OTHER kind already shipped under it; an ability record for a `bestiary`
# unit written under a literal `data/corpus/bestiary/ability/` is invisible
# to `shape_ledger.py`'s join and reports `no_record` even though the file
# exists -- caught live this cycle (30 units) by diffing the pre- and
# post-push `no_record` count, not assumed correct from a clean first run.
CORPUS_WRITE_DIR_ALIASES: dict[str, str] = {
    "bestiary": "beastiary",
}


def corpus_root() -> str:
    return os.environ.get("PCGEN_CORPUS_ROOT", os.path.expanduser("~/workspace/repos/pcgen/data"))


def build_dir_index(root: str) -> dict[str, list[str]]:
    """basename -> every real directory under `root` with that basename."""
    index: dict[str, list[str]] = defaultdict(list)
    for dirpath, _dirnames, _filenames in os.walk(root):
        index[os.path.basename(dirpath)].append(dirpath)
    return index


def find_file_under(root_dir: str, filename: str) -> list[str]:
    hits = []
    for dirpath, _dirnames, filenames in os.walk(root_dir):
        if filename in filenames:
            hits.append(os.path.join(dirpath, filename))
    return sorted(hits)


def resolve_file(dir_index: dict[str, list[str]], root: str, book: str, filename: str) -> list[str]:
    """Real path(s) of `filename` under `book`'s corpus directory, falling back
    to `core_essentials` when the book's own directory does not have it (the
    `decisions.md §9` re-attribution case -- mirrors
    `transcribe_monster_tables.py::resolve_book_file` exactly)."""
    book_dirs = dir_index.get(book, [])
    if len(book_dirs) == 1:
        hits = find_file_under(book_dirs[0], filename)
        if hits:
            return hits
    elif len(book_dirs) > 1:
        return []  # ambiguous book directory -- caller reports, never guesses
    ce_dirs = dir_index.get(CORE_ESSENTIALS_BASENAME, [])
    if len(ce_dirs) == 1 and ce_dirs[0] not in book_dirs:
        return find_file_under(ce_dirs[0], filename)
    return []


def read_row(path: str, line_no: int) -> str:
    """The cited row, byte-verbatim (including any invisible PDF-extraction
    artifact such as a soft hyphen) -- never substituted, so
    `corpus_literal_sweep`'s independent byte-for-byte re-derivation matches.
    See the module doc comment's "No soft-hyphen..." section for why this
    deliberately does NOT mirror `transcribe_monster_tables.py::read_row`."""
    with open(path, encoding="utf-8", errors="replace") as fh:
        lines = fh.read().split("\n")
    if line_no < 1 or line_no > len(lines):
        return ""
    return lines[line_no - 1]


def row_tokens(line: str) -> list[dict[str, str]]:
    """Skip the identity column, split each remaining field on its first `:`.
    Mirrors `cache_gen::class_feature::row_tokens` exactly (same rule, same
    order) so the two generators' output is directly comparable."""
    fields = [f.strip() for f in line.split("\t") if f.strip()]
    fields = fields[1:]
    tokens = []
    for field in fields:
        if ":" in field:
            key, value = field.split(":", 1)
        else:
            key, value = field, ""
        tokens.append({"key": key, "value": value})
    return tokens


def scrub_blacklist_pi_tokens(
    tokens: list[dict[str, str]], desc_already_redacted: bool
) -> tuple[list[dict[str, str]], bool]:
    """`decisions.md §17` gap-close: the SIGNED-OFF 60-term blacklist scan
    applied to EVERY raw-token VALUE, not only `DESC`.

    Before this function existed, a record whose bare `name`/`key` are
    clean never went through `scrub_name_pi_tokens` (that path only runs
    when the name itself is PI) and only its `DESC` token was screened.
    Two already-shipped records proved live that a blacklisted term can
    still leak through a DIFFERENT token's value on an otherwise-clean
    record: a `SPELLLEVEL` token's own `PREDEITY:1,<deity>` segment, and a
    `TYPE`/`PREABILITY` token naming a published campaign-setting institution.
    This function is the union-closing fix: every token value is scanned
    with `pi_scrub.blacklist_term_hit_including_concatenated` — the SAME
    word-boundary blacklist scan `scrub_name_pi_tokens` uses for the renamed
    branch, PLUS its alphanumeric-normalized (no-separator) concatenated-term
    check, so a blacklisted term joined PascalCase-style into another token's
    value (found live: a `TYPE:` token on a THIRD, otherwise-clean record,
    concatenating a blacklisted term directly onto a suffix with no
    separator) is caught here too, not only in the renamed branch.

    `desc_already_redacted` skips re-scanning a `DESC` token the caller has
    already replaced with [`REDACTED_PI_MARKER`] via the declared-PI /
    description-blacklist path — idempotent either way, but keeps the
    caller's intent single-sourced instead of two paths agreeing by luck.

    Returns `(scrubbed_tokens, any_redacted)`. Never mutates the input."""
    scrubbed = []
    any_redacted = False
    for t in tokens:
        if desc_already_redacted and t["key"] == "DESC" and t["value"] == REDACTED_PI_MARKER:
            scrubbed.append(dict(t))
            continue
        value = t["value"]
        if value and blacklist_term_hit_including_concatenated(value):
            scrubbed.append({"key": t["key"], "value": REDACTED_PI_MARKER})
            any_redacted = True
        else:
            scrubbed.append(dict(t))
    return scrubbed, any_redacted


def records_equal_ignoring_timestamp(a: dict, b: dict) -> bool:
    """`True` when two written records are identical except `ingested_at`.

    A generic ingest (`decisions.md §17`) is re-run whenever its logic
    changes, over the WHOLE population every time -- but re-running it must
    not touch a file whose content did not actually change, or every
    unrelated re-run produces a corpus-wide timestamp-only diff (concurrent
    cycles collide on files neither one meant to touch, and a real content
    fix is buried in thousands of no-op lines). Comparing everything except
    `ingested_at` keeps a re-run's git diff scoped to what actually changed."""
    a2 = {k: v for k, v in a.items() if k != "ingested_at"}
    b2 = {k: v for k, v in b.items() if k != "ingested_at"}
    return a2 == b2


def desc_value(tokens: list[dict[str, str]]) -> str | None:
    for t in tokens:
        if t["key"] == "DESC":
            return t["value"]
    return None


def declared_pi(tokens: list[dict[str, str]]) -> tuple[bool, bool]:
    name_declared = False
    desc_declared = False
    for t in tokens:
        if t["value"].strip().upper() != "YES":
            continue
        if t["key"].upper() == "NAMEISPI":
            name_declared = True
        elif t["key"].upper() == "DESCISPI":
            desc_declared = True
    return name_declared, desc_declared


def slugify(name: str, used: set[str]) -> str:
    base = re.sub(r"[^a-z0-9]+", "_", name.lower()).strip("_") or "unnamed"
    slug = base
    n = 2
    while slug in used:
        slug = f"{base}_{n}"
        n += 1
    used.add(slug)
    return slug


def sha256_file(path: str) -> str:
    h = hashlib.sha256()
    with open(path, "rb") as fh:
        h.update(fh.read())
    return h.hexdigest()


def ingested_at_now() -> str:
    out = subprocess.run(["date", "-u", "+%Y-%m-%dT%H:%M:%SZ"], capture_output=True, text=True, check=True)
    return out.stdout.strip()


def load_units() -> list[dict]:
    with open(INVENTORY_PATH, encoding="utf-8") as fh:
        doc = json.load(fh)
    units = doc["units"] if isinstance(doc, dict) and "units" in doc else doc
    return [u for u in units if u.get("kind") == "ability"]


def main() -> int:
    dry_run = "--dry-run" in sys.argv
    out_path = None
    if "--out" in sys.argv:
        out_path = sys.argv[sys.argv.index("--out") + 1]

    root = corpus_root()
    if not os.path.isdir(root):
        print(f"PCGEN_CORPUS_ROOT ({root}) is not a directory", file=sys.stderr)
        return 1

    units = load_units()
    dir_index = build_dir_index(root)

    report = {
        "population": len(units),
        "written": 0,
        # `changed`/`unchanged` split what `written` already counted: a
        # re-run whose logic did not affect a given record leaves that
        # record's file byte-identical (ignoring `ingested_at`) and does
        # not touch it on disk -- see `records_equal_ignoring_timestamp`.
        "changed": 0,
        "unchanged": 0,
        "name_pi_renamed": 0,
        "unresolved": [],
        "written_by_book": defaultdict(int),
        # `decisions.md §24b`-4: divergence entries carry coordinates and
        # the reason, never the original PI string -- see
        # `scripts/codex_neutral_name.py::divergence_entry`.
        "renamed_records": [],
    }

    file_cache: dict[tuple[str, str], list[str]] = {}
    used_by_book: dict[str, set[str]] = defaultdict(set)
    ingested_at = ingested_at_now()

    for unit in units:
        book = unit["book"]
        source_file = unit["source_file"]
        line = unit["source_line"]
        key = unit.get("corpus_key") or unit.get("key") or unit["name"]
        name = unit["name"]

        cache_key = (book, source_file)
        if cache_key not in file_cache:
            file_cache[cache_key] = resolve_file(dir_index, root, book, source_file)
        hits = file_cache[cache_key]
        if len(hits) != 1:
            report["unresolved"].append(
                {"book": book, "source_file": source_file, "hits": len(hits), "key": key}
            )
            continue
        path = hits[0]

        raw_line = read_row(path, line)
        tokens = row_tokens(raw_line)
        name_declared, desc_declared = declared_pi(tokens)
        # Scan BOTH the bare `name` and the full `key` -- a key can carry a
        # blacklisted term the bare display name does not (found live this
        # cycle: `isg_abilities_faith.lst`'s "Exalted Boon ~ Asmodeus ~
        # Hellfire Blast" has name "Hellfire Blast", clean, but its key
        # names a blacklisted deity).
        name_hit = normalized_term_hit(name) or normalized_term_hit(key)
        name_is_pi = name_declared or bool(name_hit)

        description = desc_value(tokens)
        free_text = extract_free_text(raw_line)
        desc_hit = normalized_term_hit(free_text) if free_text else None
        pi_redacted = desc_declared or bool(desc_hit)
        stored_description = description
        if pi_redacted and description is not None:
            stored_description = REDACTED_PI_MARKER
            for t in tokens:
                if t["key"] == "DESC":
                    t["value"] = REDACTED_PI_MARKER

        has_formula_token = any(t["key"] == "DEFINE" or t["key"].startswith("BONUS") for t in tokens)
        wiring_class = "static" if has_formula_token else "display"
        wiring_signals = (
            ["static:has_magnitude_token"] if has_formula_token else ["display:no_magnitude_token"]
        )

        rel_path = os.path.relpath(path, root)
        sha256 = sha256_file(path)
        used = used_by_book[book]

        fields_redacted: list[str] = []
        if pi_redacted:
            fields_redacted.append("description")

        if name_is_pi:
            # `decisions.md §24` -- ingest under a Codex-generated neutral
            # name derived ONLY from (kind, book, source_file, source_line).
            # See `scripts/codex_neutral_name.py`'s module docstring and
            # `scripts/tests/test_codex_neutral_name.py` for the `§24b`-1
            # proof this cannot be influenced by the original PI name.
            codex_name = neutral_name("ability", book, source_file, line)
            codex_key = neutral_key("ability", book, source_file, line)
            scrubbed_tokens, extra_redacted = scrub_name_pi_tokens(tokens, name, key)
            record_name = codex_name
            record_key = codex_key
            record_tokens = scrubbed_tokens
            slug = slugify(codex_name, used)
            report["name_pi_renamed"] += 1
            report["renamed_records"].append(
                divergence_entry("ability", book, source_file, line, reason="name_pi_blocked")
            )
            codex_generated_name = True
            rename_info = {
                "reason": "name_pi_blocked",
                "coordinate": f"{book}:{os.path.basename(source_file)}:{line}",
            }
            fields_redacted.append("name")
            if extra_redacted:
                fields_redacted.append("raw_tokens")
        else:
            # `decisions.md §17` gap-close: a clean bare name/key does not
            # mean the row's OTHER token values are clean -- see
            # `scrub_blacklist_pi_tokens`'s own docstring for the two
            # already-shipped records (`inner_sea_gods/ability/adept.json`,
            # `inner_sea_magic/ability/diplomatic_student.json`) that
            # proved this live.
            scrubbed_tokens, extra_redacted = scrub_blacklist_pi_tokens(tokens, desc_already_redacted=pi_redacted)
            record_name = name
            record_key = key
            record_tokens = scrubbed_tokens
            slug = slugify(name, used)
            codex_generated_name = False
            rename_info = None
            if extra_redacted:
                fields_redacted.append("raw_tokens")

        license_value = "PI-REDACTED" if fields_redacted else "OGL"

        record = {
            "population": "in_scope",
            "completeness": "full" if stored_description else "chassis_only",
            "ingested_at": ingested_at,
            "data": {
                "key": record_key,
                "name": record_name,
                "description": stored_description,
                "raw_tokens": record_tokens,
            },
            "source": {
                "kind": "lst_token",
                "path": rel_path,
                "sha256": sha256,
                "line": line,
                "record_key": record_key,
            },
            "wiring_class": wiring_class,
            "wiring_class_signals": wiring_signals,
            "license": license_value,
            "pi_field": ",".join(fields_redacted) if fields_redacted else None,
            "pi_marker": PI_MARKER_REDACTED if fields_redacted else None,
            # `decisions.md §24b`-3: "A field marks it as carrying a
            # Codex-generated name, so no reader or player mistakes it for
            # the printed name." `§24b`-4: the divergence record stops at
            # the coordinate -- never the original string.
            "codex_generated_name": codex_generated_name,
            "rename": rename_info,
        }

        write_dir_book = CORPUS_WRITE_DIR_ALIASES.get(book, book)
        out_dir = os.path.join(REPO_ROOT, "data/corpus", write_dir_book, "ability")
        out_file = os.path.join(out_dir, f"{slug}.json")
        if not dry_run:
            os.makedirs(out_dir, exist_ok=True)
            existing = None
            if os.path.exists(out_file):
                try:
                    with open(out_file, encoding="utf-8") as fh:
                        existing = json.load(fh)
                except (OSError, json.JSONDecodeError):
                    existing = None
            if existing is not None and records_equal_ignoring_timestamp(existing, record):
                report["unchanged"] += 1
            else:
                with open(out_file, "w", encoding="utf-8") as fh:
                    json.dump(record, fh, indent=2, ensure_ascii=False)
                    fh.write("\n")
                report["changed"] += 1
        report["written"] += 1
        report["written_by_book"][book] += 1

    report["written_by_book"] = dict(sorted(report["written_by_book"].items()))
    report["term_list_size"] = len(PI_BLACKLIST_TERMS)
    text = json.dumps(report, indent=2)
    print(text)
    if out_path:
        with open(out_path, "w", encoding="utf-8") as fh:
            fh.write(text)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
