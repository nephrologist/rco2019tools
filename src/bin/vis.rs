#![allow(non_snake_case)]

use proconio::{input, source::*};
use std::io::BufRead;
pub const N: usize = 50;
pub const M: usize = 2500;
pub const is_visualizeall: bool = true;
pub const dx: [usize; 4] = [1, 0, !0, 0];
pub const dy: [usize; 4] = [0, 1, 0, !0];

// const list_cid: [&str; 10] = [
//     "#FFFFFF", "#43C3D0", "#9BB9DE", "#BAE5F5", "#EDF5ED", "#D2EDC1", "#DAF161", "#E5F302",
//     "#F2E703", "#EA0406",
// ];
// const blud_cid: [&str; 10] = [
//     "aliceblue",
//     "lavender",
//     "royalblue",
//     "skyblue",
//     "lightskyblue",
//     "deepskyblue",
//     "steelblue",
//     "dodgerblue",
//     "blue",
//     "navy",
// ];

const blud_cid: [&str; 10] = [
    "#aaaaff", "#9999fa", "#8888f6", "#7777f2", "#6666ee", "#5555ea", "#4444e5", "#3333e1",
    "#2222dd", "#1111d9",
];

// const orange_cid: [&str; 10] = [
//     "#fffaf4", "#fff4ea", "#ffead5", "#ffd5aa", "#ffbf80", "#ffaa55", "#ff952b", "#ff8000",
//     "#d56a00", "#aa5500",
// ];

const orange_cid: [&str; 10] = [
    "#ffd5aa", "#ffcc99", "#ffc488", "#ffbb77", "#ffb366", "#ffaa55", "#ffa244", "#ff9933",
    "#ff9122", "#ff8811",
];

fn main() {
    if std::env::args().len() != 3 {
        eprintln!(
            "Usage: {} <input> <output>",
            std::env::args().nth(0).unwrap()
        );
        return;
    }
    let input = read_input(&std::env::args().nth(1).unwrap());
    let output = read_output(&std::env::args().nth(2).unwrap());
    // if output.vals.len() > 2500 {
    //     println!("score {}", 0);
    //     std::process::exit(1);
    // }
    // let board = Board {
    //     n: N,
    //     nums: input.board.clone(),
    //     score: 0,
    // };
    let board = calcscore(&input, &output);
    println!("score {}", &board.score);
    vis(&board, 0);
}

#[derive(Clone, Debug)]
pub struct Input {
    pub n: usize,
    pub m: usize,
    pub board: Vec<i64>,
}

pub struct Output {
    pub vals: Vec<(usize, usize, usize)>,
}

#[derive(Clone)]
pub struct Board {
    n: usize,
    nums: Vec<i64>,
    score: i64,
    is_harvested: Vec<bool>,
}

pub fn valtoidx(x: usize, y: usize, n: usize) -> usize {
    x * n + y
}

pub fn idxtoval(idx: usize, n: usize) -> (usize, usize) {
    (idx / n, idx % n)
}

pub fn calcscore(input: &Input, output: &Output) -> Board {
    let mut board = Board {
        n: input.n,
        nums: input.board.clone(),
        score: 0,
        is_harvested: vec![false; input.n * input.n],
    };
    if is_visualizeall {
        vis(&board, 0);
    }

    for i in 0..output.vals.len() {
        let a = output.vals[i].0;
        let b = output.vals[i].1;
        let c = output.vals[i].2;
        let idx = valtoidx(b, c, board.n);
        if a == 1 {
            board.nums[idx] += 1;
        } else {
            if board.is_harvested[idx] {
                continue;
            };
            let mut stack = Vec::new();
            stack.push(idx);
            while stack.len() > 0 {
                let idx = stack.pop().unwrap();
                if board.is_harvested[idx] {
                    continue;
                }
                board.is_harvested[idx] = true;
                board.score += board.nums[idx];
                let tp = idxtoval(idx, board.n);
                let x = tp.0;
                let y = tp.1;
                for t in 0..4 {
                    let nx = x + dx[t];
                    let ny = y + dy[t];
                    if nx < input.n && ny < input.n {
                        let nidx = valtoidx(nx, ny, board.n);
                        if board.is_harvested[nidx] {
                            continue;
                        }
                        if board.nums[idx] != board.nums[nidx] {
                            continue;
                        }
                        stack.push(nidx);
                    }
                }
            }
        }
        if is_visualizeall && i % 10 == 9 {
            vis(&board, i);
        }
    }

    board
}

// pub fn score(input: &Input, out: &Vec<Rect>) -> i64 {}

// read系は直した
fn read_input(f: &str) -> Input {
    let f = std::fs::File::open(f).unwrap_or_else(|_| {
        eprintln!("no such file: {}", f);
        std::process::exit(1)
    });
    let f = line::LineSource::new(std::io::BufReader::new(f));
    input! {
        from f,
        n: usize,
        m:usize,
        board:[i64;n*n],
    }
    Input { n, m, board }
}
//output は長さ無制限のときがよくわからなかったので自作
fn read_output(f: &str) -> Output {
    let f = std::fs::File::open(f).unwrap_or_else(|_| {
        eprintln!("no such file: {}", f);
        std::process::exit(1)
    });
    let mut vals = Vec::new();
    let ff = std::io::BufReader::new(f);
    for line in ff.lines() {
        let temp = line.unwrap();
        let mut val: Vec<&str> = temp.split_whitespace().collect();
        let v1: usize = val[0].parse().unwrap();
        let v2: usize = val[1].parse().unwrap();
        let v3: usize = val[2].parse().unwrap();
        vals.push((v1, v2, v3));
    }
    if vals.len() > 2500 {
        eprintln!("too many output");
        std::process::exit(1)
    }
    Output { vals }
}

use svg::node::element::{path::Data, Path};

fn vis(board: &Board, idx: usize) {
    let board_size = N as i64;
    let mut doc = svg::Document::new().set("viewBox", (0, 0, 20 * board_size, 20 * board_size));
    // 白い描画画面を作る
    doc = doc.add(
        Path::new().set("fill", "white").set(
            "d",
            Data::new()
                .move_to((0, 0))
                .line_by((0, 20 * board_size))
                .line_by((20 * board_size, 0))
                .line_by((0, -20 * board_size))
                .close(),
        ),
    );
    // 罫線を作る
    for i in 0..=N {
        //横
        let data = Data::new()
            .move_to((0, 20 * i))
            .line_by((20 * board_size, 0));
        let path = Path::new()
            .set("stroke", "black")
            .set("stroke-width", 2)
            .set("d", data);
        doc = doc.add(path);
        //縦
        let data = Data::new()
            .move_to((20 * i, 0))
            .line_by((0, 20 * board_size));
        let path = Path::new()
            .set("stroke", "black")
            .set("stroke-width", 2)
            .set("d", data);
        doc = doc.add(path);
    }

    for i in 0..N {
        for j in 0..N {
            let idx = valtoidx(i, j, N);
            let data = Data::new()
                .move_to((20 * j, 20 * i))
                .line_by((20, 0))
                .line_by((0, 20))
                .line_by((-20, 0))
                .close();
            let path = Path::new()
                .set("stroke", "black")
                .set("stroke-width", 2)
                .set("fill", "blue")
                // .set("set-fillopacity", 1.0 - board.nums[idx] as f64 / 10.0)
                .set("fill", blud_cid[board.nums[idx] as usize])
                .set("d", data);
            doc = doc.add(path);
            // eprintln!(
            //     "{} {} {}",
            //     idx, blud_cid[board.nums[idx] as usize], board.nums[idx]
            // );
        }
    }

    for i in 0..N {
        for j in 0..N {
            let idx = valtoidx(i, j, N);

            if board.is_harvested[idx] {
                let num = board.nums[idx];
                let data = Data::new()
                    .move_to((20 * j, 20 * i))
                    .line_by((20, 0))
                    .line_by((0, 20))
                    .line_by((-20, 0))
                    .close();
                let path = Path::new()
                    .set("stroke", orange_cid[num as usize])
                    .set("stroke-width", 2)
                    .set("fill", orange_cid[num as usize])
                    .set("d", data);
                doc = doc.add(path);
                if num >= 4 {
                    doc = doc.add(
                        svg::node::element::Text::new()
                            .set("x", j * 20 + 10)
                            .set("y", i * 20 + 14)
                            .set("font-size", 14)
                            .set("text-anchor", "middle")
                            .set("fill", "white")
                            // .set("stroke", "black")
                            .add(svg::node::Text::new(format!("{}", num))),
                    );
                } else {
                    doc = doc.add(
                        svg::node::element::Text::new()
                            .set("x", j * 20 + 10)
                            .set("y", i * 20 + 14)
                            .set("font-size", 14)
                            .set("text-anchor", "middle")
                            .set("fill", "black")
                            // .set("stroke", "black")
                            .add(svg::node::Text::new(format!("{}", num))),
                    );
                }
            } else {
                let num = board.nums[idx];
                if num >= 6 {
                    let c = "white";
                    doc = doc.add(
                        svg::node::element::Text::new()
                            .set("x", j * 20 + 10)
                            .set("y", i * 20 + 14)
                            .set("font-size", 14)
                            .set("text-anchor", "middle")
                            .set("fill", c)
                            .add(svg::node::Text::new(format!("{}", num))),
                    );
                } else {
                    let c = "black";
                    doc = doc.add(
                        svg::node::element::Text::new()
                            .set("x", j * 20 + 10)
                            .set("y", i * 20 + 14)
                            .set("font-size", 14)
                            .set("text-anchor", "middle")
                            .set("fill", c)
                            .add(svg::node::Text::new(format!("{}", num))),
                    );
                };
            }
        }
    }

    svg::save(format!("out{:0>4}.svg", idx), &doc).unwrap();
}

// union-find library
#[derive(Clone)]
pub struct UnionFind {
    par: Vec<usize>,
    size: Vec<usize>,
}
impl UnionFind {
    pub fn new(n: usize) -> Self {
        UnionFind {
            par: (0..n).collect(),
            size: vec![1; n],
        }
    }
    pub fn find(&mut self, x: usize) -> usize {
        if self.par[x] != x {
            self.par[x] = self.find(self.par[x]);
        }
        self.par[x]
    }

    pub fn size(&mut self, x: usize) -> usize {
        self.size[x]
    }
    pub fn is_same(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }
    pub fn unite(&mut self, a: usize, b: usize) -> bool {
        let mut aa = self.find(a);
        let mut bb = self.find(b);
        if aa == bb {
            return false;
        }
        if self.size(aa) < self.size(bb) {
            std::mem::swap(&mut aa, &mut bb);
        }
        self.size[aa] += self.size[bb];
        self.par[bb] = aa;
        true
    }
}
