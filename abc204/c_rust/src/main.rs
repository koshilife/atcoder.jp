// https://atcoder.jp/contests/abc204/tasks/abc204_c
// 公式解説を参考に実装

fn dfs(i: usize, roads_dic: &Vec<Vec<usize>>, visited: &mut Vec<bool>) {
    if visited[i] { return; }
    visited[i] = true;
    for j in &roads_dic[i] {
        dfs(*j, &roads_dic, visited);
    }
}

fn main() {
    proconio::input! {
        n: usize,
        m: usize,
        roads: [(usize, usize); m]
    }
    // roads_dic[i] は都市iから道路で直接繋がっている都市のリスト
    let mut roads_dic:Vec<Vec<usize>> = vec![vec![]; n];
    for (a, b) in roads {
        roads_dic[a-1].push(b-1);
    }

    let mut ans = 0;
    for i in 0..n {
        let mut tmp: Vec<bool> = vec![false; n];
        dfs(i, &roads_dic, &mut tmp);
        for v in tmp {
            if v { ans += 1; }
        }
    }
    println!("{}", ans);
}