use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
      n: i32,
      m: usize,
      t: i32,
      v: [(i32, i32); m],
    }
    let mut cafe_map: HashMap<i32, i32> = HashMap::new();
    for (a, b) in &v {
      cafe_map.insert(*a, *b);
    }

    let mut hp = n;
    let mut is_dead = hp > 0;
    let mut end_cafe = -1;
    for i in 1..=t {
      let index = i + 6;
      println!("{}",index);
      if i < end_cafe {
        hp += 1;
        continue;
      }
      if cafe_map.contains_key(&i) {
        hp += 1;
        end_cafe = cafe_map.get(&index);
        continue;
      }
      hp -= 1;
      if hp <= 0 {
        is_dead = true;
        break;
      }
    }
    let ans = if is_dead { "Yes" } else { "No" };
    println!("{}", ans)
}
