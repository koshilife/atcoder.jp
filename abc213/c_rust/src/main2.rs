// 公式解法 map利用パターン
use std::collections::HashMap;

fn compress(arr: &Vec<usize>) -> Vec<usize> {
    let mut sorted_arr = arr.clone();
    sorted_arr.dedup();
    sorted_arr.sort();
    let mut mp = HashMap::new();
    let mut cnt = 1;
    for key in sorted_arr {
        mp.insert(key, cnt);
        cnt += 1;
    }
    let mut ret = vec![];
    for a in arr {
        ret.push(*mp.get(a).unwrap());
    }
    ret
}

fn main() {
    proconio::input! {
        _: usize,
        _: usize,
        n: usize,
        cards: [(usize, usize); n]
    }
    let (rows, columns): (Vec<_>, Vec<_>) = cards.iter().cloned().unzip();
    let new_rows = compress(&rows);
    let new_columns = compress(&columns);
    for i in 0..n {
        println!("{} {}", new_rows[i], new_columns[i]);
    }
}
