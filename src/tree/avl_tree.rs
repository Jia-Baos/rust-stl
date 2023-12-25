// AVL树定义
#[derive(Debug)]
pub enum AvlTree<T> {
    Null,
    Tree(Box<AvlNode<T>>),
}

// Avl树节点定义
#[derive(Debug)]
pub struct AvlNode<T> {
    m_val: T,
    m_left: AvlTree<T>,
    m_right: AvlTree<T>,
    m_bfactor: i8,
}

use std::cmp::Ordering::*;
use std::cmp::max;
use std::mem::replace;
use AvlTree::*;

impl<T> AvlTree<T> where T: Ord {
    // 新树是空的
    pub fn new() -> AvlTree<T> { Null }

    pub fn insert(&mut self, val: T) -> (bool, bool) {
        let ret = match *self {
            // 没有节点，直接插入
            Null => {
                let node = AvlNode { m_val: val, m_left: Null, m_right: Null, m_bfactor: 0 };
                *self = Tree(Box::new(node));
                (true, true)
            }
            Tree(ref mut node) => match node.m_val.cmp(&val) {
                // 比较节点值，再判断该从哪边插入
                // inserted表示是否插入
                // deepened表示是否加深
                Equal => (false, false),   // 相等，无需插入
                Less => { // 比节点数据大，插入右边
                    let (inserted, deepened) = node.m_right.insert(val);
                    if deepened {
                        let ret = match node.m_bfactor {
                            -1 => (inserted, false),
                            0 => (inserted, true),
                            1 => (inserted, false),
                            _ => unreachable!(),
                        };
                        node.m_bfactor += 1;
                        ret
                    } else { (inserted, deepened) }
                }
                Greater => {  // 比节点数据小，插入左边
                    let (inserted, deepened) = node.m_left.insert(val);

                    if deepened {
                        let ret = match node.m_bfactor {
                            -1 => (inserted, false),
                            0 => (inserted, true),
                            1 => (inserted, false),
                            _ => unreachable!(),
                        };
                        node.m_bfactor -= 1;
                        ret
                    } else { (inserted, deepened) }
                }
            },
        };
        self.rebalance();
        ret
    }

    // 调整各节点的平衡因子
    fn rebalance(&mut self) {
        match *self {
            Null => (), // 没数据，不用调整
            Tree(_) => match self.node().m_bfactor {
                // 右子树重
                -2 => {
                    let lbf = self.node().m_left.node().m_bfactor;
                    if lbf == -1 || lbf == 0 {
                        let (a, b) = if lbf == -1 {
                            (0, 0)
                        } else {
                            (-1, 1)
                        };
                        self.rotate_right(); // 不平衡，旋转
                        self.node().m_right.node().m_bfactor = a;
                        self.node().m_bfactor = b;
                    } else if lbf == 1 {
                        let (a, b) = match self.node()
                            .m_left.node()
                            .m_right.node()
                            .m_bfactor {
                            -1 => (1, 0),
                            0 => (0, 0),
                            1 => (0, -1),
                            _ => unreachable!(),
                        };

                        // 先左旋再右旋
                        self.node().m_left.rotate_left();
                        self.rotate_right();
                        self.node().m_right.node().m_bfactor = a;
                        self.node().m_left.node().m_bfactor = b;
                        self.node().m_bfactor = 0;
                    } else {
                        unreachable!()
                    }
                }
                // 左子树重
                2 => {
                    let rbf = self.node().m_right.node().m_bfactor;
                    if rbf == 1 || rbf == 0 {
                        let (a, b) = if rbf == 1 {
                            (0, 0)
                        } else {
                            (1, -1)
                        };
                        self.rotate_left();
                        self.node().m_left.node().m_bfactor = a;
                        self.node().m_bfactor = b;
                    } else if rbf == -1 {
                        let (a, b) = match self.node()
                            .m_right.node()
                            .m_left.node()
                            .m_bfactor {
                            1 => (-1, 0),
                            0 => (0, 0),
                            -1 => (0, 1),
                            _ => unreachable!(),
                        };

                        // 先右旋再左旋
                        self.node().m_right.rotate_right();
                        self.rotate_left();
                        self.node().m_left.node().m_bfactor = a;
                        self.node().m_right.node().m_bfactor = b;
                        self.node().m_bfactor = 0;
                    } else {
                        unreachable!()
                    }
                }
                _ => (),
            },
        }
    }

    // 获取节点
    fn node(&mut self) -> &mut AvlNode<T> {
        match *self {
            Null => panic!("Empty tree"),
            Tree(ref mut n) => n,
        }
    }

    // 获取左右子树
    fn left_subtree(&mut self) -> &mut Self {
        match *self {
            Null => panic!("Empty tree"),
            Tree(ref mut node) => &mut node.m_left,
        }
    }

    fn right_subtree(&mut self) -> &mut Self {
        match *self {
            Null => panic!("Empty tree"),
            Tree(ref mut node) => &mut node.m_right,
        }
    }

    // 左右旋
    fn rotate_left(&mut self) {
        let mut v = replace(self, Null);
        let mut right = replace(v.right_subtree(), Null);
        let right_left = replace(right.left_subtree(), Null);
        *v.right_subtree() = right_left;
        *right.left_subtree() = v;
        *self = right;
    }

    fn rotate_right(&mut self) {
        let mut v = replace(self, Null);
        let mut left = replace(v.left_subtree(), Null);
        let left_right = replace(left.right_subtree(), Null);
        *v.left_subtree() = left_right;
        *left.right_subtree() = v;
        *self = left;
    }

    // 树节点数是左右子树节点数加根节点数，递归计算
    pub fn len(&self) -> usize {
        match *self {
            Null => 0,
            Tree(ref v) => 1 + v.m_left.len() + v.m_right.len(),
        }
    }

    // 树深度是左右子树深度最大值 + 1，递归计算
    pub fn depth(&self) -> usize {
        match *self {
            Null => 0,
            Tree(ref v) => max(v.m_left.depth(), v.m_right.depth()) + 1,
        }
    }

    pub fn is_empty(&self) -> bool {
        match *self {
            Null => true,
            _ => false,
        }
    }

    // 数据查找
    pub fn search(&self, val: &T) -> bool {
        match *self {
            Null => false,
            Tree(ref v) => {
                match v.m_val.cmp(val) {
                    Equal => { true }
                    Greater => {
                        match &v.m_left {
                            Null => false,
                            _ => v.m_left.search(val),
                        }
                    }
                    Less => {
                        match &v.m_right {
                            Null => false,
                            _ => v.m_right.search(val),
                        }
                    }
                }
            }
        }
    }
}