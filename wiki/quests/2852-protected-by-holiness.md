---
kind: quest
id: 2852
name: Protected by Holiness
status: in-game
time_limit_minutes: 70
steps: 0
source:
  data: 'LIST_QUEST.STB row 2852; QSD triggers '
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Protected by Holiness

Though the Righteous Crusaders and the Ferrell Guild threaten us, they are nothing compared to the holiness that is our protection. Let us fight for the sake of the Junon Order!  

> Open question: this quest has no steps here because its quest file (`PvP10.qsd` or `PvP13-01.qsd`) fails to load: `crates/rose-file-readers/src/qsd.rs` reads the zone-team trigger reward (type 19) with a wrong layout. Part of [[backlog/faction-wars|Faction wars]].
