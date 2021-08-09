// 公式解法 binary_search利用パターン
fn main() {
    proconio::input! {
        _: usize,
        _: usize,
        n: usize,
        cards: [(u64, u64); n]
    }
    let (mut a, mut b): (Vec<_>, Vec<_>) = cards.iter().cloned().unzip();
    a.sort();
    b.sort();
    a.dedup();
    b.dedup();
    for (ai, bi) in cards {
        println!(
            "{} {}",
            a.binary_search(&ai).unwrap() + 1,
            b.binary_search(&bi).unwrap() + 1
        );
    }
}
