use crate::egj2025::item::{Item, ItemKind};
use crate::egj2025::level::Level;

pub const LEVELS: [fn() -> Level; 2] = [level0, level1];

fn level0() -> Level {
    let mut items: [Option<Item>; 32] = [const { None }; 32];
    items[0] = Some(Item::new(0, ItemKind::Melon, 128, 2, 24));
    items[1] = Some(Item::new(1, ItemKind::Watermelon, 66, 2, 31));
    const PAIRS: [(ItemKind, ItemKind); 1] = [(ItemKind::Melon, ItemKind::Watermelon)];
    Level { items, pairs: &PAIRS }
}

fn level1() -> Level {
    let mut items: [Option<Item>; 32] = [const { None }; 32];
    items[0] = Some(Item::new(0, ItemKind::Melon, 128, 2, 24));
    items[1] = Some(Item::new(1, ItemKind::Watermelon, 66, 2, 31));
    items[2] = Some(Item::new(2, ItemKind::Watermelon, 32, 2, 40));
    const PAIRS: [(ItemKind, ItemKind); 1] = [(ItemKind::Watermelon, ItemKind::Watermelon)];
    Level { items, pairs: &PAIRS }
}
