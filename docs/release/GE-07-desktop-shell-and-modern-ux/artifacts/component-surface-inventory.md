# Component Surface Inventory

## Objective
Enumerate the required GE-07 surfaces and the data truth each one must honor.

| Surface | Primary job | Required backing truth | Explicit non-goal |
|---|---|---|---|
| Shell frame / navigation | Provide stable orientation and route access | current route, active pilot context, visibility of diagnostics/problem state | becoming a generic product shell before pilot proof |
| Pilot character workspace | Present the active pilot character state | real GE-06/GE-04 character snapshot data | computing rules answers locally |
| Explanation surface | Show why a value or invalid choice is what it is | upstream explanation payloads and provenance context | inventing explanation logic in the frontend |
| Validation / problems panel | Show current problems and failures | upstream validation/problem payloads | hiding failures for cleanliness |
| Import diagnostics surface | Show importer warnings, unsupported semantics, and provenance clues | upstream GE-03 diagnostic payloads | treating import issues as irrelevant implementation noise |
| Rules library pilot view | Inspect bounded rules objects relevant to the pilot | upstream rules object/read-model data | becoming full breadth content browsing before pilot proof |
| Source package view | Inspect package/source lineage and context | upstream package/provenance payloads | replacing structured provenance with hand-written summaries |

## Completion rule
This artifact is satisfied when later UI slices can be bounded by surface rather than by vague phrases like “work on the UX.”
