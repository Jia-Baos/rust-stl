pub fn num_coins_rec1(cashes: &[u32], amount: u32) -> u32 {
    // 全用1元纸币时的最少找零纸币数
    let mut min_cashes = amount;

    if cashes.contains(&amount) {
        return 1;
    } else {
        for c in cashes.iter()
            .filter(|&c| *c <= amount)
            .collect::<Vec<&u32>>() {
            // amount减去c，表示使用了一张面额为c的纸币，所以要加1
            let num_cashes = 1 + num_coins_rec1(&cashes, amount - c);

            // num_cashes若比min_cashes小则更新
            if num_cashes < min_cashes {
                min_cashes = num_cashes;
            }
        }
    }
    min_cashes
}

pub fn num_coins_rec2(cashes: &[u32], amount: u32, min_cashes: &mut [u32]) -> u32 {
    // 全用1元纸币的最小找零纸币数量
    let mut min_cashes_num = amount;

    if cashes.contains(&amount) {
        // 收集和当前待找零值相同的币种
        min_cashes[amount as usize] = 1;
        return 1;
    } else if min_cashes[amount as usize] > 0 {
        // 找零值amount有最小找零纸币数，直接返回
        return min_cashes[amount as usize];
    } else {
        for c in cashes.iter()
            .filter(|c| *(*c) <= amount)
            .collect::<Vec<&u32>>()
        {
            let cashes_num = 1 + num_coins_rec2(cashes, amount - c, min_cashes);

            // 更新最小找零纸币数
            if cashes_num < min_cashes_num {
                min_cashes_num = cashes_num;
                min_cashes[amount as usize] = min_cashes_num;
            }
        }
    }
    min_cashes_num
}

pub fn num_coins_dp(cashes: &[u32], amount: u32, min_cashes: &mut [u32]) -> u32 {
    // 动态收集从1到amount的最小找零币值数量，然后从小到大凑出找零纸币数量
    for denm in 1..=amount {
        let mut min_cashes_num = denm;
        for c in cashes.iter()
            .filter(|&c| *c <= denm)
            .collect::<Vec<&u32>>() {
            let index = (denm - c) as usize;
            let cashes_num = 1 + min_cashes[index];
            if cashes_num < min_cashes_num {
                min_cashes_num = cashes_num;
            }
        }
        min_cashes[denm as usize] = min_cashes_num;
    }

    // 因为收集了各个值的最小找零纸币数，所以直接返回
    min_cashes[amount as usize]
}

pub fn num_coins_dp_show(cashes: &[u32], amount: u32,
                         min_cashes: &mut [u32], cashes_used: &mut [u32]) -> u32 {
    // 动态收集从1到amount的最小找零币值数量，然后从小到大凑出找零纸币数量
    for denm in 1..=amount {
        let mut min_cashes_num = denm;
        let mut used_cashes = 1;    // 最小面额是1元
        for c in cashes.iter()
            .filter(|&c| *c <= denm)
            .collect::<Vec<&u32>>() {
            let index = (denm - c) as usize;
            let cashes_num = 1 + min_cashes[index];
            if cashes_num < min_cashes_num {
                min_cashes_num = cashes_num;
                used_cashes = *c;
            }
        }
        min_cashes[denm as usize] = min_cashes_num;
        cashes_used[denm as usize] = used_cashes;
    }

    // 因为收集了各个值的最小找零纸币数，所以直接返回
    min_cashes[amount as usize]
}

pub fn print_cashes(cashes_used: &[u32], mut amount: u32) {
    while amount > 0 {
        let curr = cashes_used[amount as usize];
        println!("${curr}");
        amount -= curr;
    }
}

#[cfg(test)]
mod tests {
    use crate::dp::num_coins;

    #[test]
    fn test_num_coins_rec1() {
        // cashes 保存各种面额的纸币
        let cashes = [1, 5, 10, 20, 50];
        let amount = 31u32;
        let cashes_num = num_coins::num_coins_rec1(&cashes, amount);
        assert_eq!(cashes_num, 3u32);
    }

    #[test]
    fn test_num_coins_rec2() {
        let amount = 81u32;
        let cashes = [1, 5, 10, 20, 50];
        let mut min_cashes: [u32; 82] = [0; 82];
        let cashes_num = num_coins::num_coins_rec2(&cashes, amount, &mut min_cashes);
        assert_eq!(cashes_num, 4u32);
    }

    #[test]
    fn test_num_coins_dp() {
        let amount = 81u32;
        let cashes = [1, 5, 10, 20, 50];
        let mut min_cashes: [u32; 82] = [0; 82];
        let cashes_num = num_coins::num_coins_dp(&cashes, amount, &mut min_cashes);
        assert_eq!(cashes_num, 4u32);
    }

    #[test]
    fn test_num_coins_dp_show() {
        let amount = 81u32;
        let cashes = [1, 5, 10, 20, 50];
        let mut min_cashes: [u32; 82] = [0; 82];
        let mut cashes_used: [u32; 82] = [0; 82];
        let cashes_num = num_coins::num_coins_dp_show(&cashes, amount, &mut min_cashes, &mut cashes_used);
        assert_eq!(cashes_num, 4u32);
    }
}
