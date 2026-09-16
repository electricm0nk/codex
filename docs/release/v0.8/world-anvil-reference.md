---
title: v0.8 — World Anvil as the DM Toolkit design anchor
status: reference
author: scout (Fable 5.1 teammate)
date: 2026-09-01
branch: tranche/14-ui
purpose: reference for adjusting DM Toolkit v1 and for a future v0.9 brief — not a backlog
---

# World Anvil as the design anchor for the DM Toolkit

The operator named World Anvil as the reference point ("something much like World Anvil").
This note answers five questions the lead asked: what World Anvil's model actually is; what it
gets right that our seven-kind model would be poorer without (ranked, with cost); what it gets
wrong or what its users complain about; whether our per-record GM/player flag is the right unit
for secrets; and what does not transfer to a local desktop app whose sharing story is an
exported HTML file.

**Sourcing.** World Anvil's own site (`worldanvil.com/learn`, `/w/WorldAnvilCodex`, its
community tracker and suggestion pages) returns HTTP 403 to non-browser fetches, and the
Wayback Machine is not reachable from this session. Of the primary pages, only the Digital
Storyteller Screen guide and two `blog.worldanvil.com` posts were read in full. Everything else
comes from search-result excerpts of World Anvil's own documentation (quoted where used) and
from third-party guides and comparisons, which are named inline. Where I am inferring rather
than reading, it says so.

**What v1 is already building** (from the lead's brief, not re-derived): seven record kinds
(World, Timeline, Place, Person, Clue, Scene, Rule), each with title / summary / Markdown body /
thin typed fields; directed links with relation labels and derived backlinks; per-kind search;
localStorage per campaign; export to a self-contained HTML file; a per-record
`visibility: 'gm' | 'players'` flag producing a full GM export or a stripped player handout.

---

## 1. World Anvil's actual model

Five structures, in the order a user meets them.

**Articles with templates.** Everything is an article. There are 28 templates (Character,
Settlement, Organization, Species, Item, Condition, Spell, Vehicle, …) plus a Generic article
with no prompts; a template adds "a bunch of helpful fields that you can fill in or ignore as
you see fit, plus prompts and insights under each field", and "it's only possible to change the
template from a generic article to something else, but all other templates can't be changed"
(World Anvil Learn, *How to Use Article Templates*; Codex, *Articles & Categories Manager*).
Every article has the same skeleton regardless of template: a body, four sidebar slots
(panel top/bottom, sidebar bottom, …), a vignette, and a footer, authored in BBCode with a
`[sidebar:Label]…` tag for ad-hoc labelled fields (Learn, *Anatomy of an Article*; Loreteller,
*10 BBCode Snippets*). Custom templates let a paid user pre-populate those slots or, in
"advanced" mode, rewrite the HTML with Twig against the article's fields
(`blog.worldanvil.com`, *Custom Article Templates*). The shape, then: **a typed record is a
free-text body plus a template-defined set of optional sidebar fields, all optional, all
prose.** There is no queryable schema; the tier list at ttrpg.bot puts it as "you can
cross-reference articles, but you cannot query your world."

**Categories: one tree, one parent.** "Articles live in exactly one category" and categories
"form the public table of contents for your world"; the official guidance is "no more than
twenty articles in a category", and third-party guidance adds "don't go deeper than three
levels" (World Anvil blog, *How to organize worldbuilding with Categories*; Loreteller,
*The Ultimate World Anvil Category Structure*). Articles can also nest under a parent article
("Minor NPCs might go underneath the Settlement article where they live"). Tags are the
orthogonal, many-to-many axis. A "Staging Area" category and a separate Notebook hold unfinished
things. So: **navigation hierarchy is a single-parent tree that doubles as the reader's table
of contents; everything cross-cutting is a tag or a link.**

**Links: the mention system.** Cross-references are `@[Label](type:UUID)` mentions typed with
`[` + search; they resolve by id, so renames do not break links, but "the @[mention] text won't
automatically update — you'll have to change it manually" (Learn, *How to Link Articles with
the Mention System*). Links are untyped inline prose links. I could not confirm from a primary
page whether World Anvil renders an automatic "mentioned in" backlink list; search excerpts did
not show one, so treat backlinks as **not a World Anvil strength** (inference).

**Timelines and Chronicles.** A *historical entry* has a start/end date and is assigned to one
or more timelines ("select the timeline(s) you want it to appear in"); a timeline renders them
with "parallel events, filters for quick event searching". *Chronicles* layers timelines on an
interactive map: events are pinned to places, "you can scale the timeline down to hours and even
minutes, so you can detail things like a battlefield or a heist", and clicking a map marker
shows "an automatically generated 'History' section" for that place (Learn, *Feature Guide to
Timelines*; `blog.worldanvil.com`, *Map Your World's History through Time & Place*). Shape:
**an event is its own record, dated, on N timelines, linked to places; a place's history is
derived from the events pointing at it.**

**Maps.** An image with pins; pins "can link location articles (geography, settlement or
building), organization articles and generic articles … historical events, categories, or even
other maps"; pins are grouped into *marker groups* with per-group visibility ("For GM Eyes Only",
"Elf Knowledge"), and clicking a pin opens the linked article in a sidebar without leaving the map
(Learn, *Feature Guide to Maps*; *How to Add Pins*). Shape: **the map is a spatial index over
the same records, with visibility applied per pin group.**

**Campaign layer.** Separate from the world: a Campaign has sessions, quests with "a GM-only
text box", a party (linked character sheets and shared equipment), and the Digital Storyteller
Screen, which is a single-page live view with a dice roller, party stream, a "Library" of
pinned articles and statblocks, the session "Plot", a handout screen for images, and a media
list for ambience (Learn, *Feature Guide to the Digital Storyteller Screen*, read in full).
The DSTS is the closest thing World Anvil has to the NoDA console's at-the-table view, and it is
notably thin: pinned shortcuts to articles, not a purpose-built scene runner.

### Where the mechanism differs from what the marketing implies

The useful findings in this note all sit in the gap between what World Anvil's feature list
suggests and what its mechanism actually is. Naming them, because a v1 spec written from the
feature list (as the per-record visibility flag was) lands on the wrong side of each:

| What the feature name suggests | What the mechanism actually is | Consequence for us |
|---|---|---|
| "Secrets" = a private flag on an article | A separately-authored chunk, embedded by reference into N articles, absent (not hidden) when unrevealed, revealed everywhere by one change | Clue is the secret unit; the per-record flag is a different, blunter tool (§4) |
| "Templates" = structured data per kind | Optional prose fields on one shared body/sidebar skeleton; not queryable; kind cannot be changed after creation | Keep body-first, thin fields, re-kindable records; labelled links are what make the world queryable |
| "Timelines" = a chronology page | Events are their own records on N timelines, linked to places; a place's history is *derived* from them | History is a backlink view, not a field on the NPC (R2) |
| "Categories" = tags | One parent per article, a tree ≤ 3 deep that *is* the public table of contents; tags are the separate many-to-many axis | Hierarchy and tagging are different decisions (Q-WA1) |
| "Player access" = share a link | Accounts, subscriber groups with per-tier caps, aggressive caching that delays reveals | None of it transfers; export-file model does the reveal by re-export (§5, Q-WA3) |
| "Export" = get your world out | A guild-only backup zip; users are asking for printable/portable export and not getting it | Our standalone HTML export is a differentiator, not a fallback |

---

## 2. What it gets right that our model would be poorer without — ranked

Ranked by how much it changes the model, with the cost given what v1 already has.

**R1. Secrets are separately-authored chunks embedded into N articles, and an unrevealed secret
is simply absent.** "Secrets are chunks of content you create separately and embed into
articles. A secret can belong to multiple groups"; for a reader without access "that section
simply doesn't exist. No blank space, no 'hidden content' message"; and "the same secret can
embed in multiple articles" so one permission change reveals it everywhere (Loreteller,
*World Anvil Secrets: Player Permissions That Work*, quoting World Anvil's own docs). This is
the one structural idea World Anvil has that a per-record flag does not capture, and it maps
onto v1 almost for free: **our Clue kind is already this object.** A Clue with
`visibility: 'gm'` linked to a Person, Place or Scene should render inline inside that record's
page in the GM export and be omitted from the player export; flipping the Clue's visibility is
the reveal. *Cost: cheap* — an exporter rule over links that already exist, no schema change.
(Sent to the lead mid-pass.)

**R2. History is derived, not authored twice.** Chronicles' auto-generated "History" section on
a place is the NoDA console's per-NPC timeline, produced from event→place links rather than
typed into the NPC card. Our Timeline kind plus backlinks gives the same thing if Timeline
records carry a sortable date/order field and the Person/Place detail renders its Timeline
backlinks sorted by it. *Cost: cheap* — a sorted backlink section. The lead confirms
`DM_KIND_FIELDS.Timeline` already carries a single-line `when` field, so the field exists; the
open question is whether free-text `when` ("Sunday night", "18:30", "Wednesday 17:02" in NoDA)
sorts usefully. NoDA's own entries only sort because the operator typed them in order. Either
keep insertion order as the sort and treat `when` as a label, or add an optional numeric
`order`/ISO-time field beside it; do not try to parse prose. (Sent to the lead mid-pass; queued
after D-6.)

**R3. Draft is a state, not a visibility.** World Anvil separates *draft/published* from
*public/private*: "Drafts are never visible to other people" and do not render even for the
author in view mode, while published articles carry the audience setting (Learn, *Feature
Guide to Articles*; *How to Set Who Can See Your Article*). Our flag conflates "not ready" with
"GM only". A DM mid-prep needs "this Scene is half-written, keep it out of *my own* table view
tonight" as well as "this is finished but players must not see it". *Cost: cheap* — one more
enum value (`'draft'`) honoured by the console's own list views and by both exports.

**R4. Linked records open in place.** Map pins and DSTS library entries open the target "in a
sidebar, so they can browse the article you've linked while staying on the map page". The NoDA
console's whole at-the-table value is exactly this: from a Scene, glance at the NPC without
losing the Scene. *Cost: moderate* — a UI pattern (peek panel or split detail) in the console
and in the exported page, not a model change. Worth doing in the export first, because the
export is what sits on the phone at the table.

**R5. A single navigation tree that is also the reader's table of contents.** One parent, depth
≤ 3, ≤ 20 per node. Our per-kind lists are flat; NoDA's are flat too and are already unwieldy
at 40 people. Something coarser than a kind and finer than a campaign is needed eventually
(District → Place; Faction → Person). *Cost: real structural change* — a parent link with
tree semantics, or a tag axis, plus list views that group by it. Not v1; a v0.9 question
(Q-WA1 below). Note World Anvil's own lesson: keep it to one parent; many-to-many is what
tags/links are for.

**R6. Pin groups with visibility on a map.** The map is the one World Anvil surface where
visibility is applied to a *group of pointers* rather than to a record. That is what a DM's
battle map or district map needs ("GM eyes only" pins). We have no map kind at all (and NoDA
carries only grid references like `H3`, not a map). *Cost: real* — a Map kind with an image,
pins as links with coordinates, and pin-group visibility. v0.9 at the earliest.

Deliberately not on the list: 28 templates with prompts (see §3), custom Twig templates, CSS
theming, family trees, the dice roller, the party stream, statblock hosting (blocker B14 owns
anything that needs rules), monetisation, and the community.

---

## 3. What it gets wrong, in users' words

Grouped by the failure mode, most relevant to an at-the-table tool first.

**Slow when you need it fast.** "Pages load slowly, especially with complex articles and
embedded maps. During actual play, when you need information fast, that lag is painful"
(StormScape blog, a competitor — discount accordingly). "Some pages load *very* slowly while
other interfaces are quicker" (PhD20, *Ultimate Guide to TTRPG Campaign Managers*, independent).
World Anvil's own tracker carries a report titled "World Anvil is very slow for me" with
"clicks taking 10 to 60 seconds for pages to load" (excerpt; page itself 403). LegendKeeper "gets
mentioned as 'the answer'" when "speed during actual play is your top priority" (PhD20).
*Implication for us:* the exported HTML must be static, single-file, and fast on a phone; the
NoDA console already is (437 KB, no network). Do not let export grow a runtime.

**Feature surface wider than a GM needs; onboarding is a cliff.** "A bit of feature bloat leads
to *so* much going on for nearly every interface" and "a moderate learning curve" (PhD20).
"There are so many features, menus, and configuration options that it takes weeks to feel
comfortable" (StormScape). Reddit sentiment as summarised by the same source: "too cluttered",
"overwhelming for new users", "work, not fun". AlternativeTo reviewer (2021-08-01): "Maybe the
worst UI I've ever seen. And it's a guessing game of which 'accept/ok/save' button is the right
one." Another (2021-03-17): "a complicated word processor with the capabilities of an old-school
myspace page." *Implication:* seven kinds with a body and thin fields is the right size. Resist
adding a template per noun.

**Templates front-load empty structure.** Each template is a page of optional prompted fields;
users are told to "fill in or ignore as you see fit" (Learn). Third-party guidance implicitly
concedes the problem by telling newcomers to "create your first ten articles without worrying
about categories" (Loreteller). No first-hand quote found that names "too many required fields"
specifically, so this is partly inference from the design: nothing is required, but the empty
sidebar renders anyway, and the 28-way choice at creation is itself friction (templates cannot
be changed afterwards except from Generic). *Implication:* keep body-first; show typed fields
only when filled; allow changing a record's kind.

**Article-first, not data-first.** "The article-first content model fights GMs who think in
databases: you can cross-reference articles, but you cannot query your world" (ttrpg.bot,
*Best World Anvil Alternatives*). *Implication:* our directed *labelled* links are already a
step past World Anvil's untyped mentions; keep the labels meaningful (`lives at`, `appears in`,
`reveals`) rather than decorative, because they are what makes "who is at Sanctuary tonight"
answerable.

**Subscription gating and privacy posture.** "The free tier shows ads to you and to your
players"; tiers are priced "by identity ('beginner, experienced, professional') rather than by
function" (ttrpg.bot). Secrets/subscriber groups, custom pins, footer sections and visibility
toggles are all guild-tier features; subscriber counts are capped per tier (Journeyman 5,
Master 10, Grandmaster 100 — Loreteller). A Goodreads reviewer's summary: "World Anvil is
primarily a worldbuilding sharing community … not a private tool. All articles are public when
you publish them in the free version." *Implication:* none of this transfers, and it is the
strongest argument for the local-app-plus-export model — see §5.

**Export is a backup, not a product.** "As a guild member you can export any world … as a
structured zip archive containing all the metadata and contents of the world serialized in JSON
format, as well as in a basic HTML format" (Learn, *How to Export Your World*, excerpt).
Community suggestions titled "Export (backup) whole database/world in one go" and "Print out or
export of articles" are open, and tracker items report "World Export not working on all worlds";
one summary: the backup "is not meant for anything but restoring on World Anvil". *Implication:*
our export-to-standalone-HTML is the feature World Anvil users are asking for and not getting.
That is a differentiator, not a fallback.

**Caching and reveal latency.** "World Anvil caches aggressively. After changing subscriber
group access, have players refresh the page or log out and back in" (Loreteller). *Implication:*
in an export-file model the "reveal" is a re-export and a re-send; that is slower in wall-clock
but deterministic, and worth designing explicitly (Q-WA3).

---

## 4. The secrets model: is per-record the right unit?

**Per-record is necessary and not sufficient.** World Anvil has both, and treats them as
different tools: article privacy "controls whether an entire article is public, private, or
restricted to subscribers" and is "blunt. You can't show players the shopkeeper's name and
appearance while hiding their secret allegiance"; secrets exist precisely to fix that
(Loreteller, quoting the docs). Our flag is World Anvil's article privacy. The shopkeeper case
is the common one at a table: the player handout needs the NPC's face, name, role and home
base; it must not carry motivation, clue fragments or the betrayal.

**Per-block is cheap for us, because the block already exists as a kind.** World Anvil's secret
is a separate object embedded by reference; ours is a Clue linked to a target. The three
properties that make World Anvil's model work all fall out of what v1 has:

| World Anvil property | v1 equivalent | Needs |
|---|---|---|
| Secret authored once, embedded in N articles | One Clue, N `reveals`/`held by` links | nothing new |
| Unrevealed → "that section simply doesn't exist" | Player export omits GM-visibility Clues, and omits the inline block, not just the Clue page | exporter rule |
| Reveal once → visible everywhere | Flip the Clue's `visibility` | nothing new |

What it does *not* give, and should not try to in v1:

- **Multiple audiences.** World Anvil's subscriber groups let one player see a secret the rest
  of the party cannot. Our flag is binary. A DM running a traitor PC needs a third value or a
  per-player audience. Q-WA2.
- **Secrets inside the body prose.** World Anvil lets a secret sit mid-paragraph. Ours sits as a
  block after the body. For a handout that is fine; for a GM copy the block placement is a
  rendering choice. Inline markers in Markdown (a fenced `:::gm` block) would be the next step
  and are cheap, but they duplicate what Clue already does — pick one, not both.
- **Visibility on a *link*.** World Anvil's map applies visibility to pin groups. The
  equivalent question for us: can a Person be player-visible while the *link* "lives at Palisade
  North" is GM-only? Today visibility lives on records, so a visible Person's backlink list
  would leak a GM-only Place's existence unless the exporter also filters links whose target is
  GM-only. **This is a correctness requirement on the exporter, not a feature** — check that the
  player export strips links to GM-only targets, and strips backlink sections accordingly.

**Verdict:** keep the per-record flag as the blunt control; make Clue the fine-grained unit; add
`draft` as a third state (R3); make the exporter filter links and backlinks by target
visibility. Per-block-in-prose is not worth a second mechanism in v1.

---

## 5. What is genuinely different about our situation

**Does not transfer:**

- **Accounts, subscriber groups, live reveal.** World Anvil's whole audience model assumes
  players log in. Ours assumes the DM hands over a file. Per-player audiences (Q-WA2) would mean
  per-player exports, which is workable but is a different feature from "groups".
- **Player-authored content.** Session notes players write, shared party equipment players
  edit, the party stream. A read-only handout cannot take input back. If the operator wants any
  of it, that is Drive or a second channel, not the export.
- **Hosted assets and embedded media.** Pins on a 20 MB map image, YouTube ambience, image
  galleries. A single-file export has a size ceiling; either link out or accept a bigger file.
- **Community, monetisation, theming, prompts.** Out of scope by definition.

**What the export-file model lets us do that World Anvil cannot:**

- **Works with no network at the table.** The one complaint that recurs across every source is
  speed under load. A static file on a phone has none of it.
- **A frozen, per-session snapshot.** World Anvil's reveal is live and cached; ours is a dated
  artefact. "What did the players know as of session 4" is a file they still have. That is a
  feature for a real-time-clock campaign like NoDA (six in-game hours across two sessions).
- **The DM's own copy is also just a file.** The GM export is the NoDA console: the DM can run
  the session from a tablet without the desktop app open at all, which World Anvil's DSTS cannot
  offer (it *is* the web app).
- **Data ownership by construction.** localStorage plus an export beats a backup zip "not meant
  for anything but restoring on World Anvil". Worth stating in the product's own words.
- **No tiering.** Secrets, visibility, export and maps are not paywalled features here; they
  are the product.

---

## Questions for the operator before a v0.9 brief

- **Q-WA1 — Hierarchy.** Do records need a parent (one tree, depth ≤ 3, World Anvil's way), a
  tag axis, or neither beyond kind + links? NoDA's People list at 40 entries is already at the
  point World Anvil says to split.
- **Q-WA2 — Audiences.** Is GM / players enough, or does the table need per-player secrets
  (traitor PC, split party)? If yes: per-player exports, or a third audience value?
- **Q-WA3 — Reveal workflow.** In an export model a reveal is "flip visibility, re-export,
  re-send". Is that acceptable at the table, or does the GM copy need an in-page "reveal to
  players" that produces a fresh handout without returning to the desktop app?
- **Q-WA4 — Maps.** Is a Map kind (image + pins + pin-group visibility) in scope for v0.9, or do
  grid references in Place records (NoDA's `H3`) suffice for the campaigns the operator runs?
- **Q-WA5 — Player input.** Does anything need to come *back* from players (session notes,
  shared loot)? If so the channel is Drive, not the export, and it should be scoped separately.
- **Q-WA6 — Kind changes.** World Anvil forbids changing a template after creation and users
  resent it. Should our records be re-kindable (a Person that turns out to be an Organisation)?
  Cheap if kinds share the same skeleton, which they do.

---

## Sources

Primary (World Anvil), read in full via direct fetch:
- Learn, *Feature Guide to the Digital Storyteller Screen* — https://www.worldanvil.com/learn/rpg/dsts
- Blog, *How to organize worldbuilding with Categories* — https://blog.worldanvil.com/worldanvil/tutorials/how-to-organize-worldbuilding-with-categories-getting-started-with-world-anvil/
- Blog, *Custom Article Templates* — https://blog.worldanvil.com/worldanvil/dev-news/custom-article-templates-what-are-they-and-how-can-they-help-you/
- Blog, *Map Your World's History through Time & Place* (Chronicles) — https://blog.worldanvil.com/worldanvil/dev-news/map-your-worlds-history-through-time-place/

Primary (World Anvil), quoted from search excerpts only (site returned 403 to direct fetch):
- Learn, *How to Use Article Templates* — https://www.worldanvil.com/learn/article-guides/article-templates
- Learn, *Anatomy of an Article* — https://www.worldanvil.com/learn/article-guides/anatomy-article
- Learn, *How to Link Articles with the Mention System* — https://www.worldanvil.com/learn/bbcode-tutorials/mention-system
- Learn, *Feature Guide to Timelines* — https://www.worldanvil.com/learn/timelines/timelines
- Learn, *Feature Guide to Maps* / *How to Add Pins* — https://www.worldanvil.com/learn/map-making/maps , https://www.worldanvil.com/learn/map-making/add-pin
- Learn, *Feature Guide to Articles*; *How to Set Who Can See Your Article*; *Visibility Toggles* — https://www.worldanvil.com/learn/article-guides/articles , https://www.worldanvil.com/learn/article-guides/set-article-subscribers , https://www.worldanvil.com/learn/article-guides/visibility-toggles
- Learn, *Getting Started with Secrets & Subscribers* — https://www.worldanvil.com/learn/beginner-tutorials/get-started-secrets
- Learn, *How to Export Your World* — https://www.worldanvil.com/learn/world/export
- Codex, *Articles & Categories Manager* — https://www.worldanvil.com/w/WorldAnvilCodex/a/articles-categories-manager
- Tracker, "World Anvil is very slow for me" — https://www.worldanvil.com/udan-tracker/a0b24053-f432-45aa-a74f-00b079c01061/view
- Suggestions, "Export (backup) whole database/world in one go"; "Print out or export of articles" — https://www.worldanvil.com/community/voting/suggestion/653c9fae-e4ce-4b8a-bd89-66acf2f079df/view , https://www.worldanvil.com/community/voting/suggestion/20e8c78e-c171-4280-8556-356052d48a35/view

Third-party:
- Loreteller, *World Anvil Secrets: Player Permissions That Work* — https://loreteller.com/learn/world-anvil-secrets-guide/
- Loreteller, *The Ultimate World Anvil Category Structure* — https://loreteller.com/learn/world-anvil-categories/
- Loreteller, *10 BBCode Snippets Every World Anvil User Needs* — https://loreteller.com/learn/world-anvil-bbcode/
- PhD20, *The Ultimate Guide to TTRPG Campaign Managers in 2025* — https://phd20.com/blog/ultimate-guide-ttrpg-campaign-managers/
- ttrpg.bot (Grimoire), *7 Best World Anvil Alternatives for DMs* — https://www.ttrpg.bot/best-world-anvil-alternatives/
- StormScape blog (competitor), *World Anvil vs LegendKeeper vs Kanka vs StormScape* — https://stormscape.app/blog/world-anvil-vs-legendkeeper-vs-kanka-vs-stormscape
- AlternativeTo, World Anvil user reviews — https://alternativeto.net/software/world-anvil/about/
- Goodreads author blog, *World Anvil Review – My 5 Favorite Features* (excerpt) — https://www.goodreads.com/author_blog_posts/21534398-world-anvil-review-my-5-favorite-features

Not obtained: first-hand Reddit threads (Reddit's JSON API and libredd.it refused this session);
Reddit sentiment above is second-hand via StormScape and PhD20 and is labelled as such.
