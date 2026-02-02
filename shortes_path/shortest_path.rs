//Submission ID: 19083130


use std::{io::{self, Read}, ops::Neg, vec};

const INF: i64 = 1_000_000_000;
const NEG_INF: i64 = -1_000_000_000;


fn floyd_warshall(edges: &[(usize, usize, i64)], n: usize) -> Vec<Vec<i64>>{
    // floyd_warshall algorithm, code inspired by https://en.wikipedia.org/wiki/Floyd%E2%80%93Warshall_algorithm

    let mut dist = vec![vec![INF; n]; n];

    for i in 0..n {
        dist[i][i] = 0;
    }

    for &(u, v, w) in edges {
        dist[u][v] = dist[u][v].min(w);
    }

    for k in 0..n {
        for i in 0..n {
            if dist[i][k] == INF {
                continue;
            }
            for j in 0..n {
                if dist[k][j] == INF {
                    continue
                }
                if dist[i][k] + dist[k][j] < dist[i][j] {
                    dist[i][j] = dist[i][k] + dist[k][j];
                }
            }
        }
    }

    for k in 0..n {
        if dist[k][k] < 0 {
            for i in 0..n {
                if dist[i][k] == INF { continue; }
                for j in 0..n {
                    if dist[k][j] == INF { continue; }
                    dist[i][j] = NEG_INF;
                }
            }
        }
    }

    return dist
}

fn load_input() -> Vec<i64>{
    // load input from kattis, got some help from GPT hihi
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let data: Vec<i64> = input
        .split_whitespace()
        .map(|x| x.parse::<i64>().unwrap())
        .collect();

    data
}

fn main() {
    let data = load_input();


    let mut it = data.into_iter();

    let mut n   = it.next().unwrap() as usize;
    let mut m   = it.next().unwrap() as usize;
    let mut q = it.next().unwrap();

    while (n, m, q) != (0, 0, 0){
        let mut edges: Vec<(usize, usize, i64)> = Vec::new();

        for _ in 0..m {
            let u = it.next().unwrap() as usize;
            let v = it.next().unwrap() as usize;
            let w = it.next().unwrap();

            edges.push((u,v,w));
        }

        let dist = floyd_warshall(&edges, n);

        for _ in 0..q {
            let u = it.next().unwrap() as usize;
            let v = it.next().unwrap() as usize;

            let shortest_distance = dist[u][v];

            if shortest_distance == NEG_INF {
                println!("-Infinity")
            }
            else if shortest_distance == INF {
                println!("Impossible");
            } else {
                println!("{}", shortest_distance);
            }
        }

        n = it.next().unwrap() as usize;
        m = it.next().unwrap() as usize;
        q = it.next().unwrap();

        if (n, m, q) != (0,0,0) {
            println!()
        }
    }

}