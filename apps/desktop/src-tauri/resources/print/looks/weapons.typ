// The `weapons` look (SD-40 E2.3): one table of the weapons in hand. The first row names the
// columns; each weapon prints its name with its critical and wield on one line, then a row of boxes
// (attack, damage dice, Str, enhancement, damage bonus, threat), then its feat notes. Two blank
// weapons follow for the pencil, then the ammunition write-in. Any other row (a per-weapon engine
// row no weapon in hand matches, "No weapon is wielded.") prints its label and values under the
// table. It formats strings it is given; it adds nothing up (NR-2).
//
// A weapon row's values, as `print/blocks/gear.rs` sends them: 0 attack, 1 damage dice, 2 Str,
// 3 enhancement, 4 damage bonus, 5 threat, 6 critical, 7 wield.

#let heading-of(b) = {
  text(size: 12pt, weight: "bold", b.heading)
  v(-0.4em)
  line(length: 100%, stroke: 0.6pt + luma(60))
  v(-0.2em)
}

#let caption(t) = text(size: 8pt, fill: luma(60), t)

// A value the engine filled, or (an empty string) a ruled write-in box the player fills.
#let value-box(v) = box(
  width: 100%, height: 0.5cm, stroke: 0.5pt + luma(60),
  align(center + horizon, text(size: 9pt, weight: "bold", v)),
)

// A ruled line the player writes on.
#let write-line = box(width: 100%, height: 0.42cm, stroke: (bottom: 0.5pt + luma(60)))

#let boxes = 6

#let name-line(name, critical, wield) = grid(
  columns: (1fr, auto),
  column-gutter: 0.2cm,
  align(left + bottom, text(size: 9pt, weight: "bold", name)),
  align(right + bottom, text(size: 8pt, critical + " · " + wield)),
)

#let box-row(values) = grid(
  columns: (1fr,) * boxes,
  column-gutter: 0.1cm,
  ..values.map(value-box),
)

#let look(b) = {
  let header = b.rows.at(0, default: (label: "", values: (), notes: ()))
  let is-weapon(r) = r.values.len() == 8 and r.values.at(1) != ""
  let rest = if b.rows.len() > 1 { b.rows.slice(1) } else { () }
  let weapons = rest.filter(is-weapon)
  let others = rest.filter(r => not is-weapon(r))
  block(breakable: false, width: 100%, above: 1em, below: 0.6em)[
    #heading-of(b)
    #stack(
      spacing: 0.12cm,
      grid(
        columns: (1fr, auto),
        caption(header.label),
        caption(if header.values.len() > boxes { header.values.slice(boxes).join(" · ") } else { "" }),
      ),
      grid(
        columns: (1fr,) * boxes,
        column-gutter: 0.1cm,
        ..header.values.slice(0, calc.min(boxes, header.values.len())).map(c => align(center, caption(c))),
      ),
      ..weapons.map(r => {
        stack(
          spacing: 0.08cm,
          name-line(r.label, r.values.at(6), r.values.at(7)),
          box-row(r.values.slice(0, boxes)),
          ..r.notes.map(n => text(size: 8pt, fill: luma(40), n)),
        )
      }),
      // Two blank weapons, for what the character picks up at the table.
      ..range(2).map(_ => stack(spacing: 0.08cm, write-line, box-row(("",) * boxes))),
      v(0.05cm),
      grid(
        columns: (auto, 1fr),
        column-gutter: 0.15cm,
        align(bottom, text(size: 8pt, weight: "bold", "Ammunition")),
        grid(
          columns: (1fr,) * 10,
          column-gutter: 0.06cm,
          ..range(10).map(_ => box(width: 100%, height: 0.32cm, stroke: 0.5pt + luma(60))),
        ),
      ),
      ..others.map(r => {
        text(size: 8.5pt, weight: "bold", r.label)
        for v in r.values.filter(v => v != "") { h(0.4em); text(size: 8.5pt, v) }
        if r.notes.len() > 0 {
          linebreak()
          stack(spacing: 0.45em, ..r.notes.map(n => text(size: 8pt, fill: luma(40), n)))
        }
      }),
    )
  ]
}
