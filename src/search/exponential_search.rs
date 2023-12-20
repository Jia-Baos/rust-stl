use crate::search::binary_search;

/// 指数查找，通过指数函数确定目标值所在区间
pub fn exponential_search(nums: &[i32], target: i32) -> bool {
    let size = nums.len();
    if size == 0 { return false; }

    // 逐步找到上界
    let mut high = 1usize;
    while high < size && nums[high] < target {
        high <<= 1;
    }

    // 上界的一半一定可以作为下界
    let low = high >> 1;

    // 使用前面实现的二分查找
    binary_search::binary_search1(&nums[low..size.min(high + 1)], target)
}

#[cfg(test)]
mod tests {
    use crate::search::exponential_search;

    #[test]
    fn test_exponential_search() {
        let nums = [1, 3, 8, 10, 15, 32, 44, 48, 50, 55, 60, 62, 64];

        let num = 8;
        let found = exponential_search::exponential_search(&nums, num);
        assert_eq!(found, true);

        let num = 45;
        let found = exponential_search::exponential_search(&nums, num);
        assert_eq!(found, false);
    }
}