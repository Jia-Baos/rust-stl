/// insertion_sort
///
/// # Examples
///
/// ```
/// use crate::rust_stl::sort::insertion_sort;
/// let mut nums = [1,2,8,3,4,9,5,6,7];
/// insertion_sort::insertion_sort(&mut nums);
/// let val = nums[nums.len()-1];
/// assert_eq!(val, 9);
///```
pub fn insertion_sort(nums: &mut [i32]) {
    for i in 1..nums.len() {
        let mut pos = i;
        let curr = nums[i];

        while pos > 0 && curr < nums[pos - 1] {
            nums[pos] = nums[pos - 1];  // 向后移动数据
            pos -= 1;
        }
        nums[pos] = curr;   // 插入数据
    }
}

/// insertion_sort
///
/// # Examples
///
/// ```
/// use crate::rust_stl::sort::insertion_sort;
/// let mut nums = [1,2,8,3,4,9,5,6,7];
/// insertion_sort::bin_insertion_sort(&mut nums);
/// let val = nums[nums.len()-1];
/// assert_eq!(val, 9);
///```
pub fn bin_insertion_sort(nums: &mut [i32]) {
    let mut tmp;
    let mut left;
    let mut mid;
    let mut right;

    for i in 1..nums.len() {
        left = 0;           // 已排序数组左右边界
        right = i - 1;
        tmp = nums[i];      // 待排序数据

        // 二分法找到tmp的位置
        while left <= right {
            mid = (left + right) >> 1;
            if tmp < nums[mid] {
                // 防止出现 right = 0 - 1
                if mid == 0 { break; }
                right = mid - 1;
            } else { left = mid + 1; }
        }

        // 将数据后移，留出空位
        for j in (left..=i - 1).rev() {
            nums.swap(j, j + 1);
        }

        // 将tmp插入空位
        if left != i { nums[left] = tmp; }
    }
}