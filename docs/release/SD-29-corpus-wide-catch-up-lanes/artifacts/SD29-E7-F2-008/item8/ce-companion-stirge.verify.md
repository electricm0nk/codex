# item-8 on-screen verification — PASS

- verdict: **PASS** — record and all expected strings rendered on the live app screen
- family: `companion` · record: `Stirge`
- expected on screen: `Stirge`
- expected on screen: `Core Essentials`
- expected on screen: `barbed legs latch onto the target`
- agent: `sd29-companion-r11` · date: 2026-08-13T02:11:46Z
- HEAD: `17121fdb`
- harness: `apps/desktop/.claude/skills/run-desktop/verify-on-screen.sh`
- screenshot: `docs/release/SD-29-corpus-wide-catch-up-lanes/artifacts/SD29-E7-F2-008/item8/ce-companion-stirge.png`
- rendered lines containing the record/expectations:
```
4:Every companion and familiar the engine has ingested, across Inner Sea Combat, Monster Codex, Inner Sea Intrigue, Horror Adventures, Bestiary 5, Bestiary 6, Bestiary 2, Bestiary 1, Bestiary 3, Bestiary 4, Ultimate Wilderness and Core Essentials — 393 creatures. Hit points, Armor Class and saves are not shown because they are not ingested: PCGen derives them at runtime from the creature’s hit-dice table rather than stating them on its corpus row, so the row prints that table’s own token instead. This is the corpus, not your character’s pet — the character sheet’s Pets tab shows the companion your own build computes.
18:Core Essentials (58)
22:StirgeTiny Magical Beast
23:Core Essentials p.260
28:When a stirge hits with a touch attack, its barbed legs latch onto the target, anchoring it in place. An attached stirge is effectively grappling its prey. The stirge loses its Dexterity bonus to AC, but holds on with great tenacity and inserts its proboscis into the grappled target's flesh. A stirge has a +8 racial bonus to maintain its grapple on a foe once it is attached. An attached stirge can be struck with a weapon or grappled itself. If its prey manages to win a grapple check or Escape Artist check against it, the stirge is removed.
```
