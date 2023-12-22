fn partition(nums: &mut [i32], low: usize, high: usize) -> usize {
    let mut lm = low;   // 左标记
    let mut rm = high;  // 右标记
    loop {
        // 左标记不断右移
        while lm <= rm && nums[lm] <= nums[low] {
            lm += 1;
        }
        // 右标记不断左移
        while lm <= rm && nums[rm] >= nums[low] {
            rm -= 1;
        }

        // 左标记越过右标记时退出并交换左右标记数据
        if lm > rm { break; } else { nums.swap(lm, rm) };
    }
    nums.swap(low, rm);
    rm
}

/// quick_sort
///
/// # Examples
///
/// ```
/// use crate::rust_stl::sort::quick_sort;
/// let mut nums = [1,2,8,3,4,9,5,6,7];
/// let len = nums.len() - 1;
/// quick_sort::quick_sort(&mut nums, 0, len);
/// let val = nums[nums.len()-1];
/// assert_eq!(val, 9);
///```
pub fn quick_sort(nums: &mut [i32], low: usize, high: usize) {
    if low < high {
        let split = partition(nums, low, high);
        if split > 1 {
            // 防止越界（split<=1）和语法错误
            quick_sort(nums, low, split - 1);
        }
        quick_sort(nums, split + 1, high);
    }
}