const PHI: f64 = 1.6180339887f64;
const PHISQR: f64 = PHI*PHI;
const PHICUB: f64 = PHISQR*PHI;

const PSI: f64 = 1f64 - PHI;
const PSISQR: f64 = PSI*PSI;
const PSICUB: f64 = PSISQR*PSI;

fn sqrtfive() -> f64 { 5.0_f64.sqrt() }

enum Efficiency { Fast, Slow }

fn fibi(n: i32) -> i32 {
    if n < 2 { 1 }
    else { fibi(n - 1) + fibi(n - 2) }
}

fn fibe(n: i32) -> i32 {
    ((PHI.powi(n + 1) - PSI.powi(n + 1)) / sqrtfive()).round() as i32
}

fn sum_nth_even_fib(n: i32, e: Efficiency) -> i32 {
    let mut result: i32 = 0;

    for i in 0..n {
        result += match e {
            Efficiency::Fast => fibe(2 + 3*i),
            Efficiency::Slow => fibi(2 + 3*i)
        }
    }

    result as i32
}

fn main() {
    let effib = sum_nth_even_fib(11, Efficiency::Fast);
    let fibstr: String = format!("{}", effib);

    println!("{}", fibstr);
}