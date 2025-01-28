fn is_prime(n: u64, primes: &Vec<u64>) -> bool {
    for i in primes {
        if n % i == 0 { return false; }
    }

   return true; 
}

// Gets the nth prime
fn nth_prime(n: u64) -> u64 {
    let mut primes: Vec<u64> = vec![2, 3, 5, 7, 11, 13];
    let mut i = 14;
    let mut pcount = 6;

    while pcount <= n {
        if is_prime(i, &primes) { 
            primes.push(i);
            pcount += 1;
        }
        i += 1;
    }

    match primes.get(n as usize - 1) {
        Some(n) => *n,
        None    => 0
    }
}

fn main() {
    println!("{}", nth_prime(10001))
}