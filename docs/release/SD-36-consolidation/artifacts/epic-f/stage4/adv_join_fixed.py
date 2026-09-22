import json, glob, os, sys, collections, random

# Independent Python re-implementation of the STAGE-4-FIXED
# src/rules_core/sheet_line_join.rs::rule_for_explanation, mirroring every change made to close
# review findings 1 and 2 (see stage4-blockers.json). Used to cross-validate the Rust join over
# the real population from a second, independently-written implementation, and to directly
# re-run the "scoped match on <=1 common word with both sides diverging" audit finding 1's own
# evidence used, over the CORRECTED algorithm's output.

ROOT = "/tmp/claude-1000/-home-ubuntu-workspace-repos-codex/6badc5b8-ae3b-4359-80c5-cd0b1598973e/scratchpad/sd36/f1/dump-after"

def load_slugs(root):
    by_slug = collections.defaultdict(list)
    printflag = {}
    for p in glob.glob(os.path.join(root, "*", "class_feature", "*.json")):
        try:
            rules = json.load(open(p))
        except Exception:
            continue
        for r in rules:
            rid = r["id"]
            parts = rid.split(":", 2)
            if len(parts) < 3 or parts[1] != "class_feature":
                continue
            if "#" in rid:
                continue
            slug = parts[2].split("#")[0]
            by_slug[slug].append(rid)
            printflag[rid] = r.get("print", True)
    return by_slug, printflag

BY_SLUG, PRINT = load_slugs(ROOT)
SLUGS = sorted(BY_SLUG)

def find(slug):
    ids = BY_SLUG.get(slug)
    if not ids: return None
    for i in sorted(ids):
        if i.startswith("core_rulebook:"):
            return i
    return min(ids)

def words(s): return [w for w in s.split("_") if w]

WORDS = {s: words(s) for s in SLUGS}

TIER_SCOPED, TIER_BARE = 0, 1

def join(class_slug, explanation_id):
    rest = explanation_id[len("class_feature."):] if explanation_id.startswith("class_feature.") else explanation_id
    segs = [s for s in rest.split(".") if s != "corpus_record"]
    if class_slug not in segs: return ("none", [])
    ci = segs.index(class_slug)
    rest_segs = segs[ci+1:]
    if not rest_segs: return ("none", [])
    class_words = words(class_slug)
    scope_prefix = class_slug + "_"
    min_scoped_len = len(class_words) + 1
    best = []
    def consider(cov, exc, tier, slug):
        nonlocal best
        if best:
            c, e, t, _ = best[0]
            key_new = (cov, -exc, -tier); key_old = (c, -e, -t)
            if key_new > key_old: best = [(cov, exc, tier, slug)]
            elif key_new == key_old: best.append((cov, exc, tier, slug))
        else:
            best = [(cov, exc, tier, slug)]
    for i in range(len(rest_segs)):
        tail_joined = "_".join(rest_segs[i:])
        tw = words(tail_joined)
        if not tw: continue
        otl = len(tw)
        raw_query = class_words + tw
        # RAW scoped tier -- FIX: refuse a partial match (coverage < tail length) that also
        # leaves the candidate with its own unexplained leftover words.
        for slug in SLUGS:
            if slug == class_slug or not slug.startswith(scope_prefix): continue
            cw = WORDS[slug]
            lcp = 0
            for a, b in zip(raw_query, cw):
                if a != b: break
                lcp += 1
            if lcp < min_scoped_len: continue
            cov = lcp - len(class_words)
            exc = len(cw) - lcp
            if cov < otl and exc > 0: continue
            consider(cov, exc, TIER_SCOPED, slug)
        # COLLAPSED scoped tier -- FIX: never promote to the full original tail length; capped
        # at the words actually explained, same partial+leftover refusal as the raw tier.
        if len(tw) > len(class_words) and tw[:len(class_words)] == class_words:
            collapsed = tw[len(class_words):]
            cq = class_words + collapsed
            for slug in SLUGS:
                if slug == class_slug or not slug.startswith(scope_prefix): continue
                cw = WORDS[slug]
                lcp = 0
                for a, b in zip(cq, cw):
                    if a != b: break
                    lcp += 1
                if lcp < min_scoped_len: continue
                cov = lcp - len(class_words)  # FIX: was `otl if mcw == len(collapsed) else mcw`
                exc = len(cw) - lcp
                if cov < otl and exc > 0: continue
                consider(cov, exc, TIER_SCOPED, slug)
        # BARE tier -- FIX: requires an EXACT match (candidate has no leftover words either),
        # and explicitly refuses the class's own principal rule slug.
        for slug in SLUGS:
            if slug == class_slug: continue
            cw = WORDS[slug]
            lcp = 0
            for a, b in zip(tw, cw):
                if a != b: break
                lcp += 1
            if lcp != len(tw) or lcp != len(cw): continue
            consider(lcp, 0, TIER_BARE, slug)
    if not best: return ("none", [])
    ids = sorted(set(x for x in (find(c[3]) for c in best) if x))
    if not ids: return ("none", [])
    if len(ids) == 1: return ("matched", ids)
    return ("ambiguous", ids)

if __name__ == "__main__":
    data = json.load(open("/tmp/claude-1000/-home-ubuntu-workspace-repos-codex/6badc5b8-ae3b-4359-80c5-cd0b1598973e/scratchpad/sd36/f1/duplicates-after-fix.json"))
    rows = data["rows"]
    seen = {}
    bad = 0
    for r in rows:
        key = (r["class"], r["explanation_id"])
        if key in seen: continue
        kind, ids = join(*key)
        got = "none" if kind == "none" else ("matched:" + ids[0] if kind == "matched" else "ambiguous:" + "|".join(ids))
        seen[key] = got
        if got != r["after"]:
            bad += 1
            if bad <= 20:
                print("MISMATCH", key, "\n  rust:", r["after"], "\n  py  :", got)
    print("distinct facets:", len(seen), "mismatches vs rust:", bad)

    # Re-run finding 1's own audit: a "matched" outcome where the SCOPED tier's own scoring
    # (recomputed independently here) explains <=1 tail word beyond the class prefix, with the
    # candidate's own excess > 0 -- the exact coincidence shape the review flagged. Report how
    # many of the population's `after=matched:*` rows are still this shape post-fix (expect 0;
    # any survivor is refused only by having a fuller/tied better match elsewhere, never by this
    # shape alone winning outright).
    def is_coincidence_shape(cls, eid, matched_slug):
        rest = eid[len("class_feature."):] if eid.startswith("class_feature.") else eid
        segs = [s for s in rest.split(".") if s != "corpus_record"]
        if cls not in segs: return False
        ci = segs.index(cls)
        rest_segs = segs[ci+1:]
        if not rest_segs: return False
        class_words = words(cls)
        cw = WORDS.get(matched_slug)
        if cw is None: return False
        for i in range(len(rest_segs)):
            tw = words("_".join(rest_segs[i:]))
            if not tw: continue
            raw_query = class_words + tw
            lcp = 0
            for a, b in zip(raw_query, cw):
                if a != b: break
                lcp += 1
            cov = lcp - len(class_words)
            exc = len(cw) - lcp
            if cov <= 1 and exc > 0 and cw[:len(class_words)] == class_words:
                return True
        return False

    coincidence = 0
    matched_rows = [r for r in rows if r["after"].startswith("matched:")]
    for r in matched_rows:
        slug = r["after"].split(":", 1)[1].rsplit(":", 1)[-1]
        if is_coincidence_shape(r["class"], r["explanation_id"], slug):
            coincidence += 1
            print("SURVIVING COINCIDENCE:", r["class"], r["explanation_id"], "->", r["after"])
    print(f"matched rows: {len(matched_rows)}; surviving <=1-word-coincidence shape: {coincidence}")

    # Hand-audit sample: a fixed-seed random sample of the newly-matched (before=None,
    # after=matched) facets, printed with enough context (tail words vs matched slug's own
    # words) for a human/reviewer to eyeball target correctness.
    random.seed(20260921)
    newly = [r for r in rows if r["before"] is None and r["after"].startswith("matched:")]
    sample = random.sample(newly, min(40, len(newly)))
    print(f"\nHAND-AUDIT SAMPLE (seed=20260921, n={len(sample)} of {len(newly)} newly-matched facets):")
    for r in sorted(sample, key=lambda r: (r["class"], r["explanation_id"])):
        print(f"  {r['build']:28s} {r['class']:14s} {r['explanation_id']:55s} -> {r['after']}")
