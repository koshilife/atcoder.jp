fn main() {
    proconio::input! {
        a: f64,
        b: f64,
    }
    let ans = (a-b)/3_f64 + b;
    println!("{}", ans);
}