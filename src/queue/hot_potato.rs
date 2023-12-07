use crate::queue::queue;


pub fn hot_potato(names: Vec<&str>, num: usize) -> &str {
    // 初始化队列、名字入队
    let mut q = queue::Queue::new(names.len());
    for name in names {
        let _rm = q.enqueue(name);
    }

    while q.size() > 1 {
        // 出入栈名字，相当于传递山芋
        for _i in 0..num {
            // 先出栈再入栈，完成一次传递
            let name = q.dequeue().unwrap();
            let _rm = q.enqueue(name);
        }

        // 出入栈达到num次，删除一人
        let _rm = q.dequeue();
    }
    q.dequeue().unwrap()
}

#[cfg(test)]
mod tests {
    use crate::queue::hot_potato::hot_potato;

    #[test]
    fn test_hot_potato() {
        let name = vec!["Shieber", "David", "Susan", "Jane", "Kew", "Brad"];
        let rem = hot_potato(name, 8);
        assert_eq!(rem, "Kew");
    }
}