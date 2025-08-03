use std::collections::HashMap;

pub struct Menu {
    pub name: String,
    pub menu_contents: HashMap<String, f32>, 
}

impl Menu {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            menu_contents: HashMap::new(),
        }
    }

    pub fn add_content(&mut self, ingredient_name: &str, price: f32) {
        self.menu_contents.insert(ingredient_name.to_string(), price);
    }

    pub fn total_price(&self) -> f32 {
        self.menu_contents.values().sum()
    }

    pub fn update_content(&mut self, ingredient_name: &str, new_price: f32){
        println!("Updated ingredient: ", ingredient_name);
        self.menu_contents.insert(ingredient_name, new_price);
    }

    pub fn remove_product(&mut self, ingredient_name: &str){
        self.menu_contents.remove(ingredient_name);
    }

    pub fn create_name(&mut self, name: &str){
        self.name = name;
    }

    pub fn update_name(&mut self, new_name: &str){
        self.name = name;
    }

}