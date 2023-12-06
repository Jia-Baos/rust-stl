use std::collections::HashMap;

use crate::stack::stack;
use crate::stack::par_checker;


pub fn infix_to_postfix(infix: &str) -> Option<String> {
    // 括号匹配检查
    if !par_checker::par_checker3(infix) {
        return None;
    }

    // 设置各个符号的优先级
    let mut prec = HashMap::new();
    prec.insert("(", 1);
    prec.insert(")", 1);
    prec.insert("+", 2);
    prec.insert("-", 2);
    prec.insert("*", 3);
    prec.insert("/", 3);

    // ops保存操作符号，postfix保存后缀表达式
    let mut op_stack = stack::Stack::new();
    let mut postfix = Vec::new();

    for token in infix.split_whitespace() {
        // 0-9和A-Z范围字符入栈
        if ("A" <= token && token <= "Z") ||
            ("0" <= token && token <= "9") {
            postfix.push(token);
        } else if "(" == token {
            // 遇到开操作符，将操作符入栈
            op_stack.push(token);
        } else if ")" == token {
            // 遇到闭操作符，将操作数入栈
            let mut top = op_stack.pop().unwrap();
            while top != "(" {
                postfix.push(top);
                top = op_stack.pop().unwrap();
            }
        } else {
            // 比较符号优先级来决定操作符是否加入postfix
            while (!op_stack.is_empty()) &&
                (prec[op_stack.peek().unwrap()] >= prec[token]) {
                postfix.push(op_stack.pop().unwrap());
            }
            op_stack.push(token);
        }
    }

    // 剩下的操作数入栈
    while !op_stack.is_empty() {
        postfix.push(op_stack.pop().unwrap());
    }

    // 出栈并组成字符串
    let mut postfix_str = "".to_string();
    for c in postfix {
        postfix_str += &c.to_string();
        postfix_str += " ";
    }

    Some(postfix_str)
}

#[cfg(test)]
mod tests {
    use crate::stack::infix_to_postfix::infix_to_postfix;

    #[test]
    fn test_infix_to_postfix() {
        let infix = "( A + B ) * ( C + D )";
        let postfix = "A B + C D + * ";
        assert_eq!(postfix, infix_to_postfix(infix).unwrap());
    }
}