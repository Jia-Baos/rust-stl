#[derive(Debug)]
pub struct Stack<T> {
    top: usize,
    // 栈顶
    data: Vec<T>,    // 栈数据容器
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Self { top: 0, data: Vec::new() }
    }

    // 栈顶恰好的就是栈中元素的个数
    pub fn size(&self) -> usize { self.top }

    pub fn is_empty(&self) -> bool {
        self.top == 0
    }

    // Vec 的末尾作为 Stack 的顶端
    pub fn push(&mut self, val: T) {
        self.data.push(val);
        self.top += 1;
    }

    // 直接弹出 Vec 末尾的元素即可
    pub fn pop(&mut self) -> Option<T> {
        if self.top == 0 { return None; }
        self.top -= 1;
        self.data.pop()
    }

    // 数据不能被move，只能返回引用
    pub fn peek(&self) -> Option<&T> {
        if self.top == 0 { return None; }
        self.data.get(self.size() - 1)
    }
}




#[test]
fn test()
{
    let mut s = Stack::new();
    assert_eq!(s.is_empty(), true);
    s.push(1);
    s.push(2);
    s.push(3);
    assert_eq!(s.size(), 3);

    println!("top {:?}, size {}", s.peek().unwrap(), s.size());
    println!("pop {:?}, size {}", s.pop().unwrap(), s.size());
    println!("is_empty:{}, stack:{:?}", s.is_empty(), s);
}
