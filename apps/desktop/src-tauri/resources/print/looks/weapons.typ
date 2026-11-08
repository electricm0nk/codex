// The `weapons` look (SD-40 E2.3): one table of the weapons in hand. The first row names the
// columns; each weapon prints its name with its critical on one line, then its wield and a row of
// boxes (attack, damage dice, Str, enhancement, damage bonus, threat), then its feat notes. Two
// blank lines follow for the pencil (a rule for the name, the six boxes), then the ammunition
// write-in. Any other row (a per-weapon engine
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

// The columns every line shares: the wield (or, on a blank line, the name the player writes), then
// the six boxes.
#let columns = (2.2cm,) + (1fr,) * boxes

#let box-row(first, values) = grid(
  columns: columns,
  column-gutter: 0.1cm,
  first,
  ..values.map(value-box),
)

#let look(b) = {
  let header = b.rows.at(0, default: (label: "", values: (), notes: ()))
  let is-weapon(r) = r.values.len() == 8 and r.values.at(1) != ""
  let rest = if b.rows.len() > 1 { b.rows.slice(1) } else { () }
  let weapons = rest.filter(is-weapon)
  let others = rest.filter(r => not is-weapon(r))
  let shown = calc.min(boxes, header.values.len())
  block(breakable: false, width: 100%, above: 1em, below: 0.6em)[
    #heading-of(b)
    #stack(
      spacing: 0.12cm,
      // The captions, laid out as each weapon is: the weapon and its critical, then the wield and
      // the boxes' columns.
      grid(
        columns: (1fr, auto),
        caption(header.label),
        caption(header.values.at(6, default: "")),
      ),
      grid(
        columns: columns,
        column-gutter: 0.1cm,
        align(left + bottom, caption(header.values.at(7, default: ""))),
        ..header.values.slice(0, shown).map(c => align(center + bottom, caption(c))),
      ),
      ..weapons.map(r => stack(
        spacing: 0.06cm,
        grid(
          columns: (1fr, auto),
          column-gutter: 0.2cm,
          align(left + bottom, text(size: 9pt, weight: "bold", r.label)),
          align(right + bottom, text(size: 8pt, r.values.at(6))),
        ),
        box-row(align(left + horizon, text(size: 8pt, r.values.at(7))), r.values.slice(0, boxes)),
        ..r.notes.map(n => text(size: 8pt, fill: luma(40), n)),
      )),
      // Two blank weapons, for what the character picks up at the table: the name on the rule.
      ..range(2).map(_ => box-row(write-line, ("",) * boxes)),
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
