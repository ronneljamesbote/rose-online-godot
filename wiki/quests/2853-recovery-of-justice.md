---
kind: quest
id: 2853
name: Recovery of Justice
status: in-game
time_limit_minutes: 70
steps: 0
source:
  data: 'LIST_QUEST.STB row 2853; QSD triggers '
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Recovery of Justice

That hypocritical Junon Order is raising an army to wipe us out! Well, let's show them that true defenders of justice will never bend to their will!  

> Open question: this quest has no steps here because its quest file (`PvP10.qsd` or `PvP13-01.qsd`) fails to load: `crates/rose-file-readers/src/qsd.rs` reads the zone-team trigger reward (type 19) with a wrong layout. Part of [[backlog/faction-wars|Faction wars]].
