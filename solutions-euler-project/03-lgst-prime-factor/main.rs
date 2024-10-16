//FIXME: Doesn't work for n < 8
fn is_prime(n: u64) -> bool {
    let nf = n as f64;
    let sqrtn = nf.sqrt().round() as u64;

    for i in 2..sqrtn {
        if n % i == 0 { return false; }
    }

   return true; 
}

fn largest_prime_factor(n: u64) -> u64 {
    let nf = (n as f64).sqrt();
    let sqrtn = nf as u64;

    let mut result = 1;

    for i in 2..sqrtn {
        if is_prime(i) && n % i == 0 {
            result = i;
        }
    }

    result
}

fn main() {
    let n = 600851475143; // let sqrtn = 775146;
    let largest_prime = largest_prime_factor(n);

    println!("Largest prime => {}", largest_prime);
}