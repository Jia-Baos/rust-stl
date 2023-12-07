use crate::deque::deque;


pub fn pal_checker(pal: &str) -> bool {
    let mut d = deque::Deque::new(pal.len());
    for c in pal.chars() {
        let _r = d.add_rear(c);
    }

    let mut is_pal = true;
    while d.size() > 1 && is_pal {
        // 比较首尾字符，若不同则非回文
        let head = d.remove_front();
        let tail = d.remove_rear();
        if head != tail {
            is_pal = false;
        }
    }

    is_pal
}

#[cfg(test)]
mod tests {
    use crate::deque::pal_checker::pal_checker;

    #[test]
    fn test_pal_checker() {
        let pal = "rustsur";
        let is_pal = pal_checker(pal);
        assert_eq!(is_pal, true);
    }
}