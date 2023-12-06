use crate::stack::stack;

pub fn base_converter_2(mut dec_num: u32) -> String {
    // 用栈来保存余数 rem
    let mut rem_stack = stack::Stack::new();

    // 余数rem入栈
    while dec_num > 0 {
        let rem = dec_num % 2;
        rem_stack.push(rem);
        dec_num /= 2;
    }

    let mut bin_str = "".to_string();
    while !rem_stack.is_empty() {
        let rem = rem_stack.pop().unwrap().to_string();
        bin_str += &rem;
    }

    bin_str
}

pub fn base_converter(mut dec_num: u32, base: u32) -> String {
    // digits，对应各种余数的字符形式，尤其是10-15
    let digits = ['0', '1', '2', '3', '4', '5', '6', '7',
        '8', '9', 'A', 'B', 'C', 'D', 'E', 'F'];

    let mut rem_stack = stack::Stack::new();

    // 余数入栈
    while dec_num > 0 {
        let rem = dec_num % base;
        rem_stack.push(rem);
        dec_num /= base;
    }

    // 余数出栈并取对应字符来拼接成字符串
    let mut base_str = "".to_string();
    while !rem_stack.is_empty() {
        let rem = rem_stack.pop().unwrap() as usize;
        base_str += &digits[rem].to_string();
    }
    base_str
}

#[cfg(test)]
mod tests {
    use crate::stack::base_converter::base_converter_2;
    use crate::stack::base_converter::base_converter;

    #[test]
    fn test_base_converter_2() {
        let str = "111";
        assert_eq!(str, base_converter_2(7))
    }

    #[test]
    fn test_base_converter() {
        let str = "111";
        assert_eq!(str, base_converter(7, 2))
    }
}