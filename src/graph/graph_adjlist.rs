use std::hash::Hash;
use std::collections::HashMap;

// 点定义
#[derive(Debug, Clone)]
pub struct VertexAdjlist<T> {
    pub m_key: T,
    pub m_connects: Vec<(T, i32)>, // 邻点集合(key, weight)
}

impl<T: Clone + PartialEq> VertexAdjlist<T> {
    pub fn new(key: T) -> Self {
        Self {
            m_key: key,
            m_connects: Vec::new(),
        }
    }

    // 判断与当前点是否相邻
    pub fn adjacent_key(&self, key: &T) -> bool {
        for (nbr, _wt) in self.m_connects.iter() {
            if nbr == key { return true; }
        }

        false
    }

    // 对于当前节点添加新的邻居，直接插入m_connects即可
    pub fn add_neighbor(&mut self, nbr: T, wt: i32) {
        self.m_connects.push((nbr, wt));
    }

    // 获取相邻的点集合
    pub fn get_connects(&self) -> Vec<&T> {
        let mut connects = Vec::new();
        for (nbr, _wt) in self.m_connects.iter() {
            connects.push(nbr);
        }

        connects
    }

    // 返回到邻点的边权重
    pub fn get_nbr_weight(&self, key: &T) -> &i32 {
        for (nbr, wt) in self.m_connects.iter() {
            if nbr == key { return wt; }
        }

        &0
    }
}

// 图定义
#[derive(Debug, Clone)]
pub struct GraphAdjlist<T> {
    m_vertnums: u32,
    // 点数
    m_edgenums: u32,
    // 边数
    m_vertices: HashMap<T, VertexAdjlist<T>>, // 点集合
}

impl<T: Hash + Eq + PartialEq + Clone> GraphAdjlist<T> {
    pub fn new() -> Self {
        Self {
            m_vertnums: 0,
            m_edgenums: 0,
            m_vertices: HashMap::<T, VertexAdjlist<T>>::new(),
        }
    }

    pub fn is_empty(&self) -> bool { self.m_vertnums == 0 }

    pub fn vertex_num(&self) -> u32 {
        self.m_vertnums
    }

    pub fn edge_num(&self) -> u32 {
        self.m_edgenums
    }

    pub fn contains(&self, key: &T) -> bool {
        for (nbr, _vertex) in self.m_vertices.iter() {
            if nbr == key { return true; }
        }

        false
    }

    pub fn add_vertex(&mut self, key: &T) -> Option<VertexAdjlist<T>> {
        let vertex = VertexAdjlist::new(key.clone());
        self.m_vertnums += 1;
        self.m_vertices.insert(key.clone(), vertex)
    }

    pub fn get_vertex(&self, key: &T) -> Option<&VertexAdjlist<T>> {
        if let Some(vertex) = self.m_vertices.get(key) {
            Some(&vertex)
        } else {
            None
        }
    }

    // 获取所有节点的 key
    pub fn vertex_keys(&self) -> Vec<T> {
        let mut keys = Vec::new();
        for key in self.m_vertices.keys() {
            keys.push(key.clone());
        }

        keys
    }

    // 删除点 (同时要删除边)
    pub fn remove_vertex(&mut self, key: &T) -> Option<VertexAdjlist<T>> {
        let old_vertex = self.m_vertices.remove(key);
        self.m_vertnums -= 1;

        // 删除从当前点出发的边
        self.m_edgenums -= old_vertex.clone()
            .unwrap()
            .get_connects()
            .len() as u32;

        // 删除到当前点的边
        for vertex in self.vertex_keys() {
            if let Some(vt) = self.m_vertices.get_mut(&vertex) {
                if vt.adjacent_key(key) {
                    vt.m_connects.retain(|(k, _)| k != key);
                    self.m_edgenums -= 1;
                }
            }
        }

        old_vertex
    }

    pub fn add_edge(&mut self, from: &T, to: &T, wt: i32) {
        // 若点不存在要先添加点
        if !self.contains(from) {
            let _fvert = self.add_vertex(from);
        }

        if !self.contains(to) {
            let _tvert = self.add_vertex(to);
        }

        // 添加边
        self.m_edgenums += 1;
        self.m_vertices.get_mut(from)
            .unwrap()
            .add_neighbor(to.clone(), wt);
    }

    // 判断两个点是否相邻
    pub fn is_adjacent(&self, from: &T, to: &T) -> bool {
        self.m_vertices.get(from).unwrap().adjacent_key(to)
    }
}

#[cfg(test)]
mod tests {
    use crate::graph::graph_adjlist;

    fn test_vertex_adjlist() {
        let mut g = graph_adjlist::GraphAdjlist::new();

        for i in 0..6 { g.add_vertex(&i); }
        assert_eq!(g.is_empty(), false);

        let vertices = g.vertex_keys();
        for vertex in vertices { print!("vertex: {}; ", vertex); }
        println!();

        g.add_edge(&0, &1, 5);
        g.add_edge(&0, &5, 2);
        g.add_edge(&1, &2, 4);
        g.add_edge(&2, &3, 9);
        g.add_edge(&3, &4, 7);
        g.add_edge(&3, &5, 3);
        g.add_edge(&4, &0, 1);
        g.add_edge(&4, &4, 8);
        assert_eq!(g.vertex_num(), 6);
        assert_eq!(g.edge_num(), 8);
        assert_eq!(g.contains(&0), true);

        let vertex = g.get_vertex(&0).unwrap();
        assert_eq!(vertex.get_nbr_weight(&1), &5i32);

        let keys = vertex.get_connects();
        for nbr in keys { print!("neighbor: {nbr}; "); }
        println!();

        for (nbr, wt) in vertex.m_connects.iter() {
            print!("0 neighbor: {nbr}, weight: {wt}; ");
        }
        println!();

        let res = g.is_adjacent(&0, &1);
        assert_eq!(res, true);
        let res = g.is_adjacent(&3, &2);
        assert_eq!(res, false);

        let rm = g.remove_vertex(&0).unwrap();
        assert_eq!(rm.m_key, 0);
        assert_eq!(g.vertex_num(), 5u32);
        assert_eq!(g.edge_num(), 5u32);
        assert_eq!(g.contains(&0), false);
    }
}