use std::collections::HashMap;
/*
This is an example of Clean Architecture, where the domain logic is separated from the data access and interface layers. 
It shows how the different layers interact with each other while keeping them separate and testable. 
The domain layer holds the business logic and the data layer holds the logic for reading and writing to the data store. 
The interface layer provides the service for the outside world to interact with the business logic.
*/
// Domain layer
pub struct ShoppingCart {
    items: HashMap<String, u32>,
}

impl ShoppingCart {
    pub fn new() -> Self {
        Self { items: HashMap::new() }
    }

    pub fn add_item(&mut self, item: &str, qty: u32) {
        let count = self.items.entry(item.to_string()).or_insert(0);
        *count += qty;
    }

    pub fn remove_item(&mut self, item: &str) -> Result<(), &str> {
        match self.items.remove(item) {
            Some(_) => Ok(()),
            None => Err("Item not found"),
        }
    }

    pub fn get_items(&self) -> &HashMap<String, u32> {
        &self.items
    }
}

// Data layer
pub struct ItemRepository {
    data: HashMap<String, u32>,
}

impl ItemRepository {
    pub fn new() -> Self {
        Self { data: HashMap::new() }
    }

    pub fn save(&mut self, item: &str, qty: u32) {
        self.data.insert(item.to_string(), qty);
    }

    pub fn get(&self, item: &str) -> Option<&u32> {
        self.data.get(item)
    }

    pub fn remove(&mut self, item: &str) -> Option<u32> {
        self.data.remove(item)
    }
}

// Interface layer
pub struct ShoppingCartService {
    cart: ShoppingCart,
    repo: ItemRepository,
}

impl ShoppingCartService {
    pub fn new(cart: ShoppingCart, repo: ItemRepository) -> Self {
        Self { cart, repo }
    }

    pub fn add_item(&mut self, item: &str, qty: u32) {
        self.repo.save(item, qty);
        self.cart.add_item(item, qty);
    }

    pub fn remove_item(&mut self, item: &str) -> Result<(), &str> {
        match self.repo.remove(item) {
            Some(_) => {
                self.cart.remove_item(item)?;
                Ok(())
            }
            None => Err("Item not found"),
        }
    }

    pub fn get_items(&self) -> &HashMap<String, u32> {
        self.cart.get_items()
    }
}
