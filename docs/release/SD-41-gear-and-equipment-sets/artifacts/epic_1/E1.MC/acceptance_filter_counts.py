"""Every root `--lib <filter> … → N passed` acceptance in epic-breakdown.md, checked against one full
`cargo test --locked -j 8 --lib -- --test-threads=8` log (R-W control for the recurring "package said N
passed" correction: 7 package + 6 lead corrections in E1, `scripts/retro.py summary`). Usage, repo root:
python3 -I docs/release/SD-41-gear-and-equipment-sets/artifacts/epic_1/E1.MC/acceptance_filter_counts.py <lib-log>
A filter matches a test when its text is a substring of the test path (cargo's rule). Exit 1 on any mismatch
or on a filter that matches 0 tests."""
import re, sys

log = open(sys.argv[1]).read()
tests = re.findall(r"^test (\S+) \.\.\. (\w+)", log, re.M)
rows = open("docs/release/SD-41-gear-and-equipment-sets/epic-breakdown.md").read()
claims = re.findall(r"cargo test --locked -j 8 --lib (\S+) -- --test-threads=8` → (\d+) passed", rows)
bad = 0
for flt, n in sorted(set(claims)):
    hits = [(t, r) for t, r in tests if flt in t]
    ok = sum(1 for _, r in hits if r == "ok")
    flag = "OK" if ok == int(n) == len(hits) and ok > 0 else "MISMATCH"
    bad += flag != "OK"
    print(f"{flag:8} --lib {flt}: claimed {n}, matched {len(hits)}, ok {ok}")
print(f"{len(set(claims))} claims, {bad} mismatches")
sys.exit(1 if bad else 0)
