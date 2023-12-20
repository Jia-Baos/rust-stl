pub fn interpolation_search(nums: &[i32], target: i32) -> bool {
    if nums.is_empty() { return false; }

    let mut flag = false;
    let mut low = 0usize;
    let mut high = nums.len() - 1;

    while low <= high || target >= nums[low] || target <= nums[high] {
        if nums[low] == nums[high] && nums[low] != target {
            break;
        }

        // 计算插值位置
        let offset = (target - nums[low]) * (high - low) as i32 / (nums[high] - nums[low]);
        let interpolant = low + offset as usize;

        // 更新上下界high、low
        if nums[interpolant] == target {
            flag = true;
            break;
        } else if nums[interpolant] > target {
            high = interpolant - 1;
        } else {
            low = interpolant + 1;
        }
    }

    flag
}

#[cfg(test)]
mod tests {
    use crate::search::interpolation_search;

    #[test]
    fn test_interpolation_search() {
        let nums = [1, 3, 8, 10, 15, 32, 44, 48, 50, 55, 60, 62, 64];

        let num = 8;
        let found = interpolation_search::interpolation_search(&nums, num);
        assert_eq!(found, true);

        let num = 45;
        let found = interpolation_search::interpolation_search(&nums, num);
        assert_eq!(found, false);
    }
}