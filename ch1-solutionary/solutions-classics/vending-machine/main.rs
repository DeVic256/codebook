mod vending_machine;
use vending_machine::VendingMachine;

fn main() {
    let mut vm: VendingMachine = VendingMachine::new();

    // Adding items to machine slots
    vm.add_item("Tea", 0.25f64);
    vm.add_item("Protein Bar", 1.75f64);
    vm.add_item("Protein Shake", 2.25f64);
    vm.add_item("Dark Chocolate", 1.25f64);

    // Buying items by index
    vm.buy_item(2);
    vm.buy_item(1);
    vm.buy_item(2);
    vm.buy_item(4);
    vm.buy_item(3);

    println!("{:?}", vm);

    vm.pay(10.0);
    vm.print_receipt();

    println!("{:?}", vm);
}