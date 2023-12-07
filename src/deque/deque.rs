#[derive(Debug)]
pub struct Deque<T> {
    cap: usize,
    // 容量
    data: Vec<T>,    // 数据容器
}

impl<T> Deque<T> {
    pub fn new(size: usize) -> Self {
        Self { cap: size, data: Vec::with_capacity(size) }
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool { Self::size(&self) == 0 }

    // Vec末尾为队首
    pub fn add_front(&mut self, val: T) -> Result<(), String> {
        if Self::size(&self) == self.cap {
            return Err("No space available".to_string());
        }
        self.data.push(val);
        Ok(())
    }

    // Vec首部为队尾
    pub fn add_rear(&mut self, val: T) -> Result<(), String> {
        if Self::size(self) == self.cap {
            return Err("No space available".to_string());
        }
        self.data.insert(0, val);
        Ok(())
    }

    // 从队首移除数据
    pub fn remove_front(&mut self) -> Option<T> {
        if Self::size(self) > 0 { self.data.pop() } else { None }
    }

    // 从队尾移除元素
    pub fn remove_rear(&mut self) -> Option<T> {
        if Self::size(self) > 0 { Some(self.data.remove(0)) } else { None }
    }

    // 返回队首元素
    pub fn elem_front(&self) -> Option<&T> {
        if Self::size(self) == 0 { return None; }
        self.data.get(self.size() - 1)
    }
    // 返回队尾元素
    pub fn elem_rear(&self) -> Option<&T> {
        if Self::size(self) == 0 { return None; }
        self.data.get(0)
    }
}

#[cfg(test)]
mod tests {
    use crate::deque::deque::Deque;

    #[test]
    fn test() {
        let mut d = Deque::new(4);
        assert_eq!(d.is_empty(), true);
        let _r1 = d.add_front(1);
        let _r2 = d.add_front(2);
        let _r3 = d.add_rear(3);
        let _r4 = d.add_rear(4);
        if let Err(error) = d.add_front(5) {
            println!("add_front error: {error}");
        }
        assert_eq!(d.size(), 4);
        assert_eq!(d.elem_front(), Some(2).as_ref());
        assert_eq!(d.elem_rear(), Some(4).as_ref());

        if let Some(data) = d.remove_rear() {
            println!("data: {data}");
        } else {
            println!("empty queue");
        }

        println!("size: {}, is_empty: {}", d.size(), d.is_empty());
        println!("content: {:?}", d);
    }
}
