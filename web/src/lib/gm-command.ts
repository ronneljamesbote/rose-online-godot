// The dev-client GM command for an item, monster or zone page, made from the page's id:
// /item type number, /spawn monster-id, /tp zone (wiki/rules/gm-commands.md, module/src/gm.rs).

// iROSE item types by wiki/items folder, as /item takes them.
const ITEM_TYPES: Record<string, number> = {
  face: 1,
  head: 2,
  body: 3,
  hands: 4,
  feet: 5,
  back: 6,
  jewellery: 7,
  weapon: 8,
  subweapon: 9,
  consumable: 10,
  gem: 11,
  material: 12,
  quest: 13,
  vehicle: 14,
};

const U16 = (s: string) => /^\d+$/.test(s) && Number(s) <= 65535;

/** The command to copy for a page, or null when it has none. */
export function gmCommand(data: Record<string, unknown>): string | null {
  if (data.status === "not-in-game-yet" || data.id === undefined || data.id === null) return null;
  const id = String(data.id);
  switch (data.kind) {
    case "item": {
      const [folder, number] = id.split("/");
      const type = ITEM_TYPES[folder];
      return type && number && U16(number) && Number(number) > 0 ? `/item ${type} ${number}` : null;
    }
    case "monster":
      return U16(id) && Number(id) > 0 ? `/spawn ${id}` : null;
    case "zone":
      return U16(id) && Number(id) > 0 ? `/tp ${id}` : null;
    default:
      return null;
  }
}
