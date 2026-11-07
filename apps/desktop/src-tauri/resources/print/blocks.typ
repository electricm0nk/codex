// The look dispatcher (SD-40 decisions.md §4, operator ruling R1): each block prints in the look
// it names, one Typst template per look under /looks/ (`typst.rs`'s `LOOKS`). `table` is the
// generic table every block printed in before SD-40. A block marked `new_page` starts a page of
// its own.

#import "/looks/table.typ": look as table-look

#let looks = (
  table: table-look,
)

// One block in its own look, without the page break: what a grid cell holds.
#let block-look(b) = looks.at(b.look)(b)

#let sheet-block(b) = {
  if b.new_page { pagebreak(weak: true) }
  block-look(b)
}
