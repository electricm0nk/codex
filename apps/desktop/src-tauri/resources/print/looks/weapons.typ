// The `weapons` look (SD-40 E2.3; E2.10 totals only, decisions.md §23.1 item 4): one table of the
// weapons in hand. The first row names the columns; each weapon prints on one line its name, its
// attack total, its damage total (`1d8+4`), its critical and its range, then its feat notes. Two
// blank lines follow for the pencil (a rule for the name, the four boxes), then the ammunition
// write-in. Any other row (a per-weapon engine row no weapon in hand matches, "No weapon is
// wielded.") prints its label and values under the table. It formats strings it is given; it adds
// nothing up (NR-2).
//
// A weapon row's values, as `print/blocks/gear.rs` sends them: 0 attack, 1 damage, 2 critical,
// 3 range.
//
// Page 1's overflow rule (E2.4, decisions.md §23.2 SD-b): the frame may mark the block `compact`
// (the lines at the 8 pt floor), then hand it a shorter `list` and `more` (the rest continues on
// page 2: the write-in lines go with it); the rest prints as a `continued` block of its own.

#let heading-of(b) = {
  text(size: 12pt, weight: "bold", b.heading)
  v(-0.4em)
  line(length: 100%, stroke: 0.6pt + luma(60))
  v(-0.2em)
}

#let caption(t) = text(size: 8pt, fill: luma(60), t)

// A value the engine filled, or (an empty string) a ruled write-in box the player fills; smaller
// at the 8 pt floor (`compact`).
#let value-box(v, compact: false) = box(
  width: 100%, height: if compact { 0.45cm } else { 0.55cm }, stroke: 0.5pt + luma(60),
  align(center + horizon, text(size: if compact { 8pt } else { 8.5pt }, weight: "bold", v)),
)

// A ruled line the player writes on.
#let write-line = box(width: 100%, height: 0.45cm, stroke: (bottom: 0.5pt + luma(60)))

#let boxes = 4

// The columns every line shares: the name, then the attack, damage, critical and range boxes.
#let columns = (1fr, 0.95cm, 1.4cm, 1.4cm, 1.1cm)

#let line-of(first, values, compact: false) = grid(
  columns: columns,
  column-gutter: 0.1cm,
  first,
  ..values.map(v => value-box(v, compact: compact)),
)

#let is-weapon(r) = r.values.len() == boxes and r.values.at(1) != ""

// The list the overflow rule may split: every row under the captions, weapons first.
#let items(b) = {
  let rest = if b.rows.len() > 1 { b.rows.slice(1) } else { () }
  rest.filter(is-weapon) + rest.filter(r => not is-weapon(r))
}

#let look(b) = {
  let header = b.rows.at(0, default: (label: "", values: (), notes: ()))
  let rest = b.at("list", default: items(b))
  let weapons = rest.filter(is-weapon)
  let others = rest.filter(r => not is-weapon(r))
  let compact = b.at("compact", default: false)
  let more = b.at("more", default: false)
  let size = if compact { 8pt } else { 9pt }
  block(breakable: false, width: 100%, above: 1em, below: 0.6em)[
    #heading-of(b)
    #stack(
      spacing: if compact { 0.08cm } else { 0.12cm },
      grid(
        columns: columns,
        column-gutter: 0.1cm,
        align(left + bottom, caption(header.label)),
        ..header.values.slice(0, calc.min(boxes, header.values.len())).map(c => align(center + bottom, caption(c))),
      ),
      ..weapons.map(r => stack(
        spacing: 0.06cm,
        line-of(align(left + horizon, text(size: size, weight: "bold", r.label)), r.values.slice(0, boxes), compact: compact),
        ..r.notes.map(n => text(size: 8pt, fill: luma(40), n)),
      )),
      // Two blank weapons, for what the character picks up at the table (the name on the rule), and
      // the ammunition; when the list continues on page 2, they print there, after the last line.
      ..if not more { (
        ..range(2).map(_ => line-of(write-line, ("",) * boxes, compact: compact)),
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
      ) },
      ..others.map(r => {
        text(size: if compact { 8pt } else { 8.5pt }, weight: "bold", r.label)
        for v in r.values.filter(v => v != "") { h(0.4em); text(size: if compact { 8pt } else { 8.5pt }, v) }
        if r.notes.len() > 0 {
          linebreak()
          stack(spacing: 0.45em, ..r.notes.map(n => text(size: 8pt, fill: luma(40), n)))
        }
      }),
      ..if more { (caption[continued on page 2],) },
    )
  ]
}
