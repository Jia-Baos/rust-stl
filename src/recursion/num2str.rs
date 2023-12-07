use crate::stack::Stack;

const BASESTR: [&str; 16] = [
    "0", "1", "2", "3", "4", "5", "6", "7",
    "8", "9", "A", "B", "C", "D", "E", "F"];

pub fn num2str_rec(num: i32, base: i32) -> String {
    if num < base {
        BASESTR[num as usize].to_string()
    } else {
        // 余数加在末尾
        num2str_rec(num / base, base) + BASESTR[(num % base) as usize]
    }
}

pub fn num2str_stk(mut num: i32, base: i32) -> String {
    let digits: [&str; 16] = [
        "0", "1", "2", "3", "4", "5", "6", "7",
        "8", "9", "A", "B", "C", "D", "E", "F"];

    let mut rem_stack = Stack::new();
    while num > 0 {
        if num < base {
            // 不超过base直接入栈
            rem_stack.push(num);
        } else {
            // 超过base余数入栈
            rem_stack.push(num % base);
        }
        num /= base;
    }

    // 出栈余数并组成字符串
    let mut numstr = "".to_string();
    while !rem_stack.is_empty() {
        numstr += digits[rem_stack.pop().unwrap() as usize];
    }

    numstr
}