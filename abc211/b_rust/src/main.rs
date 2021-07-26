
use std::collections::HashMap;

fn main() {
    proconio::input! {
        s1: String,
        s2: String,
        s3: String,
        s4: String
    };
    let mut map = HashMap::new();
    map.entry(s1).or_insert(true);
    map.entry(s2).or_insert(true);
    map.entry(s3).or_insert(true);
    map.entry(s4).or_insert(true);
    let ans = if map.keys().len() == 4 { "Yes" } else { "No" };
    println!("{}", ans);
}
