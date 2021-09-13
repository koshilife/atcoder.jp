fn main() {
    proconio::input! {
        indexes: [usize; 26]
    }
    let alphabet: Vec<char> = (b'a'..=b'z').map(|c| c as char).collect::<Vec<_>>();
    let ans = indexes
        .iter()
        .map(|i| alphabet[i - 1].to_string())
        .collect::<Vec<_>>()
        .join("");
    println!("{}", ans);
}
