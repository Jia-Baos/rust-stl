//#[allow(dead_code)]
mod func;

use self::func::*;  // 采用self
//use crate::func::*;  // 采用crate

use rust_stl::graph;

fn main()
{
    print_rust();

    // 边是无向的
    let data = [
        [1, 2], [2, 1], [1, 3], [3, 1], [2, 4],
        [4, 2], [2, 5], [5, 2], [3, 6], [6, 3],
        [3, 7], [7, 3], [4, 5], [5, 4], [6, 7],
        [7, 6], [5, 8], [8, 5], [6, 8], [8, 6]];
    let gp = graph::create_graph(data);
    //graph::bfs(gp);
    graph::dfs(gp);
}