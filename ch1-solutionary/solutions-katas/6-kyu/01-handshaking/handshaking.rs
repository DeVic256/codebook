fn p(h: u32) -> f32 {
    match h {
        0 => { 0.0 },
        _ => {
            let det: f32 = (1 + 8*h) as f32;
            (1.0f32 + det.sqrt().ceil()) / 2.0
        }
    }
}

fn main() {
    println!("{}", p(0));
    println!("{}", p(1));
    println!("{}", p(3));
    println!("{}", p(6));
    println!("{}", p(8));
}