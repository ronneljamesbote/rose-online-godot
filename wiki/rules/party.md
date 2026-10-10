---
kind: rule
id: party
name: Party
status: changed-from-irose
max_party_size: 5
invite_range_m: 30
invite_expires_s: 30
share_range_m: 50
xp_bonus_per_extra_member: 10%
party_level: always 1 (parties don't level up)
xp_sharing:
  - { rule: "Equally", who: "Every member nearby gets the same share" }
  - { rule: "By level", who: "Each member nearby gets a share in proportion to their level" }
item_sharing:
  - { rule: "Picker, Zuly split", items: "Go to the member who picks them up", zuly: "Split evenly between members nearby" }
  - { rule: "In turn", items: "Go to the members nearby in turn", zuly: "Go to the members nearby in turn (a separate turn from items)" }
xp_bonus_examples:
  - { members_nearby: 1, total_xp: "100%" }
  - { members_nearby: 2, total_xp: "110%" }
  - { members_nearby: 3, total_xp: "120%" }
  - { members_nearby: 4, total_xp: "130%" }
  - { members_nearby: 5, total_xp: "140%" }
source:
  code:
    - module/src/party.rs (party_invite, party_accept, party_decline, party_leave, party_kick, party_set_leader, party_set_rules, remove_member, reward_kill_xp, pickup_receiver, split_money, expire_invites, quest_party)
    - module/src/lib.rs (reward_kill)
    - module/src/items.rs (pickup_item)
    - module/src/chat.rs (send_chat, party channel)
    - module/src/skills.rs (basic command Party Invite)
    - godot/scripts/party_window.gd
    - godot/scripts/ui/hud/party_frames.gd
---
# Party

Up to five players can team up in a party. Members who are close together share the
experience from kills and the drops they pick up, and the party gets a small experience
bonus for every extra member who shares a kill.

## Making a party

- Right-click another player and pick **Invite to party** (or use the
  [[skills/19-party|Party]] action on them if it is on your skill list). They must be
  online, in the same zone and within **30 m** of you.
- The invited player sees "*name* invites you to a party" with **Accept** and **Decline**.
  An invitation that isn't answered within **30 seconds** is gone.
- When the first invitation is accepted, the party is made with the inviter as leader.
- You can't invite yourself, a player who is already in a party, or anyone when the party
  already has **5** members. Only the leader can invite once a party exists.
- Accepting an invitation clears any other invitations you had.

## The leader

The leader is marked with a star in the party frame. The leader can:

- invite players,
- make another member the leader,
- remove a member from the party,
- choose how experience and drops are shared (the two rules below).

When the leader leaves, the member who joined earliest becomes the leader. When only one
member is left, the party ends. Members who log out stay in the party and show as
"(offline)".

## Sharing experience

Experience from a kill is first worked out for each player who hurt the monster (see
[[rules/experience|Experience]]). If that player is in a party, their experience is shared
with the party members who are **online, in the same zone and within 50 m of the monster**
when it dies (the player themself always counts). If nobody else is near, the player keeps
it all.

With more than one member sharing, the total grows by **10% for each member after the
first**:

```math
\text{total} = \left\lfloor \frac{\text{XP} \times (100 + 10 \times (n - 1))}{100} \right\rfloor
```

where $n$ is the number of members sharing (2 to 5). The total is then split:

- **Equally**: each member gets $\lfloor \text{total} / n \rfloor$.
- **By level**: each member gets $\lfloor \text{total} \times \text{their level} / \text{sum of the levels} \rfloor$.

Every member gets at least 1 experience. Each share is added on its own, so every member
also earns [[rules/stamina|stamina]] from their share.

**Worked example.** A level 15 player kills a monster worth 257 experience to them, with
two party members nearby at levels 12 and 20 ($n = 3$).

- Total: $257 \times 120 / 100 = 308.4$, so **308**.
- Equally: $308 / 3 = 102.67$, so each of the three gets **102**.
- By level: the levels add up to $12 + 15 + 20 = 47$. The level 12 member gets
  $308 \times 12 / 47 = 78.6$, so **78**; the level 15 member gets $308 \times 15 / 47 = 98.3$,
  so **98**; the level 20 member gets $308 \times 20 / 47 = 131.06$, so **131**.

When several members hurt the same monster, each of them earns experience for their own
damage, and each of those amounts is shared as above.

## Sharing drops

Drops belong to the player who killed the monster and their party for the first 60 seconds
(see [[rules/drops|Drops]]), so any member may pick them up. Who gets a picked-up drop
depends on the leader's item rule, counting the members who are **online, in the same zone
and within 50 m of the drop**:

- **Picker, Zuly split** (the default): items go to whoever picks them up. Zuly is split
  evenly between the members nearby; the picker also gets what is left over.
- **In turn**: each drop goes to the next member nearby, in the order they joined. Items and
  Zuly keep separate turns.

If the picker is the only member nearby, they get the drop.

**Worked example (Zuly split).** Three members are near a pile of 1,234 Zuly. Each share is
$\lfloor 1234 / 3 \rfloor = 411$ and the remainder is $1234 - 3 \times 411 = 1$, so the
picker gets 412 and the other two get 411 each.

## Party chat

Start a chat line with `#` to talk to your party. See [[rules/chat|Chat]].

## Changed from iROSE

- iROSE parties have a party level and their own experience; here a party is always level 1
  (quests that check the party level see 1).
- The party experience bonus of 10% per extra member is our own rule. The iROSE party bonus
  formula isn't in the code we started from.
- The share range (50 m) and invite range (30 m) are our values.

> Open question: dead members within 50 m still receive a share of kill experience and drops; the code does not check whether a member is alive.
