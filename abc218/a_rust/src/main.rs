fn main() {
    proconio::input! {
        n: usize,
        s: String,
    }
    let chars: Vec<char> = s.chars().collect();
    let ans = if chars[n - 1] == 'o' { "Yes" } else { "No" };
    println!("{}", ans);
}
