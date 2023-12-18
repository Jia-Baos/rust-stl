pub fn sequential_search(nums: &[i32], num: i32) -> bool {
    let mut pos = 0;
    let mut found = false;

    // found表示是否找到
    // pos 在索引范围内且未找到就继续循环
    while pos < nums.len() && !found {
        if num == nums[pos] {
            found = true
        } else {
            pos += 1;
        }
    }
    found
}

pub fn sequential_search_ordered(nums: &[i32], num: i32) -> bool {
    let mut pos = 0;
    let mut found = false;
    let mut stop = false;   // 控制遇到有序数据是退出

    // found表示是否找到
    // pos 在索引范围内且未找到就继续循环
    while pos < nums.len() && !found && !stop {
        if num == nums[pos] {
            found = true
        } else if num < nums[pos] {
            stop = true;    // 数据有序，退出
        } else {
            pos += 1;
        }
    }
    found
}

pub fn sequential_search_pos(nums: &[i32], num: i32) -> Option<usize> {
    let mut pos = 0;
    let mut found = false;

    // found表示是否找到
    // pos 在索引范围内且未找到就继续循环
    while pos < nums.len() && !found {
        if num == nums[pos] {
            found = true
        } else {
            pos += 1;
        }
    }

    if found { Some(pos) } else { None }
}

#[cfg(test)]
mod tests {
    use crate::search::sequential_search;

    #[test]
    fn test_sequential_search() {
        let num = 8;
        let nums = [9, 3, 7, 4, 1, 6, 2, 8, 5];
        let found = sequential_search::sequential_search(&nums, num);
        assert_eq!(found, true);
    }

    #[test]
    fn test_sequential_search_pos() {
        let num = 8;
        let nums = [9, 3, 7, 4, 1, 6, 2, 8, 5];
        let pos = sequential_search::sequential_search_pos(&nums, num).unwrap();
        assert_eq!(pos, 7);
    }

    #[test]
    fn test_sequential_search_ordered() {
        let num = 44;
        let nums = [1, 3, 8, 10, 15, 32, 44, 48, 50, 55, 60, 62, 64];
        let found = sequential_search::sequential_search_ordered(&nums, num);
        assert_eq!(found, true);
    }
}