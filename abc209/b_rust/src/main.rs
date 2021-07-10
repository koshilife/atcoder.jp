
fn main() {
    proconio::input! {
        n: usize,
        x: u64,
        an: [u64; n]
    }
    let mut total = 0;
    for i in 0..n {
        let price = if i % 2 == 1 {
            total += an[i] - 1
        } else {
            total += an[i]
        };
    }
    let ans = if x >= total {
        "Yes"
    } else {
        "No"
    };
    println!("{}", ans);
}
