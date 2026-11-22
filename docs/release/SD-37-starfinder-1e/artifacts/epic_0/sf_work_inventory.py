#!/usr/bin/env python3
"""SD-37 E0.3 implementation A: derive the Starfinder 1e work inventory from the pinned oracle.

One unit per sheet-reachable record in the 8 in-scope books. Every F-6 data row (CUI F-6) lands in
exactly one disposition, so units + named non-unit buckets == rows, per book. Excluded books/dirs
(SSRGG, LPJ, core `_society/`) are reported by name with their row counts.
Exit: 0 written; 1 unclassifiable file or oracle off pin (fail closed); 2 usage/IO.
Usage: sf_work_inventory.py --out docs/work-inventory.starfinder-1e.json
"""
import argparse, json, os, re, subprocess, sys

PC_ROOT = os.environ.get('PCGEN_REPO_DIR') or os.path.join(os.path.expanduser('~'), 'workspace/repos/pcgen')
CORPUS = os.environ.get('PCGEN_CORPUS_ROOT') or os.path.join(PC_ROOT, 'data')
SF = os.path.join(CORPUS, 'starfinder')

IN_SCOPE = ['paizo/core', 'paizo/armory', 'paizo/character_operations_manual', 'paizo/pact_worlds',
            'paizo/near_space', 'paizo/alien_archive', 'paizo/alien_archive_2', 'paizo/alien_archive_3']
EXCLUDED = [  # (name, dir, subdir filter)
    ('core _society (SFS guide core mods)', 'paizo/core', '_society'),
    ('starfinder_society_rules (SSRGG)', 'paizo/starfinder_society_rules', None),
    ('lpj_design/infinite_space (LPJ9304)', 'lpj_design/infinite_space', None),
]
# file suffix (after the book prefix) -> (kind or None, class). None = engine config, never a unit.
KIND = {
    'abilities': 'ability', 'abilities_class': 'ability', 'abilities_race': 'ability',
    'classes': 'class', 'races': 'race', 'feats': 'feat', 'feats_spw': 'feat', 'skills': 'skill',
    'spells': 'spell', 'spells_aa': 'spell', 'equip': 'equipment', 'equip_gear': 'equipment',
    'equip_magic': 'equipment', 'equipmods': 'equipment_modifier', 'templates': 'template',
    'deities': 'deity', 'languages': 'language', 'profs_weapon': 'proficiency',
    'profs_armor': 'proficiency', 'profs_shield': 'proficiency',
}
CONFIG = {'_align', '_datacontrols', '_datatables', '_dynamic', '_globalmodifiers', '_saves', '_sizes',
          '_stats', '_variables', 'abilitycategories', 'biosettings', 'kits', 'companionmods'}
SUFFIXES = sorted(set(KIND) | CONFIG, key=len, reverse=True)
DISPOSITIONS = ['unit_declared', 'unit_copy', 'unit_mod_only_rescue', 'mod_record', 'forget_directive',
                'class_level_line', 'class_continuation_row', 'duplicate_key_row', 'internal_namespace',
                'directive_line', 'engine_config_file']


def suffix_of(fname):
    stem = fname[:-4]
    for s in SUFFIXES:  # longest first; the book prefix is everything before `_<suffix>`
        if stem.endswith('_' + s):
            return s
    return None


def lst_files(book, skip=None, only=None):
    base = os.path.join(SF, book)
    out = []
    for r, ds, fs in os.walk(base):
        rel = os.path.relpath(r, base)
        top = rel.split(os.sep)[0]
        if skip and top == skip:
            continue
        if only and top != only:
            continue
        for f in fs:
            if f.endswith('.lst'):
                out.append(os.path.join(r, f))
    return sorted(out)


def rows(path):
    data = open(path, 'rb').read().decode('latin-1')
    for i, ln in enumerate(data.split('\n'), 1):
        if not ln or ln[0] in '# \t\r\n':
            continue
        if re.match(r'(SOURCELONG|SOURCESHORT|SOURCEWEB|SOURCEDATE)', ln):
            continue
        yield i, ln


def base_of(f1):
    f1 = re.sub(r'^CATEGORY=[^|]*\|', '', f1)
    f1 = re.sub(r'^CLASS:', '', f1)
    return f1[:-4] if f1.endswith('.MOD') else f1


def main():
    ap = argparse.ArgumentParser(); ap.add_argument('--out', required=True)
    a = ap.parse_args()
    pin = dict(l.strip().split('=', 1) for l in open('scripts/pcgen-oracle-pin.env')
               if re.match(r'^PCGEN_ORACLE_SHA=', l))
    head = subprocess.run(['git', '-C', PC_ROOT, 'rev-parse', 'HEAD'], capture_output=True, text=True).stdout.strip()
    sha = pin['PCGEN_ORACLE_SHA'].split('#')[0].strip()
    if head != sha:
        print(f'OFF_PIN oracle HEAD {head} != pin {sha}', file=sys.stderr); return 1

    # pass 1: declared base names per (kind) over the in-scope books, for .MOD rescue.
    declared = {}
    for b in IN_SCOPE:
        for p in lst_files(b, skip='_society' if b == 'paizo/core' else None):
            s = suffix_of(os.path.basename(p))
            if s in KIND:
                for _, ln in rows(p):
                    f1 = ln.split('\t')[0]
                    if not f1.endswith('.MOD') and not f1.endswith('.FORGET') and '.COPY=' not in f1 \
                       and not f1.startswith('CATEGORY=Internal') and not (KIND[s] == 'class' and not f1.startswith('CLASS:')):
                        declared.setdefault(KIND[s], set()).add(base_of(f1))
    units, books, unknown = [], [], []
    seen = {}
    for b in IN_SCOPE:
        disp = {d: 0 for d in DISPOSITIONS}; nrows = 0; nfiles = 0
        by_file_config = {}
        for p in lst_files(b, skip='_society' if b == 'paizo/core' else None):
            fn = os.path.basename(p); s = suffix_of(fn); nfiles += 1
            if s is None:
                unknown.append(p); continue
            rel = os.path.relpath(p, os.path.join(SF, b))
            for ln_no, ln in rows(p):
                nrows += 1
                f1 = ln.split('\t')[0]
                if s in CONFIG:
                    disp['engine_config_file'] += 1; by_file_config[fn] = by_file_config.get(fn, 0) + 1; continue
                kind = KIND[s]
                if f1.startswith('CATEGORY=Internal') or 'CATEGORY:Internal' in ln.split('\t')[1:]:
                    disp['internal_namespace'] += 1; continue
                if kind == 'class' and not f1.startswith('CLASS:'):
                    disp['class_level_line'] += 1; continue
                if f1.endswith('.FORGET'):
                    disp['forget_directive'] += 1; continue
                origin = 'declared'
                origin_disp = {'declared': 'unit_declared', 'copy': 'unit_copy', 'mod_only': 'unit_mod_only_rescue'}
                if f1.endswith('.MOD'):
                    if base_of(f1) in declared.get(kind, ()):
                        disp['mod_record'] += 1; continue
                    origin = 'mod_only'; name = base_of(f1); disp['unit_mod_only_rescue'] += 1
                elif '.COPY=' in f1:
                    origin = 'copy'; name = re.sub(r'^CATEGORY=[^|]*\|', '', f1).split('.COPY=', 1)[1]; disp['unit_copy'] += 1
                elif re.match(r'^[A-Z][A-Z0-9_ ]*:', f1) and not f1.startswith('CLASS:'):
                    disp['directive_line'] += 1; continue
                else:
                    name = base_of(f1); disp['unit_declared'] += 1
                fields = ln.split('\t')[1:]
                key = next((x[4:] for x in fields if x.startswith('KEY:')), name)
                slug = re.sub(r'[^a-z0-9]+', '_', key.lower()).strip('_') or 'unnamed'
                uid = f"{b.split('/')[-1]}:{kind}:{slug}"
                if uid in seen:  # same (book, kind, KEY) already declared: a continuation/duplicate row
                    first = [k for k in ('unit_declared', 'unit_copy', 'unit_mod_only_rescue') if origin_disp[origin] == k][0]
                    disp[first] -= 1
                    disp['class_continuation_row' if kind == 'class' else 'duplicate_key_row'] += 1
                    continue
                seen[uid] = 1
                units.append({'id': uid, 'book': b, 'kind': kind, 'name': name, 'origin': origin,
                              'visible': 'VISIBLE:NO' not in ln.split('\t')[1:],
                              'source_file': rel, 'source_line': ln_no})
        books.append({'id': b, 'files': nfiles, 'rows': nrows, 'dispositions': disp,
                      'units': sum(v for k, v in disp.items() if k.startswith('unit_')),
                      'engine_config_rows_by_file': by_file_config})
    if unknown:
        print('UNCLASSIFIED_FILES ' + ' '.join(unknown), file=sys.stderr); return 1
    excluded = []
    for name, d, sub in EXCLUDED:
        n = sum(1 for p in lst_files(d, only=sub) for _ in rows(p))
        excluded.append({'name': name, 'dir': d + ('/' + sub if sub else ''), 'rows': n,
                         'reason': 'decisions.md section 6 (SD-a); see license-matrix.md'})
    by_kind = {}
    for u in units:
        by_kind[u['kind']] = by_kind.get(u['kind'], 0) + 1
    inv = {'schema_version': 1, 'generated_by': 'artifacts/epic_0/sf_work_inventory.py',
           'oracle_sha': sha,
           'contract': ('One unit per sheet-reachable record in the in-scope books. Every F-6 row of an in-scope '
                        'book is in exactly one disposition; units + non-unit buckets = rows (checked by '
                        'sf_work_inventory_check.py, which exits 1 on any mismatch). Deterministic: no timestamp.'),
           'row_predicate': ('F-6: first byte not #, space, tab, CR, LF and line does not start '
                             'SOURCELONG/SOURCESHORT/SOURCEWEB/SOURCEDATE'),
           'totals': {'units': len(units), 'rows_in_scope': sum(b['rows'] for b in books),
                      'rows_excluded': sum(e['rows'] for e in excluded), 'books': len(books), 'by_kind': by_kind},
           'books': books, 'excluded': excluded, 'units': units}
    with open(a.out, 'w') as fh:
        json.dump(inv, fh, indent=1, sort_keys=True); fh.write('\n')
    print(f"units {len(units)} rows_in_scope {inv['totals']['rows_in_scope']} excluded {inv['totals']['rows_excluded']}")
    return 0


if __name__ == '__main__':
    sys.exit(main())
