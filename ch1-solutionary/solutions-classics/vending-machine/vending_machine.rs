#[derive(Debug, Clone)]
struct Item {
    name: String,
    price: f64
}

#[derive(Debug)]
pub struct VendingMachine {
    items: Vec<Item>,
    sold_items: Vec<Item>,
    change: Change, 
    balance: f64
}

#[derive(Debug)]
struct Change {
    quarters: u64,
    dimes: u64,
    cents: u64
}

impl Change {
    pub fn reset(&mut self) {
        self.quarters = 0;
        self.dimes = 0;
        self.cents = 0;
    }
}

impl VendingMachine {
    pub fn new() -> VendingMachine {
        VendingMachine { items: vec![], balance: 0.0f64, sold_items: vec![], change: Change { quarters: 0, dimes: 0, cents: 0 } }
    }
    
    pub fn add_item(&mut self, name: &str, price: f64) {
        self.items.push( Item { name: name.to_string(), price: price } );
    }

    pub fn  get_balance(&self) -> f64 {
        self.balance
    }

    pub fn buy_item(&mut self, i: usize) {
        match self.items.get(i - 1) {
            Some(item) => { 
                self.balance += (*item).price; 
                self.sold_items.push(item.clone());
            },
            None => println!("Item not found")
        }
    }

    fn give_change(&mut self) {
        println!("\n{:?}\n", self.change);
        self.change.reset();
        self.balance = 0.0;
    }

    fn set_change(&mut self, m: f64) {
        let tens = m % 0.25;
        let cents = tens % 0.10;

        self.change.quarters = (m / 0.25 ) as u64;
        self.change.dimes = (tens / 0.10 ) as u64;
        self.change.cents = (cents / 0.10 ) as u64;

        self.give_change();
    }

    pub fn pay(&mut self, m: f64) -> bool {
        let diff: f64 = m - self.get_balance();
        self.balance -= m;

        if diff < 0.0 { return false; }

        self.set_change(diff);
        return true;
    }

    pub fn print_receipt(&mut self) {
        println!("");

        let mut sold_price = 0.0;

        for item in &self.sold_items {
            println!("Bought {} ....... ${}", item.name, item.price);
            sold_price += item.price;
        }

        println!("Total ...................... ${}", sold_price);
        println!("Balance ...................... ${}.00\n", self.balance);
    }
}
