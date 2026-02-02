// Submission ID: 19083435

use std::{io::{self, Read}, vec};

const INF: u64 = 1_000_000_000;

fn bellman_ford(
    edges: &[(usize, usize, u64)],
    start: usize,
    n: usize,
) -> Vec<u64> {
    let mut dist = vec![INF; n];
    dist[start] = 0;

    for _ in 0..n - 1 {
        let mut changed = false;
        for &(u, v, w) in edges {
            if dist[u] == INF { 
                continue; 
            }
            if dist[u] + w < dist[v] {
                dist[v] = dist[u] + w;
                changed = true;
            }
        }
        if !changed { break; }
    }
    dist
}

fn load_input() -> Vec<u64>{
    // load input from kattis, got some help from GPT hihi
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let data: Vec<u64> = input
        .split_whitespace()
        .map(|x| x.parse::<u64>().unwrap())
        .collect();

    data
}

fn main() {
    let data = load_input();

    let mut it = data.into_iter();

    let mut n = it.next().unwrap() as usize;
    let mut m = it.next().unwrap() as usize;
    let mut q = it.next().unwrap();
    let mut s = it.next().unwrap() as usize;

    while (n, m, q, s) != (0, 0, 0,0){
        let mut edges: Vec<(usize, usize, u64)> = Vec::new();

        for _ in 0..m {
            let u = it.next().unwrap() as usize;
            let v = it.next().unwrap() as usize;
            let w = it.next().unwrap();

            edges.push((u,v,w));
        }

        let distances = bellman_ford(&edges, s, n);

        for _ in 0..q {
            let end = it.next().unwrap() as usize;

            let dist = distances[end];
            if dist == INF {
                println!("Impossible")
            } else {
                println!("{}", dist)
            }
        }

        n = it.next().unwrap() as usize;
        m = it.next().unwrap() as usize;
        q = it.next().unwrap();
        s = it.next().unwrap() as usize;


        if (n, m, q, s) != (0,0,0,0) {
            println!()
        }
    }
}