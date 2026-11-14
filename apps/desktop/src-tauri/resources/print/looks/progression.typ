// The `progression` look (SD-40 E3.7, decisions.md §24.3.7): the Progression section's pages, the
// Show Progression drawer level by level. Each row of kind `level` is one entry, whole on a page:
// `Level <n>` and the class and class level in bold, then the drawer's lines (`notes`) under it, a
// thin rule closing it; a row of kind `apart` (what the ledger placed at no level) prints the same
// way under its own name. It formats strings it is given; it computes nothing (NR-2).

#let entry(r) = block(breakable: false, width: 100%, above: 0.9em, below: 0.4em, {
  text(size: 11pt, weight: "bold", (r.label, ..r.values).filter(t => t != "").join(" · "))
  for n in r.notes {
    // A line's own name (`Feats`, `Skill ranks` ...) in bold before its words, for the eye to find.
    let (name, ..rest) = n.split(": ")
    block(above: 0.6em, below: 0pt, text(size: 9.5pt, if rest.len() > 0 [*#name:* #rest.join(": ")] else [#n]))
  }
  v(0.35em)
  line(length: 100%, stroke: 0.4pt + luma(150))
})

#let look(b) = {
  // The Progression section prints its block untitled under the section title; a heading given prints.
  if b.heading != "" { block(sticky: true, below: 0.5em, text(size: 12pt, weight: "bold", b.heading)) }
  for r in b.rows { entry(r) }
  [#metadata((entries: b.rows.len())) <progression-look>]
}
