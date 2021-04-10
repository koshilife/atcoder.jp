use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i32; n],
        b: [i32; n],
    }
    let mut sum = 0;
    for i in 0..n {
        sum += a[i] * b[i];
    }
    let ans = if sum == 0 { "Yes" } else { "No" };
    println!("{}", ans);
}