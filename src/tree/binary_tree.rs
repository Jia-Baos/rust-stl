use std::fmt::{Debug};

// 子节点链接
type Link<T> = Option<Box<BinaryTree<T>>>;

// 二叉树
// key 保存数据
// left 和 right 保存左右子节点链接
#[derive(Debug, Clone)]
pub struct BinaryTree<T> {
    m_key: T,
    m_left: Link<T>,
    m_right: Link<T>,
}

impl<T: Clone> BinaryTree<T> {
    pub fn new(key: T) -> Self {
        BinaryTree { m_key: key, m_left: None, m_right: None }
    }

    // 新子节点作为根节点的左子节点
    pub fn insert_left_tree(&mut self, key: T) {
        if self.m_left.is_none() {
            let node = BinaryTree::new(key);
            self.m_left = Some(Box::new(node));
        } else {
            let mut node = BinaryTree::new(key);
            node.m_left = self.m_left.take();
            self.m_left = Some(Box::new(node));
        }
    }

    // 新子节点作为根节点的右子节点
    pub fn insert_right_tree(&mut self, key: T) {
        if self.m_right.is_none() {
            let node = BinaryTree::new(key);
            self.m_right = Some(Box::new(node));
        } else {
            let mut node = BinaryTree::new(key);
            node.m_right = self.m_right.take();
            self.m_right = Some(Box::new(node));
        }
    }

    // 获取左右子节点及根节点，注意使用了clone
    pub fn get_left(&self) -> Link<T> {
        self.m_left.clone()
    }

    pub fn get_right(&self) -> Link<T> {
        self.m_right.clone()
    }

    pub fn get_key(&self) -> T {
        self.m_key.clone()
    }

    pub fn set_key(&mut self, key: T) {
        self.m_key = key;
    }
}

impl<T: Clone + Debug> BinaryTree<T> {
    // 前序遍历：内部实现
    pub fn preorder(&self) {
        println!("key is {:?}", &self.m_key);
        if !self.m_left.is_none() {
            self.m_left.as_ref().unwrap().preorder();
        }
        if !self.m_right.is_none() {
            self.m_right.as_ref().unwrap().preorder();
        }
    }

    // 中序遍历：内部实现
    pub fn inorder(&self) {
        if !self.m_left.is_none() {
            self.m_left.as_ref().unwrap().inorder();
        }
        println!("key is {:?}", &self.m_key);
        if !self.m_right.is_none() {
            self.m_right.as_ref().unwrap().inorder();
        }
    }

    // 后序遍历：内部实现
    pub fn postorder(&self) {
        if !self.m_left.is_none() {
            self.m_left.as_ref().unwrap().postorder();
        }
        if !self.m_right.is_none() {
            self.m_right.as_ref().unwrap().postorder();
        }
        println!("key is {:?}", &self.m_key);
    }
}

// 前序遍历：外部实现
pub fn preorder<T: Clone + Debug>(bt: Link<T>) {
    if !bt.is_none() {
        println!("key is {:?}", bt.as_ref().unwrap().get_key());
        preorder(bt.as_ref().unwrap().get_left());
        preorder(bt.as_ref().unwrap().get_right());
    }
}

// 中序遍历：外部实现
pub fn inorder<T: Clone + Debug>(bt: Link<T>) {
    if !bt.is_none() {
        inorder(bt.as_ref().unwrap().get_left());
        println!("key is {:?}", bt.as_ref().unwrap().get_key());
        inorder(bt.as_ref().unwrap().get_right());
    }
}

// 后序遍历：外部实现
pub fn postorder<T: Clone + Debug>(bt: Link<T>) {
    if !bt.is_none() {
        postorder(bt.as_ref().unwrap().get_left());
        postorder(bt.as_ref().unwrap().get_right());
        println!("key is {:?}", bt.as_ref().unwrap().get_key());
    }
}