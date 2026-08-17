#!/usr/bin/env python3
"""scripts/site_dashboard_pi_gate.py -- Decision 12 (2026-08-17, operator
ruling) requirement #3: "A gate, proven able to fail. A verify.sh stage
must fail when the committed feed or any shard carries a declared-PI name.
Mutation-prove it by seeding one, in both the top-level feed and a shard."

WHAT THIS SCANS: every committed file under `site/dashboard/` that ends in
`.json`, walked as a decoded JSON document, every string leaf checked
against the pinned PCGen oracle's own full `NAMEISPI:YES` name index
(`scripts/observer/pi_redaction.py::build_declared_pi_name_index`) with
EXACT string-leaf equality (see `find_declared_pi_leaks`'s own docstring
for why exact-match, not a substring/word-boundary scan, is the right
default: an earlier word-boundary version flagged the ordinary, non-PI
"Shackles of Compliance" purely because "Shackles" occurs as one word
inside it). A SECOND, book-scoped pass
(`find_declared_pi_leaks_in_shard_rows`) additionally runs over every
`units/*.json` shard's own `fields`/`rows` schema against a PER-BOOK
declared-PI index (`build_declared_pi_name_book_index`), closing the
name-declared-PI-in-one-book-but-not-another gap the book-blind index
cannot see (SD31-W13-INTEGRATE-001 finding 2). This is a SAFETY NET, not
the primary defense: the primary defense is the producer redacting at
generation time (`pf1e_dashboard_producer.py`'s `build_unit_shards` and
`_parse_lst_first_field`, both wired this cycle). This gate exists because
a hand-edit, a reverted redaction, or a future change to the producer that
forgets to call the reader are all real failure modes a generation-time fix
alone cannot catch -- exactly `declared_pi_shipping_audit.rs`'s own
rationale for `data/corpus/`, applied to this second surface.

KNOWN RESIDUAL GAP (SD31-W13-INTEGRATE-001 finding 3, not yet closed): a
declared-PI name EMBEDDED inside a longer published string (e.g. a derived
`categories[*].label` built from a TYPE token) is invisible to exact-leaf
matching. `OPEN-ISSUES.md` names the count and the remedy; do not read a
CLEAN result from this gate as proof no such embedding exists.

Exit 0 and print `site-dashboard-pi-gate: CLEAN` when no declared-PI name is
found in any scanned file. Exit 1 and print every hit (file, JSON path,
name) otherwise.

Degraded-oracle posture: if the pinned checkout cannot be found at all, this
prints a loud warning and exits 1 -- a gate that cannot see the oracle
cannot prove anything clean, and "could not check" must never read as
"checked and clean" (Decision 12 requirement #2's whole point, applied to
the gate itself, not just the producer).

Run: python3 scripts/site_dashboard_pi_gate.py
Wired as the `site-dashboard-pi-gate` stage in `scripts/verify.sh`.
"""
from __future__ import annotations

import glob
import importlib.util
import json
import os
import pathlib
import sys

_REPO_ROOT = pathlib.Path(__file__).resolve().parent.parent
_PI_REDACTION = _REPO_ROOT / "scripts" / "observer" / "pi_redaction.py"
_spec = importlib.util.spec_from_file_location("pi_redaction", _PI_REDACTION)
pi_redaction = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(pi_redaction)

DASHBOARD_DIR = str(_REPO_ROOT / "site" / "dashboard")


def scanned_files(dashboard_dir: str) -> list[str]:
    """Every committed `.json` file under `site/dashboard/`, sorted for a
    deterministic report. Includes both the top-level feed and any shard
    files under `units/` -- Decision 12 flags both surfaces, and nothing
    here special-cases either path."""
    return sorted(glob.glob(os.path.join(dashboard_dir, "**", "*.json"), recursive=True))


def main() -> int:
    corpus_root = pi_redaction.pcgen_corpus_root()
    if not os.path.isdir(corpus_root):
        print(
            f"site-dashboard-pi-gate: FAIL — pinned PCGen oracle not found at "
            f"{corpus_root!r}; a gate that cannot read the oracle cannot prove "
            f"the feed clean (run scripts/fetch-pcgen-oracle.sh)",
            file=sys.stderr,
        )
        return 1

    declared_names = pi_redaction.build_declared_pi_name_index(corpus_root)
    if not declared_names:
        # A sweep that found zero declared names anywhere in a 6000+-file
        # oracle checkout is itself a red flag (same "asserts nothing"
        # posture pi-sweep/declared-pi-audit both guard against) -- more
        # likely a broken sparse checkout than a genuinely PI-free oracle.
        print(
            "site-dashboard-pi-gate: FAIL — the pinned oracle sweep found zero "
            "NAMEISPI:YES declarations anywhere; this has never been true of "
            "the real checkout and most likely means the sparse checkout is "
            "broken or empty, not that the corpus is PI-free",
            file=sys.stderr,
        )
        return 1

    patterns = pi_redaction.compile_name_patterns(declared_names)
    name_to_books = pi_redaction.build_declared_pi_name_book_index(corpus_root)

    files = scanned_files(DASHBOARD_DIR)
    if not files:
        print(
            "site-dashboard-pi-gate: CLEAN — no site/dashboard/*.json files "
            "are committed yet (nothing to scan)"
        )
        return 0

    all_hits: list[tuple[str, str, str]] = []
    for path in files:
        try:
            with open(path, encoding="utf-8") as f:
                doc = json.load(f)
        except (OSError, json.JSONDecodeError) as exc:
            print(f"site-dashboard-pi-gate: FAIL — could not read/parse {path}: {exc}", file=sys.stderr)
            return 1
        rel = os.path.relpath(path, str(_REPO_ROOT))
        for json_path, name in pi_redaction.find_declared_pi_leaks(doc, patterns):
            all_hits.append((rel, json_path, name))
        for json_path, name in pi_redaction.find_declared_pi_leaks_in_shard_rows(doc, name_to_books):
            all_hits.append((rel, json_path, name))

    if all_hits:
        print(
            f"site-dashboard-pi-gate: FAIL — {len(all_hits)} declared-PI name(s) "
            f"found across {len(files)} scanned file(s):",
            file=sys.stderr,
        )
        for rel, json_path, name in all_hits:
            print(f"  {rel}:{json_path} carries declared-PI name {name!r}", file=sys.stderr)
        return 1

    print(
        f"site-dashboard-pi-gate: CLEAN — {len(files)} file(s) scanned against "
        f"{len(declared_names)} declared-PI name(s), zero leaked"
    )
    return 0


if __name__ == "__main__":
    sys.exit(main())
