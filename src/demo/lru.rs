// LRU（Least recently used，最近最少使用）算法
use std::collections::HashMap;
use std::hash::Hash;

const CACHE_SIZE: usize = 100;

// LRU 上的元素项
pub struct Entry<K, V> {
    m_key: K,
    m_val: Option<V>,
    m_next: Option<usize>,
    m_prev: Option<usize>,
}

// LRU 缓存
pub struct LRUCache<K, V> {
    m_cap: usize,
    m_head: Option<usize>,
    m_tail: Option<usize>,
    m_map: HashMap<K, usize>,
    m_entries: Vec<Entry<K, V>>,
}


impl<K: Clone + Hash + Eq, V> LRUCache<K, V> {
    pub fn new() -> Self {
        Self::with_capacity(CACHE_SIZE)
    }

    pub fn with_capacity(cap: usize) -> Self {
        LRUCache {
            m_cap: cap,
            m_head: None,
            m_tail: None,
            m_map: HashMap::with_capacity(cap),
            m_entries: Vec::with_capacity(cap),
        }
    }

    pub fn insert(&mut self, key: K, val: V) -> Option<V> {
        if self.m_map.contains_key(&key) {
            // 存在key就更新
            self.access(&key);
            let entry = &mut self.m_entries[self.m_head.unwrap()];
            let old_val = entry.m_val.take();
            entry.m_val = Some(val);
            old_val
        } else {
            // 不存在就插入
            self.ensure_room();

            // 更新原始头指针
            let index = self.m_entries.len();
            self.m_head.map(|e| {
                self.m_entries[e].m_prev = Some(index);
            });

            // 新的头节点
            self.m_entries.push(
                Entry {
                    m_key: key.clone(),
                    m_val: Some(val),
                    m_prev: None,
                    m_next: self.m_head,
                });

            self.m_head = Some(index);
            self.m_tail = self.m_tail.or(self.m_head);
            self.m_map.insert(key, index);

            None
        }
    }

    pub fn get(&mut self, key: &K) -> Option<&V> {
        if self.contains(key) { self.access(key); }

        let entries = &self.m_entries;
        self.m_map.get(key).and_then(move |&i| {
            entries[i].m_val.as_ref()
        })
    }

    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        if self.contains(key) { self.access(key); }

        let entries = &mut self.m_entries;
        self.m_map.get(key).and_then(move |&i| {
            entries[i].m_val.as_mut()
        })
    }

    pub fn contains(&mut self, key: &K) -> bool {
        self.m_map.contains_key(key)
    }

    // 确保容量足够，满了就移除末尾的元素
    fn ensure_room(&mut self) {
        if self.m_cap == self.len() {
            self.remove_tail();
        }
    }

    fn remove_tail(&mut self) {
        if let Some(index) = self.m_tail {
            self.remove_from_list(index);
            let key = &self.m_entries[index].m_key;
            self.m_map.remove(key);
        }

        if self.m_tail.is_none() {
            self.m_head = None;
        }
    }

    // 获取某个key的值，移除原来位置的值并在头部加入
    fn access(&mut self, key: &K) {
        let i = *self.m_map.get(key).unwrap();
        self.remove_from_list(i);
        self.m_head = Some(i);
    }

    fn remove(&mut self, key: &K) -> Option<V> {
        self.m_map.remove(&key).map(|index| {
            self.remove_from_list(index);
            self.m_entries[index].m_val.take().unwrap()
        })
    }

    fn remove_from_list(&mut self, i: usize) {
        let (prev, next) = {
            let entry = self.m_entries.get_mut(i).unwrap();
            (entry.m_prev, entry.m_next)
        };

        match (prev, next) {
            // 数据项在缓存中间
            (Some(j), Some(k)) => {
                let head = &mut self.m_entries[j];
                head.m_next = next;
                let next = &mut self.m_entries[k];
                next.m_prev = prev;
            }
            // 数据项在缓存末尾
            (Some(j), None) => {
                let head = &mut self.m_entries[j];
                head.m_next = None;
                self.m_tail = prev;
            }
            // 数据项在缓存头部
            _ => {
                if self.len() > 1 {
                    let head = &mut self.m_entries[0];
                    head.m_next = None;
                    let next = &mut self.m_entries[1];
                    next.m_prev = None;
                }
            }
        }
    }

    fn len(&self) -> usize { self.m_map.len() }

    fn is_empty(&self) -> bool { self.m_map.is_empty() }

    fn is_full(&self) -> bool { self.m_map.len() == self.m_cap }
}

#[cfg(test)]
mod tests {
    use crate::demo::lru;

    #[test]
    fn test_lru() {
        // let mut cache = lru::LRUCache::new();
        let mut cache = lru::LRUCache::with_capacity(2);
        cache.insert("foo", 1);
        cache.insert("bar", 2);

        cache.get(&"foo").unwrap();
        cache.insert("baz", 3);

        assert!(cache.contains(&"foo"));
        assert!(cache.contains(&"baz"));
        assert!(!cache.contains(&"bar"));

        cache.get_mut(&"foo").unwrap();
        cache.insert("qux", 4);

        assert!(cache.contains(&"foo"));
        assert!(cache.contains(&"qux"));
        assert!(!cache.contains(&"baz"));
    }
}