#!/usr/bin/env python3
"""E3.MC: plant M1..M4 (decisions.md §8) on the mapping table THE CONVERTER reads, and require
`sheet_rule_convert --system starfinder-1e --check` to go red for each (the committed package was
generated from the unmutated table, so a load-bearing table must make it stale). E3.3's
sf_mapping_mutations.py proves the seed fixtures go red; E3.4's P1 proves M1 on the converter.
This proves M2..M4 on the converter as well. Restores the table byte for byte (sha256).
Run from the repo root. Exit 0 only when M1..M4 -> FAIL each and restored -> PASS."""
import hashlib, importlib.util, json, subprocess, sys
from pathlib import Path
HERE = Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location("m", HERE / "token-mapping" / "sf_mapping_mutations.py")
m = importlib.util.module_from_spec(spec); spec.loader.exec_module(m)
TABLE = HERE / "token-mapping" / "sf-mapping-table.v1.json"
CMD = ["cargo", "run", "--locked", "-j", "8", "-q", "-p", "codex-ingest", "--bin", "sheet_rule_convert",
       "--", "--system", "starfinder-1e", "--check"]
orig = TABLE.read_bytes(); h0 = hashlib.sha256(orig).hexdigest()
def check():
    p = subprocess.run(CMD, capture_output=True, text=True)
    out = p.stdout + p.stderr
    return p.returncode, [l for l in out.splitlines() if "stale" in l or "verdict=" in l][:3]
ok = True
try:
    for mid, what, fn in m.MUTATIONS:
        t = json.loads(orig); fn(t); TABLE.write_text(json.dumps(t, indent=2) + "\n")
        rc, lines = check()
        print(f"{mid} ({what}) -> {'FAIL' if rc else 'PASS'} (exit {rc})")
        for l in lines: print("    " + l)
        ok &= rc != 0
        TABLE.write_bytes(orig)
finally:
    TABLE.write_bytes(orig)
h1 = hashlib.sha256(TABLE.read_bytes()).hexdigest()
rc, lines = check()
print(f"restored (table sha256 {h1[:16]} {'==' if h1 == h0 else '!='} {h0[:16]}) -> {'PASS' if rc == 0 else 'FAIL'} (exit {rc})")
ok &= rc == 0 and h1 == h0
print("verdict=" + ("PASS" if ok else "FAIL")); sys.exit(0 if ok else 1)
