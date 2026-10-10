---
kind: quest
id: 2857
name: Taking Back Ferrell Road
status: in-game
time_limit_minutes: 70
steps: 0
source:
  data: 'LIST_QUEST.STB row 2857; QSD triggers '
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Taking Back Ferrell Road

The Junon Order and the Arumics attacked us by surprise and seized one of our most important trade routes, the Ferrell Road. Let's show them how powerful the Ferrell Guild really is, and take back Ferrell Road!  

> Open question: this quest has no steps here because its quest file (`PvP10.qsd` or `PvP13-01.qsd`) fails to load: `crates/rose-file-readers/src/qsd.rs` reads the zone-team trigger reward (type 19) with a wrong layout. Part of [[backlog/faction-wars|Faction wars]].
