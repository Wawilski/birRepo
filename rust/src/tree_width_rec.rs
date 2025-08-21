use crate::graph::UGraph;
use std::collections::VecDeque;
use itertools::Itertools;
use std::collections::HashMap;
use crate::utils::{mmd,out_path,passing_by};



/**
 * Give the tw of a certain graph g
 */
pub fn tree_width_rec(g:&UGraph) -> i32{   
    let mmd = mmd(g);
    _tree_width_rec(&g,&vec![],&g.nodes,mmd)
}

/**
 * Compute TWR(l,s) for the graph g 
 * with:
 *    V vertices of g,
 *    l[i] != s[i] for all i,
 *    s is invcude to V,
 * TWR(l,s) = min[between all pi (linear ordering) of V] max[v in s] | q(l U (pi < v ),v) | 
 * (see utils::outpath for informations about q)
 *
 * As TWR([],V) = tw(g) tehe call in tree_width_rec give the correct three_width of g
 */
pub fn _tree_width_rec(g:&UGraph,l:&Vec<i32>,s:&Vec<i32>,mmd:i32) -> i32{
    let n = s.len();
    if s.len() == 1 {
        return out_path(g,l,s[0]);
    }
    let mut opt:i32 = g.nodes.len() as i32;
    let iterator = s.iter().combinations(n/2);

    for item in iterator{
        let vec = convert(item);
        let (new_s,new_l) = transfer(s,l,&vec);

        let v1 = _tree_width_rec(g,l,&vec,mmd);
        let v2 = _tree_width_rec(g,&new_l,&new_s,mmd);
        
        let max = if v1 > v2 {v1} else {v2};
        if max < opt{
            opt = max;
        }
        if opt <= mmd || opt == 1 {
            return opt;
        }
    }
    opt

}

/**
 * Convert Vec of &i32 to Vec of i32
 */
pub fn convert(v:Vec<&i32>) -> Vec<i32>{
    let mut ret = vec![];
    for item in v{
        ret.push(*item);
    }
    ret
}

/**
 * s: the vec to transfer from
 * l: the vec to transfer to
 * sub: the set of item to transfer
 *
 * create 2 list which correspond to:
 * s without elements from sub
 * l with elements items from sub
 */
pub fn transfer(s:&Vec<i32>,l:&Vec<i32>,sub:&Vec<i32>) -> (Vec<i32>,Vec<i32>) {
    let mut new_s = vec![];
    let mut new_l = l.clone();
    for i in sub.iter() {
        new_l.push(*i);
    }
    for j in s.iter(){
        if !sub.contains(j){
            new_s.push(*j);
        }
    }
    (new_s,new_l)
}


/**
 *  Create the fill_in_graph of a given graph(V,E) based on w, a set of vertices.
 *  
 *  fill_in_graph is as: g_plus(w,F) with (a,b) in F iff path from a to b with only verties from V - w
 */
pub fn fill_in_graph(g:&UGraph,w:&Vec<i32>) -> UGraph{
    let mut g_plus = UGraph::new_set_graph(w.clone(),vec![]);
    for node in w{
       let path = passing_by(g,w,*node);
       for item in path.iter(){
           g_plus.add_edge((*node,*item));
       }
    }
    g_plus

}

/**
 *  Find all connected components from a given graph
 *
 */
pub fn connected_components(g:&UGraph) -> Vec<Vec<i32>>{
    let mut connected_comp = vec![];
    let mut visited = vec![];
    for node in g.nodes.clone(){
        if !visited.contains(&node){
            visited.push(node);
            let mut connex = vec![node];
            let mut stack = vec![node];

            while stack.len() != 0{
                let current = stack.pop().expect("REASON");

                for neighbor in g.neighbors.get(&current).unwrap(){
                    if !visited.contains(&neighbor){
                        stack.push(*neighbor);
                        visited.push(*neighbor);
                        connex.push(*neighbor);
                    }
                }
            }
        connected_comp.push(connex);
        }
    }
    connected_comp
}

/**
 * compute the tree width by searching with ascending values of k
 *
 */
pub fn improved_tree_width_rec_up(g:&UGraph) -> i32{
    let mut k = 1;
    while !improved_tree_width(g,k){

        k += 1;
    }
    k-1
}
/**
 * compute the tree width by searching with descending values of k
 *
 */
pub fn improved_tree_width_rec_down(g:&UGraph) -> i32{
    let mut k = g.number_of_nodes();
    while improved_tree_width(g,k){
        k -= 1;
    }
    k+1
}

/**
 * Tell if there is or not a tree decomposition <= k
 *
 */
pub fn improved_tree_width(g:&UGraph, k:i32)-> bool{
    let n = g.number_of_nodes();
    if n <= (k+1){
        return true;
    }
    let mmd = mmd(g);
    if (k as f32) <= 0.25*(n as f32) || (k as f32) >= 0.4203*(n as f32){
        let iterator = g.nodes.iter().combinations((k+1) as usize);
        for item in iterator{
            let s = convert(item);
            let g_wout_s = g.copy_without(&s);
            let comps = connected_components(&g_wout_s);
            if comps.clone().into_iter().max_by_key(|x| x.len()).unwrap().len() <=(((n - k)/2) as usize ){  
                let mut tbool = true;
                for w in comps{
                    tbool = tbool && (_tree_width_rec(&g,&vec![],&w,mmd) <= k);
                }
                if tbool{
                    return true;
                }
            }
        }
    }
    else {
        let iterator = g.nodes.iter().combinations(((0.4203*(n as f32)) as usize) + 1);
        for item in iterator {
            let s = convert(item);
            let g_wout_s = g.copy_without(&s);
            let comps = connected_components(&g_wout_s);
            if comps.clone().into_iter().max_by_key(|x| x.len()).unwrap().len() <=(((n - k)/2) as usize ){  
                let g_plus = fill_in_graph(g,&s);
                let mut tbool = improved_tree_width(&g_plus,k);
                for w in comps{
                    tbool = tbool && (_tree_width_rec(&g,&vec![],&w,mmd) <= k);
                }
                if tbool{
                    return true;
                }
            }
        }
    }
    return false;
}
