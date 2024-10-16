//FIXME: Doesn't work for n < 8
fn is_prime(n: u64) -> bool {
    let nf = n as f64;
    let sqrtn = nf.sqrt().round() as u64;

    for i in 2..sqrtn {
        if n % i == 0 { return false; }
    }

   return true; 
}

fn main() {
    let mut primes_count: u32 = 4;
    let i = 10;

    while primes_count <= 10001 {
        if is_prime(i) {
            primes_count += 1;
        }
    }

    //FIXME: Too slow - incomplete
    println!("{}", primes_count);
}