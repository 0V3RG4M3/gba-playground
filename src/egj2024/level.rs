use crate::egj2024::item::{Item, ItemKind};

pub struct Level<const A: usize, const R: usize> {
    pub available_items: [Item; A],
    pub recipe_items: [ItemKind; R],
}
