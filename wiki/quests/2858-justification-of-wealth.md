---
kind: quest
id: 2858
name: Justification of Wealth
status: in-game
time_limit_minutes: 70
steps: 0
source:
  data: 'LIST_QUEST.STB row 2858; QSD triggers '
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Justification of Wealth

That foolish Junon Order and the Arumics don't give us any respect or recognition. They underestimate us and don't understand the power of wealth. Let's show them that the Ferrell Guild is a force to be reckoned with!  

> Open question: this quest has no steps here because its quest file (`PvP10.qsd` or `PvP13-01.qsd`) fails to load: `crates/rose-file-readers/src/qsd.rs` reads the zone-team trigger reward (type 19) with a wrong layout. Part of [[backlog/faction-wars|Faction wars]].
