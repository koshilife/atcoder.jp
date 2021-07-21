use std::collections::HashMap;

fn main() {
    proconio::input! {
        n: usize,
        k: usize,
        cn: [u64; n]
    }
    // println!("n:{}, k:{}, cn:{:?}", n, k, cn);
    let mut occur_map = HashMap::new();
    for c in &cn[0..k] {
        let count = occur_map.entry(c).or_insert(0);
        *count += 1;
    }

    let mut ans = vec![0; n-k+1];
    ans[0] = occur_map.keys().len();

    for i in 1..=n-k {
        let old_n = &cn[i-1];
        let new_n = &cn[i+k-1];
        if old_n == new_n {
            ans[i] = ans[i-1];
        } else {

            // 外れた値で頻度mapを更新
            match occur_map.get(old_n) {
                Some(v) => if *v == 1 {
                    occur_map.remove(old_n);
                  } else {
                    occur_map.insert(old_n, v-1);
                  },
                None => (),
            };

            // 追加された値で頻度mapを更新
            let count = occur_map.entry(new_n).or_insert(0);
            *count += 1;

            // println!("  - [{}] map:{:?}", i, occur_map);
            ans[i] = occur_map.keys().len();
        }
    }
    println!("{}", ans.iter().max().unwrap());
}
