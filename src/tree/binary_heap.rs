macro_rules! parent {   // 计算父节点下标
    ($child:ident) => {$child>>1};
}
macro_rules! left_child {   // 计算左子节点下标
    ($parent:ident) => {$parent<<1};
}
macro_rules! right_child {  // 计算右子节点下标
    ($parent:ident) => {($parent<<1)+1};
}
/// 小顶堆
#[derive(Debug, Clone)]
pub struct BinaryHeap {
    size: usize,
    // 数据量
    data: Vec<i32>,  // 数据容器
}

impl BinaryHeap {
    pub fn new() -> Self {  // vec首位置0，但不计数总数
        BinaryHeap { size: 0, data: vec![0] }
    }

    pub fn len(&self) -> usize { self.size }

    pub fn is_empty(&self) -> bool { self.size == 0 }

    // 获取堆中最小数据
    pub fn min(&self) -> Option<i32> {
        if self.size == 0 { None } else {
            // Some(self.data[1].clone())   泛型数据用clone
            Some(self.data[1])
        }
    }

    // 小数据上冒 c(child), p(parent)
    fn move_up(&mut self, mut c: usize) {
        loop {
            // 计算当前节点的父节点
            let p = parent!(c);
            if p <= 0 { break; }

            // 当前节点数据小于父节点数据，交换
            if self.data[c] < self.data[p] {
                self.data.swap(c, p);
            }
            c = p;  // 父节点成为当前节点
        }
    }

    // 末尾添加数据，调整堆
    pub fn push(&mut self, val: i32) {
        self.data.push(val);
        self.size += 1;
        self.move_up(self.size);
    }

    // 最小子节点位置
    fn min_child(&self, i: usize) -> usize {
        // 同时计算左右子节点的位置
        let (lc, rc) = (left_child!(i), right_child!(i));
        // 1. 如果右子节点位置超过size，表示只有左子节点，则左子节点就是最小子节点
        // 2. 否则，同时存在左右子节点，需要具体判断左右子节点数据大小，然后返回最小的子节点位置
        if rc > self.size { lc } else if self.data[lc] < self.data[rc] { lc } else { rc }
    }

    // 大数据下沉 l(left), r(right)
    fn move_down(&mut self, mut c: usize) {
        loop {
            // 计算当前节点的左子节点位置
            let lc = left_child!(c);
            if lc > self.size { break; }

            // 计算当前节点的最小子节点位置
            let mc = self.min_child(c);

            // 当前节点数据大于最小子节点数据，交换
            if self.data[c] > self.data[mc] {
                self.data.swap(c, mc);
            }
            c = mc; // 最小子节点成为当前节点
        }
    }

    pub fn pop(&mut self) -> Option<i32> {
        // 没数据，返回None
        if self.size == 0 { None } else if self.size == 1 {
            self.size -= 1;   // 一个数据，比较好处理
            self.data.pop()
        } else {    // 多个数据，先交换并弹出数据，再调整堆
            self.data.swap(1, self.size);
            let val = self.data.pop();
            self.size -= 1;
            self.move_down(1);
            val
        }
    }

    // 构建新堆
    pub fn build_new(&mut self, arr: &[i32]) {
        // 删除原始数据
        for _i in 0..self.size {
            let _rm = self.data.pop();
        }

        // 添加新数据
        for &val in arr {
            self.data.push(val);
        }
        self.size = arr.len();

        // 调整小顶堆
        let size = self.size;
        let mut p = parent!(size);
        while p > 0 {
            self.move_down(p);
            p -= 1;
        }
    }

    // 切片数据逐个加入堆
    pub fn build_add(&mut self, arr: &[i32]) {
        for &val in arr {
            self.push(val)
        }
    }
}
