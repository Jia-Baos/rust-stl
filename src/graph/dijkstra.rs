use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};

// 点
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Vertex<'a> {
    pub m_name: &'a str,
}

impl<'a> Vertex<'a> {
    pub fn new(name: &'a str) -> Vertex<'a> { Vertex { m_name: name } }
}

// 访问过的点
#[derive(Debug)]
pub struct Visited<V> {
    m_vertex: V,
    m_distance: usize, // 距离
}

// 为 Visited 添加全序比较功能
impl<V> Ord for Visited<V> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.m_distance.cmp(&self.m_distance)
    }
}

impl<V> PartialOrd for Visited<V> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<V> Eq for Visited<V> {}

impl<V> PartialEq for Visited<V> {
    fn eq(&self, other: &Self) -> bool {
        self.m_distance.eq(&other.m_distance)
    }
}

// 最短路径算法
pub fn dijkstra<'a>(
    start: Vertex<'a>,
    adj_list: &HashMap<Vertex<'a>, Vec<(Vertex<'a>, usize)>>)
    -> HashMap<Vertex<'a>, usize> {
    let mut distances = HashMap::new(); // 距离
    let mut visited = HashSet::new();   // 以访问过的点
    let mut to_visit = BinaryHeap::new();   // 代访问的点

    // 设置起始点和起始距离
    distances.insert(start, 0);
    to_visit.push(Visited { m_vertex: start, m_distance: 0 });

    while let Some(Visited { m_vertex: vertex, m_distance: distance }) = to_visit.pop() {
        // 以访问过该点，继续下一个点
        if !visited.insert(vertex) { continue; }

        // 获取邻点
        if let Some(neighbors) = adj_list.get(&vertex) {
            for (neighbor, cost) in neighbors {
                let new_distance = distance + cost;
                let is_shorter = distances
                    .get(&neighbor)
                    .map_or(true, |&curr| new_distance < curr);

                // 若距离更近，则插入新距离和邻点
                if is_shorter {
                    distances.insert(*neighbor, new_distance);
                    to_visit.push(Visited {
                        m_vertex: *neighbor,
                        m_distance: new_distance,
                    });
                }
            }
        }
    }
    distances
}

#[cfg(test)]
mod tests {
    use crate::graph::dijkstra;

    fn test_dijkstra() {
        let s = dijkstra::Vertex::new("s");
        let t = dijkstra::Vertex::new("t");
        let x = dijkstra::Vertex::new("x");
        let y = dijkstra::Vertex::new("y");
        let z = dijkstra::Vertex::new("z");

        let mut adj_list = dijkstra::HashMap::new();
        adj_list.insert(s, vec![(t, 10), (y, 5)]);
        adj_list.insert(t, vec![(y, 2), (x, 1)]);
        adj_list.insert(x, vec![(z, 4)]);
        adj_list.insert(y, vec![(t, 3), (x, 9), (z, 2)]);
        adj_list.insert(z, vec![(s, 7), (x, 6)]);

        let distances = dijkstra::dijkstra(s, &adj_list);

        for (v, d) in &distances {
            println!("{} to  {}, min distance: {d}", s.m_name, v.m_name);
        }

        assert_eq!(distances.get(&t), Some(&8));
        assert_eq!(distances.get(&s), Some(&0));
        assert_eq!(distances.get(&y), Some(&5));
        assert_eq!(distances.get(&x), Some(&9));
        assert_eq!(distances.get(&z), Some(&7));
    }
}