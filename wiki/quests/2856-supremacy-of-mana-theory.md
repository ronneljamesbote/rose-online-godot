---
kind: quest
id: 2856
name: Supremacy of Mana Theory
status: in-game
time_limit_minutes: 70
steps: 0
source:
  data: 'LIST_QUEST.STB row 2856; QSD triggers '
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Supremacy of Mana Theory

The Righteous Crusaders and Ferrell Guild have taken our magic lightly, but were frequently defeated when they challenged us. Once again, they must be taught the reality of Mana Theory. Let us show them the true power of Arumic magic!  

> Open question: this quest has no steps here because its quest file (`PvP10.qsd` or `PvP13-01.qsd`) fails to load: `crates/rose-file-readers/src/qsd.rs` reads the zone-team trigger reward (type 19) with a wrong layout. Part of [[backlog/faction-wars|Faction wars]].
