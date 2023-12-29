// 点定义
#[derive(Debug)]
pub struct VertexMatrix<'a> {
    m_id: usize,
    m_name: &'a str,
}

impl VertexMatrix<'_> {
    pub fn new(id: usize, name: &'static str) -> Self { Self { m_id: id, m_name: name } }
}

// 边定义
#[derive(Debug, Clone)]
pub struct EdgeMatrix {
    m_edge: bool,    // 表示是否右边，并不需要构造一个边实体
}

impl EdgeMatrix {
    pub fn new() -> Self { Self { m_edge: false } }
    pub fn set_edge() -> Self { Self { m_edge: true } }
}

// 图定义
#[derive(Debug)]
pub struct GraphMatrix {
    m_nodes: usize,
    m_graph: Vec<Vec<EdgeMatrix>>, // 每个点的边放一个Vec
}

impl GraphMatrix {
    pub fn new(nodes: usize) -> Self {
        Self { m_nodes: nodes, m_graph: vec![vec![EdgeMatrix::new(); nodes]; nodes] }
    }

    pub fn len(&self) -> usize { self.m_nodes }

    pub fn is_empty(&self) -> bool { self.m_nodes == 0 }

    // 添加边，设置边属性为true
    pub fn add_edge(&mut self, n1: &VertexMatrix, n2: &VertexMatrix) {
        if n1.m_id < self.m_nodes && n2.m_id < self.m_nodes {
            self.m_graph[n1.m_id][n2.m_id] = EdgeMatrix::set_edge();
        } else { panic!("error") };
    }
}

#[cfg(test)]
mod tests {
    use crate::graph::graph_matrix;

    fn test_graph_matrix() {
        let mut g = graph_matrix::GraphMatrix::new(4);

        let n1 = graph_matrix::VertexMatrix::new(0, "n1");
        let n2 = graph_matrix::VertexMatrix::new(1, "n2");
        let n3 = graph_matrix::VertexMatrix::new(2, "n3");
        let n4 = graph_matrix::VertexMatrix::new(3, "n4");

        g.add_edge(&n1, &n2);
        g.add_edge(&n1, &n3);
        g.add_edge(&n2, &n3);
        g.add_edge(&n2, &n4);
        g.add_edge(&n3, &n4);
        g.add_edge(&n3, &n1);

        println!("{:#?}", g);
        assert_eq!(g.is_empty(), false);
        assert_eq!(g.len(), 4);
    }
}