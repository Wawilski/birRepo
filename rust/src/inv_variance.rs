use crate::graph::UGraph;

pub fn mean_degree(g:UGraph)->f32{
    let n = g.number_of_nodes();
    if n < 2{
        return 0.0;
    } 
    let mut count = 0;
    for node in &g.nodes{
        count += &g.degree(*node);}
    (count as f32) / (n as f32)
}

pub fn var_degree(g:UGraph)->f32{
    let n = g.number_of_nodes();
    if n < 2 {
        return 0.0;
    }
    let mean = mean_degree(g.clone());
    let mut count = 0.0;
    for node in &g.nodes{
        count += ((g.degree(*node) as f32) -mean).powf(2.0);
    }
    (count as f32) / ((n - 1) as f32)
}

