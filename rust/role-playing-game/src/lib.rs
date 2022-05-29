// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn revive(&self) -> Option<Player> {
        // below level 10
        if self.level <= 10 and self.health == 0 {
            Some(Player {health: 100, mana: None, level: self.level})
        }
        else {
            if self.health == 0 {
                Some(Player {health: 100, mana: Some(100), level: self.level})
            }
        }
        // geq level 10
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        unimplemented!("Cast a spell of cost {}", mana_cost)
    }
}
