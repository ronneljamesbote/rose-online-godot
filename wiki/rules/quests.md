---
kind: rule
id: quests
name: Quests
status: in-game
max_active_quests: 10
quest_items_per_quest: 6
quest_variables_per_quest: 10
quest_switches_per_quest: 32
quest_switches: 1024
episode_variables: 5
job_variables: 3
planet_variables: 7
union_variables: 10
world_tick_s: 10
max_steps_in_a_chain: 64
max_monsters_per_spawn_reward: 20
world_reward_rate: 300
fame: 0
reward_formulas:
  - { formula: 0, used_for: "experience", grows_with: "Charm; smaller at higher levels" }
  - { formula: 1, used_for: "experience", grows_with: "level and Charm" }
  - { formula: 2, used_for: "Zuly", grows_with: "times repeated (quest variable 9)" }
  - { formula: 3, used_for: "Zuly (one quest uses it for experience)", grows_with: "Charm; smaller at higher levels" }
  - { formula: 4, used_for: "not used by any quest", grows_with: "level and Charm" }
  - { formula: 5, used_for: "item count", grows_with: "Charm; smaller at higher levels" }
  - { formula: 6, used_for: "not used by any quest", grows_with: "level and Charm" }
source:
  code:
    - module/src/quests.rs (run_trigger, quest_trigger, abandon_quest, Run::apply_rewards, reward_value, calculated_money, calculated_item, add_item, spawn_monster)
    - crates/rose-quest/src/lib.rs (check_conditions, check_trigger, find_trigger, get_quest_variable, world_ticks)
    - crates/rose-game-irose/src/data/ability_values.rs (calculate_reward_value)
    - crates/rose-file-readers/src/qsd.rs (QsdEquation)
    - crates/rose-game-common/src/components/quest_state.rs (QuestState, ActiveQuest)
    - module/src/lib.rs (monster death trigger, world_rates_row)
    - module/src/character.rs (levelup triggers)
    - godot/scripts/quest_window.gd
    - godot/rust/src/net.rs (get_quests, time_left)
  data: "LIST_QUEST.STB (quest names, text and time limits), QSD quest trigger files, LIST_NPC.STB (each monster's death trigger)"
---
# Quests

Quests come from town NPCs. Talking to an NPC (see [[rules/npc-dialogs|NPC dialogs]])
gives you a quest, and you come back with what it asks for to finish it and get the
reward. Press **Q** to open your quest list: each quest's text, the quest items it holds,
its time left and an **Abandon** button.

You can have up to **10** quests at once. A step that would give you an eleventh fails
with "Your quest list is full".

## Steps

A quest is a set of **steps** (quest triggers in the data). Each step has a name such as
`1008-02`, a list of **checks** and a list of rewards (under "Then" on quest pages). A
step happens in one of three ways:

- **Talking to an NPC.** An answer in the dialog asks for the step (see
  [[rules/npc-dialogs|NPC dialogs]]). Your game checks it first, then the server checks it
  again and gives the rewards.
- **Killing a monster.** Many monsters have a death step. The player who lands the killing
  blow runs it. A kill by your summon runs nothing. This is how kill counts and quest item
  drops work.
- **Levelling up.** Each level you reach runs the step `levelup_<level>` if there is one
  (two levels at once run both).

A step **passes** when every check passes; then its rewards are given in order. If one
reward fails (for example your quest list is full, or a fee you can't pay), the rest of
that step's rewards are not given, but the ones before it stay.

Steps can lead to other steps:

- A passing step may say **then runs step `X`**; that step is tried next.
- A failing step falls through to the step after it in the same group of the data, so one
  answer can try several steps in order and run the first that fits.

The server follows at most 64 steps in one go. Everything a chain of steps changes is
saved at once at the end, and you get a notice for each thing you received.

## How quest pages show steps

Each page in [[quests|Quests]] lists the quest's steps. "Happens by talking to" or
"Happens by killing" says what runs it. **Checks** must all pass; **Then** lists what you
get, in order.

Checks:

| On the page | Means |
| --- | --- |
| you have *quest* | the quest is in your list; later checks and rewards work on that quest |
| you carry ≥ 10 × *item* | quest items count inside the quest; other items count in your bag |
| your Level ≥ 30 | your level, stats, Zuly and so on |
| your Job = 4 | your job is in job group 4 of the data (for example "any Soldier job"), not job number 4 |
| quest switch 0 = 1 | an on/off switch of the quest, or one of your 1,024 general switches |
| job / episode / planet variable N | your progress counters, kept between quests |
| the quest timer ≤ 0 | the quest's time limit has run out |
| a random roll 0–99 lands in 0–40 | the server rolls 0 to 99; this passes 41 times in 100 |
| time of day, date, weekday | world time (ticks of 10 seconds) or the real date and time |
| you are within N of a place | your zone and distance |
| the NPC's quest switch | a variable of an NPC (see [[rules/npc-dialogs\|NPC dialogs]]) |
| you are in a party with party level N | party checks (see [[rules/party\|Party]]) |
| clan checks | you are never in a clan yet: "has no clan" passes, other clan checks fail |

Rewards:

| On the page | Means |
| --- | --- |
| you get the quest *X* | adds the quest to your list |
| the quest becomes *X* (progress kept) | swaps the quest for the next one and keeps its items, switches, variables and timer; without "progress kept" it starts fresh |
| works on *X* | picks which of your quests the next rewards change |
| you get N × *item*, *item* is taken | quest items go into the quest, other items into your bag; a full bag drops the item at your feet |
| experience, Zuly, item count with "reward formula N" | a calculated reward, see Reward formulas below |
| set / add / take for stats, Zuly, skill points | changes your character; taking Zuly you don't have fails the step |
| the quest ends (removed from your list) | the quest is done |
| you learn *skill* | learns the skill; if you can't, the step still goes on |
| you are moved to (x, y) in *zone* | teleport |
| N × *monster* appear around you | up to 20 at a time; they don't respawn |
| the NPC says | the text shows as a notice (shouts start with "[Announcement]") |
| client script | effects in the original client; nothing happens here |

## Quest items

Quest items don't go into your bag. Each quest holds up to **6** kinds of quest item, and
"you carry N × *item*" counts what that quest holds. Abandoning the quest loses them.

## Time limits

Quests with a time limit show `time_limit_minutes` on their page, and the quest list
counts down "m:ss left". Time runs in world ticks of 10 seconds of real time, also while
you are offline. When it reaches 0 the quest stays in your list; steps that check "the
quest timer ≤ 0" then apply (usually the NPC tells you that you were too late).

> Open question: a quest whose time ran out stays in the list until a step or Abandon removes it. Should it be removed or marked failed by itself?

## Abandoning

The **Abandon** button removes the quest and the quest items it holds. Switches and
progress counters outside the quest stay as they are.

## Reward formulas

Quest pages write calculated rewards like "Zuly, base 200 (reward formula 3 ...)". The
base comes from the quest data, and the formula number says how your character changes
it. All the arithmetic is in whole numbers: every division drops the remainder.

The values the server puts in:

- **R**, the world reward rate, is **300** (the server's default; an admin can change it).
- **F**, fame, is always **0** (fame is not in the game yet).
- **L** is your level and **C** your Charm (your basic Charm stat, without item bonuses).
- **B** is the base from the page.

### Formula 0 (experience)

```math
\text{reward} = \left\lfloor \frac{\left\lfloor \dfrac{(B + 30)(C + 10) \cdot R \cdot (F + 20)}{L + 70} \right\rfloor}{30000} \right\rfloor + B
```

Example: base 200, level 1, Charm 10: (230 × 20 × 300 × 20) = 27,600,000; ÷ 71 = 388,732;
÷ 30,000 = 12; plus 200 = **212** experience. At level 50 with Charm 40 it is 219.

### Formula 1 (experience that grows with level)

```math
\text{reward} = \left\lfloor \frac{B \cdot (L + 3) \cdot (L + \lfloor C/2 \rfloor + 40) \cdot R}{10000} \right\rfloor
```

Example: base 200, level 1, Charm 10: 200 × 4 × (1 + 5 + 40) × 300 = 11,040,000; ÷ 10,000
= **1,104** experience. At level 50 with Charm 40 it is 34,980.

### Formula 2 (Zuly for repeats)

```math
\text{reward} = B \times \text{quest variable 9}
```

Quest variable 9 of the quest the step works on counts repeats (the quest's own steps
add 1 to it). Paying the reward sets it back to 0. Example: base 3,000 after 3 repeats
gives **9,000** Zuly; with no repeats counted it gives 0.

### Formula 3 (Zuly) and formula 5 (item count)

```math
\text{reward} = \left\lfloor \frac{\left\lfloor \dfrac{(B + 20)(C + 10) \cdot R \cdot (F + 20)}{L + 70} \right\rfloor}{30000} \right\rfloor + B
```

Example: base 200 Zuly, level 1, Charm 10: (220 × 20 × 300 × 20) = 26,400,000; ÷ 71 =
371,830; ÷ 30,000 = 12; plus 200 = **212** Zuly.

Formula 5 sets how many of a stackable item you get: 5 × [[items/consumable/1-health-vial-s|Health Vial (S)]]
at level 1 with Charm 10 gives (25 × 20 × 300 × 20) ÷ 71 ÷ 30,000 = 1, plus 5 = **6**. For
equipment the count is ignored and you get one. If the step names a gem, the item comes
with that gem set; otherwise an item that always has a socket gets one, and an item that
may have one gets it with a chance of (quality + 60) in 400.

### Formula 4 (not used by any quest)

```math
\text{reward} = \left\lfloor \frac{(B + 2)(L + C + 40)(F + 40) \cdot R}{140000} \right\rfloor
```

Example: base 200, level 1, Charm 10: 202 × 51 × 40 × 300 = 123,624,000; ÷ 140,000 = **883**.

### Formula 6 (not used by any quest)

```math
\text{reward} = \left\lfloor \frac{(B + 20)(L + C)(F + 20) \cdot R}{3000000} \right\rfloor + B
```

Example: base 200, level 1, Charm 10: 220 × 11 × 20 × 300 = 14,520,000; ÷ 3,000,000 = 4;
plus 200 = **204**.

A calculated reward of 0 or less gives nothing.

> Open question: the formulas multiply in 32-bit whole numbers, so large bases overflow. With formula 0 or 3 at Charm 10 a base above about 17,800 wraps around (at Charm 100 above about 3,200); for example base 12,000 at level 60 with Charm 30 gives 11,640 instead of 12,740. Quests with bases like 80,000 to 150,000 are affected. Should the game use bigger numbers?

## Changed from iROSE

- The world reward rate is 300 (rose-offline's default), so the Charm bonus in formulas
  0, 3 and 5 is three times what a rate of 100 gives, and formula 1 is three times as large.
- Clans, fame and teams are not in the game: clan rewards fail the step, fame counts as 0
  and team rewards do nothing.

> Open question: only the player who lands the killing blow runs a monster's death step; party members get no kill credit or quest drops. Is that how iROSE shares quest kills?
