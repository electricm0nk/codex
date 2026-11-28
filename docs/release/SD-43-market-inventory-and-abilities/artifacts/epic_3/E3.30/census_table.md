# E3.30 census tables

## GREEN (work commit aae6f60038, root `cargo test --locked -j 8 --lib -- --test-threads=8 --nocapture`, census test `ok`)

| abilitypool | data gap | Canny Foe | 1 | - |
| abilitypool | data gap | Swift Powder Choice | 1 | - |
| abilitypool | own sub-tab | Favored Class | 36 | - |
| abilitypool | own sub-tab | Feats | 188 | - |
| abilitypool | reachable |  | 714 | - |
| abilitypool | removes picks | an archetype's negative pool line takes the class's pick away | 30 | - |
| abilitypool | unreachable | PI redaction: the pool's category name is redacted, so its member filter is keyed to another name | 3 | E3.33 |
| abilitypool | unreachable | PI redaction: the record's type tag is redacted, so no pool offer selects it | 1 | E3.33 |
| abilitypool | unreachable | PI redaction: the variable Aspis Agent LVL has no declarer or contribution (its PI-named class's own line was redacted) | 1 | E3.33 |
| abilitypool | unreachable | PI redaction: the variable Golden Legionnaire LVL has no declarer or contribution (its PI-named class's own line was redacted) | 4 | E3.33 |
| abilitypool | unreachable | PI redaction: the variable Lantern Bearer LVL has no declarer or contribution (its PI-named class's own line was redacted) | 1 | E3.33 |
| abilitypool | unreachable | PI redaction: the variable Twilight Talon LVL has no declarer or contribution (its PI-named class's own line was redacted) | 5 | E3.33 |
| abilitypool | unreachable | PI redaction: the variable Westcrown Devil LVL has no declarer or contribution (its PI-named class's own line was redacted) | 1 | E3.33 |
| abilitypool | unreachable | converter: no converted record grants or offers the granting record | 4 | E3.33 |
| abilitypool | unreachable | converter: the record's grant chain reaches no class, archetype, feat or variant rule | 1 | E3.33 |
| abilitypool | unreachable | engine gate: the gate is not met with every rule raising its variables held (an oracle gate its own data cannot meet) | 1 | E3.34 |
| abilitypool | unreachable | engine gate: the gate names a record the converter did not convert ({"name":"Allow Vows","pool":"internal"}) | 1 | E3.34 |
| abilitypool | unreachable | engine gate: the gate reads a fact the character record does not carry (DeityAlignment) | 1 | E3.34 |
| abilitypool | unreachable | engine gate: the gate reads a fact the character record does not carry (Race) | 1 | E3.34 |
| abilitypool | unreachable | engine gate: the record's own once-only prerequisite (`!PREABILITY:<itself>`) gates its pool line | 2 | E3.34 |
| abilityselection | reachable |  | 31 | - |
| abilityselection | unreachable | converter: no converted record grants or offers the granting record | 1 | E3.33 |
| add_token | granted (one option) | the one ability is held with the record | 5 | - |
| add_token | granted (one option) | the one item prints as granted | 5 | - |
| add_token | granted (one option) | the one language is granted | 1 | - |
| add_token | granted (one option) | the one spellcasting class prints | 13 | - |
| add_token | reachable |  | 6 | - |
| domain_count | granted (its class's domains) | the class's own DOMAIN: rows fill the count | 1 | - |
| domain_count | reachable |  | 39 | - |
| domain_count | unreachable | PI redaction: the record's type tag is redacted, so no pool offer selects it | 5 | E3.33 |
| parameter_choose | reachable |  | 210 | - |
| parameter_choose | unreachable | PI redaction: the record is PI-named and no converted grant or offer reaches it | 7 | E3.33 |
| parameter_choose | unreachable | PI redaction: the record's type tag is redacted, so no pool offer selects it | 7 | E3.33 |
| parameter_choose | unreachable | converter: no converted record grants or offers the granting record | 1 | E3.33 |

Sums (`awk -F"|" "{s+=\$5; ...}"` and a Python split over these rows agree): total 1329; reachable 1000; removes picks 30; E3.30 0; E3.33 42; E3.34 6.

## RED (pre-change source 04821e9b90, census filter only, `1 passed … 2004.15s`)

| abilitypool | data gap | Canny Foe | 1 | - |
| abilitypool | data gap | Swift Powder Choice | 1 | - |
| abilitypool | own sub-tab | Favored Class | 36 | - |
| abilitypool | own sub-tab | Feats | 188 | - |
| abilitypool | reachable |  | 545 | - |
| abilitypool | unreachable | no class of the granting record to seed | 90 | E3.30 |
| abilitypool | unreachable | the pool is granted but lists no member | 1 | E3.30 |
| abilitypool | unreachable | the pool is not granted to its class at 20 holding the granting record | 135 | E3.30 |
| abilityselection | reachable |  | 28 | - |
| abilityselection | unreachable | no class of the granting record to seed | 4 | E3.30 |
| add_token | granted (one option) | the one ability is held with the record | 5 | - |
| add_token | granted (one option) | the one item prints as granted | 5 | - |
| add_token | granted (one option) | the one language is granted | 1 | - |
| add_token | granted (one option) | the one spellcasting class prints | 13 | - |
| add_token | reachable |  | 5 | - |
| add_token | unreachable | no class of the granting record to seed | 1 | E3.30 |
| domain_count | granted (its class's domains) | the class's own DOMAIN: rows fill the count | 1 | - |
| domain_count | reachable |  | 5 | - |
| domain_count | unreachable | no class of the granting record to seed | 38 | E3.30 |
| domain_count | unreachable | the pool is not granted to its class at 20 holding the granting record | 1 | E3.30 |
| parameter_choose | reachable |  | 151 | - |
| parameter_choose | unreachable | no class of the granting record to seed | 73 | E3.30 |
| parameter_choose | unreachable | the pool is not granted to its class at 20 holding the granting record | 1 | E3.30 |

E3.30 rows: 9, 344 sites (awk).
