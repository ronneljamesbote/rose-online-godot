---
kind: quest
id: 2854
name: Dignity of the Righteous Crusaders
status: in-game
time_limit_minutes: 70
steps: 0
source:
  data: 'LIST_QUEST.STB row 2854; QSD triggers '
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Dignity of the Righteous Crusaders

It's lonely to be number one, and you'll always have enemies. Let's show that jealous Junon Order just how powerful we really are, and confirm our superiority!  

> Open question: this quest has no steps here because its quest file (`PvP10.qsd` or `PvP13-01.qsd`) fails to load: `crates/rose-file-readers/src/qsd.rs` reads the zone-team trigger reward (type 19) with a wrong layout. Part of [[backlog/faction-wars|Faction wars]].
