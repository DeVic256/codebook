fn sum_to_n(n: u32) -> u32 {
    n * (n + 1) / 2
}

fn mul() -> u32 {
    let [mul3, mul5, mul15] = [3, 5, 15].map(|n| n*sum_to_n(1000/n));

    mul3 + mul5 - mul15
}

fn main() {
    println!("{}", mul())
}