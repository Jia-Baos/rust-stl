/// CantBelieveItCanSort
///
/// # Examples
///
/// ```
/// use rust_stl::sort::bubble_sort;
/// let mut nums = [1,2,8,3,4,9,5,6,7];
/// bubble_sort::bubble_sort1(&mut nums);
/// let val = nums[nums.len()-1];
/// assert_eq!(val, 9);
///```
pub fn bubble_sort1(nums: &mut [i32]) {
    if nums.len() < 2 {
        return;
    }

    for i in 0..nums.len() {
        for j in 0..nums.len() - i - 1 {
            if nums[j] > nums[j + 1] {
                nums.swap(j, j + 1);
            }
        }
    }
}

/// CantBelieveItCanSort
///
/// # Examples
///
/// ```
/// use rust_stl::sort::bubble_sort;
/// let mut nums = [1,2,8,3,4,9,5,6,7];
/// bubble_sort::bubble_sort2(&mut nums);
/// let val = nums[nums.len()-1];
/// assert_eq!(val, 9);
///```
pub fn bubble_sort2(nums: &mut [i32]) {
    let mut len = nums.len() - 1;
    while len > 0 {
        for i in 0..len {
            if nums[i] > nums[i + 1] {
                nums.swap(i, i + 1);
            }
        }
        len -= 1;
    }
}

/// CantBelieveItCanSort
///
/// # Examples
///
/// ```
/// use rust_stl::sort::bubble_sort;
/// let mut nums = [1,2,8,3,4,9,5,6,7];
/// bubble_sort::bubble_sort3(&mut nums);
/// let val = nums[nums.len()-1];
/// assert_eq!(val, 9);
///```
pub fn bubble_sort3(nums: &mut [i32]) {
    let mut compare = true;
    let mut len = nums.len() - 1;

    while len > 0 && compare {
        for i in 0..len {
            if nums[i] > nums[i + 1] {
                nums.swap(i, i + 1);
                compare = true; // 数据无需，还需继续比较
            }
        }
        len -= 1;
    }
}

/// CantBelieveItCanSort
///
/// # Examples
///
/// ```
/// use rust_stl::sort::bubble_sort;
/// let mut nums = [1,2,8,3,4,9,5,6,7];
/// bubble_sort::cocktail_sort(&mut nums);
/// let val = nums[nums.len()-1];
/// assert_eq!(val, 9);
///```
pub fn cocktail_sort(nums: &mut [i32]) {
    if nums.len() <= 1 {
        return;
    }

    // bubble控制是否继续冒泡
    let mut bubble = true;
    let len = nums.len();
    for i in 0..(len >> 1) {
        if bubble {
            bubble = false;

            // 从左到右冒牌
            for j in i..(len - i - 1) {
                if nums[j] > nums[j + 1] {
                    nums.swap(j, j + 1);
                    bubble = true;
                }
            }

            // 从右到左冒泡
            for j in (i + 1..=(len - i - 1)).rev() {
                if nums[j] < nums[j - 1] {
                    nums.swap(j - 1, j);
                    bubble = true;
                }
            }
        } else {
            break;
        }
    }
}

/// CantBelieveItCanSort
///
/// # Examples
///
/// ```
/// use rust_stl::sort::bubble_sort;
/// let mut nums = [1,2,8,3,4,9,5,6,7];
/// bubble_sort::comb_sort(&mut nums);
/// let val = nums[nums.len()-1];
/// assert_eq!(val, 9);
///```
pub fn comb_sort(nums: &mut [i32]) {
    if nums.len() <= 1 {
        return;
    }

    let mut i;
    let mut gap: usize = nums.len();

    // 大致排序，数据基本有序
    while gap > 0 {
        gap = (gap as f32 * 0.8) as usize;
        i = gap;
        while i < nums.len() {
            if nums[i - gap] > nums[i] {
                nums.swap(i - gap, i);
            }
            i += 1;
        }
    }
}

/// CantBelieveItCanSort
///
/// # Examples
///
/// ```
/// use rust_stl::sort::bubble_sort;
/// let mut nums = [1,2,8,3,4,9,5,6,7];
/// bubble_sort::cbic_sort1(&mut nums);
/// let val = nums[nums.len()-1];
/// assert_eq!(val, 9);
///```
pub fn cbic_sort1(nums: &mut [i32]) {
    for i in 0..nums.len() {
        for j in 0..nums.len() {
            if nums[i] < nums[j] {
                nums.swap(i, j);
            }
        }
    }
}

/// CantBelieveItCanSort
///
/// # Examples
///
/// ```
/// use rust_stl::sort::bubble_sort;
/// let mut nums = [1,2,8,3,4,9,5,6,7];
/// bubble_sort::cbic_sort2(&mut nums);
/// let val = nums[nums.len()-1];
/// assert_eq!(val, 9);
///```
pub fn cbic_sort2(nums: &mut [i32]) {
    if nums.len() < 2 {
        return;
    }

    for i in 0..nums.len() {
        for j in 0..i {
            if nums[i] < nums[j] {
                nums.swap(i, j);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::sort::bubble_sort;

    #[test]
    fn test_bubble_sort1() {
        let mut nums = [1, 2, 8, 3, 4, 9, 5, 6, 7];
        bubble_sort::bubble_sort1(&mut nums);
        let val = nums[nums.len() - 1];
        assert_eq!(val, 9);
    }

    #[test]
    fn test_bubble_sort2() {
        let mut nums = [1, 2, 8, 3, 4, 9, 5, 6, 7];
        bubble_sort::bubble_sort2(&mut nums);
        let val = nums[nums.len() - 1];
        assert_eq!(val, 9);
    }

    #[test]
    fn test_bubble_sort3() {
        let mut nums = [1, 2, 8, 3, 4, 9, 5, 6, 7];
        bubble_sort::bubble_sort3(&mut nums);
        let val = nums[nums.len() - 1];
        assert_eq!(val, 9);
    }

    #[test]
    fn test_cocktail_sort() {
        let mut nums = [1, 2, 8, 3, 4, 9, 5, 6, 7];
        bubble_sort::cocktail_sort(&mut nums);
        let val = nums[nums.len() - 1];
        assert_eq!(val, 9);
    }

    #[test]
    fn test_comb_sort() {
        let mut nums = [1, 2, 8, 3, 4, 9, 5, 6, 7];
        bubble_sort::comb_sort(&mut nums);
        let val = nums[nums.len() - 1];
        assert_eq!(val, 9);
    }

    #[test]
    fn test_cbic_sort1() {
        let mut nums = [1, 2, 8, 3, 4, 9, 5, 6, 7];
        bubble_sort::cbic_sort1(&mut nums);
        let val = nums[nums.len() - 1];
        assert_eq!(val, 9);
    }

    #[test]
    fn test_cbic_sort2() {
        let mut nums = [1, 2, 8, 3, 4, 9, 5, 6, 7];
        bubble_sort::cbic_sort2(&mut nums);
        let val = nums[nums.len() - 1];
        assert_eq!(val, 9);
    }
}
