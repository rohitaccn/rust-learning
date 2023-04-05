use std::collections::HashMap;
use std::error::Error;

#[derive(Debug)]
struct CartItem {
    name: String,
    price: f32,
    quantity: u32,
}

#[derive(Debug)]
struct ShoppingCart {
    items: HashMap<String, CartItem>,
}

impl ShoppingCart {
    fn new() -> Self {
        Self {
            items: HashMap::new(),
        }
    }

    fn add_item(&mut self, item: CartItem) {
        let name = item.name.clone();
        let cart_item = self.items.entry(name).or_insert(item);
        cart_item.quantity += 1;
    }

    fn remove_item(&mut self, name: &str) -> Result<(), Box<dyn Error>> {
        match self.items.remove(name) {
            Some(_) => Ok(()),
            None => Err(Box::from("Item not found in the cart")),
        }
    }

    fn get_total(&self) -> f32 {
        self.items
            .values()
            .map(|item| item.price * item.quantity as f32)
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_item() {
        let mut cart = ShoppingCart::new();
        let item = CartItem {
            name: "item1".to_string(),
            price: 2.5,
            quantity: 1,
        };
        cart.add_item(item);
        assert_eq!(cart.items.len(), 1);
    }

    #[test]
    fn test_remove_item() {
        let mut cart = ShoppingCart::new();
        let item = CartItem {
            name: "item1".to_string(),
            price: 2.5,
            quantity: 1,
        };
        cart.add_item(item);
        assert_eq!(cart.items.len(), 1);
        cart.remove_item("item1").unwrap();
        assert_eq!(cart.items.len(), 0);
    }

    #[test]
    fn test_get_total() {
        let mut cart = ShoppingCart::new();
        let item1 = CartItem {
            name: "item1".to_string(),
            price: 2.5,
            quantity: 2,
        };
        let item2 = CartItem {
            name: "item2".to_string(),
            price: 3.5,
            quantity: 1,
        };
        cart.add_item(item1);
        cart.add_item(item2);
        println!("{:?}",cart.get_total());
        assert_eq!(cart.get_total(), 14.5);
    }
}
