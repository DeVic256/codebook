use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

enum Order {
    Normal,
    Reverse
}

fn extremae_digits(str: &str, ord: Order) -> char {
    let chars = str.chars();

    match ord {
        Order::Normal => {
            for i in chars {
                if i.is_digit(10) { return i }           
            }
        }
        Order::Reverse => {
            for i in chars.rev() {
                if i.is_digit(10) { return i }           
            }
        }
    }

    return '0';
}

fn extremae_to_i(str: &str) -> u32 {
    let num = format!("{}{}", extremae_digits(&str, Order::Normal), extremae_digits(&str, Order::Reverse));
    
    num.parse::<u32>().expect("No number given")
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);

    let mut result = 0;

    for line in reader.lines() {
        match line {
            Ok(word) => result += extremae_to_i(&word),
            Err(e) => println!("{}", e),
        }
    }

    println!("{}", result);

    Ok(())
}