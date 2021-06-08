fn main() {
    proconio::input! {
        a: i32,
        b: i32,
    }
    let ans = if a == b { a } else { 3 - a - b };
    println!("{}", ans);
}