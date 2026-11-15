"""Every desktop `cargo test --locked -j 8 [--bins | --bin codex-desktop] <filter> -- --test-threads=8` → N passed
acceptance claim in epic-breakdown.md's E2 rows, checked against one full desktop log
(`cd apps/desktop/src-tauri && cargo test --locked -j 8 --no-fail-fast -- --test-threads=8`). The desktop twin of
E1.MC's acceptance_filter_counts.py: R-W control for the recurring "package said N passed" correction (E2: 17 of
38 corrections, this card's 7 included, match `passed|filter|R21|acceptance|matches 0` over the sd41-e2* event shards). Usage, repo root:
python3 -I docs/release/SD-41-gear-and-equipment-sets/artifacts/epic_2/E2.MC/desktop_filter_counts.py <desktop-log>
A filter matches a test when it is a substring of the test path (cargo's rule); `--bins`/`--bin` count the bin
suite only, an unqualified filter counts every libtest suite. `→ ≥ N` claims pass at N or more.
Exit 1 on any mismatch or on a filter that matches 0 tests."""
import re, sys

suites, current = {}, None
for line in open(sys.argv[1]):
    m = re.match(r"\s+Running (.+?) \(", line)
    if m:
        current = m.group(1)
        suites.setdefault(current, [])
        continue
    m = re.match(r"test (\S+) \.\.\. (\w+)", line)
    if m and current:
        suites[current].append((m.group(1), m.group(2)))
bin_suite = [s for s in suites if s.startswith("unittests src/main.rs")]
if not bin_suite:
    sys.exit("NO_BIN_SUITE in the log")
rows = [r for r in open("docs/release/SD-41-gear-and-equipment-sets/epic-breakdown.md") if r.startswith("| E2.")]
claims = set()
for row in rows:
    for scope, flt, ge, n in re.findall(r"cargo test --locked -j 8 (--bins |--bin codex-desktop )?(?!--)(\S+) -- --test-threads=8` → (≥ )?(\d+) passed", row):
        claims.add((row.split("|")[1].strip(), scope.strip(), flt, bool(ge), int(n)))
bad = 0
for card, scope, flt, ge, n in sorted(claims):
    pool = [t for s in (bin_suite if scope else suites) for t in suites[s]]
    hits = [(t, r) for t, r in pool if flt in t]
    ok = sum(1 for _, r in hits if r == "ok")
    good = ok == len(hits) and ok > 0 and (ok >= n if ge else ok == n)
    bad += not good
    print(f"{'OK' if good else 'MISMATCH':8} {card} {scope or 'all'} {flt}: claimed {'≥ ' if ge else ''}{n}, matched {len(hits)}, ok {ok}")
print(f"{len(claims)} claims, {bad} mismatches")
sys.exit(1 if bad else 0)
