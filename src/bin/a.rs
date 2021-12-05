#![allow(non_snake_case, unused_imports, unused_macros, dead_code)]
use proconio::input;
use rand::prelude::*;
use rand_chacha::ChaCha12Rng;
use std::collections::BTreeMap;

const N: usize = 1000;
const M: usize = 50;

struct Input {
    from: Vec<(i32, i32)>,
    to: Vec<(i32, i32)>,
}

#[derive(Clone)]
struct Output {
    r: Vec<usize>,
    path: Vec<(i32, i32)>,
}

fn main() {
    input! {
        a: [(i32, i32, i32, i32); N]
    }
    let from = a.iter().map(|&(x, y, _, _)| (x, y)).collect();
    let to = a.iter().map(|&(_, _, x, y)| (x, y)).collect();
    let input = Input { from, to };
    let output = greedy(&input);
    // eprintln!("{:?}", compute_score(&input, &output).0);
    parse_output(&output);
}

fn decide_dist_max(input: &Input) -> i32 {
    let mut ret_dist = 1000;
    let mut ng_dist = 0;
    while ret_dist - ng_dist > 1 {
        let mid = (ret_dist + ng_dist) / 2;
        let mut m = 0;
        for (from, to) in input.from.iter().zip(input.to.iter()) {
            if dist((400, 400), *from) <= mid && dist((400, 400), *to) <= mid {
                m += 1;
            }
        }
        if M <= m {
            ret_dist = mid;
        } else {
            ng_dist = mid;
        }
    }
    ret_dist
}

fn greedy(input: &Input) -> Output {
    let dist_max = decide_dist_max(input);
    let mut r = vec![];
    let mut path = vec![];
    path.push((400, 400));

    // 全レストランでnearest neighbor法をする
    // レストランがpick up済みであるかの情報を持つvec 0:未pickの注文 1:pick済みの注文 -1:注文対象外のレストラン
    let mut is_picked = vec![0; N];

    // オフィスからの{レストラン,配達先}までのマンハッタン距離がDIST_MAXを超えるレストランはpickしないようにする
    for (i, (rest, dest)) in input.from.iter().zip(input.to.iter()).enumerate() {
        if dist_max < dist((400, 400), *rest) || dist_max < dist((400, 400), *dest) {
            is_picked[i] = -1;
        }
    }

    let mut is_delivered = vec![false; M];

    // path[-1]に一番近いレストランをpathに追加していく、もしくはpath[-1]に一番近いpick済みの注文の配送先に配送する
    while is_delivered.iter().any(|b| !(*b)) {
        let mut rest_i = N;
        let mut min_dist = i32::max_value();
        let mut is_restaurant = true;
        if r.len() < M {
            for (i, p) in is_picked.iter().enumerate() {
                if *p == -1 || *p == 1 {
                    continue;
                }
                let now_dist = dist(path[path.len() - 1], input.from[i]);
                if min_dist > now_dist {
                    min_dist = now_dist;
                    rest_i = i;
                }
            }
        }
        for (i, rest) in r.iter().enumerate() {
            if is_delivered[i] {
                continue;
            }
            let now_dist = dist(path[path.len() - 1], input.to[*rest - 1]);
            if min_dist > now_dist {
                min_dist = now_dist;
                rest_i = i;
                is_restaurant = false;
            }
        }
        if is_restaurant {
            r.push(rest_i + 1);
            is_picked[rest_i] = 1;
            path.push(input.from[rest_i]);
        } else {
            is_delivered[rest_i] = true;
            path.push(input.to[r[rest_i] - 1]);
        }
    }

    path.push((400, 400));
    Output { r, path }
}

fn dist((x1, y1): (i32, i32), (x2, y2): (i32, i32)) -> i32 {
    (x1 - x2).abs() + (y1 - y2).abs()
}

fn compute_score(input: &Input, out: &Output) -> (i64, String, i64) {
    let mut time = 0;
    for i in 1..out.path.len() {
        time += dist(out.path[i - 1], out.path[i]) as i64;
    }
    for i in 0..out.r.len() {
        if out.r[i] >= N {
            return (
                0,
                format!("Illegal output (r[{}] = {})", i + 1, out.r[i] + 1),
                time,
            );
        }
        for j in 0..i {
            if out.r[i] == out.r[j] {
                return (
                    0,
                    format!("Illegal output (r[{}] = r[{}])", i + 1, j + 1),
                    time,
                );
            }
        }
    }
    for i in 0..out.path.len() {
        if out.path[i].0 < 0 || out.path[i].0 > 800 || out.path[i].1 < 0 || out.path[i].1 > 800 {
            return (0, "Illegal output".to_owned(), time);
        }
    }
    if out.path.is_empty() || out.path[0] != (400, 400) {
        return (
            0,
            "Illegal output (x[1],y[1]) != (400, 400)".to_owned(),
            time,
        );
    } else if out.path[out.path.len() - 1] != (400, 400) {
        return (
            0,
            "Illegal output (x[n],y[n]) != (400, 400)".to_owned(),
            time,
        );
    }
    let mut first_visit = BTreeMap::new();
    let mut last_visit = BTreeMap::new();
    for i in 0..out.path.len() {
        first_visit.entry(out.path[i]).or_insert(i);
        last_visit.insert(out.path[i], i);
    }
    for &i in &out.r {
        if let (Some(first), Some(last)) = (
            first_visit.get(&input.from[i - 1]),
            last_visit.get(&input.to[i - 1]),
        ) {
            if first >= last {
                return (0, format!("{}-th delivery has not been completed", i), time);
            }
        } else {
            return (0, format!("{}-th delivery has not been completed", i), time);
        }
    }
    if out.r.len() != M {
        return (0, "Illegal output (m != 50)".to_owned(), time);
    }
    let score = (1e8 / (1000 + time) as f64).round() as i64;
    (score, String::new(), time)
}

fn parse_output(output: &Output) {
    print!("{}", output.r.len());
    for order in output.r.iter() {
        print!(" {}", order);
    }
    println!();
    print!("{}", output.path.len());
    for (px, py) in output.path.iter() {
        print!(" {} {}", px, py);
    }
    println!();
}

pub fn get_time() -> f64 {
    let t = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap();
    t.as_secs() as f64 + t.subsec_nanos() as f64 * 1e-9
}

struct Timer {
    start_time: f64,
}

impl Timer {
    fn new() -> Timer {
        Timer {
            start_time: get_time(),
        }
    }

    fn get_time(&self) -> f64 {
        get_time() - self.start_time
    }

    fn reset(&mut self) {
        self.start_time = 0.0;
    }
}
