use crate::egj2025::item::{Item, ItemKind};
use crate::egj2025::level::Level;

pub const LEVELS: [fn() -> Level; 2] = [level0, level1];

fn level0() -> Level {
    let mut items: [Option<Item>; 32] = [const { None }; 32];
    items[0] = Some(Item::new(0, ItemKind::Melon, 96, 2, 8));
    items[1] = Some(Item::new(1, ItemKind::Watermelon, 34, 2, 15));
    const PAIRS: [(ItemKind, ItemKind); 1] = [(ItemKind::Melon, ItemKind::Watermelon)];
    Level { items, pairs: &PAIRS }
}

fn level1() -> Level {
    let mut items: [Option<Item>; 32] = [const { None }; 32];
    items[0] = Some(Item::new(0, ItemKind::Melon, 96, 2, 8));
    items[1] = Some(Item::new(1, ItemKind::Watermelon, 34, 2, 15));
    items[2] = Some(Item::new(2, ItemKind::Watermelon, 0, 2, 24));
    const PAIRS: [(ItemKind, ItemKind); 1] = [(ItemKind::Watermelon, ItemKind::Watermelon)];
    Level { items, pairs: &PAIRS }
}
