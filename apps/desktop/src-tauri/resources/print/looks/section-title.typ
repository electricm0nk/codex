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
