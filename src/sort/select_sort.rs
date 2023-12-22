/// select_sort
///
/// # Examples
///
/// ```
/// use crate::rust_stl::sort::select_sort;
/// let mut nums = [1,2,8,3,4,9,5,6,7];
/// select_sort::select_sort(&mut nums);
/// let val = nums[nums.len()-1];
/// assert_eq!(val, 9);
///```
pub fn select_sort(nums: &mut [i32]) {
    let mut left = nums.len() - 1;  // 待排序数据下标

    while left > 0 {
        let mut pos_max = 0;
        for i in 1..=left {
            if nums[i] > nums[pos_max] {
                pos_max = i;    // 选择当前轮次最大值下标
            }
        }
        // 数据交换，完成一个数据的排序，待排序数据量减一
        nums.swap(left, pos_max);
        left -= 1;
    }
}