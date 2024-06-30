use crate::game::item::{Item, ItemKind};
use crate::game::level::Level;

pub fn first() -> Level<20, 5> {
    let available_items = [
        Item::new(0, ItemKind::Banana, 8, 2, 8),
        Item::new(1, ItemKind::Banana, 64, 2, 8),
        Item::new(2, ItemKind::BoarThigh, 32, 2, 8),
        Item::new(3, ItemKind::BoarThigh, 48, 2, 8),
        Item::new(4, ItemKind::Melon, 96, 2, 8),
        Item::new(5, ItemKind::Watermelon, 34, 2, 15),
        Item::new(6, ItemKind::Watermelon, 0, 2, 24),
        Item::new(7, ItemKind::Watermelon, 40, 2, 44),
        Item::new(8, ItemKind::IceCream0, 61, 2, 24),
        Item::new(9, ItemKind::IceCream0, 83, 2, 30),
        Item::new(10, ItemKind::IceCream1, 54, 2, 48),
        Item::new(11, ItemKind::Crab, 8, 2, 48),
        Item::new(12, ItemKind::Crab, 11, 2, 50),
        Item::new(13, ItemKind::Crab, 25, 2, 38),
        Item::new(14, ItemKind::Artichoke, 8, 2, 64),
        Item::new(15, ItemKind::Artichoke, 100, 2, 120),
        Item::new(16, ItemKind::Artichoke, 50, 2, 110),
        Item::new(17, ItemKind::SugarPaste, 8, 2, 80),
        Item::new(18, ItemKind::SugarPaste, 11, 2, 4),
        Item::new(19, ItemKind::SugarPaste, 14, 2, 37),
    ];
    let recipe_items = [
        ItemKind::Banana,
        ItemKind::BoarThigh,
        ItemKind::Crab,
        ItemKind::IceCream0,
        ItemKind::SugarPaste,
    ];
    Level { available_items, recipe_items }
}
