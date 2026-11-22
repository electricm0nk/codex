// The `section-title` look (SD-40 E2.23, decisions.md §24.3.7): a section's prominent title, full
// width, 20 pt bold (larger and bolder than any block heading), a rule under it. It opens each
// section after page 1 (Abilities and Feats, Inventory, Spellbook, Bio, Math). A title carries no
// rows; any it is given print under the rule. It formats; it computes nothing.

#let look(b) = {
  block(width: 100%, below: 0.9em, {
    text(size: 20pt, weight: "bold", b.heading)
    v(-0.4em)
    line(length: 100%, stroke: 1.2pt)
  })
  for r in b.rows {
    par((r.label, ..r.values).filter(t => t != "").join("  "))
  }
}

// Page 1's block titles, one style for all of them (SD-41 decisions.md §34 (1)): a near-black box
// of one height with the title in white bold letters, like the PCGen sheets. Every page-1 look
// imports it from here (the frame imports the looks, so a look cannot import the frame: Typst
// refuses the cycle). It states its fill and height (`section-title-box`) so a test reads what
// printed. It formats; it computes nothing.
#let title-ink = rgb("#222222")
#let title-height = 0.5cm
#let section-title(t) = block(width: 100%, height: title-height, fill: title-ink, inset: (x: 4pt), above: 0pt, below: 0.15cm, {
  set align(left + horizon)
  text(size: 10.5pt, weight: "bold", fill: white, t)
  [#metadata((title: t, fill: title-ink.to-hex(), height: title-height / 1pt, text: "white bold")) <section-title-box>]
})
