use std::boxed::Box;

type CartItem = Box<dyn Fn() -> String  + 'static>;

struct ShoppingCart {
    items: Vec<CartItem>,
}

impl ShoppingCart {
    fn new() -> Self {
        ShoppingCart { items: vec![] }
    }

    fn add_item<T>(&mut self, item: T)
    where
        T: Fn() -> String + 'static ,
    {
        self.items.push(Box::new(item));
    }

    fn print_cart(&self) {
        for item in &self.items {
            println!("{}", item());
        }
    }
}

fn main() {
    let mut cart = ShoppingCart::new();
    cart.add_item(|| "apple".to_owned());
    cart.add_item(|| "banana".to_owned());
    cart.print_cart();
}
