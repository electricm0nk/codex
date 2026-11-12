// The shared page frame (SD-39 operator ruling R2). Every system's sheet prints through it: the
// paper, the one bundled font family, the title (unless page 1 holds the Character Name field), the page footer, then the layout file's blocks
// in order. The frame names no inch for any block; Typst flows them onto pages.
//
// `doc` is the `SheetDocument` the Rust layout file built (`src/print/model.rs`), passed as JSON.
// It carries text only: no number is computed here (NR-2).
//
// Layouts (SD-40 decisions.md §4): a sheet of `columns: 1` flows its blocks one under the other,
// as every sheet did before SD-40 (Starfinder and Cyberpunk RED still do). A sheet of more columns
// lays each run of blocks between page breaks on a page grid, each block in its cell at its
// `place` (first column `x`, `span` columns wide, `row_span` rows tall); the layout file fills each
// row in order.

#import "/blocks.typ": sheet-block, block-look, lists, boxes

// A block marked `under` shares the cell of the block before it, stacked beneath it, so a column
// of short blocks can sit beside a tall one.
#let grid-run(columns, run) = {
  let cells = ()
  for b in run {
    if b.place.under and cells.len() > 0 {
      let cell = cells.pop()
      cell.push(b)
      cells.push(cell)
    } else {
      cells.push((b,))
    }
  }
  grid(
    columns: (1fr,) * columns,
    column-gutter: 0.8em,
    row-gutter: 1em,
    ..cells.map(cell => grid.cell(
      x: cell.first().place.x,
      colspan: cell.first().place.span,
      // A cell more than one row tall (page 1's portrait beside the name row and the AC row, E2.18).
      rowspan: cell.first().place.at("row_span", default: 1),
      // Blocks stacked in one cell sit 0.7em apart: page 1's left column holds seven (E2.20).
      if cell.len() == 1 { block-look(cell.first()) } else { stack(spacing: 0.7em, ..cell.map(block-look)) },
    )),
  )
}

// The page's margins: the frame sets them and page 1's fit (below) measures against them.
#let margin = (x: 1.6cm, y: 1.8cm)

// Page 1's overflow rule (SD-40 E2.4, decisions.md §23.2 SD-b). When page 1's run is taller than
// the room page 1 has, the lists on it (`lists`: weapon lines, notes, the lines under the skills)
// first shrink to the 8 pt floor (`compact`); if the run is still too tall, the lists give up their
// last lines one at a time (the skills' lines from the foot of the column, then the weapon lines;
// only where it lowers the run) until it fits. A box of fixed height (`boxes`: the Conditional
// Bonuses, E2.20) first keeps what fills its own lines, whatever room page 1 has. What a list or a
// box gave up prints at the top of page 2 under "<heading> (continued)", in block order, each as
// wide as its block.
// No page-1 block leaves page 1 and no line is dropped. Heights are measured, never a rules
// number (NR-2).
#let cut-order = ("skills", "weapons")

// A block's cell width on a `columns`-column grid `width` wide (the grid's gutter, below).
#let cell-width(b, columns, width) = {
  let gutter = 0.8em.to-absolute()
  let column = (width - (columns - 1) * gutter) / columns
  column * b.place.span + gutter * (b.place.span - 1)
}

#let grid-runs(columns, runs) = context {
  let width = page.width - 2 * margin.x
  let room = page.height - margin.y - here().position().y
  let tall(run) = measure(grid-run(columns, run), width: width).height
  let first = runs.first().map(b => if b.look in boxes { (boxes.at(b.look))(b, cell-width(b, columns, width)) } else { b })
  let rest = runs.slice(1)
  let splits(b) = b.look in lists and b.look in cut-order
  if tall(first) > room {
    first = first.map(b => if splits(b) { b + (compact: true) } else { b })
  }
  // How many lines of each list page 1 keeps (none: the block has no list).
  let keep = first.map(b => if splits(b) { (lists.at(b.look))(b).len() } else { none })
  let cut(keep) = first.enumerate().map(((i, b)) => {
    if keep.at(i) == none { return b }
    let items = (lists.at(b.look))(b)
    if keep.at(i) == items.len() { b } else { b + (list: items.slice(0, keep.at(i)), more: true) }
  })
  let height = tall(first)
  while height > room {
    // The lists that still have a line to give, lowest priority first.
    let open = range(first.len())
      .filter(i => keep.at(i) != none and keep.at(i) > 0)
      .sorted(key: i => cut-order.position(l => l == first.at(i).look))
    if open.len() == 0 { break }
    // The first one whose last line lowers the run; if none does alone (two columns equally tall,
    // or the "continued" caption outweighs the line), the first one.
    let pick = open.first()
    let pick-height = none
    for i in open {
      let trial = keep
      trial.at(i) -= 1
      let h = tall(cut(trial))
      if h < height {
        pick = i
        pick-height = h
        break
      }
    }
    keep.at(pick) -= 1
    height = if pick-height != none { pick-height } else { tall(cut(keep)) }
  }
  let continued = ()
  for (i, b) in first.enumerate() {
    if keep.at(i) == none and b.at("rest", default: ()).len() > 0 {
      let under = b.place.under and continued.len() > 0 and continued.last().place.span == b.place.span
      continued.push(b + (heading: b.heading + " (continued)", list: b.rest, more: false, continued: true, place: (x: 0, span: b.place.span, under: under)))
    }
    if keep.at(i) == none { continue }
    let items = (lists.at(b.look))(b)
    if keep.at(i) < items.len() {
      // Each continuation keeps its block's width, at the left edge on a row of its own (a grid
      // row fills left to right, so a cell is never placed over another), stacked as on page 1
      // when the block it sat under continues too.
      let under = b.place.under and continued.len() > 0 and continued.last().place.span == b.place.span
      continued.push(b + (
        heading: b.heading + " (continued)",
        list: items.slice(keep.at(i)),
        continued: true,
        place: (x: 0, span: b.place.span, under: under),
      ))
    }
  }
  grid-run(columns, cut(keep))
  if continued.len() > 0 {
    if rest.len() > 0 { rest.at(0) = continued + rest.at(0) } else { rest.push(continued) }
  }
  for run in rest {
    pagebreak(weak: true)
    grid-run(columns, run)
  }
}

#let sheet(doc) = {
  set document(title: doc.title, date: none)
  set page(
    paper: doc.paper,
    margin: margin,
    footer: context align(center, text(size: 8pt, fill: luma(90))[
      #doc.title · page #counter(page).display() of #counter(page).final().first()
    ]),
  )
  set text(font: "Liberation Sans", size: 10pt, lang: "en")
  set par(justify: false)

  // A sheet whose page 1 opens with the Character Name field (Pathfinder, SD-40 E2.5) prints the
  // name there, once; every other sheet opens with the name as its title.
  if not doc.blocks.any(b => b.id == "nameCampaign") {
    align(left, text(size: 18pt, weight: "bold", doc.title))
    v(0.4em)
  }
  if doc.columns == 1 {
    for b in doc.blocks {
      sheet-block(b)
    }
  } else {
    // The runs of blocks between page breaks; page 1's run is fitted to its page.
    let runs = ()
    let run = ()
    for b in doc.blocks {
      if b.new_page and run.len() > 0 {
        runs.push(run)
        run = ()
      }
      run.push(b)
    }
    if run.len() > 0 {
      runs.push(run)
    }
    grid-runs(doc.columns, runs)
  }
}
