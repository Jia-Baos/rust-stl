type Link<T> = Option<Box<Node<T>>>;

// 节点定义
struct Node<T> {
    val: T,
    // 数据
    next: Link<T>,   // 下一个节点
}

// 链表定义
pub struct List<T> {
    head: Link<T>,   // 头节点
}

impl<T> List<T> {
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn is_empty(&self) -> bool { self.head.is_none() }

    pub fn size(&self) -> usize {
        let mut size = 0;
        // 只访问、不移动或改变
        let mut curr = self.head.as_ref();
        while curr.is_some() {
            size += 1;
            curr = curr.unwrap().next.as_ref();
        }
        size
    }

    // 新节点总是加到头部
    pub fn push(&mut self, val: T) {
        let next = self.head.take();
        self.head = Some(Box::new(Node { val: val, next }));
    }

    // take 会取出数据留下空位
    pub fn pop(&mut self) -> Option<T> {
        let curr = self.head.take()?;
        self.head = curr.next;
        Some(curr.val)
    }

    // peek不改变值，只能是引用
    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.val)
    }

    // peek_mut 可改变值，是可变引用
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.val)
    }

    // 以下是实现的三种迭代功能
    // into_iter: 链表改变，成为迭代器
    // iter: 链表不变，只得到不可变迭代器
    // iter_mut: 链表不变，得到可变迭代器
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    pub fn iter(&self) -> Iter<T> {
        Iter { next: self.head.as_deref() }
    }

    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut { next: self.head.as_deref_mut() }
    }
}

// 实现三种迭代功能
pub struct IntoIter<T>(List<T>);

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop() // (List<T>) 元组的第 0 项
    }
}

pub struct Iter<'a, T: 'a> {
    next: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.val
        })
    }
}

pub struct IterMut<'a, T: 'a> {
    next: Option<&'a mut Node<T>>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_deref_mut();
            &mut node.val
        })
    }
}

// 为链表实现自定义 Drop
impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut link = self.head.take();
        while let Some(mut node) = link {
            link = node.next.take();
        }
    }
}

#[test]
fn basic_test() {
    let mut list = List::new();
    list.push(1);
    list.push(2);
    list.push(3);
    assert_eq!(list.pop(), Some(3));
    assert_eq!(list.peek(), Some(&2));
    assert_eq!(list.peek_mut(), Some(&mut 2));
    list.peek_mut().map(|val| {
        *val = 4;
    });
    assert_eq!(list.peek(), Some(&4));
    println!("basics test Ok!");
}

#[test]
fn into_iter_test() {
    let mut list = List::new();
    list.push(1);
    list.push(2);
    list.push(3);
    let mut iter = list.into_iter();
    assert_eq!(iter.next(), Some(3));
    assert_eq!(iter.next(), Some(2));
    assert_eq!(iter.next(), Some(1));
    assert_eq!(iter.next(), None);
    println!("into_iter test Ok!");
}

#[test]
fn iter_test() {
    let mut list = List::new();
    list.push(1);
    list.push(2);
    list.push(3);
    let mut iter = list.iter();
    assert_eq!(iter.next(), Some(&3));
    assert_eq!(iter.next(), Some(&2));
    assert_eq!(iter.next(), Some(&1));
    assert_eq!(iter.next(), None);
    println!("iter test Ok!");
}

#[test]
fn iter_mut_test() {
    let mut list = List::new();
    list.push(1);
    list.push(2);
    list.push(3);
    let mut iter = list.iter_mut();
    assert_eq!(iter.next(), Some(&mut 3));
    assert_eq!(iter.next(), Some(&mut 2));
    assert_eq!(iter.next(), Some(&mut 1));
    assert_eq!(iter.next(), None);
    println!("iter_mut test Ok!");
}