use crate::graph::UGraph;
use std::collections::VecDeque;
use itertools::Itertools;
use std::collections::HashMap;

/**
 * Convert a tuple (u32,u32) to (i32,i32)
 */
pub fn tu32_to_ti32(x: (u32,u32)) -> (i32,i32){
    (x.0 as i32,x.1 as i32 )
}

/**
 *  Compute a lower bound for the tw of a given graph
 */
pub fn mmd(g:&UGraph) -> i32 {
    let mut h = g.clone();
    let mut maxmin = 0;
    while h.number_of_nodes()>=2 {
        let (deg,node) = h.min_degree();
        maxmin = if maxmin > deg {maxmin} else {deg};
        h.remove_node(node);
    }
    maxmin

}

/**
 *  Compute connected component in a given graph containing v and verteces only vertices from a set
 *  of vertices
 */
fn in_path(g:&UGraph,s:&Vec<i32>,v:i32)->Vec<i32>{
    let mut in_path:Vec<i32> = vec![v];
    let mut visited = vec![v];
    let mut queue = VecDeque::new();
    queue.push_back(v);

    while queue.len() != 0{
        let current = queue.pop_front().expect("REASON");

        for neighbor in g.neighbors.get(&current).unwrap(){
            if !visited.contains(&neighbor){
                if s.contains(&neighbor){
                    queue.push_back(*neighbor);
                    in_path.push(*neighbor);
                }
                visited.push(*neighbor);
            }
        }
    }

    in_path
}

/**
 * Compute Q(l,v) which is the number of nodes w in V - l - {v} where there is a path from v to w
 * pasing only by vertices from l  
 */
pub fn out_path(g:&UGraph,l:&Vec<i32>,v:i32)->i32{
    let mut out:Vec<i32> = vec![];
    for node in g.nodes.iter(){
        if !l.contains(node) && v != *node {
            out.push(*node);
        }
    }

    let s = in_path(g,l,v);
    let mut q = 0;
    for w in out.iter(){
        for neighbor in g.neighbors.get(&w).unwrap(){
            if s.contains(&neighbor){
                q += 1;
                break;
            }
        }
    }
    q
}

/**
 * return all vertices w where there is a path from a given node to w in the given graph g passing
 * only by vertices in s
 */
pub fn passing_by(g:&UGraph,s:&Vec<i32>,node:i32)-> Vec<i32>{
    let mut in_path:Vec<i32> = vec![];
    let mut visited = vec![node];
    let mut queue = VecDeque::new();
    queue.push_back(node);

    while queue.len() != 0{
        let current = queue.pop_front().expect("REASON");

        for neighbor in g.neighbors.get(&current).unwrap(){
            if !visited.contains(&neighbor){
                if !s.contains(&neighbor){
                    queue.push_back(*neighbor);
                }
                else if node < *neighbor{
                    in_path.push(*neighbor);
                }
                visited.push(*neighbor);
            }
        }
    }

    in_path
}
