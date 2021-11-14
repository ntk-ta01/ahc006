#![allow(non_snake_case, unused_imports, unused_macros, dead_code)]
use proconio::input;
use rand::prelude::*;
use std::collections::BTreeMap;

const TIMELIMIT: f64 = 0.9;
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
    let mut timer = Timer::new();
    input! {
        a: [(i32, i32, i32, i32); N]
    }
    let from = a.iter().map(|&(x, y, _, _)| (x, y)).collect();
    let to = a.iter().map(|&(_, _, x, y)| (x, y)).collect();
    let input = Input { from, to };
    let mut s = (1..=M).collect::<Vec<usize>>();
    let mut output = greedy(&input, s.clone());
    // for _ in 0..10 {
    order_climbing(&input, &mut output, &mut timer, &mut s);
    // climbing_2_opt(&input, &mut output, &mut timer, &mut s);
    // timer.reset();
    // }
    parse_output(&output);
}

fn climbing_2_opt(input: &Input, output: &mut Output, _timer: &mut Timer, s: &mut Vec<usize>) {
    let mut best_score = compute_score(input, output).0;

    for i in 1..M - 1 {
        for j in i + 1..M {
            let mut new_s = s.clone();
            new_s[i..=j].reverse();
            let new_output = greedy(input, new_s.clone());
            let new_score = compute_score(input, &new_output).0;
            if best_score < new_score {
                best_score = new_score;
                *output = new_output;
                *s = new_s;
            }
        }
    }
    eprintln!("{}", best_score);
}

fn order_climbing(input: &Input, output: &mut Output, timer: &mut Timer, s: &mut Vec<usize>) {
    let mut rng = rand_chacha::ChaCha20Rng::seed_from_u64(0);
    let mut count = 0;

    let mut best_score = compute_score(input, output).0;

    loop {
        if count >= 100 {
            let passed = timer.get_time() / TIMELIMIT;
            if passed >= 1.0 {
                break;
            }
            count = 0;
        }
        count += 1;

        let mut new_s = s.clone();
        // 2点swap と 1点変更 をやる
        if rng.gen_bool(0.5) {
            // update近傍
            let mut new_type = rng.gen_range(0, N) + 1;
            while s.iter().any(|e| *e == new_type) {
                new_type = rng.gen_range(0, N) + 1;
            }
            let update_index = rng.gen_range(0, M);
            new_s[update_index] = new_type;
        } else {
            // swap近傍
            let swap_index1 = rng.gen_range(0, M);
            let swap_index2 = rng.gen_range(0, M);
            let out1 = new_s[swap_index1];
            let out2 = new_s[swap_index2];
            new_s[swap_index2] = out2;
            new_s[swap_index2] = out1;
        }
        let new_output = greedy(input, new_s.clone());
        let new_score = compute_score(input, &new_output).0;
        if best_score < new_score {
            best_score = new_score;
            *output = new_output;
            *s = new_s;
        }
    }
    eprintln!("{}", best_score);
}

fn greedy(input: &Input, r: Vec<usize>) -> Output {
    let mut path = vec![];
    path.push((400, 400));
    for i in r.iter() {
        let (x, y) = input.from[*i - 1];
        path.push((x, y));
        let (x, y) = input.to[*i - 1];
        path.push((x, y));
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
