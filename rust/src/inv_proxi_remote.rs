use crate::graph::UGraph;
use std::cmp;
use std::collections::VecDeque;
use std::time::Instant;


pub fn mean_distances(g:&UGraph,node:i32)-> f32{
    let n = g.number_of_nodes();
    if n < 2 {
        return (n+1) as f32;
    }
    let mut visited = vec![node];
    let mut queue = VecDeque::new();
    let mut distances = vec![0;n.try_into().unwrap()];
    queue.push_back(node);
    while queue.len() != 0{
        let current = queue.pop_front().expect("REASON");

        for neighbor in g.neighbors(current){
            if !visited.contains(&neighbor){

                queue.push_back(neighbor);
                visited.push(neighbor);
                distances[neighbor as usize] = distances[current as usize] + 1;
            }
        }
    }
    if visited.len() < (n as usize){
        return (n+1)as f32;
    }
    (distances.iter().sum::<i32>() as f32)/((n-1) as f32)
}


pub fn minmax_mean_distance(g:UGraph, option:i32) -> f32 {
    let n = g.number_of_nodes();
    let mut minmax_dist = (n+1) as f32;
    if option == 1{
        minmax_dist = 0.0;
    }
    for node in &g.nodes{
        let mean = mean_distances(&g,*node);
        if mean > minmax_dist && option == 1 {
            minmax_dist = mean;
        }
        else if mean < minmax_dist && option == 0 {
            minmax_dist = mean;
        }   
    }
    minmax_dist
}
