// The `square-block` look (SD-40 E2.9, decisions.md §23.1 item 3a, §23.3, SD-m): the screen's
// square group. Hit Points: the big current-HP area for the pencil (3 cm tall, at least 3 cm wide
// on Letter), the max the engine holds and the small nonlethal box, Damage Reduction when the
// screen shows one; Initiative: the total; Conditions: a write-in box holding the saved
// conditions; Encumbrance: the limits, carried, remaining, gear value and the load's name. Then the
// hit point, initiative and encumbrance notes, each list followed by two ruled lines. It formats
// strings it is given; it computes nothing (NR-2).
//
// The rows, as `print/blocks/core.rs` `square_block` sends them, by label: Max (its notes are the
// hit point notes), Current, Nonlethal, Damage Reduction (when there is one), Initiative (its notes
// are the initiative notes), Encumbrance (light, medium, heavy, carried, remaining, gear value,
// load), Conditions.
//
// Page 1's overflow rule (E2.4, decisions.md §23.2 SD-b): the hit point and initiative notes are
// the list the frame may cut (`list`, `more`: the rest and the ruled lines continue on page 2); a
// `continued` block prints the rest of the notes lists and their ruled lines under its heading.

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

// A ruled line the player writes on.
#let write-line = box(width: 100%, height: 0.4cm, stroke: (bottom: 0.5pt + luma(60)))

// One notes list: its caption, each note, then two ruled lines (none when the list continues on
// page 2: they print there).
#let notes-area(label, notes, more: false) = stack(
  spacing: 0.08cm,
  caption(label),
  ..notes.map(n => text(size: 8pt, fill: luma(40), n)),
  ..if not more { (write-line, write-line) },
)

// The list the overflow rule may split: each hit point note, then each initiative note, as (its
// list's caption, the note).
#let items(b) = {
  let notes(label) = { let r = b.rows.find(r => r.label == label); if r == none { () } else { r.notes } }
  notes("Max").map(n => ("Hit point notes", n)) + notes("Initiative").map(n => ("Initiative notes", n))
}

#let look(b) = {
  let row(label) = b.rows.find(r => r.label == label)
  let value(label) = { let r = row(label); if r == none { "" } else { r.values.at(0) } }
  let list = b.at("list", default: items(b))
  let notes(kind) = list.filter(e => e.at(0) == kind).map(e => e.at(1))
  let more = b.at("more", default: false)
  let load = { let r = row("Encumbrance"); if r == none { ("",) * 7 } else { r.values } }
  let reduction = row("Damage Reduction")
  // A row this look has no place for prints its label and values under the block.
  let known = ("Max", "Current", "Nonlethal", "Damage Reduction", "Initiative", "Encumbrance", "Conditions")
  let others = b.rows.filter(r => r.label not in known)
  // Three columns: Hit Points (the current-HP area, 3 cm square, SD-m, with the max and nonlethal
  // boxes beside it), Initiative over Conditions, Encumbrance; under each, its notes list.
  let columns = (4.5cm, 2.4cm, 1fr)
  let notes-grid = grid(
    columns: columns,
    column-gutter: 0.25cm,
    notes-area("Hit point notes", notes("Hit point notes"), more: more),
    notes-area("Initiative notes", notes("Initiative notes"), more: more),
    notes-area("Encumbrance notes", (), more: more),
  )
  if b.at("continued", default: false) {
    return block(breakable: false, width: 100%, above: 1em, below: 0.6em, title(b.heading) + notes-grid)
  }
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
        ..if reduction != none { (text(size: 8.5pt)[*Damage Reduction* #reduction.values.at(0)],) },
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
    #v(0.1cm)
    #notes-grid
    #if more { v(0.1cm); text(size: 8pt, fill: luma(60))[continued on page 2] }
    #for r in others {
      v(0.1cm)
      text(size: 8.5pt, weight: "bold", r.label)
      for v in r.values.filter(v => v != "") { h(0.4em); text(size: 8.5pt, v) }
    }
  ]
}
