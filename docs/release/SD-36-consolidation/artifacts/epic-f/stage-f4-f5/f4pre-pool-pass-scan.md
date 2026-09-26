# F4pre: SD-32 generic pool-group pass scan, before and after (SD-36 Epic F4pre, FS-21)

Script: `artifacts/epic-f/scripts/f4pre_pool_pass_scan.rs` -- the F3c4 scan (`f3c4_pool_pass_scan.rs`) unchanged plus one `SEL` row per selection. Appended temporarily to `src/rules_core/pilot_compute/pool_groups.rs`, run with `cargo test --locked -j 8 --lib -- --test-threads=8 --nocapture f4pre_pool_pass_scan` (before: 2,447 s on `6db56623e8`; after: 3,374 s on the F4pre tree), then removed. Raw rows (LINE rows dropped): `f4pre-pool-pass-scan-before.log`, `f4pre-pool-pass-scan-after.log`.

**Denominator** (the F3c4 population): 6 pools the pass prints (its 6 callers); every selection whose slug resolves to a real corpus group through the pass's own `real_pool_group_for_selection_slug` (191 selections); levels 1..=20; a human census-fixture character of that class. A value is **ungated** when the pass prints it for a selection the package does not link to a held option.

| pool | selections | linked + held before | after | member values the pass resolves | ungated before | **ungated after** |
|---|---:|---:|---:|---:|---:|---:|
| Sorcerer Bloodline | 53 | 30 | 30 | 9800 | 1,040 | **1,040** |
| Bloodrager Bloodline | 12 | 10 | 10 | 1880 | 220 | **220** |
| Cleric Domain | 73 | 0 | 30 | 2960 | 2,960 | **1,920** |
| Shaman Spirit | 14 | 0 | 12 | 1240 | 1,240 | **240** |
| Warpriest Blessing | 37 | 0 | 0 | 0 | 0 | **0** |
| Cavalier Order | 2 | 0 | 0 | 0 | 0 | **0** |
| **total** | 191 | | | | **5,460** | **3,420** |

Newly linked selections (before: not linked; after: linked and held at every level): 
42: `domain:air`, `domain:animal`, `domain:artifice`, `domain:charm`, `domain:community`, `domain:darkness`, `domain:death`, `domain:destruction`, `domain:earth`, `domain:fire`, `domain:glory`, `domain:healing`, `domain:knowledge`, `domain:liberation`, `domain:luck`, `domain:madness`, `domain:magic`, `domain:nobility`, `domain:plant`, `domain:protection`, `domain:repose`, `domain:rune`, `domain:scalykind`, `domain:strength`, `domain:sun`, `domain:travel`, `domain:trickery`, `domain:void`, `domain:water`, `domain:weather`, `spirit:battle`, `spirit:bones`, `spirit:flame`, `spirit:heavens`, `spirit:life`, `spirit:lore`, `spirit:mammoth`, `spirit:nature`, `spirit:stone`, `spirit:waves`, `spirit:wind`, `spirit:wood`.

The cleric `LINE` rows of the after-log read `NOT-HELD`: the pass reads the `<X> Domain ~ <power>` records (`cr_abilities_class.lst:3179+`), which only the inquisitor's `Inquisitor Domain ~ <X>` records grant (`apg_abilities_class.lst:367`), while the held set holds the `Domain Power ~ <power>` records the domain grants (`Core Domain ~ Air Domain` -> `Domain Power ~ Lightning Arc`, gated `DomainAirAbilityTriggerLVL >= 1`; Electricity Resistance `>= 6`). The yield removes those values; the held set prints the domain's own lines at their levels (`tests/sd36_f4pre_pool_choices.rs`).

## The 3,420 still printed, by selection and mechanism

| pool | group | selection | ungated values | mechanism |
|---|---|---|---:|---|
| Sorcerer Bloodline | Aerial Bloodline | `bloodline:aerial` | 20 | (b) wildblooded mutation (`Wildblooded ~ <X>`, UM p.70, behind the Wildblooded archetype), not a bloodline |
| Sorcerer Bloodline | Bedrock Bloodline | `bloodline:bedrock` | 40 | (b) wildblooded mutation (`Wildblooded ~ <X>`, UM p.70, behind the Wildblooded archetype), not a bloodline |
| Sorcerer Bloodline | Brutal Bloodline | `bloodline:brutal` | 20 | (b) wildblooded mutation (`Wildblooded ~ <X>`, UM p.70, behind the Wildblooded archetype), not a bloodline |
| Sorcerer Bloodline | Empyreal Bloodline | `bloodline:empyreal` | 80 | (b) wildblooded mutation (`Wildblooded ~ <X>`, UM p.70, behind the Wildblooded archetype), not a bloodline |
| Sorcerer Bloodline | Envenomed Bloodline | `bloodline:envenomed` | 20 | (b) wildblooded mutation (`Wildblooded ~ <X>`, UM p.70, behind the Wildblooded archetype), not a bloodline |
| Sorcerer Bloodline | Imperious Bloodline | `bloodline:imperious` | 220 | (c) option gate excludes: race template (FS-19) |
| Sorcerer Bloodline | Karmic Bloodline | `bloodline:karmic` | 20 | (b) wildblooded mutation (`Wildblooded ~ <X>`, UM p.70, behind the Wildblooded archetype), not a bloodline |
| Sorcerer Bloodline | Kobold Bloodline | `bloodline:kobold` | 260 | (c) option gate excludes: race template (FS-19) |
| Sorcerer Bloodline | Linnorm Bloodline | `bloodline:linnorm` | 20 | (b) wildblooded mutation (`Wildblooded ~ <X>`, UM p.70, behind the Wildblooded archetype), not a bloodline |
| Sorcerer Bloodline | Pit-Touched Bloodline | `bloodline:pit_touched` | 20 | (b) wildblooded mutation (`Wildblooded ~ <X>`, UM p.70, behind the Wildblooded archetype), not a bloodline |
| Sorcerer Bloodline | Rime-Blooded Bloodline | `bloodline:rime_blooded` | 60 | (b) wildblooded mutation (`Wildblooded ~ <X>`, UM p.70, behind the Wildblooded archetype), not a bloodline |
| Sorcerer Bloodline | Sage Bloodline | `bloodline:sage` | 60 | (b) wildblooded mutation (`Wildblooded ~ <X>`, UM p.70, behind the Wildblooded archetype), not a bloodline |
| Sorcerer Bloodline | Seaborn Bloodline | `bloodline:seaborn` | 20 | (b) wildblooded mutation (`Wildblooded ~ <X>`, UM p.70, behind the Wildblooded archetype), not a bloodline |
| Sorcerer Bloodline | Sylvan Bloodline | `bloodline:sylvan` | 20 | (b) wildblooded mutation (`Wildblooded ~ <X>`, UM p.70, behind the Wildblooded archetype), not a bloodline |
| Sorcerer Bloodline | Umbral Bloodline | `bloodline:umbral` | 40 | (b) wildblooded mutation (`Wildblooded ~ <X>`, UM p.70, behind the Wildblooded archetype), not a bloodline |
| Sorcerer Bloodline | Visionary Bloodline | `bloodline:visionary` | 20 | (b) wildblooded mutation (`Wildblooded ~ <X>`, UM p.70, behind the Wildblooded archetype), not a bloodline |
| Sorcerer Bloodline | Void-Touched Bloodline | `bloodline:void_touched` | 80 | (b) wildblooded mutation (`Wildblooded ~ <X>`, UM p.70, behind the Wildblooded archetype), not a bloodline |
| Sorcerer Bloodline | Warped Bloodline | `bloodline:warped` | 20 | (b) wildblooded mutation (`Wildblooded ~ <X>`, UM p.70, behind the Wildblooded archetype), not a bloodline |
| Bloodrager Bloodline | Bloodrager Bloodline | `bloodline:bloodrager` | 40 | (a) not a pick the pool offers: the pool's own name |
| Bloodrager Bloodline | Verdant Bloodrager Bloodline | `bloodline:verdant` | 180 | (e) no converted pick option: UW Verdant Bloodrager has no bloodrager pick option and no choice selects its record |
| Cleric Domain | Aquatic Domain | `domain:aquatic` | 20 | (c) option gate excludes: druid-only domain (`PRECLASS:1,Druid=1`, `um_domains.lst`) |
| Cleric Domain | Arctic Domain | `domain:arctic` | 20 | (c) option gate excludes: druid-only domain (`PRECLASS:1,Druid=1`, `um_domains.lst`) |
| Cleric Domain | Chaos Domain | `domain:chaos` | 40 | (c) option gate excludes: alignment domain, and the character record carries no alignment |
| Cleric Domain | Core Domain | `domain:core` | 40 | (a) not a pick the pool offers: `Core Domain` is the domains' internal grant group |
| Cleric Domain | Eagle Domain | `domain:eagle` | 40 | (c) option gate excludes: druid-only domain (`PRECLASS:1,Druid=1`, `um_domains.lst`) |
| Cleric Domain | Evil Domain | `domain:evil` | 40 | (c) option gate excludes: alignment domain, and the character record carries no alignment |
| Cleric Domain | Forbidden Rites Domain | `domain:forbidden_rites` | 1380 | (a) not a pick the pool offers: `Forbidden Rites Domain` is a magus archetype group |
| Cleric Domain | Frog Domain | `domain:frog` | 60 | (c) option gate excludes: druid-only domain (`PRECLASS:1,Druid=1`, `um_domains.lst`) |
| Cleric Domain | Heresy Domain | `domain:heresy` | 20 | (d) an inquisition (`Inquisition ~ <X>`, UM), not a domain |
| Cleric Domain | Jungle Domain | `domain:jungle` | 20 | (c) option gate excludes: druid-only domain (`PRECLASS:1,Druid=1`, `um_domains.lst`) |
| Cleric Domain | Law Domain | `domain:law` | 40 | (c) option gate excludes: alignment domain, and the character record carries no alignment |
| Cleric Domain | Monkey Domain | `domain:monkey` | 40 | (c) option gate excludes: druid-only domain (`PRECLASS:1,Druid=1`, `um_domains.lst`) |
| Cleric Domain | Mountain Domain | `domain:mountain` | 40 | (c) option gate excludes: druid-only domain (`PRECLASS:1,Druid=1`, `um_domains.lst`) |
| Cleric Domain | Oblivion Domain | `domain:oblivion` | 20 | (d) an inquisition (`Inquisition ~ <X>`, UM), not a domain |
| Cleric Domain | Plains Domain | `domain:plains` | 20 | (c) option gate excludes: druid-only domain (`PRECLASS:1,Druid=1`, `um_domains.lst`) |
| Cleric Domain | Serpent Domain | `domain:serpent` | 20 | (c) option gate excludes: druid-only domain (`PRECLASS:1,Druid=1`, `um_domains.lst`) |
| Cleric Domain | Swamp Domain | `domain:swamp` | 20 | (c) option gate excludes: druid-only domain (`PRECLASS:1,Druid=1`, `um_domains.lst`) |
| Cleric Domain | Tactics Domain | `domain:tactics` | 20 | (d) an inquisition (`Inquisition ~ <X>`, UM), not a domain |
| Cleric Domain | Wolf Domain | `domain:wolf` | 20 | (c) option gate excludes: druid-only domain (`PRECLASS:1,Druid=1`, `um_domains.lst`) |
| Shaman Spirit | Shaman Spirit | `spirit:shaman` | 240 | (a) not a pick the pool offers: the pool's own name |

By mechanism: (a) 1,700; (b) 560; (c) 920; (d) 60; (e) 180 -- total 3,420.
