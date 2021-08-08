fn main() {
    proconio::input! {
        n: usize,
        scores: [u64; n]
    }
    let mut sorted: Vec<(u64, usize)> = vec![];
    for (i, s) in scores.iter().enumerate() {
        sorted.push((*s, i + 1));
    }
    sorted.sort();
    println!("{}", sorted[n - 2].1);
}
