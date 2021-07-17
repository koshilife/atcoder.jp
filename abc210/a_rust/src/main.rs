fn main() {
    proconio::input! {
        n: i32,
        a: i32,
        x: i32,
        y: i32
    }
    let ans = if n > a {
        x * a + (n - a) * y
    } else {
        x * n
    };
    println!("{}", ans);
}