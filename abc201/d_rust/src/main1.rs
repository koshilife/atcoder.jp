use proconio::input;
use std::collections::HashMap;

struct MapInfo {
    h_index: i32,
    w_index: i32,
    game_map: Vec<Vec<i32>>
}

fn search(i:i32, j:i32, memo: &mut HashMap<String, i32>, map_info: &MapInfo ) -> i32 {
    if i == map_info.h_index && j == map_info.w_index {
        return 0;
    }

    let memo_key = format!("{}_{}", i, j);
    if memo.contains_key(&memo_key) {
        return *memo.get(&memo_key).unwrap();
    }

    let mut res = -1001001001;
    if i < map_info.h_index {
        let pt = map_info.game_map[(i+1) as usize][j as usize];
        res = res.max(pt - search(i+1, j, memo, map_info));
    }
    if j < map_info.w_index {
        let pt = map_info.game_map[i as usize][(j+1) as usize];
        res = res.max(pt - search(i, j+1, memo, map_info));
    }
    memo.insert(memo_key, res);
    return res;
}

fn main() {
    input! {
        h: i32,
        w: i32,
        lines: [String; h],
    }
    let mut game_map :Vec<Vec<i32>> = Vec::new();
    for line in lines.iter() {
        let line_map :Vec<i32> = line.split("").filter(|c| c != &"" ).map(|c| if &c == &"+" {1} else { -1 } ).collect();
        game_map.push(line_map);
    }
    let map_info = MapInfo {
        h_index: h - 1,
        w_index: w - 1,
        game_map: game_map
    };
    let mut memo: HashMap<String, i32> = HashMap::new();
    let result = search(0, 0, &mut memo, &map_info);

    let ans = if result == 0 {
        "Draw"
    } else if result > 0 {
        "Takahashi"
    } else {
        "Aoki"
    };
    println!("{}", ans);
}

