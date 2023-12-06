use crate::stack::stack;    // 引用同级文件，crate::文件所在文件夹名::所引用文件名


pub fn par_checker1(par: &str) -> bool {
    let mut char_list = Vec::new();
    for c in par.chars() {
        char_list.push(c);
    }

    let mut index = 0;
    let mut balance = true; // 括号是否匹配的标识
    let mut stack = stack::Stack::new();   // 使用前面实现的栈
    while index < char_list.len() && balance {
        let c = char_list[index];

        if '(' == c {
            stack.push(c);
        } else {
            if stack.is_empty() {
                balance = false;    // 为空则不平衡
            } else {
                let _r = stack.pop();
            }
        }
        index += 1;
    }
    balance && stack.is_empty() // 平衡且栈为空，括号表达式才是匹配的
}


// 同时检测多种开闭符号是否匹配
fn par_match(open: char, close: char) -> bool {
    let opens = "([{";
    let closes = ")]}";
    opens.find(open) == closes.find(close)
}

pub fn par_checker2(par: &str) -> bool {
    let mut char_list = Vec::new();
    for c in par.chars() {
        char_list.push(c);
    }

    let mut index = 0;
    let mut balance = true;
    let mut stack = stack::Stack::new();
    while index < char_list.len() && balance {
        let c = char_list[index];

        // 同时判断三种开符号
        if '(' == c || '[' == c || '{' == c {
            stack.push(c);
        } else {
            if stack.is_empty() {
                balance = false;
            } else {
                let top = stack.pop().unwrap();
                if !par_match(top, c) {
                    balance = false;
                }
            }
        }
        index += 1;
    }
    balance && stack.is_empty()
}

pub fn par_checker3(par: &str) -> bool {
    let mut char_list = Vec::new();
    for c in par.chars() {
        char_list.push(c);
    }

    let mut index = 0;
    let mut balance = true;
    let mut stack = stack::Stack::new();

    while index < char_list.len() && balance {
        let c = char_list[index];

        // 开符号入栈
        if '(' == c || '[' == c || '{' == c {
            stack.push(c);
        }

        // 闭符号则判断是否平衡
        if ')' == c || ']' == c || '}' == c {
            if stack.is_empty() {
                balance = false;
            } else {
                let top = stack.pop().unwrap();
                if !par_match(top, c) {
                    balance = false;
                }
            }
        }
        index += 1;
    }

    balance && stack.is_empty()
}

#[cfg(test)]
mod tests {
    use crate::stack::par_checker::{par_checker1, par_checker2, par_checker3};

    #[test]
    fn test_par_checker1() {
        let sa = "()(())";
        let sb = "()((()";
        let res1 = par_checker1(sa);
        let res2 = par_checker1(sb);
        println!("sa balanced: {}", res1);
        println!("sb balanced: {}", res2);
    }

    #[test]
    fn test_par_checker2() {
        let sa = "(){[]}";
        let sb = "(){[}]";
        let res1 = par_checker2(sa);
        let res2 = par_checker2(sb);
        println!("sa balanced: {}", res1);
        println!("sb balanced: {}", res2);
    }

    #[test]
    fn test_par_checker3() {
        let sa = "(){123[123]}";
        let sb = "(){[}]";
        let res1 = par_checker3(sa);
        let res2 = par_checker3(sb);
        println!("sa balanced: {}", res1);
        println!("sb balanced: {}", res2);
    }
}

