# E5.4 attempt 2 — oracle and SRD evidence (decisions.md §21 (b), (c))

Oracle: PCGen at `7f818006e371188e5717fd18d74d18a420747fc6` (`scripts/pcgen-oracle-pin.env`), run 2026-10-05.

1. **Single-character drone run** (`scripts/pcgen-run-character.sh -c $PCGEN_REPO_DIR/code/testsuite/PCGfiles/sf_mechanic_drone.pcg -e single-character.ftl`)
   → `sf_mechanic_drone.single-character.oracle.txt`: `hp=0`, `var.drone_lvl=0`. Without its master loaded,
   `MASTERVAR("DroneCompanionLVL")` is 0, so this run cannot observe drone hit points.
2. **Party run, master loaded** (`bash ../../epic_3/token-mapping/oracle-builds/run_drone_party.sh`)
   → `../../epic_3/token-mapping/oracle-builds/sf_mechanic_drone.oracle.txt`: `hp=100`, `var.drone_lvl=10`,
   `var.race_hp=0`, `althp=0` (master Yo-yo, `master.var.drone_companion_lvl=10`). The mapping table's
   `hit_points` / `hit_die_offset` rows cite it.
3. **Hover drone without its master** (`run_hover_drone_alone.sh` derives it from PCGen's `sf_mechanic_drone.pcg` with
   `KEY:Drone Chassis ~ Stealth` → `Hover` and its chosen `Drone Mod ~ Flight System` line deleted;
   through `scripts/pcgen-run-character.sh -e abil.ftl`; the .pcg is not copied into the repo) → `hover_drone_alone.oracle.txt`:
   `dronemastertotallvl=0`, `flighttaken=2`, internal `Drone Flight System` held. Flight System's own
   `PREMULT:1,[PREPCLEVEL:MIN=11],[PREVARGTEQ:DroneMasterTotalLVL,11]` fails for this character (level 1,
   DroneMasterTotalLVL 0), yet both automatic grants from the chassis apply: PCGen does not test an
   `ABILITY:<cat>|AUTOMATIC|<key>` target's own prerequisites. (Its `fly=60` is PCGen's: 30 from the
   chassis' `BONUS:VAR|Fly|30` plus the internal `Drone Flight System`; the SRD hover chassis states
   "fly 30 feet (perfect)", "flight system (x2, included in its speed)". The codex render prints the
   chassis' Fly 30 ft.; the internal row is not converted. E7.1 parity: named in the receipt.)

SRD (Archives of Nethys Starfinder SRD, fetched 2026-10-05 with curl; text extracted in `srd-extracts.txt`):
- https://www.aonsrd.com/Classes.aspx?ItemName=Drone — drone base statistics table, Hit Points column:
  1st 10, 2nd 20, … 10th 100, … 17th 170, 18th 190, 19th 210, 20th 230; "Drones do not have Stamina Points."
- https://www.aonsrd.com/DroneChassis.aspx?ItemName=All — Hover Drone (Core Rulebook p. 75): Speed "30 feet,
  fly 30 feet (perfect)"; Initial Mods "flight system (x2, included in its speed), weapon mount"; and the page's
  general text: "Each chassis comes with initial drone mods that are a part of the chassis itself."
