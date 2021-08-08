use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    proconio::input! {
        _: usize,
        _: usize,
        n: usize,
        cards: [(usize, usize); n]
    }
    let mut show_rows = vec![];
    let mut show_columns = vec![];
    for (_h, _w) in &cards {
        show_rows.push(_h);
        show_columns.push(_w);
    }

    // H座標調整用のハッシュマップを作成
    let mut adjust_h: HashMap<&usize, usize> = HashMap::new();
    let rows_set: HashSet<&usize> = show_rows.into_iter().collect();
    let mut uniq_rows = rows_set.into_iter().collect::<Vec<_>>();
    uniq_rows.sort();
    for (i, row) in uniq_rows.iter().enumerate() {
        let mut point = 0;
        if i == 0 {
            point = *row - 1;
        } else {
            let before_v = uniq_rows[i - 1];
            let diff = *row - before_v - 1;
            point += diff;
            if let Some(v) = adjust_h.get(before_v) {
                point += v;
            }
        }
        adjust_h.entry(row).or_insert(point);
    }

    // W座標調整用のハッシュマップを作成
    let mut adjust_w: HashMap<&usize, usize> = HashMap::new();
    let columns_set: HashSet<&usize> = show_columns.into_iter().collect();
    let mut uniq_columns = columns_set.into_iter().collect::<Vec<_>>();
    uniq_columns.sort();
    for (i, column) in uniq_columns.iter().enumerate() {
        let mut point = 0;
        if i == 0 {
            point = *column - 1;
        } else {
            let before_v = uniq_columns[i - 1];
            let diff = *column - before_v - 1;
            point += diff;
            if let Some(v) = adjust_w.get(before_v) {
                point += v;
            }
        }
        adjust_w.entry(column).or_insert(point);
    }

    // for Debug
    // println!("adjust_h:{:?}", adjust_h);
    // println!("adjust_w:{:?}", adjust_w);

    for (_h, _w) in &cards {
        let mut result_h = *_h;
        let mut result_w = *_w;
        if let Some(v) = adjust_h.get(_h) {
            result_h -= v;
        }
        if let Some(v) = adjust_w.get(_w) {
            result_w -= v;
        }
        println!("{} {}", result_h, result_w);
    }
}
