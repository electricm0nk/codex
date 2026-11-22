import json, re, os

root = os.path.expanduser("~/workspace/repos/pcgen/data")
BOOK_DIRS = {
    "bestiary": "pathfinder/paizo/roleplaying_game/bestiary",
    "bestiary_2": "pathfinder/paizo/roleplaying_game/bestiary_2",
    "bestiary_3": "pathfinder/paizo/roleplaying_game/bestiary_3",
    "bestiary_4": "pathfinder/paizo/roleplaying_game/bestiary_4",
    "bonus_bestiary": "pathfinder/paizo/roleplaying_game/bonus_bestiary",
    "monster_codex": "pathfinder/paizo/roleplaying_game/monster_codex",
    "book_of_the_damned_volume_1": "pathfinder/paizo/campaign_setting/book_of_the_damned_volume_1",
    "book_of_the_damned_volume_2": "pathfinder/paizo/campaign_setting/book_of_the_damned_volume_2",
    "inner_sea_world_guide": "pathfinder/paizo/campaign_setting/inner_sea_world_guide",
    "inner_sea_bestiary": "pathfinder/paizo/campaign_setting/inner_sea_bestiary",
    "inner_sea_gods": "pathfinder/paizo/campaign_setting/inner_sea_gods",
    "ultimate_psionics": "pathfinder/dreamscarred_press/ultimate_psionics",
    "horror_adventures": "pathfinder/paizo/roleplaying_game/horror_adventures",
}

d = json.load(open('docs/work-inventory.json'))
mon = [u for u in d['units'] if u.get('kind') == 'monster' and u.get('wiring_class') == 'derived' and u.get('status') == 'grounded']
print("total", len(mon))

ABIL = re.compile(r'\b(STR|DEX|CON|INT|WIS|CHA)\b')

file_cache = {}


def lines(book, fname):
    key = (book, fname)
    if key in file_cache:
        return file_cache[key]
    bookdir = os.path.join(root, BOOK_DIRS[book])
    found = None
    for dp, _, fs in os.walk(bookdir):
        if fname in fs:
            found = os.path.join(dp, fname)
            break
    text = open(found, encoding='utf-8', errors='replace').read().split('\n') if found else []
    file_cache[key] = text
    return text


ability_scaling = 0
missing_book = 0
examples = []
for u in mon:
    book = u['book']
    if book not in BOOK_DIRS:
        missing_book += 1
        continue
    ls = lines(book, u['source_file'])
    ln = u['source_line']
    if ln - 1 >= len(ls):
        continue
    row = ls[ln - 1]
    fields = [f.strip() for f in row.split('\t') if f.strip()]
    hit = False
    for f in fields:
        if f.startswith('BONUS:STAT|'):
            continue
        if f.startswith('BONUS:VAR|') or f.startswith('DR:') or f.startswith('SR:') or f.startswith('BONUS:COMBAT|') or f.startswith('BONUS:SKILL|'):
            if ABIL.search(f):
                hit = True
    if hit:
        ability_scaling += 1
        if len(examples) < 5:
            examples.append((u['id'], row[:200]))

print("ability_scaling", ability_scaling)
print("missing_book", missing_book)
for e in examples:
    print(e)
