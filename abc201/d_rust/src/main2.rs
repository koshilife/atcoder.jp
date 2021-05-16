use proconio::input;

// v1からの改善点:
// - メモのHashMapをやめて配列を用いる.
// - i32 ではなく usize 型を用いて キャストを減らす

const INF: i32 = -1001001001;

struct MapInfo {
    h_index: usize,
    w_index: usize,
    game_map: Vec<Vec<i32>>
}

fn search(i:usize, j:usize, memo: &mut Vec<Vec<i32>>, map_info: &MapInfo ) -> i32 {
    if i == map_info.h_index && j == map_info.w_index {
        return 0;
    }
    if memo[i][j] != INF {
        return memo[i][j];
    }
    let mut res = INF;
    if i < map_info.h_index {
        let pt = map_info.game_map[i+1][j];
        res = res.max(pt - search(i+1, j, memo, map_info));
    }
    if j < map_info.w_index {
        let pt = map_info.game_map[i][j+1];
        res = res.max(pt - search(i, j+1, memo, map_info));
    }
    memo[i][j] = res;
    return res;
}

fn main() {
    input! {
        h: usize,
        w: usize,
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
    let mut memo: Vec<Vec<i32>> = vec![vec![INF; w]; h];
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

