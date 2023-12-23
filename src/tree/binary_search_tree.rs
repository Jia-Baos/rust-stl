use std::cmp::Ordering;

// 二叉查找树子节点链接
type Link<T, U> = Option<Box<BST<T, U>>>;

// 二叉查找树定义
pub struct BST<T, U> {
    m_key: Option<T>,
    m_val: Option<U>,
    m_left: Link<T, U>,
    m_right: Link<T, U>,
}

impl<T, U> BST<T, U>
    where T: Clone + Ord + std::fmt::Debug,
          U: Clone + std::fmt::Debug {
    pub fn new() -> Self {
        BST { m_key: None, m_val: None, m_left: None, m_right: None }
    }

    pub fn is_empty(&self) -> bool { self.m_key.is_none() }

    pub fn len(&self) -> usize { self.calc_len(0) }

    fn calc_len(&self, mut i: usize) -> usize {
        if self.m_key.is_none() { return i; }

        // 当前节点加入总节点数i
        i += 1;

        // 计算左右子节点数
        if !self.m_left.is_none() {
            i = self.m_left.as_ref().unwrap().calc_len(i);
        }
        if !self.m_right.is_none() {
            i = self.m_right.as_ref().unwrap().calc_len(i);
        }
        i
    }

    // 前序遍历
    pub fn preorder(&self) {
        println!("key: {:#?}, val: {:#?}", &self.m_key, &self.m_val);
        match &self.m_left {
            Some(node) => node.preorder(),
            None => (),
        }
        match &self.m_right {
            Some(node) => node.preorder(),
            None => (),
        }
    }

    // 中序遍历
    pub fn inorder(&self) {
        match &self.m_left {
            Some(node) => node.inorder(),
            None => (),
        }
        println!("key: {:#?}, val: {:#?}", &self.m_key, &self.m_val);
        match &self.m_right {
            Some(node) => node.inorder(),
            None => (),
        }
    }

    // 后序遍历
    pub fn postorder(&self) {
        match &self.m_left {
            Some(node) => node.postorder(),
            None => (),
        }
        match &self.m_right {
            Some(node) => node.postorder(),
            None => (),
        }
        println!("key: {:#?}, val: {:#?}", &self.m_key, &self.m_val);
    }

    pub fn insert(&mut self, key: T, val: U) {
        // 没数据直接插入
        if self.m_key.is_none() {
            self.m_key = Some(key);
            self.m_val = Some(val);
        } else {
            match &self.m_key {
                Some(k) => {
                    // 存在key，则更新val
                    if key == *k {
                        self.m_val = Some(val);
                        return;
                    }

                    // 未找到key，需要插入新节点
                    // 先找到需要插入的子树
                    let child = if key < *k { &mut self.m_left } else { &mut self.m_right };

                    // 根据节点递归下去，直到插入
                    match child {
                        Some(ref mut node) => {
                            node.insert(key, val);
                        }
                        None => {
                            let mut node = BST::new();
                            node.insert(key, val);
                            *child = Some(Box::new(node));
                        }
                    }
                }
                None => (),
            }
        }
    }

    pub fn search(&self, key: &T) -> bool {
        match &self.m_key {
            Some(k) => {
                // 比较key值，并判断是否继续递归查找
                match k.cmp(&key) {
                    Ordering::Equal => { true }    // 找到数据
                    Ordering::Greater => {    // 在左子树查找
                        match &self.m_left {
                            Some(node) => node.search(key),
                            None => false,
                        }
                    }
                    Ordering::Less => {   // 在右子树查找
                        match &self.m_right {
                            Some(node) => node.search(key),
                            None => false,
                        }
                    }
                }
            }
            None => false,
        }
    }

    pub fn min(&self) -> (Option<&T>, Option<&U>) {
        // 最小值一定在最左侧
        match &self.m_left {
            Some(node) => node.min(),
            None => match &self.m_key {
                Some(key) => (Some(&key), self.m_val.as_ref()),
                None => (None, None),
            },
        }
    }

    pub fn max(&self) -> (Option<&T>, Option<&U>) {
        // 最大值一定在最右侧
        match &self.m_right {
            Some(node) => node.max(),
            None => match &self.m_key {
                Some(key) => (Some(&key), self.m_val.as_ref()),
                None => (None, None),
            },
        }
    }

    // 获取值，和查找流程相似
    pub fn get(&self, key: &T) -> Option<&U> {
        match &self.m_key {
            None => None,
            Some(k) => {
                match k.cmp(&key) {
                    Ordering::Equal => self.m_val.as_ref(),
                    Ordering::Greater => {
                        match &self.m_left {
                            None => None,
                            Some(node) => node.get(key),
                        }
                    }
                    Ordering::Less => {
                        match &self.m_right {
                            None => None,
                            Some(node) => node.get(key),
                        }
                    }
                }
            }
        }
    }
}