use std::fmt::{Debug, Display};

// 子节点链接
type Link<T> = Option<Box<BinaryTree<T>>>;

// 二叉树
// key 保存数据
// left 和 right 保存左右子节点链接
#[derive(Debug, Clone)]
struct BinaryTree<T> {
    key: T,
    left: Link<T>,
    right: Link<T>,
}