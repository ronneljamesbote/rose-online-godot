---
name: implement-wiki-change
description: Implement a wiki pull request in the game. Use when someone shares a pull request that edits wiki/ pages (or asks to make the game match the wiki), so the code or data change lands in that same pull request.
argument-hint: "<pull request link or number>"
---

# Implement a wiki change

The wiki (`wiki/`) is the source of truth. A pull request that edits it is a spec: make the
game match it inside the same pull request, so that merging changes the page and the game
together. Read `AGENTS.md` and `wiki/README.md` first if you haven't.

## Steps

1. **Read the change.** Open the pull request ($ARGUMENTS) and read its diff of `wiki/`.
   For every edited page note: the page (`kind` and `id` in its front matter), each value
   or sentence before and after, and anything added or removed. Ask the person only when a
   change can be read two ways; otherwise state how you read it.
2. **Check out the pull request's branch** and work on it. Don't open a second pull request
   unless you can't push to that branch; then branch from it and open a pull request into
   it, and say so.
3. **Decide where each change belongs.** The page's `source:` names the data row and the
   code. `.claude/rules/wiki-sync.md` maps code to pages.
   - A **data value** (an item's stat or price, a monster's level, HP or drop rate, what it
     drops, what a shop sells, a skill level's cost, power or cooldown, a status effect's
     amount per second, who may learn a skill, a zone setting, a warp gate's target, a
     crafting recipe, what a new character starts with) → the resource in `godot/data/`
     (`items/`, `monsters/`, `npcs/`, `drop_tables/`, `shop_tabs/`, `skills/<skill>/` (one
     file per level), `status_effects/`, `job_classes/`, `zones/`, `warp_gates/`,
     `craft_recipes/`, `new_characters/`, `effect_files/`, `attack_effects/`, `quests/`,
     `quest_triggers/`, `dialogs/`, `ai/`; the page's `source:` names the file or folder),
     then `godot --headless --path godot -- --export-data` and commit both. See
     `godot/data/README.md`. A spawn, NPC or named spot of a zone edited in Godot → its
     scene in `godot/zones/`, exported the same way. A value still read from the iROSE
     files (a zone not converted to a scene yet) can't be changed as data yet: say so on
     the pull request, or change it in code if the page asks for a rule.
   - A **rule or formula** (damage, hit chance, XP, death penalty, how a skill behaves) →
     code in `crates/rose-game-irose/` or `module/src/`, and the client in `godot/` when
     players see it (tooltips, windows).
   - A **new feature** described on a page → build it; a page with
     `status: not-in-game-yet` changed to `in-game` means the feature is wanted now.
4. **Implement it.** Keep the change to what the page asks. If the page's worked examples
   (for example "ATK 100 against DEF 50 = 87 damage") are given, add them as tests and make
   them pass.
5. **Test it in the game.** Build the module and the client, run the server, and show the
   change working: a screenshot or short video of the new value or behaviour (damage
   numbers, tooltip, drop, dialog). Attach it when you report; media is not committed.
6. **Update the page's details** that the edit implies but didn't write: `source:` if the
   code moved, "Changed from iROSE" if we now differ from iROSE, other pages that repeat
   the same value (search `wiki/` for it). Run `node web/scripts/check-wiki.mjs`.
7. **Rebuild what players download** when the client or the data export
   file changed: the Windows client zip includes `export/data/`, and the server uploads
   `export/` with the game data. Only when the person asks for a
   build.
8. **Push to the pull request** and reply with what changed in the game, the media, and
   anything you could not do. Never merge it yourself.

## Don'ts

- Don't change a page's meaning to fit the code. If something can't be done as written,
  say why on the pull request and leave the page as the person wrote it.
- Don't edit the iROSE data files; change the resource in `godot/data/`.
- Don't leave a value in two places: if code hard-codes a number the wiki now owns, read it
  from data, or at least point `source:` at the constant.
