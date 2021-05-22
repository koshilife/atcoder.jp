use proconio::input;

fn main() {
    input! {
        a: i32,
        b: i32,
        c: i32
    }
    println!("{}", (7 - a) + (7 - b) + (7 - c));
}