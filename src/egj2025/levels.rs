use crate::egj2025::item::{Item, ItemKind};
use crate::egj2025::level::Level;

pub fn first() -> Level {
    let mut items: [Option<Item>; 32] = [const { None }; 32];
    items[0] = Some(Item::new(4, ItemKind::Melon, 96, 2, 8));
    items[1] = Some(Item::new(5, ItemKind::Watermelon, 34, 2, 15));
    const PAIRS: [(ItemKind, ItemKind); 1] = [(ItemKind::Melon, ItemKind::Watermelon)];
    Level { items, pairs: &PAIRS }
}
