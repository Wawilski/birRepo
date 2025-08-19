mod graph;
mod inv_girth;
mod inv_tree_width;
mod inv_proxi_remote;
mod inv_variance;
mod tree_width_rec;

use std::any::type_name;
use std::time::Instant;
use crate::graph::UGraph;
use inv_girth::girth;
use inv_tree_width::tree_width;
use inv_variance::var_degree;
use inv_proxi_remote::{Options,minmax_mean_distance};
use tree_width_rec::{out_path,tree_width_rec,fill_in_graph,connected_components,improved_tree_width_rec_down,improved_tree_width};
use itertools::Itertools;
use std::fs::File;
use std::io::{self, BufRead,BufReader};
use std::path::Path;
use std::collections::HashMap;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn compare<P>(filename1: P , filename2: P) -> io::Result<()>where P: AsRef<Path>,{
    println!("k");
    let file_path1 = "../outputs/r_outputs_middle_tree_width.txt";
    let file_path2 = "middle_down.txt";
    if let Ok(lines_1) = read_lines(file_path1) {
        println!("in");
        if let Ok(lines_2) = read_lines(file_path2) {
                

            let it_1: Vec<_> = lines_1.map_while(Result::ok).collect();
            let it_2 : Vec<_>= lines_2.map_while(Result::ok).collect();
                    
            for i in 0..it_1.len(){
                if it_1[i][1..] != it_2[i]{
                    println!("{:?} != {:?}",it_1[i],it_2[i]);
                }
            }
                
            }
        
    }   
    Ok(())
}
fn main() -> io::Result<()>{

    //let g = UGraph::new_n_graph(4,vec![(0,1),(0,2),(0,3),(1,2),(1,3),(2,3)]);
    // let g = UGraph::new_n_graph(8,vec![(0,1),(0,2),(0,3),(1,2),(1,3),(2,3),(3,4),(3,5),(4,5),(4,6),(4,7),(5,6),(5,7)]);
    //let g = UGraph::new_n_graph(8,vec![(0,1),(0,2),(1,2),(4,5),(4,6),(4,7),(5,6),(5,7)]);
    // let g = UGraph::new_n_graph(10,vec![(0,1),(0,4),(0,5),(1,2),(1,6),(2,3),(2,7),(3,4),(3,8),(4,9),(5,7),(5,8),(6,8),(6,9),(7,9)]);
    // println!("{:?}",conn(&g,&vec![],&g.nodes));
    // println!("{:?}",tree_width_rec(&g,&vec![],&g.nodes));
    // println!("{:?}",tree_width(g));
    // let mut mem: HashMap<Vec<i32>,i32> = HashMap::new();
    // let var =tree_width_rec(&g,&vec![],&g.nodes,&mut mem);
    // println!("{:?}",mem);
    // println!("{:?}",var);
    let graph = UGraph::graph_from_g6("F?AZO".to_string());
    // println!("{}",improved_tree_width(&graph,1));
    println!("{}",tree_width_rec(&graph));
    
    // let file_path = "../g6Files/middle.g6";
    // let input = File::open(file_path)?;
    // let buffered = BufReader::new(input);
    // let count = buffered.lines().count() as u128;
    //
    // if let Ok(lines) = read_lines(file_path) {
    //     let now = Instant::now();
    //     for line in lines.map_while(Result::ok) {
    //
    //         let graph = UGraph::graph_from_g6((&line[1..]).to_string());
    //         // println!("{},{:?}",&line[1..],tree_width(graph));
    //         // let var =tree_width_rec(&graph,&vec![],&graph.nodes);
    //         //
    //         // println!("{},{:?}",&line[1..],tree_width_rec(&graph));
    //         // let var = minmax_mean_distance(graph,Options::Remoteness);
    //         // let var = girth(graph);
    //         // let var = tree_width(graph);
    //         // let var = improved_tree_width_rec_down(&graph);
    //         println!("{},{:?}",&line[1..],improved_tree_width_rec_down(&graph));
    //     }
    //     let end = now.elapsed().as_nanos();
    //     println!("{}",end);
    //     println!("{}",end/count);
    // }
    Ok(())
    // compare("../g6/Files/small.g6","small_down.g66")
    
}
