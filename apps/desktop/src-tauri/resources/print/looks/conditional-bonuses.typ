// The `conditional-bonuses` look (SD-40 E2.20, decisions.md §24.3.5, operator Q1 §24.6): the heading
// "Conditional Bonuses" over an outlined box of fixed height, 10 ruled lines of 0.4 cm (the write-in
// line of the other page-1 looks; SD-m's 0.5 cm leaves Letter's page 1 no room for three weapons). Each
// conditional the engine states takes the lines its words need, `<what>: <the engine's words>`;
// the lines left are blank for the pencil, at least 4 always. It formats strings it is given; it
// computes nothing (NR-2).
//
// The rows, as `print/blocks/core.rs` `conditional_bonuses` sends them: label = what the bonus is
// to (HP, Initiative, Saves, Fortitude, Reflex, Will, AC, CMB, CMD, Skills); value 0 = the words.
//
// Page 1's overflow rule (SD-b, decisions.md §23.2): `fit(b, width)` (the frame calls it with the
// box's cell width) keeps the rows that fill the box's written lines, first at 8.5 pt, else at the
// 8 pt floor (`compact`); the rest (`rest`) continues at the top of page 2 under "Conditional
// Bonuses (continued)" (`continued`: the rows, no box). The 4 blank lines stay on page 1.

#let lines = 10
#let blank-min = 4
#let slot = 0.4cm
#let inset-x = 4pt

#let heading-of(b) = {
  text(size: 12pt, weight: "bold", b.heading)
  v(-0.4em)
  line(length: 100%, stroke: 0.6pt + luma(60))
  v(-0.2em)
}

// The list the overflow rule may split: the rows, in order.
#let items(b) = b.rows

#let words(r, size) = text(size: size)[*#r.label:* #r.values.at(0)]

// How many lines `r`'s words take at `size` in a box `width` wide.
#let slots(r, width, size) = calc.max(1, calc.ceil(measure(block(width: width - 2 * inset-x, words(r, size))).height / slot))

// How many of `rows`, from the first, fill no more than the box's written lines.
#let fitting(rows, width, size) = {
  let used = 0
  let n = 0
  for r in rows {
    used += slots(r, width, size)
    if used > lines - blank-min { break }
    n += 1
  }
  n
}

// The frame's overflow step: the rows the box keeps (`list`), at 8 pt when 8.5 pt cannot hold them
// all, and the rest (`rest`) for page 2. Called in context (it measures).
#let fit(b, width) = {
  let rows = items(b)
  let n = fitting(rows, width, 8.5pt)
  let compact = n < rows.len()
  if compact { n = fitting(rows, width, 8pt) }
  b + (list: rows.slice(0, n), compact: compact, more: n < rows.len(), rest: rows.slice(n))
}

#let look(b) = {
  let size = if b.at("compact", default: false) { 8pt } else { 8.5pt }
  let rows = b.at("list", default: items(b))
  if b.at("continued", default: false) {
    return block(breakable: false, width: 100%, above: 1em, below: 0.6em)[
      #heading-of(b)
      #v(0.15em)
      #stack(spacing: 0.35em, ..rows.map(r => words(r, size)))
    ]
  }
  block(breakable: false, width: 100%, above: 1em, below: 0.6em)[
    #heading-of(b)
    #v(0.15em)
    #layout(size-of => {
      let width = size-of.width
      let used = rows.map(r => slots(r, width, size))
      // Outside the frame's fit (a one-column sheet), every row prints and the box grows to keep
      // its blank lines; on page 1 the fit leaves the box its 10 lines.
      let total = calc.max(lines, used.sum(default: 0) + blank-min)
      let blank = total - used.sum(default: 0)
      let ruled(n, body) = box(width: 100%, height: n * slot, stroke: (bottom: 0.4pt + luma(170)), inset: (x: inset-x), align(left + horizon, body))
      let body = box(
        width: 100%,
        stroke: 0.6pt + luma(60),
        stack(..rows.zip(used).map(((r, n)) => ruled(n, words(r, size))), ..range(blank).map(_ => ruled(1, []))),
      )
      body
      let height = measure(body, width: width).height
      [#metadata((lines: total, blank: blank, height: height / 1pt)) <conditional-bonuses-box>]
    })
    // When the list continues on page 2, a caption saying so on the heading's line: it takes no
    // height from the column, and its words follow the box's.
    #if b.at("more", default: false) { place(top + right, dy: 0.3em, text(size: 8pt, fill: luma(60))[continued on page 2]) }
  ]
}
