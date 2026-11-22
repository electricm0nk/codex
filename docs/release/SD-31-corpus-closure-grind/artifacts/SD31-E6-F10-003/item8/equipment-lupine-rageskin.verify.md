# DoD-8 on-screen verification — SD31-E6-F10-003

**Command:** `apps/desktop/.claude/skills/run-desktop/driver.sh` driven directly
(`RUN_DESKTOP_AGENT=sd31equipclass3`) — `npm ci` (fresh worktree, no `node_modules`),
`launch`, `click` "Browse Equipment Catalog", `click` the search field, `type "Lupine Rageskin"`,
`screenshot`.

**What the screenshot proves:**

1. **The book chip counts on screen match this cycle's own final, gate-verified figures
   exactly**: `HA (117)`, `ISR (71)`, `ISWG (46)`, `MC (49)`, `B2 (7)`, `B3 (8)`, `B4 (5)`,
   `OA (119)` — all 8 newly-extended books, all live in the running desktop app, all matching
   `tests/equipment_gap_tables.rs`'s `EXPECTED_PER_BOOK`. Total `7336` matches
   `equipment_resolver.rs`'s own pinned `rows.len()`.
2. **A real, newly-recovered record renders its real corpus value** — `Lupine Rageskin` (Horror
   Adventures, `ha_equip_arms_armor.lst:7`) shows the FULL real description recovered by this
   cycle's `description_token_value` fix (the `DESC:.CLEAR` → real-`DESC:` skip): "This +1
   leather armor consists of wolf skins sewn together with sinew. When the wearer rages, he
   automatically turns into a Medium wolf, as if using change shape (beast shape I)..." joined
   with its `SPROP:` text. Before the fix this record would have shipped the literal string
   `.CLEAR` instead of this prose.
3. Cost renders as `—` (no fabricated price) since the real corpus row carries no `COST:` token
   — matches `data/corpus/horror_adventures/equipment/lupine_rageskin.json`'s own `cost_gp: null`.

**PASS.** `driver.sh stop` run after capture.
