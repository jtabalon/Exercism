// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub struct User {
    name: String,
    age: u32,
    weight: f32,
}

impl User {
    pub fn new(name: String, age: u32, weight: f32) -> Self {
        // unimplemented!()
        User {
            name: name, 
            age: age,
            weight: weight,
        }
    }

    pub fn name(&self) -> &str {
        // unimplemented!()
        let get_name = &self.name; // notice the &self to reference a string here.
        get_name
    }

    pub fn age(&self) -> u32 {
        // unimplemented!()
        let get_age = self.age; // notice there should not be an &
        get_age
    }

    pub fn weight(&self) -> f32 {
        let get_weight = self.weight;
        get_weight
        // unimplemented!()
    }

    pub fn set_age(&mut self, new_age: u32) {
        self.age = new_age;
        // unimplemented!()
    }

    pub fn set_weight(&mut self, new_weight: f32) {
        self.weight = new_weight;
        // unimplemented!()
    }
}
