use crate::graph::UGraph;
use std::collections::VecDeque;
use itertools::Itertools;


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


pub fn tree_width_rec(g:&UGraph,l:&Vec<i32>,s:&Vec<i32>) -> i32{
    let n = s.len();
    if s.len() == 1 {
        return out_path(g,l,s[0]);
    }
    let mut opt = g.nodes.len();
    let iterator = s.iter().combinations(n/2);

    for item in iterator{
        let vec = convert(item);
        let (new_s,new_l) = transfer(s,l,&vec);

        let v1 = tree_width_rec(g,l,&vec);
        let v2 = tree_width_rec(g,&new_l,&new_s);
        
        let max = if v1 > v2 {v1} else {v2};
        if (max as usize) < opt{
            opt = max as usize;
        }
    }
    opt as i32

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
