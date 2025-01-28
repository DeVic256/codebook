use std::fs::File;
use std::io::prelude::*;

// Returns the highest product of a 13 digit seq from a 13+ seq
fn higher(seq: &str) -> u64 {
    let mut lb = 0; //lower bound
    let mut ub = 12; //upper bound
    let mut product = 1;
    let mut highest = 1;

    while ub < seq.len() {
        product = seq[lb..ub + 1].chars()
                             .map( |c| c.to_digit(10).unwrap() as u64 )
                             .reduce( |a, b| a * b ).expect("Not an integer");
        lb += 1; ub += 1;
        if product > highest { highest = product; }
    }

    return highest as u64;
}

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let mut file = File::open("input")?;
    let mut input = String::new();
    file.read_to_string(&mut input);

    let result = input.split('0')
         .filter( |seq| seq.len() >= 13 )
         .map( |seq| higher(seq) ) // get longest 13 char long seq
         .max();

    println!("{}", result.unwrap());

    Ok(())
}