use crate::graph::UGraph;
use std::collections::VecDeque;
use itertools::Itertools;
use std::collections::HashMap;
use crate::tree_width_rec::out_path;


pub fn twdp(g:UGraph) -> i32 {
    let n = g.number_of_nodes();
    let up = n-1;
    let mut tw_pred:Vec<(Vec<i32>,i32)> = vec![(vec![],-1)];
    let mut tw:Vec<(Vec<i32>,i32)> = vec![] ;

    for _ in 0..n{
        tw = vec![];
        for pair in tw_pred.iter(){
            let diff = difference(&g.nodes,&pair.0);        
            for x in diff.iter() {
                let q = out_path(&g,&pair.0,*x);
                let r = if q < pair.1 {pair.1} else {q};
                if r < up {
                    let mut s_x = pair.0.clone();
                    s_x.push(*x);
                    let mut found = false;
                    for item in tw.iter_mut(){
                        if item.0 == s_x{
                            item.1 = if item.1 > r {r} else {item.1};
                            found = true;
                        }
                    }
                    if !found {
                        tw.push((s_x, r));
                    }

                }   
            }
        }
        tw_pred = tw.clone();
    }
    for item in tw.iter(){
        if item.0 == g.nodes {
            println!("{:?}",item);
            return item.1;
        }

    }
    return up;
    
}


pub fn difference(v:&Vec<i32>,s:&Vec<i32>) -> Vec<i32>{
    let mut diff = vec![];
    for i in v.iter(){
        if !s.contains(i){
            diff.push(*i);
        }
    }
    diff
}
