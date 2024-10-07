use std::fs::File;
use std::io::Read;

fn advance(c: char) -> i32 {
    match c {
        '(' =>  1,
          _ => -1,
    }
}

fn floor(str: &str) -> i32 {
    let mut result = 0;

    for c in str.chars() {
       result += advance(c);
    }

    result
}

fn check_bsmt(str: &str) -> i32 {
    let mut result: i32 = 0;
    let chars = str.chars();

    for (i, c) in chars.enumerate() {
        result += advance(c);
        if result < 0 { return i as i32 + 1i32}
    }

    return -1i32;
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("input");
    let mut input = String::new();

    file.expect("Could not read File").read_to_string(&mut input)?;

    // Part I
    println!("{}", floor(&input));

    // Part II
    println!("{}", check_bsmt(&input));

    Ok(())
}