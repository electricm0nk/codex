#!/usr/bin/env python3
"""SD-32 `decisions.md §20` — generic ingest for the `SIMPLE_FILENAME_KINDS`
population (`decisions.md §17` item 1, `src/bin/v06_work_inventory.rs`
`file_kind`): `template`, `power`, `domain`, `language`, `skill`, `deity`.

**Why one script for six kinds.** All six were enumerated by a single
filename-substring rule (`SIMPLE_FILENAME_KINDS`) because their corpus rows
share one shape: a flat, single-line PCGen LST record with no `.MOD`/`.COPY=`
overlay chain of its own.

**`deity` — `decisions.md §24` neutral-name treatment (this cycle).** Every
`deity` unit's own row identity (its `KEY`/name field) IS a deity's proper
name — unlike the other five kinds, whose identity strings are game-
mechanical labels the blacklist's own §2.2 table classifies as OGL-inlinable
(module docstring below, "Why `deity` was excluded before this cycle",
explains why the 57-60-term blacklist alone was never sufficient cover for
this kind). `decisions.md §24`'s operator ruling settles this: `deity` is
now ingested UNCONDITIONALLY under a Codex-generated neutral name derived
ONLY from `(kind, book, source_file, source_line)` — never from the row's
own name/key, not transformed, not truncated, not hashed
(`scripts/codex_neutral_name.py`, reused unchanged; see that module's own
docstring and `scripts/tests/test_codex_neutral_name.py` for the `§24b`-1
proof). `data.name`/`data.key` become the Codex name; `data.codex_generated_name`
is `True` (`§24b`-3); any OTHER raw token whose value restates the record's
own original name/key is separately scrubbed
(`scripts/ingest_ability.py::scrub_name_pi_tokens`, imported, not re-typed —
the same worked precedent `ability` already proved out for this exact
"a KEY: token repeats the row's PI name" shape). The description field still
runs the ordinary declared/blacklist screen below, independent of the name
rename.

**What this does.** For every not-yet-ingested unit of the five target kinds
in `docs/work-inventory.json`:
1. Locates the unit's own book directory via `census_independent.py`'s own
   `discover_book_dirs`/`classify_scope` (reused, not re-derived).
2. Re-reads the cited `(source_file, source_line)` from the pinned oracle and
   asserts the row's own leading field matches the unit's `corpus_key` —
   the same citation-verification discipline
   `scripts/transcribe_monster_tables.py`'s module docstring describes
   ("every emitted value is a substring of the cited row").
3. Parses the row's tab-delimited `KEY:VALUE` fields into `raw_tokens` —
   this is what `scripts/shape_ledger.py`'s join reads to classify the
   unit's formula family; a record with no `source.path`/`source.line`
   match is `no_record` (`decisions.md §20`), and this step is what makes
   the join succeed.
4. PI-screens the row per `docs/governance/ogl-pi-blacklist.md §2.3a`
   (imports the SAME `PI_BLACKLIST_TERMS`/`normalized_term_hit`/
   `canonicalize` this bundle's T9 review scripts use — no second copy of
   the term list) plus PCGen's own declared `NAMEISPI:YES`/`DESCISPI:YES`
   tokens. A hit redacts the offending field to the standing
   `"[redacted PI]"` marker (`src/rules_core/shape_b_v1.rs::REDACTED_PI_MARKER`)
   and stamps `license: "PI-REDACTED"`; no hit stamps `license: "OGL"`.
5. Writes `data/corpus/<book>/<kind>/<slug>.json` in Shape B v1
   (`src/rules_core/shape_b_v1.rs::CorpusRecordV1`) — the same schema
   `gen_book_cache.rs`'s Rust generators emit, byte-compatible (a v1 JSON
   record from either path deserializes into the same Rust type).

**Why `deity` was excluded before this cycle (decisions.md §15 disposition
2, now superseded by §24 for this kind only).**
`ogl-pi-blacklist.md §2.1` names `deity`/`deity_name` as "Product Identity in
CRB" as a FIELD CATEGORY, not as a closed list of terms — but the mechanized
blacklist screen this script (and every other ingest tool in this repo) uses
is a 57-60 TERM list, not a field-category rule, and that term list covers
only the ~20 core Golarion deities plus a few incident-driven additions. A
`deity` record's own row identity (its `KEY`/name field) IS a deity's proper
name in every case — unlike `template`/`power`/`domain`/`language`/`skill`,
whose identity strings are game-mechanical labels (a template's name, a
skill's name) the blacklist's own §2.2 table classifies as OGL-inlinable.
Relying on that same 57-60-term list to decide WHICH deity rows are PI would
have been exactly the shape `decisions.md §15` exists to refuse (a term list
never built to cover ~440 non-core-20 names). `decisions.md §24` replaces
that whole question for `deity`: since the name IS PI in every case, no
term-list judgment call is needed at all — every `deity` row renames
unconditionally under the Codex-generated neutral name, `§24b`'s six binding
conditions (not the blacklist screen) governing shippability.

Usage::

    python3 scripts/ingest_simple_filename_kinds.py \
        --inventory docs/work-inventory.json \
        --pcgen-root "$PCGEN_CORPUS_ROOT" \
        --out-root data/corpus \
        [--kind template] [--dry-run]

`--dry-run` performs every step (citation re-read, PI screen) but does not
write any file — used to produce the pre-flight summary in the cycle
receipt without touching `data/corpus`.
"""
from __future__ import annotations

import argparse
import hashlib
import json
import os
import re
import sys
from collections import Counter, defaultdict
from datetime import datetime, timezone

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import census_independent as ci  # noqa: E402
from sd32_t9_pi_review_feat_equipment import PI_BLACKLIST_TERMS  # noqa: E402
# t9-onboarding-pi-final-leaks-and-generators cycle: `normalized_term_hit`
# (word-bounded only) missed an adjectival/demonym form of a blacklisted
# place name (`PI_BLACKLIST_TERMS` indices 23 and 33) with a suffix
# concatenated directly onto the root with no separator -- the root term
# followed immediately by an alphanumeric suffix has NO word boundary
# after the root, so the word-bounded scan never fires. This is the exact
# shape `pi_scrub.blacklist_term_hit_including_concatenated` exists for --
# `declared_pi_shipping_audit`'s corpus-wide CHECK C found 3 live
# `template` leaks of exactly this shape shipped by this script, in
# `data/corpus/inner_sea_world_guide/template/` (coordinates in this
# cycle's receipt). Used for BOTH `name` and `description` screening
# below -- the weaker call was symmetric across both fields.
from pi_scrub import blacklist_term_hit_including_concatenated  # noqa: E402
from shape_ledger import BOOK_CORPUS_DIR_ALIASES  # noqa: E402
from codex_neutral_name import (  # noqa: E402
    divergence_entry,
    neutral_key,
    neutral_name,
)
from ingest_ability import scrub_blacklist_pi_tokens, scrub_name_pi_tokens  # noqa: E402

REDACTED_PI_MARKER = "[redacted PI]"

TARGET_KINDS = ("template", "power", "domain", "language", "skill", "deity")

# `decisions.md §24`: the name IS Product Identity for every unit of this
# kind (module docstring's "deity" section) -- no blacklist/declared-PI
# judgment call decides it per-record the way it does for the other five
# `TARGET_KINDS`; every `deity` unit renames unconditionally.
NAME_ALWAYS_PI_KINDS = frozenset({"deity"})

FREE_TEXT_TAG_PREFIXES = ("DESC:", "BENEFIT:", "SPECIALS:", "SA:")


def compose_source_path(file_path: str, pcgen_root: str) -> str:
    """Compose a corpus record's `source.path` relative to the PCGen data
    root, and refuse to produce a shape `scripts/corpus_literal_sweep` (the
    Rust `book_dir_of` shape check, `src/bin/corpus_literal_sweep.rs`) would
    reject.

    A record's `source.path` is not cosmetic: `scripts/shape_ledger.py`
    joins units to corpus records on `(book, source_basename, source_line)`,
    and `corpus_literal_sweep`'s own `book_dir_of` requires the path keep
    its leading `<system>/` segment (e.g. `pathfinder/...`) — at least 5
    `/`-separated segments (4 for the `dreamscarred_press` publisher, which
    has no `<line>` segment). `pcgen_root` must therefore be the PCGen data
    root itself (`PCGEN_CORPUS_ROOT`, e.g. `.../pcgen/data`), NOT that root
    joined with `"pathfinder"` — the exact defect this check exists to catch
    mechanically rather than by a later sweep stage finding it downstream
    (SD-32 `decisions.md §20`, 3,124 records shipped with the leading
    `pathfinder/` segment missing before `corpus_literal_sweep` caught it).
    """
    rel = os.path.relpath(file_path, pcgen_root).replace(os.sep, "/")
    segments = [s for s in rel.split("/") if s]
    is_shaped = len(segments) >= 5 or (len(segments) == 4 and segments[1] == "dreamscarred_press")
    if not is_shaped:
        raise ValueError(
            f"source.path {rel!r} is not <system>/<publisher>/<line>/<book>/<file>-shaped -- "
            "pcgen_root must be the PCGen data root (PCGEN_CORPUS_ROOT), not that root joined "
            "with a system segment such as 'pathfinder'"
        )
    return rel


def row_identity(raw_line: str) -> str:
    """The row's real identifying string for citation matching.

    PCGen's LST format lets a row declare an explicit `KEY:` token that
    OVERRIDES the leading (display-name) column as the row's actual lookup
    key -- the identical convention `src/bin/ingest_races.rs`'s
    `row.first("KEY")`, `src/bin/ingest_race_traits.rs`'s `.find(|f| f.key
    == "KEY")`, and `scripts/derive_monster_ability_save_dc_fixtures.py`'s
    `token(fields, "KEY") or fields[0]` all already honour. Without this, a
    row like `ma_templates.lst:15` ("Has Swim Speed  KEY:Swimming Master ~
    Has Swim  ...") citation-mismatches forever: the inventory's own
    `corpus_key` ("Swimming Master ~ Has Swim") is the row's declared KEY,
    not its leading display name. Falls back to the leading field when no
    `KEY:` token is present (most rows)."""
    for field in raw_line.split("\t"):
        field = field.strip()
        if field.upper().startswith("KEY:"):
            return field.split(":", 1)[1].strip()
    return raw_line.split("\t", 1)[0].strip()


def unit_in_scope(kind: str, kinds: set[str], book: str, books: set[str] | None) -> bool:
    """`True` when `(kind, book)` should be processed this run -- `kind`
    must be a member of `kinds`, and `book` must be a member of `books`
    when `books` is not `None` (the `--book` scoped-remediation filter,
    `t9-onboarding-pi-final-leaks-and-generators` cycle). `books=None`
    means "no restriction", matching every existing caller's behaviour
    before this cycle added the flag."""
    if kind not in kinds:
        return False
    if books is not None and book not in books:
        return False
    return True


def resolve_out_dir(out_root: str, book: str, kind: str) -> str:
    """`shape_ledger.py`'s reader joins a `book`-named unit against the
    `BOOK_CORPUS_DIR_ALIASES`-aliased corpus directory (e.g. `bestiary`
    reads from `data/corpus/beastiary/`, historical spelling) -- the writer
    must honour the same alias or every record it writes under the
    unaliased directory is invisible to the join and reports `no_record`
    forever (decisions.md §20's "search for an existing ingest path"
    footgun 1, re-confirmed live: 1,050 `template` + 14 `language` `bestiary`
    units written to `data/corpus/bestiary/` instead of
    `data/corpus/beastiary/`)."""
    return os.path.join(out_root, BOOK_CORPUS_DIR_ALIASES.get(book, book), kind)


def slugify(name: str) -> str:
    s = re.sub(r"[^a-z0-9]+", "_", name.strip().lower())
    return s.strip("_") or "unnamed"


def parse_row(raw_line: str) -> list[dict]:
    """Tab-delimited `KEY:VALUE` fields -> raw_tokens list, same shape as
    every existing corpus record's `data.raw_tokens` (see
    `scripts/shape_ledger.py`'s own docstring for the join contract)."""
    tokens = []
    for field in raw_line.split("\t"):
        field = field.strip()
        if not field or ":" not in field:
            continue
        key, _, value = field.partition(":")
        key = key.strip()
        if not key:
            continue
        tokens.append({"key": key, "value": value})
    return tokens


def declared_pi(raw_tokens: list[dict]) -> tuple[bool, bool]:
    """(name_is_pi, desc_is_pi) from PCGen's own NAMEISPI:/DESCISPI: tokens."""
    name_pi = any(
        t["key"].upper() == "NAMEISPI" and t["value"].strip().upper() == "YES"
        for t in raw_tokens
    )
    desc_pi = any(
        t["key"].upper() == "DESCISPI" and t["value"].strip().upper() == "YES"
        for t in raw_tokens
    )
    return name_pi, desc_pi


def free_text_of(raw_tokens: list[dict]) -> str:
    parts = []
    for t in raw_tokens:
        if f"{t['key'].upper()}:" in FREE_TEXT_TAG_PREFIXES:
            parts.append(t["value"].split("|", 1)[0])
    return " ".join(parts).strip()


def build_book_index(pcgen_root: str, inv: dict) -> tuple[dict[str, str], dict[str, list[str]]]:
    """(book_id -> absolute book directory path, book_id -> pcc_includes ids),
    in-scope books only. A unit's own `source_file` sometimes resolves not in
    its own book's directory but in a `pcc_includes` dependency's directory
    (e.g. `core_essentials/ce_templates.lst`, shared via PCC include rather
    than duplicated per book — `docs/work-inventory.json` `.books[].
    pcc_includes`) — `find_file` below falls back to these."""
    book_dirs = ci.discover_book_dirs(pcgen_root)
    scope = ci.classify_scope(book_dirs, inv)
    pathfinder_root = os.path.join(pcgen_root, "pathfinder")
    paths = {bd.book_id: os.path.join(pathfinder_root, bd.rel_path) for bd in scope.in_scope}
    includes = {b["id"]: b.get("pcc_includes") or [] for b in inv["books"]}
    return paths, includes


def find_file(book: str, book_paths: dict, includes: dict, basename: str, cache: dict) -> str | None:
    def index_for(bdir: str) -> dict:
        if bdir not in cache:
            idx = {}
            for dirpath, _dn, filenames in os.walk(bdir):
                for fn in filenames:
                    idx.setdefault(fn, os.path.join(dirpath, fn))
            cache[bdir] = idx
        return cache[bdir]

    own_dir = book_paths.get(book)
    if own_dir is not None:
        hit = index_for(own_dir).get(basename)
        if hit is not None:
            return hit
    for dep in includes.get(book, []):
        dep_dir = book_paths.get(dep)
        if dep_dir is None:
            continue
        hit = index_for(dep_dir).get(basename)
        if hit is not None:
            return hit
    return None


def sha256_of(path: str) -> str:
    h = hashlib.sha256()
    with open(path, "rb") as fh:
        h.update(fh.read())
    return h.hexdigest()


def read_line(path: str, line_no: int) -> str | None:
    with open(path, "r", encoding="utf-8", errors="replace") as fh:
        for i, raw in enumerate(fh, start=1):
            if i == line_no:
                return raw.rstrip("\n").replace("­", "-")
    return None


def main(argv: list[str] | None = None) -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--inventory", default="docs/work-inventory.json")
    ap.add_argument("--pcgen-root", required=True)
    ap.add_argument("--out-root", default="data/corpus")
    ap.add_argument("--kind", action="append", choices=TARGET_KINDS)
    # t9-onboarding-pi-final-leaks-and-generators cycle: a scoped-remediation
    # mode, the Python-side sibling of `gen_cache_class_feature.rs`'s own
    # `--coordinates <file>` mode (`decisions.md §19`'s "no re-screen on
    # amendment" gap) -- re-screening a full `--kind` pass touches every
    # unit of that kind corpus-wide (2,248 for `template` alone), an
    # unacceptable blast radius for closing a handful of newly-confirmed
    # leaks. `--book` narrows the SAME kind pass to one book's units only,
    # so a re-run after a blacklist amendment (or, as here, a scan-strength
    # fix) touches only the book that actually needs it.
    ap.add_argument("--book", action="append", help="restrict to these book id(s) only (repeatable)")
    ap.add_argument("--dry-run", action="store_true")
    ap.add_argument("--report-out", default=None, help="write the full JSON summary (incl. renamed_records) here")
    args = ap.parse_args(argv)

    kinds = set(args.kind) if args.kind else set(TARGET_KINDS)
    books = set(args.book) if args.book else None

    with open(args.inventory, "r", encoding="utf-8") as fh:
        inv = json.load(fh)

    book_paths, book_includes = build_book_index(args.pcgen_root, inv)
    file_cache: dict = {}
    slug_seen: dict[tuple[str, str], set] = defaultdict(set)

    stats = Counter()
    citation_mismatches = []
    written = []
    pi_hits = Counter()
    name_pi_renamed = Counter()
    # `decisions.md §24b`-4: divergence entries carry coordinates and the
    # reason, never the original PI string -- see
    # `scripts/codex_neutral_name.py::divergence_entry`.
    renamed_records = []

    for unit in inv["units"]:
        kind = unit.get("kind")
        book = unit["book"]
        if not unit_in_scope(kind, kinds, book, books):
            continue
        basename = unit["source_file"]
        line_no = unit["source_line"]
        corpus_key = unit["corpus_key"]
        name = unit["name"]
        stats[f"{kind}:seen"] += 1

        if book not in book_paths:
            stats[f"{kind}:no_book_dir"] += 1
            continue
        file_path = find_file(book, book_paths, book_includes, basename, file_cache)
        if file_path is None:
            stats[f"{kind}:no_file"] += 1
            continue
        raw_line = read_line(file_path, line_no)
        if raw_line is None:
            stats[f"{kind}:no_line"] += 1
            continue
        identity = row_identity(raw_line)
        # `v06_work_inventory.rs` composes some kinds' `corpus_key` as
        # "<group header> ~ <row identity>" (grouping context prepended,
        # same convention already shipped e.g. `data/corpus/.../
        # air_domain/lightning_arc.json`'s `record_key: "Air Domain ~
        # Lightning Arc"`) while the LST row's own leading field is the
        # bare leaf identity -- accept that shape as a citation match, not
        # only a byte-exact one.
        if identity != corpus_key and not corpus_key.endswith(" ~ " + identity):
            citation_mismatches.append(
                {"book": book, "file": basename, "line": line_no, "expected": corpus_key, "found": identity}
            )
            stats[f"{kind}:citation_mismatch"] += 1
            continue

        raw_tokens = parse_row(raw_line)
        name_pi, desc_pi = declared_pi(raw_tokens)
        term_hit_name = blacklist_term_hit_including_concatenated(name)
        free_text = free_text_of(raw_tokens)
        term_hit_desc = blacklist_term_hit_including_concatenated(free_text) if free_text else None

        # `decisions.md §24`: `deity` (and any other `NAME_ALWAYS_PI_KINDS`
        # member) renames UNCONDITIONALLY -- the name IS PI in every case
        # for this kind, so no declared/blacklist judgment call decides it
        # the way it still does for the other five kinds below.
        always_pi = kind in NAME_ALWAYS_PI_KINDS
        name_is_pi = always_pi or name_pi or bool(term_hit_name)
        redact_desc = desc_pi or bool(term_hit_desc)

        out_tokens = []
        for t in raw_tokens:
            v = t["value"]
            if redact_desc and f"{t['key'].upper()}:" in FREE_TEXT_TAG_PREFIXES:
                head, _, tail = v.partition("|")
                v = REDACTED_PI_MARKER + (("|" + tail) if tail else "")
            out_tokens.append({"key": t["key"], "value": v})

        # `decisions.md §17` gap-close (found live, `pi-key-rawtokens-screen`
        # cycle 2026-08-23): a record whose bare `name`/`description` are
        # clean can still carry a blacklisted term inside a DIFFERENT
        # raw-token value -- e.g. a `SPELLLEVEL` token's own
        # `PREDEITY:1,<deity>` segment. The description-only redaction loop
        # above never touched those. `scrub_blacklist_pi_tokens` (the same
        # generic fix `ingest_ability.py` proved) scans EVERY token value
        # against the SIGNED-OFF 60-term blacklist, independent of whether
        # the name triggered a rename below. Applied before the rename
        # branch so a renamed record's `scrub_name_pi_tokens` pass (which
        # scrubs restatements of the ORIGINAL name/key) runs on top of an
        # already blacklist-clean token set.
        out_tokens, blacklist_extra_redacted = scrub_blacklist_pi_tokens(
            out_tokens, desc_already_redacted=redact_desc
        )

        fields_redacted: list[str] = []
        codex_generated_name = False
        rename_info = None

        if name_is_pi:
            # `decisions.md §24` gap-close (found live, `pi-key-rawtokens-
            # screen` follow-up cycle, 2026-08-23,
            # `cargo run --bin declared_pi_shipping_audit`'s 28
            # `NAME-PI-SHIPPED` violations in `language`/`template`): this
            # branch used to fire ONLY for `always_pi` kinds (`deity`), with
            # every OTHER `TARGET_KINDS` member falling through to a legacy
            # pre-§24 path that replaced `name`/`key` with the literal
            # `REDACTED_PI_MARKER` string in place. `declared_pi_shipping_
            # audit.rs`'s own check (`§24b`-3's own reasoning: "a key/name
            # cannot be redacted -- its mere presence on disk IS the
            # violation") rejects that shape outright: a declared-PI name
            # must ship under a Codex-generated NEUTRAL name
            # (`codex_generated_name: true`), never as an in-place marker
            # substitution, for every kind this script serves, not only the
            # `always_pi` ones. `always_pi` still exists (it decides
            # whether a CLEAN-looking name is treated as PI regardless of
            # declaration/blacklist for `deity`), but no longer decides
            # WHICH remediation a PI name gets -- there is only one now.
            #
            # `§24b`-1: derived ONLY from (kind, book, source_file,
            # source_line) -- never from `name`/`corpus_key`. `§24b`-2: any
            # OTHER token restating the original name/key is separately
            # scrubbed (`scrub_name_pi_tokens`, the same worked precedent
            # `ability` proved).
            codex_name = neutral_name(kind, book, basename, line_no)
            codex_key = neutral_key(kind, book, basename, line_no)
            scrubbed_tokens, extra_redacted = scrub_name_pi_tokens(out_tokens, name, corpus_key)
            out_name = codex_name
            out_key = codex_key
            out_tokens = scrubbed_tokens
            codex_generated_name = True
            fields_redacted.append("name")
            if extra_redacted and "raw_tokens" not in fields_redacted:
                fields_redacted.append("raw_tokens")
            rename_info = {
                "reason": "name_pi_blocked",
                "coordinate": f"{book}:{os.path.basename(basename)}:{line_no}",
            }
            renamed_records.append(divergence_entry(kind, book, basename, line_no, reason="name_pi_blocked"))
            name_pi_renamed[kind] += 1
        else:
            out_name = name
            out_key = corpus_key

        if redact_desc:
            fields_redacted.append("description")
        if blacklist_extra_redacted and "raw_tokens" not in fields_redacted:
            fields_redacted.append("raw_tokens")

        if fields_redacted:
            license_val = "PI-REDACTED"
            pi_field = ",".join(fields_redacted)
            pi_marker = "redacted"
            pi_hits[kind] += 1
        else:
            license_val = "OGL"
            pi_field = None
            pi_marker = None

        rec = {
            "population": "in_scope",
            "completeness": "full" if free_text else "chassis_only",
            "ingested_at": datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ"),
            "data": {
                "key": out_key,
                "name": out_name,
                "raw_tokens": out_tokens,
            },
            "source": {
                "kind": "lst_token",
                "path": compose_source_path(file_path, args.pcgen_root),
                "sha256": sha256_of(file_path),
                "line": line_no,
                "record_key": out_key,
            },
            "wiring_class": "display",
            "wiring_class_signals": ["display:sd32_simple_filename_kind_ingest"],
            "license": license_val,
            "pi_field": pi_field,
            "pi_marker": pi_marker,
            # `decisions.md §24b`-3/4: visible rename marker + coordinate-
            # only divergence record. Always present (even `False`/`None`)
            # so every record of a kind this script touches carries the
            # same shape.
            "codex_generated_name": codex_generated_name,
            "rename": rename_info,
        }

        slug = slugify(codex_name if name_is_pi else corpus_key)
        seen = slug_seen[(book, kind)]
        if slug in seen:
            slug = f"{slug}_{line_no}"
        seen.add(slug)

        out_dir = resolve_out_dir(args.out_root, book, kind)
        out_path = os.path.join(out_dir, f"{slug}.json")
        if not args.dry_run:
            os.makedirs(out_dir, exist_ok=True)
            with open(out_path, "w", encoding="utf-8") as fh:
                json.dump(rec, fh, indent=2, ensure_ascii=False)
                fh.write("\n")
        written.append(out_path)
        stats[f"{kind}:written"] += 1

    summary = {
        "stats": dict(stats),
        "pi_redacted_by_kind": dict(pi_hits),
        "name_pi_renamed_by_kind": dict(name_pi_renamed),
        "citation_mismatches": citation_mismatches,
        "written_count": len(written),
        "renamed_count": len(renamed_records),
        # `decisions.md §24b`-4: coordinates + reason only, never the
        # original PI string.
        "renamed_records": renamed_records,
    }
    print(json.dumps(summary, indent=2))
    if args.report_out:
        with open(args.report_out, "w", encoding="utf-8") as fh:
            json.dump(summary, fh, indent=2, ensure_ascii=False)
            fh.write("\n")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
