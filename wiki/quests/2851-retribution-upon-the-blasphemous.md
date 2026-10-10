---
kind: quest
id: 2851
name: Retribution upon the Blasphemous
status: in-game
time_limit_minutes: 70
steps: 0
source:
  data: 'LIST_QUEST.STB row 2851; QSD triggers '
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Retribution upon the Blasphemous

Both the Righteous Crusaders and the Ferrell Guild have blasphemed the Junon Order, and they must be punished in the name of Junon. Let them feel our righteous wrath!  

> Open question: this quest has no steps here because its quest file (`PvP10.qsd` or `PvP13-01.qsd`) fails to load: `crates/rose-file-readers/src/qsd.rs` reads the zone-team trigger reward (type 19) with a wrong layout. Part of [[backlog/faction-wars|Faction wars]].
