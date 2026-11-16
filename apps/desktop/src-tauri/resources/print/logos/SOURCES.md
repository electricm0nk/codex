# Print logos — sources

The portrait box on page 1 prints the character's saved portrait, else its game system's logo
(SD-41 `decisions.md §34` (7)). The operator ruled the logos go in git (`decisions.md §35` Q10) and
accepts the trademark risk. Each file was fetched from the publisher's own site with
`curl -A 'Mozilla/5.0 …' -o <file> <url>` and converted from JPEG to PNG unchanged in size
(Pillow `Image.open(...).convert('RGB').save(...)`); the hash is of the committed PNG.

| System | File | Source URL | Fetched | Source sha256 (JPEG) | File sha256 (PNG) | Pixels |
|---|---|---|---|---|---|---|
| Pathfinder 1e | `pf1e.png` | https://paizo.com/image/content/Logos/PathfinderRPGLogo.jpg | 2026-11-16 | `68ef12e478a99959fee517970ef420a81ace4fc7e01484bd790a6f9fdb80eb99` | `a5e60d228bd493a8625a53115258980548d75ea16f4dfd1cf8e23d5697f81a8a` | 500 × 120 |
| Starfinder 1e | `sf1e.png` | https://paizo.com/image/content/Logos/Starfinder.jpg | 2026-11-16 | `4bcd9fac2d57cf3425e92bf52eff95bf211db77862eeed3a47ba4cd1d99058ea` | `0dc314bc99095d74a28b02e177a6a136109d54160e15cbb17659217fd44807e9` | 1000 × 163 |

Cyberpunk RED (`cpr.png`) is not here: R. Talsorian Games' site answered HTTP 429 to
`https://rtalsoriangames.com/cyberpunk-red/` on 2026-11-16 (SD-41 E4.4 receipt, `LEAD_WEB_FETCH`).
No Cyberpunk RED layout prints a portrait box, so no sheet reads it yet.

Re-check: `sha256sum apps/desktop/src-tauri/resources/print/logos/*.png`.
