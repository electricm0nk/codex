"""Write transcribed tables into the `rules_tables` data package (SD-37 E4a.4a).

SD-37 E4a.4 removed the compiled `src/rules_core/rules_tables/` module; the data package
(`data/rules_tables/<table id>.json`, `src/rules_core/rules_data_package.rs`) is the tables' only
home. The two Python transcribers (`transcribe_monster_tables.py`,
`transcribe_companion_tables.py`) build each table's rows as dicts shaped like the table's row type
and hand them here.

Two steps, both required:

1. `write_tables` writes each table as a package file (`format`, `table`, a provisional licence
   stamp, `rows`), atomically per file (same-directory temp file, then `os.replace`).
2. `normalise_package` runs the package's own normaliser
   (`cargo run --bin rules_tables_package -- --write`). It loads every table as its Rust row type
   (a row that does not match the type fails the run and nothing is written), renders it in the
   canonical form, and re-stamps the licence from `pi_screening`, so the file on disk is exactly
   what `rules_tables_package --check` accepts. The stamp is computed by the Rust screen, never
   re-implemented here.

`write_book`-style callers run both, then `check_written` confirms every file they wrote is still
there: the normaliser deletes a `*.json` no registered table owns, so a table id the package does
not register would otherwise vanish without a word.
"""

from __future__ import annotations

import json
import os
import subprocess

PACKAGE_DIR = "data/rules_tables"
FORMAT = "codex.rules_tables/1"
SCREEN = "pi_screening::classify_field"

REPO_ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))


def table_path(table_id: str, repo_root: str = REPO_ROOT) -> str:
    return os.path.join(repo_root, PACKAGE_DIR, f"{table_id}.json")


def write_tables(tables: dict[str, list[dict]], repo_root: str = REPO_ROOT) -> list[str]:
    """Write `{table id: rows}` as package files under `repo_root`; return the paths written."""
    written = []
    for table_id, rows in tables.items():
        envelope = {
            "format": FORMAT,
            "table": table_id,
            # Provisional: `normalise_package` re-stamps from the Rust screen.
            "licence": {
                "license": "OGL",
                "book": table_id.split("/")[0],
                "screen": SCREEN,
                "pi_hits": [],
                "redacted": [],
            },
            "rows": rows,
        }
        path = table_path(table_id, repo_root)
        os.makedirs(os.path.dirname(path), exist_ok=True)
        tmp_path = f"{path}.tmp"
        with open(tmp_path, "w", encoding="utf-8") as handle:
            json.dump(envelope, handle, ensure_ascii=False, indent=2)
            handle.write("\n")
        os.replace(tmp_path, path)
        written.append(path)
    return written


def normalise_package(repo_root: str = REPO_ROOT) -> str:
    """Run `rules_tables_package --write` over the package; return its stdout. Raises on failure."""
    result = subprocess.run(
        ["cargo", "run", "--locked", "-q", "-p", "codex", "--bin", "rules_tables_package", "--", "--write"],
        cwd=repo_root,
        capture_output=True,
        text=True,
    )
    if result.returncode != 0 or "verdict=WROTE" not in result.stdout:
        raise SystemExit(
            "rules_tables_package --write failed; the package was not normalised:\n"
            + result.stdout
            + result.stderr
        )
    return result.stdout


def check_written(paths: list[str]) -> None:
    """Every path written must survive the normaliser (it removes files no table owns)."""
    missing = [p for p in paths if not os.path.exists(p)]
    if missing:
        raise SystemExit(
            "rules_tables_package removed table file(s) no registered table owns "
            "(register the table in src/rules_core/rules_data_package.rs first): " + ", ".join(missing)
        )
