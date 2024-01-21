use std::collections::hash_map::DefaultHasher;
use std::marker::PhantomData;
use std::hash::{BuildHasher, Hash, Hasher};
use std::collections::hash_map::RandomState;

// 布隆过滤器
pub struct BloomFilter<T> {
    bits: Vec<bool>,
    // 比特桶
    hash_fn_count: usize,
    // 哈希函数个数
    hashers: [DefaultHasher; 2],
    // 两个哈希函数
    _phantom: PhantomData<T>,
    // T 占位，欺骗编译器
}

impl<T: Sized + Hash> BloomFilter<T> {
    pub fn new(cap: usize, ert: f64) -> Self {
        let ln22 = std::f64::consts::LN_2.powf(2f64);
        // 计算比特桶大小和哈希函数个数
        let bits_count = -1f64 * cap as f64 * ert.ln() / ln22;
        let hash_fn_count = -1f64 * ert.log2();

        // 随机哈希函数
        let hashers_tmp = [
            RandomState::new().build_hasher(),
            RandomState::new().build_hasher(),
        ];

        Self {
            bits: vec![false; bits_count.ceil() as usize],
            hash_fn_count: hash_fn_count.ceil() as usize,
            hashers: hashers_tmp,
            _phantom: PhantomData,
        }
    }

    // 按照 hash_fn_count 计算值并置比特桶相应位位true
    pub fn insert(&mut self, elem: &T) {
        let hashes = self.make_hash(elem);
        for fn_i in 0..self.hash_fn_count {
            let index = self.get_index(hashes, fn_i as u64);
            self.bits[index] = true;
        }
    }

    // 数据查询
    pub fn contains(&self, elem: &T) -> bool {
        let hasher = self.make_hash(elem);
        (0..self.hash_fn_count).all(|fn_i| {
            let index = self.get_index(hasher, fn_i as u64);
            self.bits[index]
        })
    }

    // 计算哈希值
    fn make_hash(&self, elem: &T) -> (u64, u64) {
        let hasher0 = &mut self.hashers[0].clone();
        let hasher1 = &mut self.hashers[1].clone();

        elem.hash(hasher0);
        elem.hash(hasher1);
        (hasher0.finish(), hasher1.finish())
    }

    // 获取比特桶某位下标
    fn get_index(&self, (h1, h2): (u64, u64), fn_i: u64) -> usize {
        let ih2 = fn_i.wrapping_mul(h2);
        let h1pih2 = h1.wrapping_add(ih2);
        (h1pih2 % self.bits.len() as u64) as usize
    }
}

#[cfg(test)]
mod test {
    use crate::demo::bloom_filter;

    #[test]
    fn test_bloom_filter() {
        let mut bf = bloom_filter::BloomFilter::new(100, 0.08);
        (0..20).for_each(|i| bf.insert(&i));
        let res1 = bf.contains(&2);
        let res2 = bf.contains(&50);
        assert_eq!(res1, true);
        assert_eq!(res2, false);
    }
}
