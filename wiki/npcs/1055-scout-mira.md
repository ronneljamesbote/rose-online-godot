---
kind: npc
id: 1055
name: '[Scout] Mira'
status: in-game
zone: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
services: save point
quests:
- '[[quests/2101-dust-on-the-road|Dust on the Road]]'
- '[[quests/2103-glowing-stingers|Glowing Stingers]]'
locations:
- zone: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
  x: 5228
  y: 5080
source:
  data: godot/data/npcs/1055-scout-mira.tres; dialog godot/data/dialogs/em21-005-scout-mira.tres
  code:
  - module/src/npcs.rs
  - godot/rust/src/conversation.rs
---
# [Scout] Mira

Stands at her lookout rock above the Sandwind Caravan Camp in [[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]].

## Dialog

The NPC says the **last** message in a list whose condition passes; the lines under
it are your choices. See [[rules/npc-dialogs|NPC dialogs]].

- **NPC:** From up here I can see the whole valley. Most days I wish I couldn't.
  - **You:** About "Dust on the Road"  *(if you have [[quests/2101-dust-on-the-road|Dust on the Road]]; quest step `2101-02` doesn't pass)*
    - **NPC:** Mira is on the rock just north-east of the camp. You can't miss her.
  - **You:** "Dust on the Road": here I am.  *(if quest step `2101-02` passes)*  → runs quest step `2101-02` ([[quests/2101-dust-on-the-road|Dust on the Road]])
    - **NPC:** So Dorran sent you up here. Good. From this rock I can see the whole valley, and the dust on the road gets thicker every day. Stay close to the camp at first. If you fall out there, our medic will patch you up here.
  - **You:** About "Glowing Stingers"  *(if you don't have [[quests/2103-glowing-stingers|Glowing Stingers]]; quest step `2103-01` passes)*
    - **NPC:** Have you seen the Honeybees at the dead trees? At dusk their stingers glow. Bees don't glow. Bring me 8 of those stingers and I'll find out why.
      - **You:** I'll do it.  → runs quest step `2103-01` ([[quests/2103-glowing-stingers|Glowing Stingers]])
        - **NPC:** They swarm around the dead trees with hives, by the road and in the fields.
      - **You:** Not now.
  - **You:** About "Glowing Stingers"  *(if you have [[quests/2103-glowing-stingers|Glowing Stingers]]; quest step `2103-02` doesn't pass)*
    - **NPC:** Eight stingers. Watch the Needle Honeybees, they sting harder.
  - **You:** "Glowing Stingers": it's done.  *(if quest step `2103-02` passes)*  → runs quest step `2103-02` ([[quests/2103-glowing-stingers|Glowing Stingers]])
    - **NPC:** Look at them shine. That's mana, and a lot of it. The bees drink from the oasis, and the oasis is fed by the spring under the tower. Something is leaking out of that tower.
  - **You:** Make this camp my save point.  → makes this your save point
    - **NPC:** Done. If you fall out there, you will wake up here at the camp.
  - **You:** Goodbye.

After an answer the NPC offers "Something else." (back to the first list) and "Goodbye."

## Changed from iROSE

A new NPC of the Valley of Luxem Tower redesign, using an existing iROSE model.
