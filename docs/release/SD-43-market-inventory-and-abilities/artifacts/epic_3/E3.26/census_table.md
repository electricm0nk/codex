# E3.26 census table (root full lib run, --nocapture)

Command: `cargo test --locked -j 8 --lib -- --test-threads=8 --nocapture` (repo root, E3.26 work tree); rows as printed by `ability_budget_class_features_census_every_class_granted_choice_site_is_reachable_or_a_named_card`.

    choice-site census (1329 sites): mechanism | state | cause | sites | card

| mechanism | state | cause | sites | card |
|---|---|---|---|---|
| abilitypool | data gap | Canny Foe | 1 | - |
| abilitypool | data gap | Swift Powder Choice | 1 | - |
| abilitypool | own sub-tab | Favored Class | 36 | - |
| abilitypool | own sub-tab | Feats | 188 | - |
| abilitypool | reachable |  | 536 | - |
| abilitypool | unreachable | abilitypool: the granting record carries no converted Pool line for the pool | 5 | E3.29 |
| abilitypool | unreachable | no class of the granting record to seed | 90 | E3.30 |
| abilitypool | unreachable | the pool is granted but lists no member | 5 | E3.30 |
| abilitypool | unreachable | the pool is not granted to its class at 20 holding the granting record | 135 | E3.30 |
| abilityselection | reachable |  | 12 | - |
| abilityselection | unreachable | abilityselection of PCGen's base category FEAT: no Class Features pool | 9 | E3.29 |
| abilityselection | unreachable | abilityselection of PCGen's base category Internal: no Class Features pool | 8 | E3.29 |
| abilityselection | unreachable | abilityselection of PCGen's base category Special Ability: no Class Features pool | 2 | E3.29 |
| abilityselection | unreachable | no class of the granting record to seed | 1 | E3.30 |
| add_token | unreachable | add_token: no Class Features pool (the record offers Languages) | 3 | E3.27 |
| add_token | unreachable | add_token: no Class Features pool (the record offers Rules) | 5 | E3.27 |
| add_token | unreachable | add_token: no Class Features pool (the record offers nothing) | 22 | E3.27 |
| domain_count | reachable |  | 3 | - |
| domain_count | unreachable | domain_count: no Class Features pool (the record offers Domains) | 40 | E3.28 |
| domain_count | unreachable | domain_count: no Class Features pool (the record offers Languages) | 1 | E3.28 |
| domain_count | unreachable | domain_count: no Class Features pool (the record offers nothing) | 1 | E3.28 |
| parameter_choose | reachable |  | 151 | - |
| parameter_choose | unreachable | no class of the granting record to seed | 73 | E3.30 |
| parameter_choose | unreachable | the pool is not granted to its class at 20 holding the granting record | 1 | E3.30 |
