use proconio::input;

fn main() {
    input! {
        n: i32,
    }
    let mut century = n / 100;
    if n % 100 >= 1 {
        century += 1
    }
    println!("{}", century);
}