"""Parses PCGen BatchExporter output in the `KEY=VALUE`-per-line shape
emitted by `docs/release/SD-33-computed-value-verification/artifacts/epic-2-oracle-harness/computed-values.txt.ftl`
(`AT-33-E2-002`). This is the harness's one read path for real oracle
export bytes -- kept in its own module and its own test class
(`OracleExportParsingTest` in `scripts/tests/test_oracle_harness.py`) so the
comparison-logic tests (`CompareUnitTest`) never have to call it, per the
fixture-discipline note in that test file's module docstring.
"""

from __future__ import annotations


def parse_oracle_export(text):
    """Parse `KEY=VALUE` lines into a dict. Blank lines and lines starting
    with `#` are skipped. The value is split on the *first* `=` only, so a
    value that itself contains `=` (none of this template's tokens do, but
    a future one might) is not truncated. A key the export never emitted is
    simply absent from the returned dict -- callers distinguish "no such
    key" (`.get(key)` -> `None`) from a key present with an empty value
    (`""`, `.get(key)` -> `""`) on purpose; `compare_unit` treats both as
    `unverifiable`, but the two are not conflated inside this function.
    """
    result = {}
    for raw_line in text.splitlines():
        line = raw_line.strip()
        if not line or line.startswith("#"):
            continue
        if "=" not in line:
            continue
        key, _, value = line.partition("=")
        result[key.strip()] = value.strip()
    return result


def load_oracle_export(path):
    """Read and parse a real BatchExporter output file from disk."""
    with open(path, "r", encoding="utf-8") as f:
        return parse_oracle_export(f.read())
