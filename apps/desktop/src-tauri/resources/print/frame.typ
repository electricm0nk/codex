// The shared page frame (SD-39 operator ruling R2). Every system's sheet prints through it: the
// paper, the one bundled font family, the title, the page footer, then the layout file's blocks
// in order. The frame names no inch for any block; Typst flows them onto pages.
//
// `doc` is the `SheetDocument` the Rust layout file built (`src/print/model.rs`), passed as JSON.
// It carries text only: no number is computed here (NR-2).
//
// Layouts (SD-40 decisions.md §4): a sheet of `columns: 1` flows its blocks one under the other,
// as every sheet did before SD-40 (Starfinder and Cyberpunk RED still do). A sheet of more columns
// lays each run of blocks between page breaks on a page grid, each block in its cell at its
// `place` (first column `x`, `span` columns wide); the layout file fills each row in order.

#import "/blocks.typ": sheet-block, block-look

#let grid-run(columns, run) = grid(
  columns: (1fr,) * columns,
  column-gutter: 0.8em,
  row-gutter: 1em,
  ..run.map(b => grid.cell(x: b.place.x, colspan: b.place.span, block-look(b))),
)

#let sheet(doc) = {
  set document(title: doc.title, date: none)
  set page(
    paper: doc.paper,
    margin: (x: 1.6cm, y: 1.8cm),
    footer: context align(center, text(size: 8pt, fill: luma(90))[
      #doc.title · page #counter(page).display() of #counter(page).final().first()
    ]),
  )
  set text(font: "Liberation Sans", size: 10pt, lang: "en")
  set par(justify: false)

  align(left, text(size: 18pt, weight: "bold", doc.title))
  v(0.4em)
  if doc.columns == 1 {
    for b in doc.blocks {
      sheet-block(b)
    }
  } else {
    let run = ()
    for b in doc.blocks {
      if b.new_page and run.len() > 0 {
        grid-run(doc.columns, run)
        pagebreak(weak: true)
        run = ()
      }
      run.push(b)
    }
    if run.len() > 0 {
      grid-run(doc.columns, run)
    }
  }
}
