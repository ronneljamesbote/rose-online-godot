---
kind: skill
id: 18
name: Add Friend
status: in-game
icon: skill/5
type: Basic Action
job: any
max_level: 0
target: Yourself
levels:
- level: 0
  id: 18
  learn_points: 0
  range: 0.1
source:
  data: LIST_SKILL.STB rows 18
  code: module/src/skills.rs
---
# Add Friend

Ask someone to be your friend.

Range and area are in metres, cooldown and duration in seconds, success in percent.
How power turns into damage is on [[rules/skills|Skills (rules)]].
