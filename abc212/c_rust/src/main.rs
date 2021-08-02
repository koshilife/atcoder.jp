fn binary_search(
    arr: &Vec<i64>,
    search_value: i64,
    lower: usize,
    upper: usize,
) -> (Option<usize>, Option<usize>) {
    let index = (lower + upper) / 2;
    let arr_v = arr[index];

    if search_value == arr_v {
        return (Some(index), None);
    }

    if search_value > arr_v {
        if index >= upper || lower == index {
            return (None, Some(upper));
        }
        return binary_search(arr, search_value, index, upper);
    } else {
        if lower >= index || upper == index {
            return (None, Some(lower));
        }
        return binary_search(arr, search_value, lower, index);
    }
}

fn main() {
    proconio::input! {
        n: usize,
        m: usize,
        mut an: [i64; n],
        mut bn: [i64; m],
    }
    an.sort();
    bn.sort();

    let inf = 10i64.pow(13);
    let mut min = inf;
    let mut lower = 0;
    let upper = bn.len();
    for a in an {
        let (hit, near) = binary_search(&bn, a, lower, upper);
        match hit {
            Some(_) => {
                min = 0;
                break;
            }
            _ => {}
        }
        match near {
            Some(n_index) => {
                if n_index == m {
                    min = min.min(a - bn[m - 1]);
                    continue;
                } else if n_index == 0 {
                    min = min.min((a - bn[0]).abs());
                } else {
                    min = min
                        .min((a - bn[n_index - 1]).abs())
                        .min((a - bn[n_index]).abs());
                    lower = n_index - 1;
                }
            }
            _ => {}
        }
    }
    println!("{}", min);
}
