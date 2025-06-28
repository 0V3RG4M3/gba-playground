use crate::egj2025::item::{Item, ItemKind};

pub struct Level {
    pub items: [Option<Item>; 32],
    pub pairs: &'static [(ItemKind, ItemKind)],
}

impl Level {
    pub fn process(&self, player_item_index: Option<usize>) -> bool {
        self.pairs.iter().all(|(p0, p1)| {
            for (i0, e0) in self.items.iter().enumerate() {
                if Some(i0) == player_item_index {
                    continue;
                }
                let e0 = match e0 {
                    Some(e0) if e0.kind == *p0 => e0,
                    _ => continue,
                };
                for (i1, e1) in self.items.iter().enumerate() {
                    if i1 == i0 {
                        continue;
                    }
                    if Some(i1) == player_item_index {
                        continue;
                    }
                    let e1 = match e1 {
                        Some(e1) if e1.kind == *p1 => e1,
                        _ => continue,
                    };
                    let pos = e1.sprite.pos - e0.sprite.pos;
                    let sq_dist = pos.dot(pos);
                    if sq_dist.into_int() < 32 * 32 {
                        return true;
                    }
                }
            }
            return false;
        })
    }
}
