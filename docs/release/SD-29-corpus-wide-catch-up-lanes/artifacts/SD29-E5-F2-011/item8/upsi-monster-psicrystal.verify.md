# item-8 on-screen verification — PASS

- verdict: **PASS** — record and all expected strings rendered on the live app screen
- family: `monster` · record: `Psicrystal`
- expected on screen: `Psicrystal`
- expected on screen: `CR 0`
- expected on screen: `Ultimate Psionics`
- expected on screen: `Telepathic Link`
- agent: `sd29-monster-r12` · date: 2026-08-13T03:14:31Z
- HEAD: `e13a43dd`
- harness: `apps/desktop/.claude/skills/run-desktop/verify-on-screen.sh`
- screenshot: `docs/release/SD-29-corpus-wide-catch-up-lanes/artifacts/SD29-E5-F2-011/item8/upsi-monster-psicrystal.png`
- rendered lines containing the record/expectations:
```
4:Every real stat block the engine knows about, across Bestiary 1, Bonus Bestiary, Monster Codex, Book of the Damned, Volume 1, Book of the Damned, Volume 2, Inner Sea World Guide, Bestiary 2, Bestiary 3, Bestiary 4, Inner Sea Bestiary, Inner Sea Gods and Ultimate Psionics — 1239 monsters. Armor Class, hit points and saves are not shown because they are not ingested: PCGen derives them at runtime from the creature’s hit-dice table rather than stating them on its corpus row, so the row prints that table’s own token instead. Bestiary 1’s rows carry the land speed only; the other books’ carry every movement mode and their special abilities.
30:PsicrystalDiminutive Construct
31:CR 0
32:Speed 30 ft., climb 20 ft. · Ultimate Psionics p.48,448 · Hit dice Construct:1
35:A psicrystal cannot heal damage, but it can be repaired. Psicrystals do not have the usual construct traits of darkvision and low-light vision.
42:Telepathic Link — Special Quality (Su)p.49
45:The owner can have any power that he manifests on himself affect the psicrystal, if the psicrystal is within 5 feet. The owner can manifest a power with Target:You on the Psicrystal instead.
```
