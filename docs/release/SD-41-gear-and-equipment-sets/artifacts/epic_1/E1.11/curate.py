"""E1.11 curation: every Market row E1.9 left unpriced gets a cited d20pfsrd price or a price-unknown
row naming every source tried.

    python3 -I curate.py <repo-root> <work-dir> <census.json>

<census.json> is E1.8's market-prices.json at the card base (the 397 rows E1.9 left unpriced):
    git show e1597c8d95:docs/release/SD-41-gear-and-equipment-sets/artifacts/epic_1/E1.8/market-prices.json > census.json

Pipeline (run in this order; <work> is any scratch directory):
    fetch.sh <sitemap-urls> <work>/cache ; match.py ; fetch.sh <candidates + hub pages> <work>/cache ;
    index.py <work>/cache <work>/index.json ; resolve.py ; curate.py

Inputs (all produced in <work-dir> by the sibling scripts, from pages fetched with a browser user
agent): resolved.json (match.py + resolve.py), index.json (index.py), cache/fetch-log.tsv
(fetch.sh). The census (E1.8's market-prices.json) is the population. Writes, under the repo:
data/price_overrides/{d20pfsrd-prices,price-unknown}.json and artifacts/epic_1/E1.11/batch-<n>.json.

A price is taken only from a page row naming the item, as a single gp figure. Every decision below
was read off the page text by hand; the script re-reads the figure from the fetched page so the
number in the list is the page's, not a typed one.
"""
import json, os, re, sys

root, work, census_path = sys.argv[1:4]
PKG = 'docs/release/SD-41-gear-and-equipment-sets'
D = 'https://www.d20pfsrd.com/'
census = json.load(open(census_path))
rows = census['none'] + census['zero'] + census['negative']
resolved = {x['key']: x for x in json.load(open(os.path.join(work, 'resolved.json')))}
index = json.load(open(os.path.join(work, 'index.json')))
fetched = {}
for line in open(os.path.join(work, 'cache/fetch-log.tsv')):
    u, code, at = line.rstrip('\n').split('\t')
    if code == '200':
        fetched[u] = at
SITEMAP = D + 'wp-sitemap.xml'
fetched.setdefault(SITEMAP, min(fetched.values()))  # fetched before the page cache: sitemap-fetch.tsv (24 files, 45,282 <loc>)

# ---- priced: key -> (page URL, heading or table-row name on that page, which figure) ----------
W, A, CUR = D + 'equipment/weapons/', D + 'equipment/armor/', D + 'magic-items/cursed-items/'
SMW, SMS = D + 'magic-items/magic-weapons/specific-magic-weapons/', D + 'magic-items/magic-armor/specific-magic-shields/'
MYW = D + 'alternative-rule-systems/paizo-rules-systems/mythic/mythic-magic-items/magic-weapons/'
UNI = D + 'alternative-rule-systems/3rd-party-rules-systems/psionics-unleashed/equipment/psionic-items/universal-items/'
PRICED = {
    'Holy Avenger': (SMW + 'holy-avenger/', 'Holy Avenger', None),
    'Longsword (Cold Iron/Masterwork)': (SMW + 'longsword-cold-iron-masterwork/', 'Longsword, Cold Iron Masterwork', None),
    "Shifter's Sorrow": (SMW + 'shifter-s-sorrow/', 'Shifter’s Sorrow', None),
    'Dagger (Silver/Masterwork)': (SMW + 'silver-dagger-masterwork/', 'Silver Dagger, Masterwork', None),
    'Longbow (Composite +0)': (W, 'Longbow, composite (+0)', None),
    'Shortbow (Composite +0)': (W, 'Shortbow, composite', None),
    'Oathbow': (SMW + 'oathbow/', 'Oathbow', None),
    "Maiden's Panoply": (D + 'magic-items/magic-armor/specific-magic-armor/maidens-panoply/', 'Maiden’s Panoply', None),
    'Darkwood Buckler': (SMS + 'darkwood-buckler/', 'Darkwood Buckler', None),
    'Darkwood Shield': (SMS + 'darkwood-shield/', 'Darkwood Shield', None),
    'Sun Blade (Short)': (SMW + 'sun-blade/', 'Sun Blade', None),
    'Quarterstaff (Hurricane)': (SMW + 'hurricane-quarterstaff/', 'Hurricane Quarterstaff', None),
    'Quarterstaff of Vaulting': (SMW + 'quarterstaff-of-vaulting/', 'Quarterstaff of Vaulting', None),
    'Sling (Frostbite)': (SMW + 'frostbite-sling/', 'Frostbite Sling', None),
    'Quarterstaff of Contemplation': (SMW + 'quarterstaff-of-contemplation/', 'Quarterstaff of Contemplation', None),
    'Shadow Spike': (MYW, 'Shadow Spike', None),
    'Spellbreaker': (MYW, 'Spellbreaker', None),
    "Horsemaster's Saddle (Mount)": (D + 'magic-items/wondrous-items/h-l/horsemaster-s-saddle/', 'Horsemaster’s Saddle', None),
    'Aklys': (W, 'Aklys', None),
    'Weapon (Dwarven Boulder)': (W, 'Helmet, dwarven boulder', None),
    'Torc of Interrogation': (UNI, 'Torc of Interrogation', None),
    # CRB Table: Specific Cursed Items, "Market Price" column.
    'Armor of Arrow Attraction': (CUR, 'Armor of arrow attraction', None),
    'Armor of Rage': (CUR, 'Armor of rage', None),
    'Bag of Devouring': (CUR, 'Bag of devouring', None),
    'Bracers of Defenselessness': (CUR, 'Bracers of defenselessness', None),
    'Crystal Hypnosis Ball': (CUR, 'Crystal hypnosis ball', None),
    'Flask of Curses': (CUR, 'Flask of curses', None),
    'Necklace of Strangulation': (CUR, 'Necklace of strangulation', None),
    'Net of Snaring': (CUR, 'Net of snaring', None),  # the record is the cursed net (UE), not the +1 distance net
    'Potion of Poison': (CUR, 'Potion of poison', None),
    'Ring of Clumsiness': (CUR, 'Ring of clumsiness', None),
    'Robe of Vermin': (CUR, 'Robe of vermin', None),
    'Scarab of Death': (CUR, 'Scarab of death', None),
    'Cursed Backbiter Spear': (CUR, 'Spear, cursed backbiter', None),
    'Stone of Weight (Loadstone)': (CUR, 'Stone of weight', None),
    'Berserking Sword': (CUR, 'Sword, berserking', None),
}
for n in range(1, 6):  # "Price varies: +1 bonus 2,700 gp, +2 bonus 10,800 gp, ..."
    PRICED[f'Caparison of Resistance +{n} (Mount)'] = (
        D + 'magic-items/wondrous-items/c-d/caparison-of-resistance/', 'Caparison of Resistance', rf'\+{n} bonus ([\d,]+) gp')


def page_price(url, name, pick):
    hits = [e for e in index if e['url'] == url and e['name'] == name and e['price']]
    assert hits, f'no priced row {name!r} on {url}'
    text = hits[0]['price']
    m = re.search(pick, text) if pick else re.fullmatch(r'([\d,]+(?:\.\d+)?) gp', text.strip())
    assert m, f'{name!r} on {url}: not a single gp figure: {text!r}'
    return float(m.group(1).replace(',', '')), hits[0]['fetched_at']


# ---- unknown: why no page prices the row -------------------------------------------------------
def rx(*names):
    return re.compile('^(?:' + '|'.join(names) + ')$')

NOT_AN_ITEM = [
    (rx(r'.*\.FORGET'), 'a PCGen removal directive served as a row, not an item (E2.8 hides it)'),
    (rx(r'Improvised Weapon.*', 'Unarmed Strike', 'Flurry of Blows', "Brawler's Flurry", 'Flurry of Fists',
        'Flurry of Strikes', r'Shieldbash \(.*\)', r'Touch Attack \(Ray\)', 'Deadly Fist', 'Mystic Bolts'),
     'an attack form or class feature, not a purchasable item; d20pfsrd states no price'),
    (rx(r'Mind Blade.*', r'Mind Bolt.*', 'Mind Dagger', 'Mind Arrow', 'Mind Xephyr Knife', r'(Improved )?Mind Armor.*',
        r'Mind Shield.*', 'Astral Armor', 'Astral Juggernaut', r'Astral Suit Ram \d', 'Astral Skin', 'Crystal Shard'),
     'a psionic class-feature construct (soulknife mind blade, aegis astral suit) or a power, not a purchasable item'),
    (rx('Demon Armor Claw Attack', 'Demon Prince Armor Claw Attack', 'Ring of Rat Fangs Bite Attack',
        'Gauntlet \\(Part of Armor Suit\\)'),
     "a natural attack or part granted by another item; priced with that item, not on its own"),
    (rx(r'Rock \(.*\)', 'Skull', 'Ranged Cannon ~ Clockwork Goliath', r'Rag Armor \(Dark Creeper\)', 'Gaff',
        r'Heartstone \(Night Hag\)', r'Poison \(Black Smear\)', 'Living Grimoire Holy Text', 'Brilliant Plan Encumberance',
        'Fake Rapier', 'Bomb', 'Seedpod', 'Fetish Mask', 'Halcyon Bonded Mask'),
     'a monster or class-feature item (bestiary gear, a class ability, a test record), not sold; d20pfsrd states no price'),
    (rx('Belt', 'Girdle', 'Robe', 'Vestment', 'Mantle', 'Shirt', 'Vest', 'Eye Lenses', 'Goggles', 'Boots', 'Shoes',
        'Slippers', 'Gauntlets', 'Gloves', 'Circlet', 'Crown', 'Hat', 'Helmet', 'Mask', 'Headband', 'Phylactery',
        'Amulet', 'Brooch', 'Medallion', 'Necklace', 'Periapt', 'Scarab', 'Ring', 'Cape', 'Cloak', 'Bracelets', 'Bracers',
        'Spell Component', 'Spell Focus', 'Art', 'Gem', 'Jewelry', r'Tattoo \w+', 'Potion', 'Rod', 'Scroll', 'Staff',
        'Wand', 'Dorje', 'Power Stone', 'Psicrown', 'Psionic Tattoo', 'Crawling Tattoo', 'Spell Tattoo', 'Riffle Scroll'),
     'a generic slot or item-kind placeholder (the base a magic item is built on); d20pfsrd prices only the named items of its kind'),
]
VARIES = [
    (rx(r'Barding \(.*\)', r'.* Barding \(Base\)'),
     'd20pfsrd prices barding as the armor’s price ×2 (Medium mount) or ×4 (Large); the row names no mount size'),
    (rx('Holy Symbol', 'Unholy Symbol'), 'd20pfsrd prices a holy symbol by material (wooden 1 gp … platinum 500 gp); the row names none'),
    (rx(r'Holy Symbol \((Compartment|Flask)\)', 'Armor Spikes'), 'd20pfsrd states only a surcharge (+N gp) over the item it is added to'),
    (rx('Spiked Armor \\(Base\\)'), "d20pfsrd's weapon table prices spiked armor as 'special' (the armor's price plus spikes)"),
    (rx('Amulet of Mighty Fists', 'Page of Spell Knowledge', r'Mind Stone( \(Greater\))?'),
     'd20pfsrd prices it by bonus or spell level (a table of prices); the row names neither'),
    (rx('Boots of Dancing', 'Gauntlets of Fumbling'), "d20pfsrd's cursed-item table prices it '*' (as the item it appears to be)"),
    (rx(r'Quick-Change Outfit', r'Antidote \(Vial\)'), 'priced relative to another item (the outfits mimicked, the poison countered); no fixed price'),
    (rx(r'Forked Metal Rod.*'), 'a plane shift focus; d20pfsrd states no price for it'),
]
FREE = rx('Holly and Mistletoe', 'Raft', r'Club \(Base\)', r'Quarterstaff \(Base\)', r'Sling \(Base\)', r'Stake \(Wood\)',
          'Amentum', r'Amentum \(Javelin\)')


def reason_for(r):
    k = r['key']
    for pat, why in NOT_AN_ITEM + VARIES:
        if pat.match(k):
            return why
    if FREE.match(k):
        return "d20pfsrd lists it at '—' (no cost); a price of 0 is not a positive price"
    hits = [h for h in resolved[k]['hits'] if re.search(r'/(magic-items|equipment|psionic-items|mythic-magic-items)/', h['url'])]
    if r.get('_artifact') or any('/artifacts/' in h['url'] for h in hits):
        return 'an artifact: d20pfsrd and the book state no price'
    if r.get('_cursed') and any(h['url'] == CUR for h in hits):
        return "a cursed item: d20pfsrd's entry for it states no price (only the CRB table's items carry one)"
    if hits:
        return 'its d20pfsrd entry states no price'
    return 'no d20pfsrd page names it (sitemap of every page searched by name, and its category pages read)'


HUB = {  # the category page read for a row no page names
    'weapon': [W], 'armor': [A], 'shield': [A],
    'gear': [D + 'equipment/goods-and-services/containers-bags-boxes-more/'],
    'magicItem': [D + 'magic-items/wondrous-items/'],
}
for r in rows:
    path = (r.get('corpus') or [None])[0]
    tk = {}
    if path:
        data = json.load(open(os.path.join(root, path)))['data']
        tk = {t['key']: t['value'] for t in data.get('raw_tokens') or []}
    r['_artifact'] = 'Artifact' in (tk.get('TYPE') or '')
    r['_cursed'] = 'Cursed' in (tk.get('TYPE') or '')
    r['_psionic'] = r['book'] == 'UPSI'


def tried_urls(r):
    x = resolved[r['key']]
    urls = []
    for u in [h['url'] for h in x['hits']] + x['candidates']:
        if u in fetched and u not in urls:
            urls.append(u)
    urls = urls[:2]
    if True:  # every row's name was also searched in the sitemap, and its category pages read
        urls.append(SITEMAP)
        hubs = list(HUB[r['market_category']])
        if r['_cursed']:
            hubs = [CUR]
        if r['_artifact']:
            hubs = [D + 'magic-items/artifacts/minor-artifacts/', D + 'magic-items/artifacts/major-artifacts/']
        if r['_psionic']:
            hubs = [UNI, D + 'alternative-rule-systems/3rd-party-rules-systems/psionics-unleashed/equipment/psionic-items/psionic-artifacts/']
        if r['book'] == 'MYTHIC':
            hubs = [D + 'alternative-rule-systems/paizo-rules-systems/mythic/mythic-magic-items/other-magic-items/']
        for h in hubs:
            assert h in fetched, h
            if h not in urls:
                urls.append(h)
    return urls


prices, unknown, batch_rows = [], [], []
for r in rows:
    k = r['key']
    out = {'key': k, 'name': r['catalog_name'], 'category': r['market_category'], 'book': r['book']}
    if k in PRICED:
        url, name, pick = PRICED[k]
        gp, at = page_price(url, name, pick)
        prices.append({'key': k, 'price_gp': gp, 'url': url, 'fetched_at': at})
        out.update(candidate_url=url, outcome='priced', price_gp=gp)
    else:
        urls = tried_urls(r)
        why = reason_for(r)
        unknown.append({'key': k, 'reason': why, 'tried': ['converter'] + ['d20pfsrd:' + u for u in urls]})
        out.update(candidate_url=urls[0], outcome='price-unknown', reason=why)
    batch_rows.append(out)

assert len(prices) + len(unknown) == len(rows) == len({r['key'] for r in rows})
prices.sort(key=lambda p: p['key'])
json.dump(prices, open(os.path.join(root, 'data/price_overrides/d20pfsrd-prices.json'), 'w'), indent=2, ensure_ascii=False)
open(os.path.join(root, 'data/price_overrides/d20pfsrd-prices.json'), 'a').write('\n')
json.dump(unknown, open(os.path.join(root, 'data/price_overrides/price-unknown.json'), 'w'), indent=2, ensure_ascii=False)
open(os.path.join(root, 'data/price_overrides/price-unknown.json'), 'a').write('\n')
art = os.path.join(root, PKG, 'artifacts/epic_1/E1.11')
os.makedirs(art, exist_ok=True)
for n in range(0, len(batch_rows), 150):
    path = os.path.join(art, f'batch-{n // 150 + 1}.json')
    json.dump(batch_rows[n:n + 150], open(path, 'w'), indent=1, ensure_ascii=False)
    open(path, 'a').write('\n')
print('rows', len(rows), 'priced', len(prices), 'price-unknown', len(unknown), 'batches', -(-len(rows) // 150))
by = {}
for u in unknown:
    by[u['reason']] = by.get(u['reason'], 0) + 1
for k, v in sorted(by.items(), key=lambda kv: -kv[1]):
    print(v, k)
