use crate::graph::UGraph;
use std::collections::VecDeque;
use std::time::Instant;

#[derive(Debug,PartialEq)] 
pub enum Options {
    Proximity,
    Remoteness,
}

pub fn mean_distances(g:&UGraph,node:i32)-> f32{
    let n = g.number_of_nodes();
    if n < 2 {
        return (n+1) as f32;
    }
    let mut visited = vec![node];
    let mut queue = VecDeque::new();
    let mut distances = vec![0;n as usize];
    let mut sum_dist = 0;
    queue.push_back(node);

    while queue.len() != 0{
        let current = queue.pop_front().expect("REASON");

        for neighbor in g.neighbors.get(&current).unwrap(){
            if !visited.contains(&neighbor){
                queue.push_back(*neighbor);
                visited.push(*neighbor);
                distances[*neighbor as usize] = distances[current as usize] + 1;
                sum_dist += distances[*neighbor as usize];
            }
        }
    }
    if visited.len() < (n as usize){
        return (n+1)as f32;
    }
    (sum_dist as f32)/((n-1) as f32)
}


pub fn minmax_mean_distance(g:UGraph, option:Options) -> f32 {
    let n = g.number_of_nodes();
    let mut minmax_dist = (n+1) as f32;
    if option == Options::Remoteness{
        minmax_dist = 0.0;
    }
    for node in &g.nodes{
        let mean = mean_distances(&g,*node);
        if mean > minmax_dist && option == Options::Remoteness {
            minmax_dist = mean;
        }
        else if mean < minmax_dist && option == Options::Proximity {
            minmax_dist = mean;
        }   
    }
    minmax_dist
}
