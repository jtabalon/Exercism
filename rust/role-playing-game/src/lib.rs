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
        if self.level < 10 {
            if mana_cost <= self.health {
                self.health -= mana_cost;
            }
            else {
                self.health = 0;
            }
            return 0
        }
        if let Some(mana) = self.mana {
            if mana >= mana_cost { 
                self.mana = Some(mana - mana_cost);
                return mana_cost * 2;
            }
            else {
                return 0;
            }
        }

        self.health -= mana_cost;
        return 0;

    }
}
