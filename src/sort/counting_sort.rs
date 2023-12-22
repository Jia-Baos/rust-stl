/// counting_sort
///
/// # Examples
///
/// ```
/// use crate::rust_stl::sort::counting_sort;
/// let mut nums = [1,2,8,3,4,9,5,6,7];
/// counting_sort::counting_sort(&mut nums);
/// let val = nums[nums.len()-1];
/// assert_eq!(val, 9);
///```
pub fn counting_sort(nums: &mut [usize]) {
    if nums.len() <= 1 { return; }

    // 桶数量为nums中最大值加一，保证数据都有桶放
    let max_bkt_num = nums.iter().max().unwrap() + 1;
    let mut counter = vec![0; max_bkt_num];
    for &v in nums.iter() {
        counter[v] += 1;    // 将数据标记到桶
    }

    // 数据写回原nums切片
    let mut j = 0;
    for i in 0..max_bkt_num {
        while counter[i] > 0 {
            nums[j] = i;
            counter[i] -= 1;
            j += 1;
        }
    }
}