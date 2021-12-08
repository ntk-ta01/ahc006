#![allow(non_snake_case, unused_imports, unused_macros)]
use proconio::input;
use rand::prelude::*;
use rand_chacha::ChaCha12Rng;
use std::collections::BTreeMap;

const TIMELIMIT: f64 = 1.8;
const N: usize = 1000;
const M: usize = 50;

struct Input {
    from: Vec<(i32, i32)>,
    to: Vec<(i32, i32)>,
}

#[derive(Clone)]
struct Output {
    r: Vec<usize>,
    path: Vec<usize>,
}

fn main() {
    input! {
        a: [(i32, i32, i32, i32); N]
    }
    let mut timer = Timer::new();
    let mut rng = rand_chacha::ChaCha20Rng::seed_from_u64(0);
    let from = a.iter().map(|&(x, y, _, _)| (x, y)).collect();
    let to = a.iter().map(|&(_, _, x, y)| (x, y)).collect();
    let input = Input { from, to };
    let mut output = greedy(&input);
    // eprintln!("{:?}", compute_score(&input, &output));
    annealing_2opt(&input, &mut output, &mut timer, &mut rng);
    // eprintln!("{:?}", compute_score(&input, &output));
    parse_output(&input, &output);
}

fn get_pos(input: &Input, i: usize) -> (i32, i32) {
    if i == !0 {
        (400, 400)
    } else if i & 1 == 0 {
        input.from[i >> 1]
    } else {
        input.to[i >> 1]
    }
}

const T0: f64 = 100.0;
const T1: f64 = 1.0;

fn annealing_2opt(
    input: &Input,
    output: &mut Output,
    timer: &mut Timer,
    rng: &mut rand_chacha::ChaCha20Rng,
) {
    let mut picked = [-1; N];
    let mut temp = T0;

    'lp: for iter in 0.. {
        if iter % 100 == 0 {
            let passed = timer.get_time() / TIMELIMIT;
            if passed >= 1.0 {
                break;
            }
            temp = T0.powf(1.0 - passed) * T1.powf(passed);
        }
        // 異なる2本のルートそれぞれを前半と後半の二つのパスに分け、後半のパスを交換することにより新しい解を作る
        let mut i = rng.gen_range(1, 2 * M + 1);
        let mut j = rng.gen_range(1, 2 * M + 1);
        if i == j {
            continue;
        }
        if i > j {
            std::mem::swap(&mut i, &mut j);
        }
        let pi_1 = get_pos(input, output.path[i - 1]);
        let pi = get_pos(input, output.path[i]);
        let pj = get_pos(input, output.path[j]);
        let pj_1 = get_pos(input, output.path[j + 1]);
        let diff = dist(pi_1, pj) + dist(pi, pj_1) - dist(pi_1, pi) - dist(pj, pj_1);
        if diff <= 0 || rng.gen_bool(f64::exp(-diff as f64 / temp)) {
            for k in i..=j {
                if picked[output.path[k] >> 1] == iter {
                    // ひっくり返すパスに番号の同じレストランと配達先が含まれる場合
                    // レストランより先に配達先に行ってしまうのでひっくり返せない
                    continue 'lp;
                }
                picked[output.path[k] >> 1] = iter;
            }
            output.path[i..=j].reverse();
        }
    }
}

fn greedy(input: &Input) -> Output {
    let mut r = vec![];
    let mut path = vec![!0, !0];

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
                let now_dist = dist(get_pos(input, path[j - 1]), input.from[id])
                    + dist(input.from[id], get_pos(input, path[j]))
                    - dist(get_pos(input, path[j - 1]), get_pos(input, path[j]));
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
                    dist(get_pos(input, path[k - 1]), input.to[id])
                        + dist(input.to[id], get_pos(input, path[k]))
                        - dist(get_pos(input, path[k - 1]), get_pos(input, path[k]))
                        + min_rest_dist[k]
                } else {
                    dist(get_pos(input, path[k - 1]), input.from[id])
                        + dist(input.from[id], input.to[id])
                        + dist(input.to[id], get_pos(input, path[k]))
                        - dist(get_pos(input, path[k - 1]), get_pos(input, path[k]))
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
        path.insert(path_rest_i, rest_id * 2);
        path.insert(path_deli_i, rest_id * 2 + 1);
        ordered_restaurant[rest_id] = true;
    }

    Output { r, path }
}

fn dist((x1, y1): (i32, i32), (x2, y2): (i32, i32)) -> i32 {
    (x1 - x2).abs() + (y1 - y2).abs()
}

#[allow(dead_code)]
fn compute_score(input: &Input, out: &Output) -> i64 {
    let mut time = 0;
    for i in 1..out.path.len() {
        time += dist(get_pos(input, out.path[i - 1]), get_pos(input, out.path[i])) as i64;
    }
    (1e8 / (1000 + time) as f64).round() as i64
}

fn parse_output(input: &Input, output: &Output) {
    print!("{}", output.r.len());
    for order in output.r.iter() {
        print!(" {}", order);
    }
    println!();
    print!("{}", output.path.len());
    for p in output.path.iter() {
        let (px, py) = get_pos(input, *p);
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

    #[allow(dead_code)]
    fn reset(&mut self) {
        self.start_time = 0.0;
    }
}
