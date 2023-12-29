use std::rc::Rc;
use std::cell::RefCell;

// 因为节点存在多个共享的链接，Box不可共享，Rc才可共享
// 又因为Rc不可变，所以使用具有内部可变性的RefCell包裹
type Link = Option<Rc<RefCell<Node>>>;

// 节点
pub struct Node {
    m_data: usize,
    m_next: Link,
}

impl Node {
    pub fn new(data: usize) -> Self { Self { m_data: data, m_next: None } }
}

// 图定义及实现
pub struct Graph {
    m_first: Link,
    m_last: Link,
}

impl Graph {
    pub fn new() -> Self { Self { m_first: None, m_last: None } }

    pub fn is_empty(&self) -> bool { self.m_first.is_none() }

    pub fn get_first(&self) -> Link { self.m_first.clone() }

    // 打印节点
    pub fn print_node(&self) {
        let mut curr = self.m_first.clone();
        while let Some(val) = curr {
            print!("[{}]", &val.borrow().m_data);
            curr = val.borrow().m_next.clone();
        }
        println!();
    }

    // 插入节点，RefCell使用borrow_mut修改
    pub fn insert(&mut self, data: usize) {
        let node = Rc::new(RefCell::new(Node::new(data)));
        if self.is_empty() {
            self.m_first = Some(node.clone());
            self.m_last = Some(node);
        } else {
            self.m_last.as_mut().unwrap().borrow_mut().m_next = Some(node.clone());
            self.m_last = Some(node);
        }
    }
}

// 根据data构建图
pub fn create_graph(data: [[usize; 2]; 20]) -> Vec<(Graph, usize)> {
    let mut arr: Vec<(Graph, usize)> = Vec::new();
    for _ in 0..9 { arr.push((Graph::new(), 0)) };

    for i in 1..9 {
        for j in 0..data.len() {
            if data[j][0] == i {
                arr[i].0.insert(data[j][1]);
            }
        }
        print!("[{i}]->");
        arr[i].0.print_node();
    }
    arr
}

pub fn bfs(graph: Vec<(Graph, usize)>) {
    let mut gp = graph;
    let mut nodes = Vec::new();

    gp[1].1 = 1;
    let mut curr = gp[1].0.get_first().clone();

    // 打印图
    print!("{}->", 1);
    while let Some(val) = curr {
        nodes.push(val.borrow().m_data);
        curr = val.borrow().m_next.clone();
    }

    // 打印广度优先图
    loop {
        if nodes.len() == 0 { break; } else {
            let data = nodes.remove(0);
            if gp[data].1 == 0 {
                gp[data].1 = 1;
                print!("{data}->");
                let mut curr = gp[data].0.get_first().clone();
                while let Some(val) = curr {
                    nodes.push(val.borrow().m_data);
                    curr = val.borrow().m_next.clone();
                }
            }
        }
    }
    println!();
}

pub fn dfs(graph: Vec<(Graph, usize)>) {
    let mut gp = graph;
    let mut nodes: Vec<usize> = Vec::new();
    let mut tmp: Vec<usize> = Vec::new();

    gp[1].1 = 1;
    let mut curr = gp[1].0.get_first().clone();

    // 打印图
    print!("{}->", 1);
    while let Some(val) = curr {
        nodes.insert(0, val.borrow().m_data);
        curr = val.borrow().m_next.clone();
    }

    // 打印深度优先图
    loop {
        if nodes.len() == 0 { break; } else {
            let data = nodes.pop().unwrap();
            if gp[data].1 == 0 {
                gp[data].1 = 1;
                print!("{data}->");

                // 节点加入tmp
                let mut curr = gp[data].0.get_first().clone();
                while let Some(val) = curr {
                    tmp.push(val.borrow().m_data);
                    curr = val.borrow().m_next.clone();
                }

                while !tmp.is_empty() {
                    nodes.push(tmp.pop().unwrap());
                }
            }
        }
    }
    println!();
}

pub fn test_bfs() {
    let data = [[1, 2], [2, 1], [1, 3], [3, 1], [2, 4], [4, 2], [2, 5],
        [5, 2], [3, 6], [6, 3], [3, 7], [7, 3], [4, 5], [5, 4],
        [6, 7], [7, 6], [5, 8], [8, 5], [6, 8], [8, 6]];
    let gp = create_graph(data);
    bfs(gp);
}

pub fn test_dfs() {
    let data = [[1, 2], [2, 1], [1, 3], [3, 1], [2, 4], [4, 2], [2, 5],
        [5, 2], [3, 6], [6, 3], [3, 7], [7, 3], [4, 5], [5, 4],
        [6, 7], [7, 6], [5, 8], [8, 5], [6, 8], [8, 6]];
    let gp = create_graph(data);
    bfs(gp);
}