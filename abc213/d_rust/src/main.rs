fn dfs(
    city: usize,
    edges_info: &Vec<Vec<usize>>,
    visited_info: &mut Vec<bool>,
    histories: &mut Vec<usize>,
) {
    visited_info[city] = true;
    histories.push(city + 1);
    let edges = edges_info.get(city).unwrap();
    for next_city in edges {
        if !visited_info.get(*next_city).unwrap() {
            dfs(*next_city, edges_info, visited_info, histories);
            histories.push(city + 1);
        }
    }
}

fn main() {
    proconio::input! {
        n: usize,
        edges: [(usize, usize); n-1],
    }
    let mut edges_each_city = vec![vec![]; n];
    for (a, b) in edges {
        edges_each_city[a - 1].push(b - 1);
        edges_each_city[b - 1].push(a - 1);
    }
    for i in 0..n {
        edges_each_city[i].sort();
    }
    let mut hisitories = vec![];
    let mut visited_info = vec![false; n];
    dfs(0, &edges_each_city, &mut visited_info, &mut hisitories);

    let ans: Vec<_> = hisitories.iter().map(|x| x.to_string()).collect();
    println!("{}", ans.join(" "));
}
