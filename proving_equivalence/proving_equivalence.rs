// Submission ID: 19117284

use std::{io::{self, Read}, vec};

fn load_input() -> Vec<u64>{
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let data: Vec<u64> = input
        .split_whitespace()
        .map(|x| x.parse::<u64>().unwrap())
        .collect();

    data
}

struct Kosaraju<'a> {
    adj: &'a Vec<Vec<usize>>,
    radj: &'a Vec<Vec<usize>>,
    visited: Vec<bool>,
    component: Vec<usize>,
    l: Vec<usize>,
}

// kosaraju code inspired by https://en.wikipedia.org/wiki/Kosaraju%27s_algorithm
impl<'a> Kosaraju<'a>{
    fn new(adj: &'a Vec<Vec<usize>>, radj: &'a Vec<Vec<usize>>) -> Self {
        let n = adj.len();

        Self { adj, radj, visited: vec![false; n], component: vec![usize::MAX; n], l: Vec::new()}
    }

    fn visit(&mut self, u: usize) {
        if self.visited[u] {
            return;
        }

        // depth first search (DFS). goes through recursively, node -> connected node -> so on
        // to make an array of an ordered path through the nodes. 
        self.visited[u] = true;
        let neighbors = self.adj[u].clone();

        for v in neighbors {
            self.visit(v);
            
        }
        self.l.push(u);
    }

    fn assign(&mut self, u: usize, root: usize) {
        if self.component[u] != usize::MAX {
            return;
        }
        
        // DFS on reversed graph to place nodes in its
        // strongly connected component (SCC).
        self.component[u] = root;
        let neighbors = self.radj[u].clone();
        for v in neighbors {
            self.assign(v, root);
            
        }
    }

    fn run(&mut self) -> usize{
        // forward pass
        for u in 0..self.adj.len() {
            if !self.visited[u] {
                self.visit(u);
            }
        }

        // order is ordered reverse right now.
        let order = self.l.clone();

        // sets the root node to node 0
        let mut root = 0;
        for &u in order.iter().rev() {
            if self.component[u] == usize::MAX {
                self.assign(u, root);
                root += 1;
            }
        }
        root
    }

}
fn main() {

    let data = load_input();

    let mut it = data.into_iter();
    let tests = it.next().unwrap() as usize;

    for _ in 0..tests {
        let n = it.next().unwrap() as usize;
        let m = it.next().unwrap() as usize;


        // make adjacent list and reversed adjancent list
        let mut adj = vec![Vec::new(); n];
        let mut radj = vec![Vec::new(); n];

        for _ in 0..m {
            let u = it.next().unwrap() as usize - 1;
            let v = it.next().unwrap() as usize - 1;
            adj[u].push(v);
            radj[v].push(u);
        }

        // run kosaraju
        let mut kosaraju = Kosaraju::new(&adj, &radj);
        let k_clusters = kosaraju.run();
        let clusters = kosaraju.component;

        // count number of outgoing and ingoing edges from different SCCs
        let mut ingoing = vec![0; k_clusters];
        let mut outgoing = vec![0; k_clusters];

        // if an edge u->v, and u and v not in same SCC
        // we have ingoing for vs SCC and outgoing for us SCC.

        for u in 0..n {
            for &v in &adj[u] {
                let cu = clusters[u];
                let cv = clusters[v];

                if cu != cv {
                    outgoing[cu] += 1;
                    ingoing[cv] += 1;
                }
            }
        }

        // as each SCC is internally "equivalant" it could be treated as a single node
        // for this task. To make the whole graph equivalent we to make
        // all theese SCC / new nodes equivalent. 
        // for a graph with no SSC we can count how many "start" nodes
        // and "end" nodes a graph has. (start = no ingoing edges, end=no outgoing edges)

        let mut starts = 0;
        let mut ends = 0;

        for c in 0..k_clusters {
            if ingoing[c] == 0 {
                starts += 1
            }
            if outgoing[c] == 0 {
                ends += 1
            }
        }

        // Now, every start needs atleast one ingoing edge, 
        // and every end needs atleast one outgoing edge to 
        // make the whole graph strongly connected.
        // each added edge can go from one start to one end
        // "fixing" both a start and an end at the same time.
        // if there are more ends than starts, we therefore need to add
        // number of ends - edges to make the graph equivalent. 

        if k_clusters == 1 {
            println!("0")
        } else {
            println!("{}", starts.max(ends))
        }

        // I have to confess, this problem was not solved without AI... :D, I couldnt figure it
        // out completelye. Especially when i got "time limit exceeded" on my first submission...
        // hope its fine :D
    }
}