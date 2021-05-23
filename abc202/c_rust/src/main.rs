// https://atcoder.jp/contests/abc200/tasks/abc202_c
// refs: https://atcoder.jp/contests/abc202/editorial/1859
// 公式解説のコードを元に勉強

// Usize1=> 入力値を-1して読み取る際はusizeとして振る舞う型
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        a: [Usize1; n],
        b: [Usize1; n],
        c: [Usize1; n],
    }
    let mut count = vec![0; n];
    for x in c {
        count[b[x]] += 1;
    }
    // into_iter は 参照ではなく値のコレクションを返却する
    // Trait std::iter::Sum https://doc.rust-lang.org/std/iter/trait.Sum.html
    println! {
        "{}",
        a.into_iter()
            .map(|x| count[x])
            .sum::<u64>()
    }
}