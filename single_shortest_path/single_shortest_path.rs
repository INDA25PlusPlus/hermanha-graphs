// Submission ID: 19107021

use std::{cmp::Reverse, collections::BinaryHeap, io::{self, Read}, vec};

const INF: u64 = 1_000_000_000;

struct Graph {
    edges: Vec<Vec<(usize, u64)>>
}

impl Graph {
    pub fn new(n: usize) -> Self {
        Self {
            edges: vec![Vec::new(); n],
        }
    }

    fn make_adjacent(&mut self, edges: &[(usize, usize, u64)]) {
        for &(u,v,w) in edges {
            self.edges[u].push((v,w))
        }
    }

    fn distance(&self, u: usize, v: usize) -> u64{
        for &(to, w) in &self.edges[u] {
            if to == v{
                return w
            }
        }
        INF
    }
}

fn dijkstra(
    edges: &[(usize, usize, u64)],
    start: usize,
    n: usize,
) -> Vec<u64> {
    let mut graph = Graph::new(n);
    graph.make_adjacent(edges);

    let mut dist = vec![INF;n];
    dist[start] = 0;

    let mut heap = BinaryHeap::new();
    heap.push((Reverse(0u64), start));
    
    while let Some((Reverse(d), u)) = heap.pop() {
        for &(v,w) in &graph.edges[u] {
            let alt = dist[u] + w;
            if alt < dist[v] {
                dist[v] = alt;
                heap.push((Reverse(alt), v));
            }
        }
    }

    dist
}

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

        let distances = dijkstra(&edges, s, n);

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