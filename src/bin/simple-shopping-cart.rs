use std::collections::HashMap;
use proptest::prelude::*;

struct ShoppingCart {
    items: HashMap<String, i32>,
}

impl ShoppingCart {
    fn new() -> Self {
        Self { items: HashMap::new() }
    }

    fn add_item(&mut self, item: &str, qty: i32) {
        let count = self.items.entry(item.to_string()).or_insert(0);
        *count += qty;
    }

    fn remove_item(&mut self, item: &str) -> Result<(), &str> {
        match self.items.remove(item) {
            Some(_) => Ok(()),
            None => Err("Item not found"),
        }
    }

    fn get_items(&self) -> &HashMap<String, i32> {
        &self.items
    }
}


proptest! {
    #[test]
    fn test_shopping_cart(item in "\\PC*", qty in 1..100i32) {
        let mut cart = ShoppingCart::new();
        cart.add_item(&item, qty);
        let items = cart.get_items();
        prop_assert_eq!(items[&item], qty);
    }
    #[test]
    fn add_item_increases_count(item in "\\PC*", qty in 1..100i32) {
        let mut cart = ShoppingCart::new();
        {
        let count = cart.items.entry(item.to_string()).or_insert(0);
        }
        cart.add_item(&item, qty);
        prop_assert_eq!(*count, qty);
    }

    #[test]
    fn remove_item_decreases_count(item in "\\PC*", qty in 1..100i32) {
        let mut cart = ShoppingCart::new();
        cart.add_item(&item, qty);
        let count = cart.items.get(&item.to_string()).unwrap();
        cart.remove_item(&item).unwrap();
        prop_assert_eq!(*count, qty);
    }

    #[test]
    fn remove_item_fails_if_not_exists(item in "\\PC*") {
        let mut cart = ShoppingCart::new();
        prop_assert_eq!(cart.remove_item(&item), Err("Item not found"));
    }
}