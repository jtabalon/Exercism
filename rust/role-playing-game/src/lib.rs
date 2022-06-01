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
        if self.health > 0 {
            return None;
        }

        Some(Self {
            health: 100,
            level: self.level, 
            mana: if self.level >= 10 { Some(100) } else { None },
        })
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {

        
    }
}
