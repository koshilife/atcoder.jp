use std::collections::HashMap;

const TARGET_STR: &str = "chokudai";

fn get_reminder(num:u64) -> u64 {
    num % (10u64.pow(9) + 7)
}

fn main() {
    proconio::input! {
        s: String
    }
    let mut target_map = HashMap::new();
    for c in target_chars() {
        target_map.entry(c).or_insert(true);
    }

    // 頻度マップ作成
    let mut indexes_map = HashMap::new();
    let chars = s.chars();
    for (i, c) in chars.enumerate() {
        match target_map.get(&c) {
            Some(_) => {
                let indexes = indexes_map.entry(c).or_insert(vec![]);
                indexes.push(i);
            },
            None => (),
        };
    }

    // 対象文字が全て含まれていない場合はスキップ
    if indexes_map.keys().len() < TARGET_STR.len() {
        println!("{}", 0);
        return;
    }
    let mut memo: HashMap<isize, HashMap<usize, u64>> = HashMap::new();
    let ans = search(&mut memo, &indexes_map, -1, 0, 1);
    println!("{}", ans)
}

fn target_chars() -> Vec<char> {
    TARGET_STR.chars().collect()
}

fn search(memo: &mut HashMap<isize, HashMap<usize, u64>>, indexes_map: &HashMap<char, Vec<usize>>, current_index: isize, word_index:usize, current_ans: u64) -> u64 {
    // キャッシュ済みであればキャッシュを返却
    let cache = find_memo(memo, current_index, word_index);
    if cache >= 0 {
        return cache as u64;
    }

    // 利用可能なインデックスを求めます
    let c = target_chars()[word_index];
    let mut available_indexes:Vec<isize> = vec![];
    match indexes_map.get(&c) {
        Some(indexes) => {
            for i in indexes.iter() {
                let index = *i as isize;
                if index > current_index {
                    available_indexes.push(index);
                }
            }
        },
        None => ()
    };
    let patterns = available_indexes.len() as u64;

    // パターンなし
    if patterns == 0 {
        return update_memo(memo, current_index, word_index, 0);
    }

    // 最後の文字に到達した場合
    if TARGET_STR.len() <= word_index + 1 {
        return update_memo(
            memo,
            current_index,
            word_index,
            get_reminder(current_ans * patterns));
    }

    let mut ans = 0;
    for next_i in available_indexes {
      let next_patterns = search(memo, indexes_map, next_i, word_index + 1, current_ans);
      if next_patterns > 0 {
        ans = get_reminder(ans + (current_ans * next_patterns));
      }
    }
    update_memo(memo, current_index, word_index, ans)
}

fn find_memo(memo: &HashMap<isize, HashMap<usize, u64>>, index:isize, w_index:usize) -> i64 {
    match memo.get(&index) {
        Some(v1) => {
            match v1.get(&w_index) {
                Some(v2) => return *v2 as i64,
                None => ()
            }
        },
        None => ()
    };
    return -1;
}

fn update_memo(memo: &mut HashMap<isize, HashMap<usize, u64>>, index: isize, word_index:usize, value: u64) -> u64 {
    let memo_index_dic = memo.entry(index).or_insert(HashMap::new());
    memo_index_dic.entry(word_index).or_insert(value);
    value
}
