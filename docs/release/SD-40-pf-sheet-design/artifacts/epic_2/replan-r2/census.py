"""SD-40 E2.G round-2 replan census (2026-11-12). Read-only. Run from the repo root:
    python3 -I docs/release/SD-40-pf-sheet-design/artifacts/epic_2/replan-r2/census.py
Answers decisions.md §24's five census questions (a-e) with file:line evidence (`git grep -n`)
and counts over the converted rule package (data/sheet_rules/<book>/<kind>/*.json) and the
ingested corpus (data/corpus/<book>/<kind>/**.json). Counts are Python tallies over parsed JSON;
section (b)'s headline is cross-checked by an awk count over `grep -rl` file lists."""
import collections, glob, json, re, subprocess

ENGINE = ['src', 'apps/desktop/src-tauri/src', 'apps/desktop/src']


def sh(cmd):
    return subprocess.run(cmd, shell=True, capture_output=True, text=True).stdout.rstrip()


def grep(pattern, *paths, flags='-n'):
    """`git grep` lines (path:line:text, text cut at 140 chars); none -> '(no match)'."""
    out = sh(f"git grep {flags} -e {json.dumps(pattern)} -- {' '.join(paths or ENGINE)}")
    lines = [l[:180] for l in out.splitlines()]
    return lines or ['(no match)']


def show(title, lines, limit=12):
    print(f'  {title}  [{len(lines) if lines != ["(no match)"] else 0} line(s)]')
    for l in lines[:limit]:
        print('    ' + l)


def rules():
    for path in sorted(glob.glob('data/sheet_rules/*/*/*.json')):
        try:
            rows = json.load(open(path))
        except (ValueError, UnicodeDecodeError):
            continue
        for rule in rows:
            yield path, rule


def corpus(kinds):
    for kind in kinds:
        for path in sorted(glob.glob(f'data/corpus/*/{kind}/**/*.json', recursive=True)):
            try:
                rec = json.load(open(path))
            except (ValueError, UnicodeDecodeError):
                continue
            if isinstance(rec, dict) and isinstance(rec.get('data'), dict):
                yield kind, path, rec['data']


def text(prose):
    return ''.join(p.get('Text', '') for p in prose.get('pieces', []))


print('== (a) speeds: land / swim / fly / maneuverability')
show('engine rows named speed.<mode> (producer of speed.land):', grep('id: "speed.'))
show('speed.land producer and its only input (race walk speed):', grep('fn land_speed_row', 'apps/desktop/src-tauri/src/character_hub.rs') + grep('walk_speed_ft', 'src/rules_core/race_creation.rs')[:3])
show('speed.swim / speed.fly rows anywhere in the PF engine:', grep('speed\\.swim\\|speed\\.fly', 'src', 'apps/desktop/src-tauri/src/character_hub.rs', 'apps/desktop/src-tauri/src/pf1_adapter.rs'))
show('feat-only swim/fly rows (Aquatic Ancestry, Stretched Wings):', grep('arg_standalone.aquatic_ancestry_swim_speed\\|arg_standalone.stretched_wings_fly_speed', 'src/rules_core/pilot_compute'))
show('maneuverability anywhere in the engine (words only):', grep('aneuverab', 'src/rules_core', 'apps/desktop/src-tauri/src/character_hub.rs', 'apps/desktop/src-tauri/src/pf1_adapter.rs'))
show('the print Speed block (Land only):', grep('pub fn speed', 'apps/desktop/src-tauri/src/print/blocks/core.rs', flags='-n -A3'))
move = collections.Counter(); move_ex = {}
for kind, path, d in corpus(['race', 'race_trait']):
    for tok in d.get('raw_tokens') or []:
        if tok.get('key') == 'MOVE':
            modes = re.findall(r'(Walk|Swim|Fly|Climb|Burrow)', tok.get('value', ''))
            for m in set(modes):
                move[(kind, m)] += 1
                move_ex.setdefault((kind, m), f"{path} MOVE:{tok['value']}")
print('  corpus MOVE tokens by record kind and mode (records):')
for key in sorted(move):
    print(f'    {key[0]:10s} {key[1]:6s} {move[key]:5d}  e.g. {move_ex[key]}')
speed_prose = collections.Counter()
for _, rule in rules():
    for p in rule.get('prose') or []:
        if p.get('family') == {'StatBlock': 'Speed'}:
            t = text(p)
            for m in ('Swim', 'Fly', 'Climb', 'Burrow'):
                if m + ' ' in t:
                    speed_prose[m] += 1
print(f'  sheet_rules StatBlock Speed prose naming a mode (rules): {dict(sorted(speed_prose.items()))}')

print('\n== (b) energy resistance, DR, SR, immunities')
show('engine: snapshot damage_reduction (barbarian only) and its print line:', grep('pub damage_reduction', 'src/rules_core/pilot_view_model.rs', 'apps/desktop/src-tauri/src/character_hub.rs') + grep('Damage Reduction', 'apps/desktop/src-tauri/src/print/blocks/core.rs'))
show('engine: spell resistance (worn items only):', grep('pub spell_resistance_total', 'apps/desktop/src-tauri/src/character_hub.rs'))
show('engine: energy resistance rows (feats + two class features):', grep('arg_energy_resistance.{feat_slug}\\|fn inquisitor_resistance_judgment_energy_resistance\\|fn sorcerer_draconic_dragon_resistances_energy_resistance_bonus', 'src/rules_core'))
show('engine: any resistances/immunities field on the load response:', grep('pub resistances\\|pub immunities\\|pub energy_resistance', 'apps/desktop/src-tauri/src/character_hub.rs', 'src/rules_core/pilot_view_model.rs'))
energy = re.compile(r'^(Acid|Cold|Electricity|Fire|Sonic)ResistanceBonus$|^Resistance(Acid|Cold|Electricity|Fire|Sonic)$')
tally = collections.Counter(); ex = {}
for kind, path, d in corpus(['race', 'race_trait', 'class_feature', 'ability', 'feat']):
    for chain in d.get('raw_bonus_chains') or []:
        q = chain.get('qualifiers') or []
        if len(q) > 1 and q[0] == 'VAR' and any(energy.match(v) for v in q[1].split(',')):
            tally[(kind, 'energy resistance (BONUS:VAR <Energy>ResistanceBonus)')] += 1
            ex.setdefault((kind, 'energy'), f'{path} {q}')
            break
    for tok in d.get('raw_tokens') or []:
        if tok.get('key') == 'DR':
            tally[(kind, 'damage reduction (DR: token)')] += 1; ex.setdefault((kind, 'DR'), f"{path} DR:{tok['value']}"); break
    for tok in d.get('raw_tokens') or []:
        if tok.get('key') == 'SR':
            tally[(kind, 'spell resistance (SR: token)')] += 1; ex.setdefault((kind, 'SR'), f"{path} SR:{tok['value']}"); break
    if 'Immun' in (d.get('key') or d.get('name') or ''):
        tally[(kind, 'immunity (record KEY names Immun*)')] += 1; ex.setdefault((kind, 'Immun'), f"{path} {d.get('key')}")
print('  corpus records granting each (records, by kind):')
for key in sorted(tally):
    print(f'    {key[0]:13s} {key[1]:55s} {tally[key]:5d}')
for key in sorted(ex):
    print(f'    e.g. {key}: {ex[key]}')
for race in ('ifrit', 'undine'):
    path = f'data/sheet_rules/bestiary_2/race_trait/{race}_energy_resistance.json'
    rule = json.load(open(path))[0]
    print(f"  {path}: target={rule.get('target')} value={rule.get('value')} print={rule.get('print')} prose={[text(p) for p in rule['prose']]}")
src = json.load(open(glob.glob('data/corpus/bestiary_2/race_trait/ifrit/ifrit_energy_resistance.json')[0]))['data']
print(f"  data/corpus/bestiary_2/race_trait/ifrit/ifrit_energy_resistance.json raw_bonus_chains={src.get('raw_bonus_chains')} (the number the converted rule drops)")
dr_target = sum(1 for _, r in rules() if r.get('target') == 'Dr')
print(f'  sheet_rules rules with target Dr: {dr_target}')
print('  awk cross-check (files whose JSON names FireResistanceBonus, race_trait only):',
      sh("grep -rl 'FireResistanceBonus' data/corpus/*/race_trait | awk 'END{print NR}'"))

print('\n== (c) conditional bonuses per target')
print('  re-derived round-1 census (artifacts/epic_2/replan-r1/census.py):')
for l in sh('python3 -I docs/release/SD-40-pf-sheet-design/artifacts/epic_2/replan-r1/census.py').splitlines():
    print('    ' + l)
sit = collections.Counter(); cb = collections.Counter(); cb_ex = None
for _, rule in rules():
    t = rule.get('target')
    if t is not None and '"Situational"' in json.dumps(rule.get('applies')):
        sit[t if isinstance(t, str) else next(iter(t))] += 1
    for p in rule.get('prose') or []:
        if p.get('family') == {'Aspect': 'CombatBonus'}:
            cb['all'] += 1
            if re.search(r'\bCM[BD]\b|combat maneuver', text(p), re.I):
                cb['names CMB/CMD/combat maneuver'] += 1
                cb_ex = cb_ex or f"{rule['id']}: {text(p)[:110]}"
print(f"  situational rules, more targets: Skill {sit['Skill']}, SkillGroup {sit['SkillGroup']}, SkillSituation {sit['SkillSituation']}, Attack {sit['Attack']}, Dr {sit['Dr']}, Cmb {sit['Cmb']}, Cmd {sit['Cmd']}")
print(f"  Aspect CombatBonus prose: {dict(cb)}; e.g. {cb_ex}")
show('engine ConditionalNotes lists (six; no CMB/CMD, no skills):', grep('pub [a-z_]*: Vec<ConditionalNote>', 'src/rules_core/pilot_compute/conditional_notes.rs'))
show('Aspect families gathered (CombatBonus is not one):', grep('const [A-Z_]*ASPECT', 'src/rules_core/pilot_compute/conditional_notes.rs'))
show('skills: situational feat bonuses (a separate list, feat_skill_bonuses.situational):', grep('situational', 'apps/desktop/src-tauri/src/print/blocks/skills.rs'))
show('where page 1 prints notes today:', grep('with_notes', 'apps/desktop/src-tauri/src/print/blocks/core.rs', 'apps/desktop/src-tauri/src/print/blocks/skills.rs'))
show('AC notes printed anywhere in print/ (armor_class list):', grep('notes.armor_class\\|conditional_notes.armor_class', 'apps/desktop/src-tauri/src/print'))
elf = json.load(open('data/sheet_rules/core_rulebook/race_trait/elf_elven_immunities.json'))[0]
print(f"  Elf: {elf['id']} prose families {[p['family'] for p in elf['prose']]}: {[text(p) for p in elf['prose'] if p['family'] != 'Desc']}")
show('Aspect SaveBonus with no save named feeds all three saves:', grep('if named.is_empty()', 'src/rules_core/pilot_compute/conditional_notes.rs'))

print('\n== (d) print sections today')
show('pf1e.rs SHEET list (page 1 = the Placed::at/under rows; page 2 opens at characterDetails):', grep('Placed::', 'apps/desktop/src-tauri/src/print/layout/pf1e.rs'), 30)
show('option pages and the page-2 break:', grep('request\\.spellbook\\|request\\.math\\|request\\.bio\\|PAGE_TWO_STARTS_WITH: &str', 'apps/desktop/src-tauri/src/print/layout/pf1e.rs'))
show('blocks that start a new page:', grep('on_new_page()', 'apps/desktop/src-tauri/src/print/blocks'))
show('block headings of the later sections:', grep('SheetBlock::new("', 'apps/desktop/src-tauri/src/print/blocks/feats.rs', 'apps/desktop/src-tauri/src/print/blocks/gear.rs', 'apps/desktop/src-tauri/src/print/blocks/spells.rs', 'apps/desktop/src-tauri/src/print/blocks/extras.rs', 'apps/desktop/src-tauri/src/print/blocks/bio_notes.rs', 'apps/desktop/src-tauri/src/print/blocks/math.rs'), 20)
show('progression in print/ (none) and its backend:', grep('rogression', 'apps/desktop/src-tauri/src/print/layout', 'apps/desktop/src-tauri/src/print/blocks') + grep('pub(crate) fn list_progression_ledger_at_root\\|pub struct ProgressionLedgerDto', 'apps/desktop/src-tauri/src/character_ledger.rs'))
show('section headings / titles (none today):', grep('section_title\\|SectionTitle', 'apps/desktop/src-tauri/src/print', 'apps/desktop/src-tauri/resources/print'))

print('\n== (e) Aldric and his languages')
show('the seed:', grep('"Aldric Ironhand"', 'apps/desktop/src-tauri/src/character_hub.rs') + grep('const ALDRIC_SEED_SKILL_RANKS', 'apps/desktop/src-tauri/src/character_hub.rs'))
show('known languages (one source, every PF character):', grep('LanguagesDto { known', 'apps/desktop/src-tauri/src/character_hub.rs'))
show('tests that pin languages:', grep('\\["Common"\\]\\|contains("Common")\\|"languages"\\]', 'apps/desktop/src-tauri/src', 'apps/desktop/src', 'tests'))
show('page-1 anchors naming the languages block:', grep('"languages"\\]', 'apps/desktop/src-tauri/src/print/print_golden.rs'))
show('Linguistics ranks in a seed (line 2396 is Elowen\'s list; Aldric\'s, lines 2372-2380, has none):', grep('skill:linguistics', 'apps/desktop/src-tauri/src/character_hub.rs'))
show('print entry: load + layout in one function (the seam a print-only fixture needs):', grep('pub fn sheet_document_at_root\\|let loaded = load_saved_character_via_envelope', 'apps/desktop/src-tauri/src/print/mod.rs'))
show('print_proof cases:', grep('assert_eq!(written', 'apps/desktop/src-tauri/src/print/print_proof.rs'))
gold = json.load(open('apps/desktop/src-tauri/src/print/golden/print_golden.json'))
print(f'  print_golden.json: {len(gold)} cases, keys per case {sorted(next(iter(gold.values())))} (no language text pinned)')

print('\n== (f) the desktop UI (apps/desktop/src) today: speed, AC, saves, notes, resistances')
UI = ['apps/desktop/src/characterHub/CharacterSheet.tsx', 'apps/desktop/src/characterHub/panels/CombatPanels.tsx', 'apps/desktop/src/characterHub/customModel.ts']
show('Speed: the panel shows Land only, from the engine row speed.land:', grep('export function SpeedPanel\\|label="Land"', *UI) + grep('<SpeedPanel', *UI) + grep("engineRowValue(explanations, 'speed.land')", *UI))
show('Swim / Fly anywhere in the UI (speed.swim, speed.fly, swim/fly speed):', grep('speed\\.swim\\|speed\\.fly\\|swimSpeed\\|flySpeed', 'apps/desktop/src/characterHub', 'apps/desktop/src/boundary'))
show('AC and saves panels (sheet boxes armorClass, saves) and the Defense tab saves:', grep('export function ArmorClassPanel\\|export function SavingThrowsPanel', *UI) + grep('<ArmorClassPanel\\|<SavingThrowsPanel\\|^function SavesPanel\\|<SavesPanel', *UI))
show('conditional notes read by any UI file (conditionalNotes off the load response):', grep('conditionalNotes', 'apps/desktop/src/characterHub'))
show('the TS type that carries them:', grep('conditionalNotes?: ConditionalNotesDto\\|export interface ConditionalNotesDto', 'apps/desktop/src/boundary/loadSavedCharacterDetail.ts'))
show('the only conditional list the UI shows (skills, "Not added to a total"):', grep('Not added to a total', *UI))
show('Damage Reduction and Spell Resistance in the Defense tab / equipment effects:', grep('Damage Reduction:\\|label="Spell Resistance"\\|^function DefenseTab', *UI))
show('energy resistance or immunities shown anywhere in the UI (words outside comments):', grep('[Ee]nergy [Rr]esistance\\|[Ii]mmunit', 'apps/desktop/src/characterHub/CharacterSheet.tsx', 'apps/desktop/src/characterHub/panels'))
spec = json.load(open('apps/desktop/scripts/ui-smoke/spec.json'))['rows']
hits = [r['id'] for r in spec if re.search(r'Swim|Fly|Resistance|Immun|Bravery|Conditional', json.dumps(r.get('expect', [])))]
print(f"  ui-smoke spec rows: {len(spec)}; rows expecting Swim/Fly/Resistance/Immunity/Bravery/Conditional: {len(hits)} {hits[:5]}")
