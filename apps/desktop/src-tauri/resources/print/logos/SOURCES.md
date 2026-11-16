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
| Cyberpunk RED | `cpr.png` | https://rtalsoriangames.com/wp-content/uploads/2020/10/RTG-CPR-Core-Cover.jpg (core-book cover, 1386 × 1800; crop box (166, 52, 1220, 406); cropped from cover by operator ruling 2026-11-16) | 2026-11-16 | `49f825b19f9c202ec3c7b3c4d118d4d74c4f9e6ea06510b3eb2a21a9d86aa3ca` | `9d94d2caf92119e6bbc4128c36b071c7d4116d45832439c5bf2e7550ba6cee76` | 1054 × 354 |

Cyberpunk RED (`cpr.png`): R. Talsorian Games publishes no standalone logo file, so the wordmark was
cropped from the cover by operator ruling 2026-11-16 (SD-41 E4.4 Open blocker). Crop box (left, top, right,
bottom) = (166, 52, 1220, 406) in cover pixels: the bounding box of the red wordmark pixels (R > 120, G < 80,
B < 80) in the top 470 rows, padded 12 px; saved with Pillow `Image.crop(box).save(...)`. No Cyberpunk RED
layout prints a portrait box yet, so no sheet reads it.

Re-check: `sha256sum apps/desktop/src-tauri/resources/print/logos/*.png`.
