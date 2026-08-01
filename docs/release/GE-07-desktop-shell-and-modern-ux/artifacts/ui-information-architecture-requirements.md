# UI Information Architecture Requirements

## Objective
Define how the first GE-07 shell organizes information so users can move between current character state, explanations, diagnostics, rules, and source-package context without losing meaning.

## Primary IA nodes
1. Pilot character workspace
2. Explanation detail surface
3. Validation / problems surface
4. Import diagnostics surface
5. Rules library pilot surface
6. Source package inspection surface

## IA rules
- the pilot character workspace is the center of gravity for the first shell
- explanation, diagnostics, and provenance views must remain cross-linked back to the specific value, choice, or rule the user is inspecting
- rules-library and source-package views must not become isolated browsers with no path back to the active character context
- the IA must support both overview-first and “why is this wrong / why is this value here?” workflows

## User questions this IA must answer
- What does my current pilot character look like?
- Why is this value what it is?
- Why is this choice unavailable or invalid?
- What warnings or unsupported semantics affect this character?
- Which rule/source object produced this behavior?

## Completion rule
This artifact is satisfied when a future bounded UI slice can choose routes/panels/components without re-litigating what information must be reachable and how the major surfaces relate.
