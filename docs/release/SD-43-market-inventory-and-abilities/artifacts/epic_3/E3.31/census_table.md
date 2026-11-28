# E3.31 census table (GREEN, root lib run on the E3.31 change)

Source: `cargo test --locked -j 8 --lib -- --test-threads=8 --nocapture` (3254 passed), lines starting with `|` printed by `ability_budget_class_features_census_every_class_granted_choice_site_is_reachable_or_a_named_card`.

| abilitypool | data gap | Canny Foe | 1 | - |
| abilitypool | data gap | Swift Powder Choice | 1 | - |
| abilitypool | own sub-tab | Favored Class | 36 | - |
| abilitypool | own sub-tab | Feats | 188 | - |
| abilitypool | reachable |  | 715 | - |
| abilitypool | removes picks | an archetype's negative pool line takes the class's pick away | 30 | - |
| abilitypool | unreachable | PI redaction: the pool's category name is redacted, so its member filter is keyed to another name | 3 | E3.33 |
| abilitypool | unreachable | PI redaction: the record's type tag is redacted, so no pool offer selects it | 1 | E3.33 |
| abilitypool | unreachable | PI redaction: the variable Aspis Agent LVL has no declarer or contribution (its PI-named class's own line was redacted) | 1 | E3.33 |
| abilitypool | unreachable | PI redaction: the variable Golden Legionnaire LVL has no declarer or contribution (its PI-named class's own line was redacted) | 4 | E3.33 |
| abilitypool | unreachable | PI redaction: the variable Lantern Bearer LVL has no declarer or contribution (its PI-named class's own line was redacted) | 1 | E3.33 |
| abilitypool | unreachable | PI redaction: the variable Twilight Talon LVL has no declarer or contribution (its PI-named class's own line was redacted) | 5 | E3.33 |
| abilitypool | unreachable | PI redaction: the variable Westcrown Devil LVL has no declarer or contribution (its PI-named class's own line was redacted) | 1 | E3.33 |
| abilitypool | unreachable | converter: no converted record grants or offers the granting record | 4 | E3.33 |
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
