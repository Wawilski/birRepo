use crate::graph::UGraph;
use std::collections::VecDeque;
use itertools::Itertools;
use std::collections::HashMap;
use crate::inv_tree_width::mmd;

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
        // println!("s {:?}",s);
        // println!("sub {:?}",sub);
        if !sub.contains(j){
            new_s.push(*j);
        }
    }
    (new_s,new_l)
}



// pub fn improved_tree_width_rec(g:UGraph, k:i32){
//     let n = g.number_of_nodes();
//     if n <= (k+1){
//         return true;
//     }
//     if (k as f32) <= 0.25*n || (k as f32) >= 0.4203*n{
//         let iterator = g.nodes.iter().combinations(k+1);
//         for item in iterator{
//             let vec = convert(item);
//             }
//     }
// }
