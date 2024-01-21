pub fn hamming_distance1(source: u64, target: u64) -> u32 {
    let mut count = 0;
    let mut xor = source ^ target;

    // 异或取值
    while xor != 0 {
        count += xor & 1;
        xor >>= 1;
    }

    count as u32
}

pub fn hamming_distance2(source: u64, target: u64) -> u32 {
    (source ^ target).count_ones()
}

pub fn hamming_distance_str(source: &str, target: &str) -> u32 {
    let mut count = 0;
    let mut source = source.chars();
    let mut target = target.chars();

    // 两字符逐字符比较可能出现按如下四种情况
    loop {
        match (source.next(), target.next()) {
            (Some(cs), Some(ct)) if cs != ct => count += 1,
            (Some(_), None) | (None, Some(_)) => panic!("Must have the same length..."),
            (None, None) => break,
            _ => continue,
        }
    }
    count as u32
}

#[cfg(test)]
mod tests {
    use crate::demo::hamming_distance;

    #[test]
    fn test_hamming_distance1() {
        let source = 1;
        let target = 2;
        let distance = hamming_distance::hamming_distance1(source, target);
        assert_eq!(distance, 2);
    }

    #[test]
    fn test_hamming_distance2() {
        let source = 1;
        let target = 2;
        let distance = hamming_distance::hamming_distance2(source, target);
        assert_eq!(distance, 2);
    }

    #[test]
    fn test_hamming_distance_str() {
        let source = "abce";
        let target = "edcf";
        let distance = hamming_distance::hamming_distance_str(source, target);
        println!("the hamming distance is {distance}");
    }
}