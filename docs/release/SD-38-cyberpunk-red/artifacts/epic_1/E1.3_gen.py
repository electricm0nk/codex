#!/usr/bin/env python3
"""E1.3 generator: writes data/cyberpunk-red/corpus/{skill,difficulty_value,skill_modifier}.json.
Usage: python3 -I E1.3_gen.py <pdf> <pages-dir> <skills.tsv> <corpus-dir>
Skill category/STAT/order come from skills.tsv; the description text comes from E1.3_extract.py;
the short-list page is found by a per-name dot-leader search of p083-p086."""
import json, os, re, subprocess, sys
PDF, P, TSV, OUT = sys.argv[1:5]
HERE = os.path.dirname(os.path.abspath(__file__))
ext = json.loads(subprocess.run([sys.executable, "-I", os.path.join(HERE, "E1.3_extract.py"), PDF],
                                capture_output=True, text=True, check=True).stdout)
CATDROP = re.compile(r"\s+(?:[A-Z] [a-z]+\s)+S kills$")
SPECIALTY = {
    "Language": "a specific language",
    "Local Expert": "a location (single neighborhood or community)",
    "Science": "an area of study",
    "Martial Arts": "a Martial Arts Form",
    "Play Instrument": "an instrument (can choose which instrument)",
}
OPTIONS = {
    "Martial Arts": ["Karate", "Taekwondo", "Judo", "Aikido"],
    "Science": ["Geology", "Mathematics", "Physics", "Zoology", "Anthropology", "Biology", "Chemistry", "History"],
    "Play Instrument": ["singing", "guitar", "drums", "violin", "piano"],
}
short = {}
for pg in (83, 84, 85, 86):
    t = re.sub(r"[ \t]+", " ", open(f"{P}/p{pg:03d}.txt").read())
    short[pg] = t
def short_page(name, stat):
    pat = re.compile(re.escape(name) + r"( \(x2\))? *\.{3,} *" + stat)
    hits = [pg for pg, t in short.items() if len(pat.findall(t)) == 1]
    return "p%03d" % hits[0] if len(hits) == 1 else None

skills = []
for ln in open(TSV).read().splitlines():
    cat, name, stat = ln.split("|")
    e = ext[name]
    text = e["text"]
    text = CATDROP.sub("", text)
    text = re.sub(r"\s+BY ADRIAN MARC$", "", text)
    if name == "Weaponstech":
        text = text[: text.index("Hot Zone.") + len("Hot Zone.")]
    assert e["stat"] == stat, (name, e["stat"], stat)
    rec = {"kind": "skill", "name": name, "category": cat, "stat": stat, "x2": e["x2"],
           "specialty_required": name in SPECIALTY, "short_list_page": short_page(name, stat),
           "source_page": "p%03d" % e["page"], "text": text}
    if name in SPECIALTY:
        rec["specialty"] = SPECIALTY[name]
    if name in OPTIONS:
        rec["specialty_examples"] = OPTIONS[name]
    skills.append(rec)

DV = [
 ("Simple", 9, "This is something most people can do without thinking, but which might be hard for a small child."),
 ("Everyday", 13, "This feat is something most people can do without a lot of special training."),
 ("Difficult", 15, "This feat is difficult to accomplish without training or natural talent."),
 ("Professional", 17, "This feat takes actual training and the user can be considered to be a professional, skilled in their abilities."),
 ("Heroic", 21, "This is a highly skilled feat; one that only the best of the best can pull off. This is the level of sports stars and other highly regarded superstars."),
 ("Incredible", 24, "This is a tremendous feat. Pulling this off would rate you among the very best of your class professionally. You are of truly Olympian mettle."),
 ("Legendary", 29, "An awe-inspiring feat. This is something people write stories about; a truly amazing accomplishment that will be spoken of in hushed tones for years to come."),
]
MOD = [("Night or low lighting conditions", -1), ("Have never done this before", -1), ("Complex task", -2),
       ("Don't have right tools or parts", -2), ("Slept uncomfortable the night before.", -2),
       ("Under extreme stress", -2), ("Exhausted", -4), ("Extremely drunk or sedated", -4),
       ("Trying to perform task secretly", -4), ("Task obscured by smoke, darkness", -4)]
dvs = [{"kind": "difficulty_value", "name": n, "dv": v, "source_page": "p130", "text": t} for n, v, t in DV]
mods = [{"kind": "skill_modifier", "name": n, "modifier": v, "source_page": "p131", "text": n} for n, v in MOD]
for fn, recs in (("skill", skills), ("difficulty_value", dvs), ("skill_modifier", mods)):
    with open(os.path.join(OUT, fn + ".json"), "w") as f:
        json.dump(recs, f, indent=1, ensure_ascii=False)
        f.write("\n")
print({k: len(v) for k, v in (("skill", skills), ("difficulty_value", dvs), ("skill_modifier", mods))})
