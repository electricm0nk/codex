#!/usr/bin/env python3
"""E1.V sample draw: ceil(0.1 * n) records per kind (at least 1), seed recorded.

Usage (from the repo root):
  python3 -I docs/release/SD-38-cyberpunk-red/artifacts/epic_1/E1.V_sample.py           # print the sample TSV
  python3 -I docs/release/SD-38-cyberpunk-red/artifacts/epic_1/E1.V_sample.py --check   # verify the verdicts file

The draw is random.Random(SEED).sample(range(n), k) per kind, kinds taken in
MANIFEST order, one Random object per kind seeded with f"{SEED}:{kind}" so a
kind's draw does not move when another kind's count changes.

--check asserts, per kind, that E1.V_verdicts.tsv holds exactly the drawn
indices (sampled = ceil(0.1 * n)) and that no row's verdict is `disagree`.
"""
import json
import math
import os
import random
import sys

SEED = 38
HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.abspath(os.path.join(HERE, "..", "..", "..", "..", ".."))
CORPUS = os.path.join(ROOT, "data", "cyberpunk-red", "corpus")
VERDICTS = os.path.join(HERE, "E1.V_verdicts.tsv")


def draw():
    man = json.load(open(os.path.join(CORPUS, "MANIFEST.json")))
    out = []
    for e in man["kinds"]:
        recs = json.load(open(os.path.join(CORPUS, e["file"])))
        n = len(recs)
        k = max(1, math.ceil(0.1 * n))
        idx = sorted(random.Random(f"{SEED}:{e['kind']}").sample(range(n), k))
        for i in idx:
            r = recs[i]
            out.append((e["kind"], i, r.get("name", ""), r.get("source_page", ""), n, k))
    return out


def main(argv):
    rows = draw()
    if "--check" not in argv:
        print("kind\tindex\tname\tsource_page\tn\tk")
        for r in rows:
            print("\t".join(str(x) for x in r))
        return 0
    want = {}
    for kind, i, _name, _sp, n, k in rows:
        want.setdefault(kind, (n, k, set()))[2].add(i)
    got = {}
    bad = []
    with open(VERDICTS) as f:
        hdr = f.readline().rstrip("\n").split("\t")
        assert hdr[:5] == ["kind", "index", "name", "source_page", "verdict"], hdr
        for line in f:
            c = line.rstrip("\n").split("\t")
            got.setdefault(c[0], set()).add(int(c[1]))
            if c[4] == "disagree":
                bad.append(f"DISAGREE {c[0]}[{c[1]}]")
            if c[4] not in ("agree", "fixed"):
                bad.append(f"VERDICT {c[0]}[{c[1]}]: {c[4]!r}")
    for kind, (n, k, idx) in want.items():
        if got.get(kind) != idx:
            bad.append(f"SAMPLE {kind}: drawn {sorted(idx)}, verdicts {sorted(got.get(kind, []))}")
        if len(idx) != max(1, math.ceil(0.1 * n)):
            bad.append(f"SIZE {kind}: {len(idx)} != ceil(0.1*{n})")
    for kind in sorted(set(got) - set(want)):
        bad.append(f"EXTRA {kind}")
    for b in bad:
        print(b)
    tot = sum(len(v[2]) for v in want.values())
    print(f"kinds {len(want)} sampled {tot} " + ("FAIL" if bad else "OK"))
    return 1 if bad else 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
