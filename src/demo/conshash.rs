use std::fmt::Debug;
use std::clone::Clone;
use std::string::ToString;
use std::hash::{Hash, Hasher};
use std::collections::BTreeMap;
use std::collections::hash_map::DefaultHasher;

const DEFAULT_REPLICAS: usize = 10;

// 环上节点
#[derive(Clone, Debug)]
pub struct Node {
    pub m_host: &'static str,
    pub m_ip: &'static str,
    pub m_port: u16,
}

// 环
pub struct Ring<T: Clone + ToString + Debug> {
    m_replicas: usize,
    // 分区数
    m_ring: BTreeMap<u64, T>,     // 保存数据的环
}

// 哈希计算函数
pub fn hash_conshash<T: Hash>(val: &T) -> u64 {
    let mut hasher = DefaultHasher::new();
    val.hash(&mut hasher);
    hasher.finish()
}

// 为 Node 添加 to_string() 功能
impl ToString for Node {
    fn to_string(&self) -> String {
        format!("{}:{}", self.m_ip.to_string(), self.m_port.to_string())
    }
}

impl<T> Ring<T> where T: Clone + ToString + Debug {
    pub fn new() -> Self { Self::with_capacity(DEFAULT_REPLICAS) }

    pub fn with_capacity(replicas: usize) -> Self {
        Ring {
            m_replicas: replicas,
            m_ring: BTreeMap::new(),
        }
    }

    // 批量插入结点
    pub fn add_multi(&mut self, nodes: &[T]) {
        if !nodes.is_empty() {
            for node in nodes.iter() {
                self.add(node);
            }
        }
    }

    pub fn add(&mut self, node: &T) {
        for i in 0..self.m_replicas {
            let key = hash_conshash(&format!("{}{}", node.to_string(), i.to_string()));
            self.m_ring.insert(key, node.clone());
        }
    }

    // 批量删除结点
    pub fn remove_multi(&mut self, nodes: &[T]) {
        if !nodes.is_empty() {
            for node in nodes.iter() { self.remove(node); }
        }
    }

    pub fn remove(&mut self, node: &T) {
        assert!(!self.m_ring.is_empty());
        for i in 0..self.m_replicas {
            let key = hash_conshash(&format!("{}{}", node.to_string(), i.to_string()));
            self.m_ring.remove(&key);
        }
    }

    // 查询结点
    pub fn get(&self, key: u64) -> Option<&T> {
        if self.m_ring.is_empty() { return None; }
        let mut keys = self.m_ring.keys();
        keys.find(|&k| k >= &key)
            .and_then(|k| self.m_ring.get(k))
            .or(keys.nth(0).and_then(|x| self.m_ring.get(x)))
    }
}

#[cfg(test)]
mod tests {
    use crate::demo::conshash;

    #[test]
    fn test_conshash() {
        let replica = 3;
        let mut ring = conshash::Ring::with_capacity(replica);

        let node = conshash::Node {
            m_host: "localhost",
            m_ip: "127.0.0.1",
            m_port: 23,
        };
        ring.add(&node);

        for i in 0..replica {
            let key = conshash::hash_conshash(&format!("{}{}", node.to_string(), i.to_string()));
            let res = ring.get(key);
            assert_eq!(node.m_host, res.unwrap().m_host);
        }

        ring.remove(&node);
    }
}