fn place(n: &u64, i: u64) -> u64 {
    let p10 = 10_u64.pow(i as u32);
    let p10next = 10_u64.pow((i-1) as u32);

    ( ((n / p10next) * p10next) - ((n / p10) * p10) ) / p10next
}

fn is_palindrome_6(n: u64) -> bool {
    n == (place(&n, 1)*100001 + place(&n, 2)*1001 + place(&n, 3)*1100)
}

fn largest_palindrome_product() -> Option<u64> {
    for i in (200..999).rev() {
        for j in (500..999).rev() {
            let k = i*j;
            if is_palindrome_6(k) { return Some(k); }
        }
    }
    
    None
}

fn main() {
    println!("{}", largest_palindrome_product().unwrap());
}
