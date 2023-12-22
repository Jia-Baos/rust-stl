fn merge(nums: &mut [i32], mid: usize) {
    let mut i = 0;       // 标记前半部分数据
    let mut k = mid;    // 标记后半部分数据
    let mut tmp = Vec::new();

    for _j in 0..nums.len() {
        if nums.len() == k || i == mid { break; }

        // 数据放到临时集合tmp
        if nums[i] < nums[k] {
            tmp.push(nums[i]);
            i += 1;
        } else {
            tmp.push(nums[k]);
            k += 1;
        }
    }

    // 合并的两部分数据长度大概率不一样，所以要将未处理完集合的数据全部加入
    if i < mid && k == nums.len() {
        for j in i..mid {
            tmp.push(nums[j]);
        }
    } else if i == mid && k < nums.len() {
        for j in k..nums.len() {
            tmp.push(nums[j]);
        }
    }

    // tmp数据放回nums，完成排序
    for j in 0..nums.len() {
        nums[j] = tmp[j];
    }
}

/// merge_sort
///
/// # Examples
///
/// ```
/// use crate::rust_stl::sort::merge_sort;
/// let mut nums = [1,2,8,3,4,9,5,6,7];
/// merge_sort::merge_sort(&mut nums);
/// let val = nums[nums.len()-1];
/// assert_eq!(val, 9);
///```
pub fn merge_sort(nums: &mut [i32]) {
    if nums.len() > 1 {
        let mid = nums.len() >> 1;
        merge_sort(&mut nums[..mid]);   // 排序前半部分
        merge_sort(&mut nums[mid..]);       // 排序后半部分
        merge(nums, mid);                        // 合并排序结果
    }
}