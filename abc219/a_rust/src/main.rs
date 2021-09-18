fn main() {
    proconio::input! {
        n: usize,
    }
    if n >= 90 {
        println!("expert");
    } else if n >= 70 {
        println!("{}", 90 - n);
    } else if n >= 40 {
        println!("{}", 70 - n);
    } else {
        println!("{}", 40 - n);
    }
}
