---
kind: quest
id: 2855
name: Wounded Pride
status: in-game
time_limit_minutes: 70
steps: 0
source:
  data: 'LIST_QUEST.STB row 2855; QSD triggers '
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Wounded Pride

The Arumics have been greatly aided by the Junon Order ever since our organization has left Luna and come to Junon. We've got swallow our pride and return the favor. We must join our ally in the fight against the Righteous Crusaders and Ferrell Guild, and show them our magic power!  

> Open question: this quest has no steps here because its quest file (`PvP10.qsd` or `PvP13-01.qsd`) fails to load: `crates/rose-file-readers/src/qsd.rs` reads the zone-team trigger reward (type 19) with a wrong layout. Part of [[backlog/faction-wars|Faction wars]].
