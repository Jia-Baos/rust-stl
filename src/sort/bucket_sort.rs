use std::fmt::Debug;

// hash是一个函数，计算时传入
// values是数据容器，保存数据
struct Bucket<H, T> {
    m_hasher: H,
    m_value: Vec<T>,
}

impl<H, T> Bucket<H, T> {
    fn new(hasher: H, value: T) -> Bucket<H, T> {
        Bucket { m_hasher: hasher, m_value: vec![value] }
    }
}

/// bucket_sort
///
/// 桶排序，Debug特性是为了打印T
///
/// # Examples
///
/// ```
/// use crate::rust_stl::sort::bucket_sort;
/// let mut nums = [1,2,8,3,4,9,5,6,7];
/// bucket_sort::bucket_sort(&mut nums, |t| t/5);
/// let val = nums[nums.len()-1];
/// assert_eq!(val, 9);
///```

pub fn bucket_sort<H, T, F>(nums: &mut [T], hasher: F)
    where H: Ord, T: Ord + Clone + Debug, F: Fn(&T) -> H {
    let mut buckets: Vec<Bucket<H, T>> = Vec::new();

    for val in nums.iter() {
        let hasher = hasher(&val);

        // 对桶中数据二分搜索并排序
        match buckets.binary_search_by(|bct| bct.m_hasher.cmp(&hasher)) {
            Ok(idx) => buckets[idx].m_value.push(val.clone()),
            Err(idx) => buckets.insert(idx, Bucket::new(hasher, val.clone())),
        }
    }

    // 拆桶，将所有排序数据融入到一个Vec中
    let ret = buckets.into_iter().flat_map(|mut bucket| {
        bucket.m_value.sort();
        bucket.m_value
    }).collect::<Vec<T>>();

    nums.clone_from_slice(&ret);
}