use std::cmp;
use std::collections::VecDeque;
use crate::graph::UGraph;

/**
 * Compute girth of a given graph
 * */
pub fn girth(g:UGraph) -> Option<i32> {
    let n_size:usize = g.number_of_nodes().try_into().unwrap();
    let n = g.number_of_nodes();
    let mut girth:i32 = n+1;

    for node in g.nodes.clone(){
        let mut dist = vec![n+1;n_size];
        let mut prev = vec![-1;n_size];
        let mut queue= VecDeque::new();
        
        dist[node as usize] = 0;
        queue.push_back(node);

        while queue.len() != 0 {
            let current = queue.pop_front().expect("REASON");
            for w in g.neighbors.get(&current).unwrap().iter(){
                let index:usize = <i32 as TryInto<usize>>::try_into(*w).unwrap();
                let current_index:usize = current.try_into().unwrap();

                if dist[index] ==n+1 {
                    dist[index] = dist[current_index] + 1;
                    prev[index] = current;
                    queue.push_back(*w);
                }
                if prev[index] != current && prev[current_index] != *w{
                    girth = cmp::min(girth, dist[index] + dist[current_index] + 1);
                }
            }
        }
    }
    if girth == n+1{
        return None;
    }
        Some(girth)
}

