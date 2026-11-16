// The `weapons` look (SD-40 E2.3; E2.10 totals only, decisions.md §23.1 item 4): one table of the
// weapons in hand. The first row names the columns; each weapon prints on one line its name, its
// attack total, its damage total (`1d8+4`), its critical and its range, then its feat notes. Only
// the engine's lines print: no blank write-in line and no empty box (SD-41 decisions.md §34 (5)).
// A weapon that fires ammunition (a row of kind `ammunition`, SD-41 decisions.md §23) prints 20 tick
// boxes under its line, after the words of its fifth value (`Arrow: 60 carried`, or none); the
// others print none, and there is no ammunition row for the whole block. Any other row (a per-weapon engine row no weapon in hand matches, "No weapon is
// wielded.") prints its label and values under the table. It formats strings it is given; it adds
// nothing up (NR-2).
//
// A weapon row's values, as `print/blocks/gear.rs` sends them: 0 attack, 1 damage, 2 critical,
// 3 range, and for a row of kind `ammunition` 4 the count's words.
//
// Page 1's overflow rule (E2.4, decisions.md §23.2 SD-b): the frame may mark the block `compact`
// (the lines at the 8 pt floor), then hand it a shorter `list` and `more` (the rest continues on
// page 2); the rest prints as a `continued` block of its own.

#import "/looks/section-title.typ": section-title

#let heading-of(b) = section-title(b.heading)

#let caption(t) = text(size: 8pt, fill: luma(60), t)

// A value the engine filled, in its box; smaller at the 8 pt floor (`compact`).
#let value-box(v, compact: false) = box(
  width: 100%, height: if compact { 0.45cm } else { 0.55cm }, stroke: 0.5pt + luma(60),
  align(center + horizon, text(size: if compact { 8pt } else { 8.5pt }, weight: "bold", v)),
)

#let boxes = 4

// The columns every line shares: the name, then the attack, damage, critical and range boxes.
#let columns = (1fr, 0.95cm, 1.4cm, 1.4cm, 1.1cm)

#let line-of(first, values, compact: false) = grid(
  columns: columns,
  column-gutter: 0.1cm,
  first,
  ..values.map(v => value-box(v, compact: compact)),
)

#let is-weapon(r) = r.values.len() >= boxes and r.values.at(1) != ""

// Tick boxes under a weapon that fires ammunition (decisions.md §23): the player marks what is shot.
#let ticks = 20

#let ammunition(r, compact: false) = grid(
  columns: (auto, 1fr),
  column-gutter: 0.15cm,
  align(bottom, text(size: if compact { 7.5pt } else { 8pt }, r.values.at(4, default: ""))),
  grid(
    columns: (1fr,) * ticks,
    column-gutter: 0.06cm,
    ..range(ticks).map(_ => box(width: 100%, height: 0.32cm, stroke: 0.5pt + luma(60))),
  ),
)

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
        ..if r.at("kind", default: "") == "ammunition" { (ammunition(r, compact: compact),) },
        ..r.notes.map(n => text(size: 8pt, fill: luma(40), n)),
      )),
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
    #metadata((
      ammunition: weapons.filter(r => r.at("kind", default: "") == "ammunition").len(),
      boxes: ticks,
      lines: weapons.len(),
      empty: weapons.map(r => r.values.slice(0, boxes).filter(v => v == "").len()).sum(default: 0),
    )) <weapons-look>
  ]
}
