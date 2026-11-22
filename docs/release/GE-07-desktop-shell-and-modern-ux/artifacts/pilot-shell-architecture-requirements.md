# Pilot Shell Architecture Requirements

## Objective
Define the minimum desktop shell frame that can host the pilot character experience, explanations, diagnostics, and bounded rules/source inspection without expanding into a broad application platform.

## Required shell frame
The first truthful GE-07 shell must provide:
1. an application frame with stable primary navigation
2. a pilot-character-centered workspace
3. persistent access to problems/diagnostics state
4. a path into explanation detail without losing current context
5. bounded routes into rules-library and source-package inspection

## Layout obligations
- the shell must let the user see the active character context while moving into explanation or diagnostics detail
- explanation and diagnostics should be presented as adjacent or layered detail surfaces, not as disconnected standalone worlds
- the shell must keep enough structural room for source-package and rules-library inspection, but those surfaces may remain bounded to pilot scope in the first slice

## Non-goals for the first shell
- full product navigation breadth
- public-launch polish
- every future authoring surface
- dashboard pages with no pilot relevance

## Completion rule
This artifact is satisfied when a future implementation team can identify the minimum shell frame, main workspace, and supporting surfaces without mistaking the requirement for a broad “build the whole app” order.
