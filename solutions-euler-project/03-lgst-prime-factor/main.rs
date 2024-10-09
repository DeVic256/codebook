fn is_prime(n: u64) -> bool {
    let nf = n as f64;
    let sqrtn = nf.sqrt().round() as u64;

    for i in 2..sqrtn {
        if n % i == 0 { return false; }
    }

   return true; 
}

fn main() {

    let n = 600851475143;
    // let sqrtn = (n as f64).sqrt() as u64;
    let sqrtn = 775146;

    let mut largest_prime = 1;

    for i in 2..sqrtn {
        if is_prime(i) && n % i == 0 {
            largest_prime = i;
        }
    }

    println!("Largest prime => {}", largest_prime);
}