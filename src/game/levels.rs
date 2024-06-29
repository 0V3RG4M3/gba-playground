use crate::game::item::{Item, ItemKind};
use crate::game::level::Level;

pub fn first() -> Level<6, 2> {
    let available_items = [
        Item::new(0, ItemKind::Banana, 8, 2, 8),
        Item::new(1, ItemKind::Banana, 64, 2, 8),
        Item::new(2, ItemKind::BoarThigh, 32, 2, 8),
        Item::new(3, ItemKind::BoarThigh, 48, 2, 8),
        Item::new(4, ItemKind::Melon, 96, 2, 8),
        Item::new(5, ItemKind::Watermelon, 96, 2, 16),
    ];
    let recipe_items = [ItemKind::Banana, ItemKind::BoarThigh];
    Level { available_items, recipe_items }
}
