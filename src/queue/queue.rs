#[derive(Debug)]
pub struct Queue<T> {
    cap: usize,
    // 容量
    data: Vec<T>,   // 数据容器
}

impl<T> Queue<T> {
    pub fn new(size: usize) -> Self {
        Self { cap: size, data: Vec::with_capacity(size) }
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        Self::size(&self) == 0
    }

    // 判断是否有剩余空间，有则数据加入队列
    pub fn enqueue(&mut self, val: T) -> Result<(), String> {
        if Self::size(&self) == self.cap {
            return Err("No space available".to_string());
        }
        // Vec 的末尾作为 Queue 的头部
        self.data.insert(0, val);
        Ok(())
    }

    // 数据出队
    pub fn dequeue(&mut self) -> Option<T> {
        if Self::size(&self) > 0 {
            self.data.pop()
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::queue::queue::Queue;

    #[test]
    fn test() {
        let mut q = Queue::new(3);
        assert_eq!(q.is_empty(), true);
        let _r1 = q.enqueue(1);
        let _r2 = q.enqueue(2);
        let _r3 = q.enqueue(3);
        if let Err(error) = q.enqueue(4) {
            println!("Enqueue error: {error}");
        }
        assert_eq!(q.size(), 3);

        if let Some(data) = q.dequeue() {
            println!("data: {data}");
        } else {
            println!("empty queue");
        }

        println!("size: {}, empty: {}", q.size(), q.is_empty());
        println!("content: {:?}", q);
    }
}
