fn main() {
    proconio::input! {
        h: usize,
        w: usize,
        n: usize,
        cards: [(usize, usize); n]
    }
    let mut has_num_each_rows = vec![false; h];
    let mut has_num_each_columns = vec![false; w];
    for (_h, _w) in &cards {
        has_num_each_rows[_h - 1] = true;
        has_num_each_columns[_w - 1] = true;
    }

    let mut adjust_h: Vec<usize> = vec![];
    for (i, has_num) in has_num_each_rows.iter().enumerate() {
        let before = if i == 0 { 0 } else { adjust_h[i - 1] };
        adjust_h.push(if *has_num { before } else { before + 1 });
    }

    let mut adjust_w: Vec<usize> = vec![];
    for (i, has_num) in has_num_each_columns.iter().enumerate() {
        let before = if i == 0 { 0 } else { adjust_w[i - 1] };
        adjust_w.push(if *has_num { before } else { before + 1 });
    }

    for (_h, _w) in &cards {
        println!("{} {}", _h - adjust_h[_h - 1], _w - adjust_w[_w - 1]);
    }
}
