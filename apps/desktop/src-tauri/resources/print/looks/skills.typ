// The `skills` look (SD-40 E2.2; E2.11 one column, totals only, `decisions.md §23.1` item 3c): the
// skills in ONE column, each row on one line: the name and its signed total, nothing else (the
// breakdown prints on the Show-the-math pages). A Knowledge name shortens to `Know. (<Subject>)`
// only when the full name, measured at the row's font, is wider than its cell; every other name
// prints as the block sends it. Any other row (the ranks with no panel total) prints its label and
// its notes under the column; what a feat states and no total adds prints in the Conditional
// Bonuses box (E2.20). It formats; it adds nothing up (NR-2).
//
// A skill row's values, as `print/blocks/skills.rs` sends them: 0 the signed total.
//
// Page 1's overflow rule (E2.4, decisions.md §23.2 SD-b): the skill lines, then the lines under the
// column, are the list the frame may shrink (`compact`: the lines under the column at 8 pt; the
// names are at the floor already) and cut from its end (`list`, `more`: the rest continues on page
// 2); a `continued` block prints the rest the same way under its heading.

#let heading-of(b) = {
  text(size: 12pt, weight: "bold", b.heading)
  v(-0.4em)
  line(length: 100%, stroke: 0.6pt + luma(60))
  v(-0.2em)
}

#let caption(t) = text(size: 8pt, fill: luma(60), t)

#let name-size = 8pt
#let total-width = 0.95cm
#let gutter = 0cm

// A skill line has its total; `SheetBlock::new` pads every row to one value, so a line of words alone
// (the ranks with no panel total) arrives with one empty value and is not a skill line (E3.MC).
#let is-skill(r) = r.values.at(0, default: "") != ""

// The name as it fits `width`: a Knowledge name too wide for it prints as `Know. (<Subject>)`.
#let fitted(name, width) = {
  let full = text(size: name-size, name)
  if name.starts-with("Knowledge (") and measure(full).width > width {
    text(size: name-size, "Know." + name.slice("Knowledge".len()))
  } else {
    full
  }
}

// One ruled line per skill: the rule under the row carries the eye from the name to its total.
#let column-of(rows) = layout(size => grid(
  columns: (1fr, total-width),
  column-gutter: gutter,
  rows: (0.32cm,) * rows.len() + (auto,),
  align: (x, y) => if x == 0 { left + horizon } else { right + horizon },
  stroke: (x, y) => if y < rows.len() { (bottom: 0.3pt + luma(170)) },
  ..rows.map(r => (
    fitted(r.label, size.width - total-width - gutter),
    text(size: name-size, weight: "bold", r.values.at(0)),
  )).flatten(),
  [], caption[Total],
))

// The list the overflow rule may split: each skill line, as `(row: <the row>)`; then each line under
// the column, as `(label: <its row's label>, note: <the note, or none for a row of words alone>)`.
#let items(b) = {
  let out = b.rows.filter(is-skill).map(r => (row: r))
  for r in b.rows.filter(r => not is-skill(r)) {
    if r.notes.len() == 0 { out.push((label: r.label, note: none)) } else { for n in r.notes { out.push((label: r.label, note: n)) } }
  }
  out
}

// The lines under the column: each row's label once, then its notes.
#let words-of(entries, size) = {
  let groups = ()
  for e in entries {
    if groups.len() == 0 or groups.last().label != e.label { groups.push((label: e.label, notes: ())) }
    if e.note != none { groups.last().notes.push(e.note) }
  }
  for g in groups {
    v(0.2em)
    text(size: size, weight: "bold", g.label)
    if g.notes.len() > 0 {
      v(-0.3em)
      stack(spacing: 0.45em, ..g.notes.map(n => text(size: size, fill: luma(40), n)))
    }
  }
}

// The skills print whole beside the Weapons, then the lines that are not a panel total.
#let look(b) = {
  let size = if b.at("compact", default: false) { 8pt } else { 8.5pt }
  let entries = b.at("list", default: items(b))
  let skills = entries.filter(e => "row" in e).map(e => e.row)
  block(breakable: false, width: 100%, above: 1em, below: 0.6em)[
    #heading-of(b)
    #if skills.len() > 0 or not b.at("continued", default: false) { column-of(skills) }
    #words-of(entries.filter(e => "note" in e), size)
    #if b.at("more", default: false) { v(0.2em); caption[continued on page 2] }
  ]
}
