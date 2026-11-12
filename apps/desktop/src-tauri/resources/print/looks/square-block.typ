// The `square-block` look (SD-40 E2.9, decisions.md §23.1 item 3a, §23.3, SD-m): the screen's
// square group. Hit Points: the big current-HP area for the pencil (3 cm tall, at least 3 cm wide
// on Letter), the max the engine holds and the small nonlethal box (Damage Reduction prints in
// the Resistances block, E2.22); Initiative: the total; Conditions: a write-in box holding the saved
// conditions; Encumbrance: the limits, carried, remaining, gear value and the load's name. The
// hit point and initiative notes print in the Conditional Bonuses box (E2.20, decisions.md
// §24.3.4). It formats strings it is given; it computes nothing (NR-2).
//
// The rows, as `print/blocks/core.rs` `square_block` sends them, by label: Max, Current,
// Nonlethal, Initiative, Encumbrance (light, medium, heavy,
// carried, remaining, gear value, load), Conditions.

#let title(t) = {
  text(size: 12pt, weight: "bold", t)
  v(-0.4em)
  line(length: 100%, stroke: 0.6pt + luma(60))
  v(0.05cm)
}

#let caption(t) = text(size: 8pt, weight: "bold", t)

#let value-box(v, height: 0.8cm, size: 14pt) = box(
  width: 100%, height: height, stroke: 0.6pt + luma(60),
  align(center + horizon, text(size: size, weight: "bold", v)),
)

#let labelled(label, body) = stack(spacing: 0.1cm, caption(label), body)

#let look(b) = {
  let row(label) = b.rows.find(r => r.label == label)
  let value(label) = { let r = row(label); if r == none { "" } else { r.values.at(0) } }
  let load = { let r = row("Encumbrance"); if r == none { ("",) * 7 } else { r.values } }
  // A row this look has no place for prints its label and values under the block.
  let known = ("Max", "Current", "Nonlethal", "Initiative", "Encumbrance", "Conditions")
  let others = b.rows.filter(r => r.label not in known)
  // Three columns: Hit Points (the current-HP area, 3 cm square, SD-m, with the max and nonlethal
  // boxes beside it), Initiative over Conditions, Encumbrance.
  let columns = (4.5cm, 2.4cm, 1fr)
  block(breakable: false, width: 100%, above: 1em, below: 0.6em)[
    #grid(
      columns: columns,
      column-gutter: 0.25cm,
      stack(
        spacing: 0.1cm,
        title(b.heading),
        grid(
          columns: (3cm, 1fr),
          column-gutter: 0.15cm,
          labelled("Current", value-box(value("Current"), height: 3cm)),
          stack(
            spacing: 0.2cm,
            labelled("Max", value-box(value("Max"))),
            labelled("Nonlethal", value-box(value("Nonlethal"))),
          ),
        ),
      ),
      stack(
        spacing: 0.1cm,
        title("Initiative"),
        value-box(value("Initiative"), height: 0.9cm, size: 16pt),
        v(0.2cm),
        title("Conditions"),
        box(width: 100%, height: 1.3cm, stroke: 0.6pt + luma(60), inset: 4pt,
          align(left + top, text(size: 9pt, value("Conditions")))),
      ),
      stack(
        spacing: 0.1cm,
        title("Encumbrance"),
        grid(
          columns: (auto, 1fr),
          column-gutter: 0.15cm,
          row-gutter: 0.2cm,
          ..(("Light", 0), ("Medium", 1), ("Heavy", 2), ("Carried", 3), ("Remaining", 4), ("Gear value", 5))
            .map(((label, i)) => (caption(label), align(right, text(size: 8.5pt, load.at(i)))))
            .flatten(),
        ),
        // The load's name on a line of its own: it is words, wider than a number.
        grid(columns: (auto, 1fr), column-gutter: 0.15cm, caption("Load"), align(right, text(size: 8.5pt, weight: "bold", load.at(6)))),
      ),
    )
    #for r in others {
      v(0.1cm)
      text(size: 8.5pt, weight: "bold", r.label)
      for v in r.values.filter(v => v != "") { h(0.4em); text(size: 8.5pt, v) }
    }
  ]
}
