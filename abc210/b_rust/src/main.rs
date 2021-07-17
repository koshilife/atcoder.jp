
fn main() {
    proconio::input! {
        _n: usize,
        s: String
    }
    let mut index = 0;
    for (i, c) in s.chars().enumerate() {
        if c == '1' {
            index = i;
            break;
        }
    }
    let ans = if index % 2 == 0 { "Takahashi" } else { "Aoki" };
    println!("{}", ans);
}
