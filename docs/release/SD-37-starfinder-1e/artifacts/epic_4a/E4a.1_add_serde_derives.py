#!/usr/bin/env python3
"""SD-37 E4a.1: one-shot edit that makes every rules_tables row type serde-capable.

For every `struct`/`enum` defined at file level under src/rules_core/rules_tables/ (except the
registry/report types in SKIP and the json_cache types that already derive serde):
  * add `serde::Serialize, serde::Deserialize` to its `#[derive(..)]` (or insert one),
  * add `#[cfg_attr(test, derive(schemars::JsonSchema))]` (the E2.2 schema generator is test-only),
  * give a type whose NAME is defined in more than one module a unique schema name,
  * put `deserialize_with = leak_slice/leak_opt_slice` on every `&'static [..]` field, because
    serde has no `Deserialize` for a borrowed slice (rules_data_package.rs leaks a Vec once).
Idempotent: a type that already names serde::Serialize is left alone.
Run from the repo root: python3 <this file>
"""
import os, re, collections

ROOT = 'src/rules_core/rules_tables'
SKIP = {'BookFeatTable', 'CompanionBook', 'MonsterBook', 'SimpleKindTable', 'EquipmentFieldCoverage',
        'SpellFieldCoverage', 'AcgClassCoverage', 'ApgClassCoverage'}
LEAK = 'crate::rules_core::rules_data_package::leak_slice'
LEAK_OPT = 'crate::rules_core::rules_data_package::leak_opt_slice'
DEF = re.compile(r'^(pub(?:\([a-z]+\))?\s+)?(struct|enum)\s+(\w+)\b')

files = []
for d, _, fs in os.walk(ROOT):
    for f in fs:
        if f.endswith('.rs') and f != 'json_cache.rs':
            files.append(os.path.join(d, f))
files.sort()

names = collections.Counter()
for p in files:
    for line in open(p).read().split('\n'):
        m = DEF.match(line)
        if m:
            names[m.group(3)] += 1

def modpath(p):
    rel = p[len(ROOT) + 1:-3]
    if rel.endswith('/mod'):
        rel = rel[:-4]
    return rel.replace('/', '__') if rel != 'mod' else 'rules_tables'

edited = collections.Counter()
for p in files:
    lines = open(p).read().split('\n')
    out = []
    i = 0
    while i < len(lines):
        line = lines[i]
        # DEF only matches column 0, so a type inside an indented `mod tests` never matches.
        m = DEF.match(line)
        if m and m.group(3) not in SKIP and line.rstrip().endswith('{'):
            name = m.group(3)
            # find the attribute block directly above (skip doc comments / attrs)
            j = len(out) - 1
            derive_idx = None
            while j >= 0 and (out[j].startswith('#[') or out[j].startswith('///')):
                if out[j].startswith('#[derive('):
                    derive_idx = j
                j -= 1
            already = derive_idx is not None and 'serde::Serialize' in out[derive_idx]
            if not already:
                extra = ['#[cfg_attr(test, derive(schemars::JsonSchema))]']
                if names[name] > 1:
                    extra.append(f'#[cfg_attr(test, schemars(rename = "{modpath(p)}__{name}"))]')
                if derive_idx is not None:
                    out[derive_idx] = out[derive_idx].replace(')]', ', serde::Serialize, serde::Deserialize)]', 1)
                    for k, e in enumerate(extra):
                        out.insert(derive_idx + 1 + k, e)
                else:
                    out.append('#[derive(serde::Serialize, serde::Deserialize)]')
                    out.extend(extra)
                edited[p] += 1
                # body: annotate slice fields until the closing brace at column 0
                out.append(line)
                i += 1
                while i < len(lines) and lines[i] != '}':
                    fl = lines[i]
                    fm = re.match(r'^(\s*)(pub(?:\([a-z]+\))?\s+)?\w+\s*:\s*(.+?),?\s*(//.*)?$', fl)
                    if fm and not fl.strip().startswith('//'):
                        ty = fm.group(3).strip()
                        if ty.startswith("&'static [") or ty.startswith('&['):
                            out.append(f'{fm.group(1)}#[serde(deserialize_with = "{LEAK}")]')
                        elif ty.startswith("Option<&'static [") or ty.startswith('Option<&['):
                            out.append(f'{fm.group(1)}#[serde(deserialize_with = "{LEAK_OPT}")]')
                    out.append(fl)
                    i += 1
                continue
        out.append(line)
        i += 1
    new = '\n'.join(out)
    if new != '\n'.join(lines):
        open(p, 'w').write(new)

print('types edited', sum(edited.values()), 'files', len(edited))
