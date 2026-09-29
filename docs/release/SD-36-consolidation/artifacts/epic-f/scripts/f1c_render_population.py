#!/usr/bin/env python3
"""SD-36 Epic F1c render receipt: the build population, rendered by one class_census binary.

Population (stated denominator): every non-prestige class in census-f1c.json (61) at levels
1, 5, 10 and 20, each capped at the class's own max_level and de-duplicated, plus the 15
builds of the F1c merge-readiness check (prestige and multiclass mixes included).

  f1c_render_population.py builds                 -> prints the build list, one per line
  f1c_render_population.py render <bin> <outdir>  -> <outdir>/<build>.txt per build, 4 at a time;
                                                     a build the binary refuses is written with
                                                     its stderr and 'EXIT=<n>' so it still counts
"""
import json, os, subprocess, sys
from concurrent.futures import ThreadPoolExecutor

HERE = os.path.dirname(os.path.abspath(__file__))
CENSUS = os.path.join(HERE, "..", "census-f1c.json")
CHECK_BUILDS = [
    "antipaladin:1", "magus:5", "commoner:1", "fighter:1", "monk:5", "psion:3", "ninja:5",
    "unchained_monk:5", "occultist:1", "vigilante:1", "summoner:1", "fighter:6+duelist:1",
    "fighter:6+golden_legionnaire:1", "fighter:6+wizard:4", "cleric:5+rogue:3",
]


def builds():
    out = []
    for c in json.load(open(CENSUS))["classes"]:
        cid = c["class_id"].split(":", 1)[1]
        for lvl in sorted({min(l, c["max_level"]) for l in (1, 5, 10, 20)}):
            out.append(f"{cid}:{lvl}")
    for b in CHECK_BUILDS:
        if b not in out:
            out.append(b)
    return out


def render(binary, outdir, build):
    p = subprocess.run([binary, "--sheet-dump", build, "--with-sheet-rules"], capture_output=True, text=True)
    body = p.stdout if p.returncode == 0 else p.stdout + p.stderr + f"\nEXIT={p.returncode}\n"
    with open(os.path.join(outdir, build.replace(":", "_").replace("+", "__") + ".txt"), "w") as f:
        f.write(body)
    return build, p.returncode


if __name__ == "__main__":
    if sys.argv[1] == "builds":
        print("\n".join(builds()))
    elif sys.argv[1] == "render":
        binary, outdir = sys.argv[2], sys.argv[3]
        os.makedirs(outdir, exist_ok=True)
        with ThreadPoolExecutor(4) as pool:
            for b, rc in pool.map(lambda b: render(binary, outdir, b), builds()):
                print(f"{b} exit={rc}", flush=True)
