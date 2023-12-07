// 节点连接用Box指针（大小确定），因为确定大小才能分配内存
type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug, Clone)]
pub struct Node<T> {
    data: T,
    next: Link<T>,
}

impl<T> Node<T> {
    pub fn new(data: T) -> Self {
        // 初始化时无下一链接
        Self { data, next: None }
    }
}


// 链表栈
#[derive(Debug, Clone)]
pub struct Stack<T> {
    size: usize,
    top: Link<T>,    // 栈顶控制整个栈
}

impl<T: Clone> Stack<T> {
    pub fn new() -> Self {
        Stack { size: 0, top: None }
    }

    pub fn size(&self) -> usize { self.size }

    pub fn is_empty(&self) -> bool { self.size == 0 }

    // take 取出top 中节点， 留下空位， 所以可以回填节点
    pub fn push(&mut self, val: T) {
        let mut node = Node::new(val);
        node.next = self.top.take();
        self.top = Some(Box::new(node));
        self.size += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        self.top.take().map(|node| {
            let node = *node;
            self.top = node.next;
            self.size -= 1;
            node.data
        })
    }

    // as_ref 将top 节点转为引用对象
    pub fn peek(&self) -> Option<&T> {
        self.top.as_ref().map(|node| {
            &node.data
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::linked_list::list_stack::Stack;

    #[test]
    fn test_list_stack() {
        let mut s = Stack::new();
        s.push(1);
        s.push(2);
        s.push(4);
        assert_eq!(s.peek(), Some(&4));
        assert_eq!(s.pop(), Some(4));
        assert_eq!(s.size(), 2);
        assert_eq!(s.is_empty(), false);
    }
}