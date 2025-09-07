impl<T> SinglyLinkedList<T> {
    /// 创建一个空的单链表
    fn new() -> Self {
        Self { head: None }
    }

    /// 在链表头部插入一个值
    fn push_front(&mut self, value: T) {
        let new_node = Box::new(Node {
            value,
            next: self.head.take(), // 旧 head 变成新节点的 next
        });
        self.head = Some(new_node); // 更新 head
    }

    /// 打印链表
    fn print(&self) {
        let mut current = self.head.as_ref();
        print!("LinkedList: ");
        while let Some(node) = current {
            print!("{} -> ", node.value);
            current = node.next.as_ref();
        }
        println!("None");
    }
}

