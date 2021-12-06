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
    // eprintln!("{:?}", compute_score(&input, &output));
    parse_output(&output);
}

fn greedy(input: &Input) -> Output {
    let mut r = vec![];
    let mut path = vec![(400, 400), (400, 400)];

    let mut ordered_restaurant = vec![false; N];
    while r.len() < M {
        let mut min_dist = i32::max_value();
        let mut rest_id = N;
        let mut path_rest_i = 2 * M + 2;
        let mut path_deli_i = 2 * M + 2;

        let mut min_rest_dist = vec![i32::max_value(); path.len()];
        let mut min_rest_index = vec![M; path.len()];
        // r.len() + 1回目に挿入するレストランと配送先を決める
        for (id, is_ordered) in ordered_restaurant.iter_mut().enumerate() {
            if *is_ordered {
                continue;
            }
            min_rest_dist[0] = i32::max_value();
            // レストランを挿入したときの、増加量の累積min
            for j in 1..path.len() {
                let now_dist = dist(path[j - 1], input.from[id]) + dist(input.from[id], path[j])
                    - dist(path[j - 1], path[j]);
                min_rest_dist[j] = now_dist;
                if min_rest_dist[j - 1] > min_rest_dist[j] {
                    min_rest_index[j] = j;
                } else {
                    min_rest_dist[j] = min_rest_dist[j - 1];
                    min_rest_index[j] = min_rest_index[j - 1];
                }
            }

            for k in 1..path.len() {
                let now_dist = if k != min_rest_index[k] {
                    dist(path[k - 1], input.to[id]) + dist(input.to[id], path[k])
                        - dist(path[k - 1], path[k])
                        + min_rest_dist[k]
                } else {
                    dist(path[k - 1], input.from[id])
                        + dist(input.from[id], input.to[id])
                        + dist(input.to[id], path[k])
                        - dist(path[k - 1], path[k])
                };
                if min_dist > now_dist {
                    min_dist = now_dist;
                    rest_id = id;
                    path_rest_i = min_rest_index[k];
                    path_deli_i = k + 1;
                }
            }
        }
        r.push(rest_id + 1);
        path.insert(path_rest_i, input.from[rest_id]);
        path.insert(path_deli_i, input.to[rest_id]);
        ordered_restaurant[rest_id] = true;
    }

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
