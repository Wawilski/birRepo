use crate::graph::UGraph;
use std::collections::VecDeque;
use itertools::Itertools;
use std::collections::HashMap;


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
pub fn tree_width_rec(g:&UGraph) -> i32{   
    let mut mem: HashMap<Vec<i32>,i32> = HashMap::new();
    let mmd = mmd(g);
    tree_width(&g,&vec![],&g.nodes,&mut mem,mmd)
}

pub fn tree_width(g:&UGraph,l:&Vec<i32>,s:&Vec<i32>,mem:&mut HashMap<Vec<i32>,i32>,mmd:i32) -> i32{
    let n = s.len();
    let scl = s.clone();
    if s.len() == 1 {
        mem.entry(scl).or_insert(out_path(g,l,s[0]));
        return *mem.get(s).unwrap();
    }
    if s.len() == 2 {
        if mem.contains_key(&scl){
            return *mem.get(s).unwrap();
        }
    }
    let mut opt:i32 = g.nodes.len() as i32;
    let iterator = s.iter().combinations(n/2);

    for item in iterator{
        let vec = convert(item);
        let (new_s,new_l) = transfer(s,l,&vec);

        let v1 = tree_width(g,l,&vec,mem,mmd);
        let v2 = tree_width(g,&new_l,&new_s,mem,mmd);
        
        let max = if v1 > v2 {v1} else {v2};
        if max < opt{
            opt = max;
        }
        if vec.len() == 2 {
            mem.insert(vec,opt);
        }
        if opt <= mmd || opt == 1 {
            return opt;
        }
    }
    opt

}

pub fn convert(v:Vec<&i32>) -> Vec<i32>{
    let mut ret = vec![];
    for item in v{
        ret.push(*item);
    }
    ret
}

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

pub fn improved_tree_width_rec_up(g:&UGraph) -> i32{
    let mut k = 1;
    while !improved_tree_width(g,k){

        k += 1;
    }
    k-1
}
pub fn improved_tree_width_rec_down(g:&UGraph) -> i32{
    let mut k = g.number_of_nodes();
    while improved_tree_width(g,k){
        k -= 1;
    }
    k+1
}

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
                    let mut mem: HashMap<Vec<i32>,i32> = HashMap::new();
                    tbool = tbool && (tree_width(&g,&vec![],&w,&mut mem,mmd) <= k);
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
                    let mut mem: HashMap<Vec<i32>,i32> = HashMap::new();
                    tbool = tbool && (tree_width(&g,&vec![],&w,&mut mem,mmd) <= k);
                }
                if tbool{
                    return true;
                }
            }

        }
    }
    return false;
}
