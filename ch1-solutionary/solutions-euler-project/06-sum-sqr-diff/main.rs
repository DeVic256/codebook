fn sum_sqr_diff(n: u32) -> u32 {
    let sqrn: u32 = n.pow(2);
    n*(3*n + 2)*(sqrn - 1) / 12
}

fn main() {
    let r: u32 = sum_sqr_diff(100);

    println!("{}", r);
}