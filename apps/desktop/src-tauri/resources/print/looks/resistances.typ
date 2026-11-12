// The `resistances` look (SD-40 E2.22, decisions.md §24.3.6): the heading "Resistances" over two
// lines of boxes, each its label beside a box holding what the engine states, or empty for the
// pencil: the five energies (Acid, Cold, Electricity, Fire, Sonic), then Damage Reduction, Spell
// Resistance and Immunities (the widest box, for the names). Two lines, not four: Letter's left
// column holds it under the Conditional Bonuses box (E2.4's rule: page 1 stays whole). It formats
// the strings it is given; it computes nothing (NR-2).
//
// The rows, as `print/blocks/core.rs` `resistances` sends them, by label: Acid, Cold, Electricity,
// Fire, Sonic, Damage Reduction, Spell Resistance, Immunities; value 0 = what prints ("" = empty).

#let heading-of(b) = {
  text(size: 12pt, weight: "bold", b.heading)
  v(-0.4em)
  line(length: 100%, stroke: 0.6pt + luma(60))
  v(-0.2em)
}

#let caption(t) = text(size: 7.5pt, weight: "bold", t)

// A box the engine filled, or (an empty string) a ruled write-in box; words wrap and the box grows.
#let value-box(v) = box(
  width: 100%, stroke: 0.6pt + luma(60), inset: (x: 3pt),
  block(height: auto, above: 0pt, below: 0pt, align(left + horizon, box(height: 0.4cm, align(horizon, text(size: 8.5pt, weight: "bold", v))))),
)

#let look(b) = {
  let value(label) = { let r = b.rows.find(r => r.label == label); if r == none { "" } else { r.values.at(0) } }
  let line-of(labels, widths) = grid(
    columns: labels.zip(widths).map(((_, w)) => (auto, w)).flatten(),
    column-gutter: (0.08cm, 0.2cm) * labels.len(),
    align: left + horizon,
    ..labels.map(l => (caption(l), value-box(value(l)))).flatten(),
  )
  let energies = ("Acid", "Cold", "Electricity", "Fire", "Sonic")
  let defenses = ("Damage Reduction", "Spell Resistance", "Immunities")
  // A row this look has no place for prints its label and values under the boxes.
  let others = b.rows.filter(r => r.label not in energies + defenses)
  block(breakable: false, width: 100%, above: 1em, below: 0.6em)[
    #heading-of(b)
    // The rows sit close under the rule: page 1's left column has no height to spare on Letter.
    #v(-0.45em)
    #stack(
      spacing: 0.06cm,
      line-of(energies, (1fr,) * energies.len()),
      line-of(defenses, (1.3fr, 1fr, 2.4fr)),
      ..others.map(r => text(size: 8.5pt)[*#r.label* #r.values.filter(v => v != "").join(" ")]),
    )
  ]
}
