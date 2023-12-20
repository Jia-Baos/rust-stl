pub fn binary_search1(nums: &[i32], num: i32) -> bool {
    let mut low = 0;
    let mut high = nums.len() - 1;
    let mut found = false;

    // 注意是<=，不是<
    while low <= high && !found {
        let mid = (low + high) >> 1;

        // 若low+high可能移除，可转换为减法
        // let mid = low + ((high - low) >> 1);

        if num == nums[mid] {
            found = true;
        } else if num < nums[mid] {
            high = mid - 1;   // num<中间值，省去后半部数据
        } else {
            low = mid + 1;    // num>=中间值，省去前半部数据
        }
    }
    found
}

pub fn binary_search2(nums: &[i32], num: i32) -> bool {
    // 基本情况1：项不存在
    if nums.len() == 0 { return false; }

    let mid = nums.len() >> 1;

    // 基本情况2：项存在
    return if num == nums[mid] {
        true
    } else if num < nums[mid] {
        binary_search2(&nums[..mid], num)
    } else {
        binary_search2(&nums[mid + 1..], num)
    };
}

#[cfg(test)]
mod tests {
    use crate::search::binary_search;

    #[test]
    fn test_binary_search1() {
        let nums = [1, 3, 8, 10, 15, 32, 44, 48, 50, 55, 60, 62, 64];

        let num = 3;
        let found = binary_search::binary_search1(&nums, num);
        assert_eq!(found, true);

        let num = 45;
        let found = binary_search::binary_search1(&nums, num);
        assert_eq!(found, false);
    }

    #[test]
    fn test_binary_search2() {
        let nums = [1, 3, 8, 10, 15, 32, 44, 48, 50, 55, 60, 62, 64];

        let num = 3;
        let found = binary_search::binary_search2(&nums, num);
        assert_eq!(found, true);

        let num = 45;
        let found = binary_search::binary_search2(&nums, num);
        assert_eq!(found, false);
    }
}
