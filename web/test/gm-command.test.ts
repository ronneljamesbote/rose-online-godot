import assert from "node:assert/strict";
import { test } from "node:test";
import { gmCommand } from "../src/lib/gm-command.ts";

test("item, monster and zone pages get their GM command", () => {
  assert.equal(gmCommand({ kind: "item", id: "weapon/43" }), "/item 8 43");
  assert.equal(gmCommand({ kind: "item", id: "subweapon/5" }), "/item 9 5");
  assert.equal(gmCommand({ kind: "item", id: "vehicle/12" }), "/item 14 12");
  assert.equal(gmCommand({ kind: "monster", id: 101 }), "/spawn 101");
  assert.equal(gmCommand({ kind: "zone", id: 20 }), "/tp 20");
});

test("pages without a valid command get none", () => {
  assert.equal(gmCommand({ kind: "item", id: "weapon" }), null);
  assert.equal(gmCommand({ kind: "item", id: "nothing/3" }), null);
  assert.equal(gmCommand({ kind: "monster", id: 101, status: "not-in-game-yet" }), null);
  assert.equal(gmCommand({ kind: "zone" }), null);
  assert.equal(gmCommand({ kind: "npc", id: 1001 }), null);
  assert.equal(gmCommand({ kind: "skill", id: 1 }), null);
});
