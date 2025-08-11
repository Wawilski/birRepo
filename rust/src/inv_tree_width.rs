use crate::graph::UGraph;

struct Permutation {
    array:Vec<i32>
}

impl Permutation{

    pub fn new(n:i32)->Self{
        let array = (0..n).collect();
        Self{    
            array
        }
    }
    pub fn next(&mut self)-> bool{
        let mut pivot: usize = self.array.len()-1;

        while pivot > 0 && (self.array[pivot] < self.array[pivot-1]){
            pivot-=1;
        }
        if pivot > 0 {pivot-=1;}
        else{ return false; }

        let mut j:usize = (self.array.len()-1).try_into().unwrap();
        while j > pivot {
            if self.array[j] > self.array[pivot]{
                let c = self.array[j];
                self.array[j] = self.array[pivot];
                self.array[pivot] = c;
                break;
            }
            j-=1;
        }
        
        let first:usize = pivot+1;
        let last:usize = (self.array.len()-1).try_into().unwrap();
        j = 0;
        while j < (first +last)/2 - first{
            let c = self.array[first+j];
            self.array[first+j] = self.array[last-j];
            self.array[last-j] = c;
            j+=1;
        }
        return true;
    }
}


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



pub fn tree_width(g:UGraph)->i32{
    let n = g.number_of_nodes();
    let g_edges = g.edges.clone();
    let mut o_perm = Permutation::new(n);
    let mut n_min_max_deg = n;
    let lower_bound = mmd(&g); 

    loop {
        let mut c = UGraph::new_n_graph(n,vec![]);
        for i in 0..n-1 {
            for j in (i+1)..n{
                if g_edges.contains(&(i,j))|| g.edges.contains(&(j,i)){
                    c.add_edge((o_perm.array[i as usize],o_perm.array[j as usize]));
                }
            }
        }
        let mut n_max_deg = 0;

        for i in 0..n{
            let mut n_deg = 0;
            let mut first = 0;

            for j in (i+1)..n{
                if c.edges.contains(&(i,j))|| c.edges.contains(&(j,i)){
                    n_deg += 1;
                    
                    if first>0{ c.add_edge((first,j));}
                    else { first = j;}
                }
            }
            if n_deg > n_max_deg {
                n_max_deg = n_deg;
            }

            if n_deg >= n_min_max_deg {
                break;
            }
        }

        if n_max_deg < n_min_max_deg{
            n_min_max_deg = n_max_deg;
        }
        if n_min_max_deg <= lower_bound || n_min_max_deg == 1 {
            return n_min_max_deg;
        }

        if !o_perm.next(){
            break;
        }
    }
    n_min_max_deg
}


