// The `armor-class` look (SD-40 E2.1): Armor Class, Touch and Flat-Footed as three boxes, then
// a strip of the AC's component cells (armor, shield, Dex, size, natural, deflection, misc). A
// component the engine gives no row for is a ruled write-in. It formats; it adds nothing up (NR-2).

#let heading-of(b) = {
  text(size: 12pt, weight: "bold", b.heading)
  v(-0.4em)
  line(length: 100%, stroke: 0.6pt + luma(60))
  v(-0.2em)
}

// A value the engine filled, or (an empty string) a ruled write-in box the player fills.
#let value-box(v, width: 100%, height: 0.75cm, size: 11pt) = box(
  width: width, height: height, stroke: 0.6pt + luma(60),
  align(center + horizon, text(size: size, weight: "bold", v)),
)

#let caption(t) = text(size: 8pt, fill: luma(60), t)

// A row's notes (a rule's words, a condition), under the boxes.
#let notes-of(b) = {
  let notes = b.rows.map(r => r.notes).flatten()
  if notes.len() > 0 {
    v(0.2em)
    stack(spacing: 0.45em, ..notes.map(n => text(size: 8.5pt, fill: luma(40), n)))
  }
}

// One labelled box: the label above (so it reads before its value), the box under it.
#let labelled(r, height: 0.75cm, size: 11pt) = stack(
  spacing: 0.12cm,
  text(size: 8pt, weight: "bold", r.label),
  ..r.values.map(v => value-box(v, height: height, size: size)),
)

// The first three rows are the totals; the rest are the totals' parts.
#let look(b) = {
  let cut = if b.rows.len() < 3 { b.rows.len() } else { 3 }
  let totals = b.rows.slice(0, cut)
  let parts = b.rows.slice(cut)
  block(breakable: false, width: 100%, above: 1em, below: 0.6em)[
    #heading-of(b)
    #grid(
      columns: (1fr,) * (if totals.len() > 0 { totals.len() } else { 1 }),
      column-gutter: 0.25cm,
      ..totals.map(r => labelled(r, height: 1.1cm, size: 15pt)),
    )
    #if parts.len() > 0 {
      v(0.15cm)
      // Each part's cell as wide as its word asks (a few letters' room more), so "Deflection"
      // keeps clear of "Misc" beside the Abilities on page 1.
      grid(
        columns: parts.map(r => (r.label.len() + 4) * 1fr),
        column-gutter: 0.12cm,
        ..parts.map(r => labelled(r, height: 0.75cm, size: 10pt)),
      )
    }
    #notes-of(b)
  ]
}
