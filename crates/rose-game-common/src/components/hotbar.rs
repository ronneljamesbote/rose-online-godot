use serde::{Deserialize, Serialize};

use crate::components::{ItemSlot, SkillSlot};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum HotbarSlot {
    Inventory(ItemSlot),
    Command(u16),
    Skill(SkillSlot),
    Emote(u16),
    Dialog(u16),
    ClanSkill(u16),
}

pub const HOTBAR_PAGE_SIZE: usize = 8;
pub const HOTBAR_NUM_PAGES: usize = 4;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Hotbar {
    pub pages: [[Option<HotbarSlot>; HOTBAR_PAGE_SIZE]; HOTBAR_NUM_PAGES],
}

impl Hotbar {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn set_slot(&mut self, index: usize, slot: Option<HotbarSlot>) -> Option<()> {
        let page = self.pages.get_mut(index / HOTBAR_PAGE_SIZE)?;
        let page_slot = page.get_mut(index % HOTBAR_PAGE_SIZE)?;
        *page_slot = slot;
        Some(())
    }

    /// Empties the shortcuts to inventory slots whose item is gone (dropped, used up, sold...).
    pub fn clear_missing_items(&mut self, has_item: impl Fn(ItemSlot) -> bool) {
        for slot in self.pages.iter_mut().flatten() {
            if let Some(HotbarSlot::Inventory(item_slot)) = slot {
                if !has_item(*item_slot) {
                    *slot = None;
                }
            }
        }
    }

    /// Two inventory slots swapped their contents: shortcuts follow their item.
    pub fn swap_inventory_slots(&mut self, a: ItemSlot, b: ItemSlot) {
        for slot in self.pages.iter_mut().flatten() {
            if let Some(HotbarSlot::Inventory(item_slot)) = slot {
                if *item_slot == a {
                    *item_slot = b;
                } else if *item_slot == b {
                    *item_slot = a;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::InventoryPageType;

    #[test]
    fn shortcuts_follow_moved_items() {
        let (a, b, c) = (
            ItemSlot::Inventory(InventoryPageType::Consumables, 0),
            ItemSlot::Inventory(InventoryPageType::Consumables, 5),
            ItemSlot::Inventory(InventoryPageType::Consumables, 9),
        );
        let mut hotbar = Hotbar::new();
        hotbar.set_slot(0, Some(HotbarSlot::Inventory(a)));
        hotbar.set_slot(1, Some(HotbarSlot::Inventory(b)));
        hotbar.set_slot(2, Some(HotbarSlot::Inventory(c)));
        hotbar.swap_inventory_slots(a, b);
        let slots: Vec<_> = hotbar.pages[0][..3]
            .iter()
            .map(|s| match s {
                Some(HotbarSlot::Inventory(slot)) => *slot,
                _ => panic!("not an item"),
            })
            .collect();
        assert_eq!(slots, vec![b, a, c]);

        hotbar.clear_missing_items(|slot| slot != c);
        assert!(hotbar.pages[0][0].is_some() && hotbar.pages[0][1].is_some());
        assert!(hotbar.pages[0][2].is_none());
    }
}
