use crate::stack::stack;    // 引用同级文件，crate::文件所在文件夹名::所引用文件名


pub fn par_checker(par: &str) -> bool {
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

#[test]
fn test() {
    let sa = "()(())";
    let sb = "()((()";
    let res1 = par_checker(sa);
    let res2 = par_checker(sb);
    println!("sa balanced: {}", res1);
    println!("sb balanced: {}", res2);
}
