use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

enum Order { Normal, Reverse }
enum Part { One, Two }

const DIGITWORDS: [&str; 9] = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

fn extremae_digits(str: &str, ord: Order) -> char {
    let chars = str.chars();

    match ord {
        Order::Normal => { for i in chars { if i.is_digit(10) { return i } } }
        Order::Reverse => { for i in chars.rev() { if i.is_digit(10) { return i } } }
    }

    return '0';
}

fn extremae_to_i(str: &str) -> u32 {
    let num = format!("{}{}", extremae_digits(&str, Order::Normal), extremae_digits(&str, Order::Reverse));
    
    println!("From {} got {}", str, num);
    
    num.parse::<u32>().expect("No number given")
}

fn numbers_in_word(stro: String) -> Vec<(usize, usize)> {
    let mut result: Vec<(usize, usize)> = Vec::new();
    
    for (i, w) in DIGITWORDS.iter().enumerate() {
        // (find_index, digiword_index)
        let v = stro.match_indices(w).map(|(fidx, _)| (fidx, i) );
        
        result.extend(v);
    }

    // Sort by substr find-index
    result.sort_by( |(i, _), (j, _)| { i.cmp(&j) } );

    let le = result.len();

    // Returning [First, Last] only
    match le {
        0 => vec![(100usize, 100usize)],
        _ => vec![result[0], result[le - 1]],
    }
}

fn fixed(stro: String) -> String {
    let mut word = stro.clone();

    //vector with (find_idx, digit_idx)-like items
    let digits_in_extremes = numbers_in_word(stro);

    // Replacing the String finally
    for (fidx, idx) in digits_in_extremes.iter() {
        let str_index = (idx + 1).to_string();
        match fidx {
            100usize => return word,
                   _ => { word = word.replace(DIGITWORDS[*idx] , &str_index); }
        }
    }

    word
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let part = Part::Two;
    
    let file = File::open("input")?;
    let reader = BufReader::new(&file);

    let mut result = 0;

    for line in reader.lines() {
        match line {
            Ok(word) => { 
                match part {
                    Part::One => result += extremae_to_i(&word),
                    Part::Two => { result += extremae_to_i(&fixed(word)); }
                }
            } 
            Err(e) => println!("{}", e),
        }
    }

    println!("Result: {}", result);

    Ok(())
}