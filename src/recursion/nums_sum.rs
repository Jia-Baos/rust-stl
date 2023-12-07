pub fn nums_sum1(nums: Vec<i32>) -> i32 {
    let mut sum = 0;
    for num in nums {
        sum += num;
    }
    sum
}

pub fn nums_sum2(nums: &[i32]) -> i32 {
    if nums.len() == 1 {
        nums[0]
    } else {
        let first = nums[0];
        first + nums_sum2(&nums[1..])
    }
}

pub fn nums_sum3(nums: &[i32]) -> i32 {
    if nums.len() == 1 {
        nums[0]
    } else {
        let last = nums[nums.len() - 1];
        last + crate::recursion::nums_sum::nums_sum3(&nums[..nums.len() - 1])
    }
}

// 尾递归版本
pub fn nums_sum4(sum: i32, nums: &[i32]) -> i32 {
    if nums.len() == 1 {
        sum + nums[0]
    } else {
        // 使用sum来接收中间计算值
        nums_sum4(sum + nums[0], &nums[1..])
    }
}

pub fn nums_sum5(sum: i32, nums: &[i32]) -> i32 {
    if nums.len() == 1 {
        sum + nums[0]
    } else {
        nums_sum5(sum + nums[nums.len() - 1], &nums[..nums.len() - 1])
    }
}