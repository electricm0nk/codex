#!/usr/bin/env python3
"""Check data/cyberpunk-red/corpus/ against MANIFEST.json.

MANIFEST.json pins one entry per corpus kind: kind, file, expected (record
count, or null while the kind has no inventory), figure (the content-unit
inventory figure id), print_only, nested (expected counts of sub-tables inside
records, or null) and lane.

Usage:
  python3 scripts/cpr_manifest_check.py --all
  python3 scripts/cpr_manifest_check.py --kinds weapon,ammo
  python3 scripts/cpr_manifest_check.py --selftest

Exit 0 when every check passes, 1 when any fails, 2 on a usage or setup error.

Failures (each printed as `<CODE> <kind-or-file>: <detail>`):
  MISSING        the manifest names a kind whose file does not exist
  EXTRA          a corpus file exists that the manifest does not name
  COUNT          a kind file's record count differs from `expected`
  NULL_COUNT     (--all only) a manifest entry still has expected: null
  BAD_SOURCE     a record has no `source_page`, or one that is not pNNN in p001-p459
  BAD_FILE       a kind file is not a JSON array of objects
  SUM            the manifest `expected` sum differs from the CUI section 3 column sum
  MANIFEST       the manifest itself is unreadable or malformed

The sum check is always run: it parses the kind table in
docs/release/SD-38-cyberpunk-red/content-unit-inventory.md section 3 and
compares kind names and the sum of the leading integer of each Expected cell
with the manifest. It fails closed on a missing kind, an extra kind or a
mismatch.

Does not cover: the correctness of any record's field values (that is the
page-image sample in E1.V), nested sub-table counts (pinned in the manifest,
not re-counted here), or that a source_page is the right page.
"""
import json
import os
import re
import shutil
import subprocess
import sys
import tempfile

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
CORPUS = os.path.join(ROOT, "data", "cyberpunk-red", "corpus")
CUI = os.path.join(
    ROOT, "docs", "release", "SD-38-cyberpunk-red", "content-unit-inventory.md"
)
NON_KIND_FILES = {"MANIFEST.json", "LICENSE.json"}
PAGE_RE = re.compile(r"^p(\d{3})$")
MAX_PAGE = 459


def load_manifest(path):
    try:
        with open(path) as f:
            m = json.load(f)
    except (OSError, ValueError) as e:
        return None, [f"MANIFEST {path}: cannot read ({e})"]
    errs = []
    kinds = m.get("kinds") if isinstance(m, dict) else None
    if not isinstance(kinds, list) or not kinds:
        return None, [f"MANIFEST {path}: no `kinds` array"]
    seen = set()
    for e in kinds:
        k = e.get("kind") if isinstance(e, dict) else None
        if not isinstance(k, str) or not k:
            errs.append(f"MANIFEST entry without a kind: {e!r}")
            continue
        if k in seen:
            errs.append(f"MANIFEST {k}: duplicate entry")
        seen.add(k)
        if e.get("file") != f"{k}.json":
            errs.append(f"MANIFEST {k}: file must be {k}.json, got {e.get('file')!r}")
        x = e.get("expected")
        if x is not None and (not isinstance(x, int) or isinstance(x, bool) or x < 0):
            errs.append(f"MANIFEST {k}: expected must be a non-negative int or null")
        if not isinstance(e.get("print_only"), bool):
            errs.append(f"MANIFEST {k}: print_only must be a bool")
    return (None if errs else m), errs


def cui_table(path):
    """Return {kind: leading integer or None} from CUI section 3's kind table."""
    try:
        text = open(path).read()
    except OSError as e:
        return None, [f"SUM cannot read {path}: {e}"]
    m = re.search(r"^## §3 .*?(?=^## §4)", text, re.S | re.M)
    if not m:
        return None, ["SUM CUI section 3 not found"]
    out = {}
    for line in m.group(0).splitlines():
        if not line.startswith("|"):
            continue
        cells = [c.strip() for c in re.split(r"(?<!\\)\|", line.strip().strip("|"))]
        if len(cells) < 2 or cells[0] in ("Kind",) or set(cells[0]) <= set("-: "):
            continue
        names = re.findall(r"`([a-z_0-9]+)`", cells[0])
        lead = re.match(r"\s*(\d+)", cells[1])
        for n in names:
            out[n] = int(lead.group(1)) if lead else None
    return out, []


def check_sum(manifest, cui_path):
    table, errs = cui_table(cui_path)
    if table is None:
        return errs
    mk = {e["kind"]: e.get("expected") for e in manifest["kinds"]}
    for k in sorted(set(table) - set(mk)):
        errs.append(f"SUM {k}: in CUI section 3, absent from the manifest")
    for k in sorted(set(mk) - set(table)):
        errs.append(f"SUM {k}: in the manifest, absent from CUI section 3")
    for k in sorted(set(mk) & set(table)):
        if mk[k] != table[k]:
            errs.append(f"SUM {k}: manifest expected {mk[k]!r}, CUI section 3 says {table[k]!r}")
    ms = sum(v for v in mk.values() if v is not None)
    ts = sum(v for v in table.values() if v is not None)
    if ms != ts:
        errs.append(f"SUM total: manifest {ms}, CUI section 3 {ts}")
    return errs


def check_kind(entry, corpus, require_count):
    k = entry["kind"]
    errs = []
    path = os.path.join(corpus, entry["file"])
    exp = entry.get("expected")
    if exp is None and require_count:
        errs.append(f"NULL_COUNT {k}: expected is null")
    if not os.path.isfile(path):
        return errs + [f"MISSING {k}: {entry['file']}"]
    try:
        recs = json.load(open(path))
    except ValueError as e:
        return errs + [f"BAD_FILE {k}: not JSON ({e})"]
    if not isinstance(recs, list) or not all(isinstance(r, dict) for r in recs):
        return errs + [f"BAD_FILE {k}: not a JSON array of objects"]
    if exp is not None and len(recs) != exp:
        errs.append(f"COUNT {k}: {len(recs)} records, expected {exp}")
    bad = 0
    for i, r in enumerate(recs):
        sp = r.get("source_page")
        m = PAGE_RE.match(sp) if isinstance(sp, str) else None
        if not m or not (1 <= int(m.group(1)) <= MAX_PAGE):
            bad += 1
            if bad <= 3:
                errs.append(f"BAD_SOURCE {k}[{i}]: source_page {sp!r}")
    if bad > 3:
        errs.append(f"BAD_SOURCE {k}: {bad} records in all")
    return errs


def run(manifest_path, corpus, cui_path, kinds, all_kinds):
    m, errs = load_manifest(manifest_path)
    if m is None:
        return errs
    errs += check_sum(m, cui_path)
    by = {e["kind"]: e for e in m["kinds"]}
    if all_kinds:
        sel = list(by)
        listed = {e["file"] for e in m["kinds"]}
        for f in sorted(os.listdir(corpus)) if os.path.isdir(corpus) else []:
            if f.endswith(".json") and f not in NON_KIND_FILES and f not in listed:
                errs.append(f"EXTRA {f}: not named by the manifest")
    else:
        sel = kinds
        for k in sel:
            if k not in by:
                errs.append(f"MANIFEST {k}: not a manifest kind")
    for k in sel:
        if k in by:
            errs += check_kind(by[k], corpus, all_kinds)
    return errs


def selftest():
    """Plant each failure in a scratch corpus and require exit 1 with the right code."""
    me = os.path.abspath(__file__)
    tmp = tempfile.mkdtemp(prefix="cprman-")
    fails = []
    try:
        man = {"kinds": [
            {"kind": "alpha", "file": "alpha.json", "expected": 3, "figure": "F-1",
             "print_only": False, "nested": None, "lane": "E1.x"},
            {"kind": "beta", "file": "beta.json", "expected": None, "figure": None,
             "print_only": True, "nested": None, "lane": "E1.8"}]}
        cui = os.path.join(tmp, "cui.md")
        open(cui, "w").write(
            "## §3 Kinds\n\n| Kind | Expected records | Figure |\n|---|---|---|\n"
            "| `alpha` | 3 | F-1 |\n| `beta` | set by E1.8 | - |\n\n## §4 next\n")
        mp = os.path.join(tmp, "MANIFEST.json")
        json.dump(man, open(mp, "w"))
        cd = os.path.join(tmp, "corpus")

        def mk(recs_alpha, extra=None, beta=None):
            shutil.rmtree(cd, ignore_errors=True)
            os.makedirs(cd)
            if recs_alpha is not None:
                json.dump(recs_alpha, open(os.path.join(cd, "alpha.json"), "w"))
            if beta is not None:
                json.dump(beta, open(os.path.join(cd, "beta.json"), "w"))
            if extra:
                json.dump([], open(os.path.join(cd, extra), "w"))

        def go(args):
            p = subprocess.run(
                [sys.executable, me, "--manifest", mp, "--corpus", cd, "--cui", cui] + args,
                capture_output=True, text=True)
            return p.returncode, p.stdout

        good = [{"source_page": f"p{n:03d}"} for n in (1, 2, 459)]

        def expect(name, args, rc, code):
            got, out = go(args)
            ok = got == rc and (code is None or code in out)
            if not ok:
                fails.append(f"{name}: rc {got} (want {rc}), want {code!r}, out {out!r}")

        mk(good)
        expect("kinds ok", ["--kinds", "alpha"], 0, None)
        expect("all with null count", ["--all"], 1, "NULL_COUNT beta")
        expect("all missing beta", ["--all"], 1, "MISSING beta")
        mk(good[:2])
        expect("short count", ["--kinds", "alpha"], 1, "COUNT alpha")
        mk(good[:2] + [{"source_page": "p460"}])
        expect("p460", ["--kinds", "alpha"], 1, "BAD_SOURCE alpha")
        mk(good[:2] + [{"name": "x"}])
        expect("no source_page", ["--kinds", "alpha"], 1, "BAD_SOURCE alpha")
        mk(None)
        expect("missing file", ["--kinds", "alpha"], 1, "MISSING alpha")
        mk(good, extra="gamma.json")
        expect("extra file", ["--all"], 1, "EXTRA gamma.json")
        mk(good)
        man["kinds"][0]["expected"] = 4
        json.dump(man, open(mp, "w"))
        expect("sum mismatch", ["--kinds", "alpha"], 1, "SUM alpha")
        man["kinds"][0]["expected"] = 3
        man["kinds"].append({"kind": "delta", "file": "delta.json", "expected": 1,
                             "figure": None, "print_only": False, "nested": None, "lane": "x"})
        json.dump(man, open(mp, "w"))
        expect("extra manifest kind", ["--kinds", "alpha"], 1, "SUM delta")
    finally:
        shutil.rmtree(tmp, ignore_errors=True)
    for f in fails:
        print("SELFTEST FAIL " + f)
    print("selftest: %s" % ("FAILED" if fails else "ok (10 plants)"))
    return 1 if fails else 0


def main(argv):
    manifest_path, corpus, cui_path = os.path.join(CORPUS, "MANIFEST.json"), CORPUS, CUI
    kinds, all_kinds, i = [], False, 0
    while i < len(argv):
        a = argv[i]
        if a == "--selftest":
            return selftest()
        if a == "--all":
            all_kinds = True
        elif a in ("--kinds", "--manifest", "--corpus", "--cui") and i + 1 < len(argv):
            i += 1
            if a == "--kinds":
                kinds = [k for k in argv[i].split(",") if k]
            elif a == "--manifest":
                manifest_path = argv[i]
            elif a == "--corpus":
                corpus = argv[i]
            else:
                cui_path = argv[i]
        else:
            print(__doc__)
            return 2
        i += 1
    if not all_kinds and not kinds:
        print(__doc__)
        return 2
    errs = run(manifest_path, corpus, cui_path, kinds, all_kinds)
    for e in errs:
        print(e)
    if not errs:
        print("OK cpr_manifest_check: %s" % ("--all" if all_kinds else ",".join(kinds)))
    return 1 if errs else 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
