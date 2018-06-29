#[derive(PartialEq, Debug)]
struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

#[derive(PartialEq, Debug)]
struct LinkedList<T> {
    head: Option<Node<T>>,
}

impl<T> LinkedList<T> {
    fn new() -> Self {
        LinkedList { head: Option::None }
    }

    fn peek(&self) -> Option<&T> {
        match self.head {
            Option::Some(ref node) => Option::Some(&node.value),
            _ => Option::None,
        }
    }

    fn push(&mut self, item: T) {
        self.head = Option::Some(Node {
            value: item,
            next: self.head.take().map(|node| Box::new(node)),
        });
    }

    fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            let value = node.value;
            self.head = node.next.map(|b| *b);
            value
        })
    }
}

#[cfg(test)]
mod tests {
    use super::LinkedList;

    #[test]
    fn test_new() {
        let list: LinkedList<u32> = LinkedList::new();
        let list2: LinkedList<u32> = LinkedList::new();
        assert_eq!(list, list2);
    }

    #[test]
    fn test_peek() {
        let list: LinkedList<u32> = LinkedList::new();
        assert_eq!(list.peek(), Option::None);
    }

    #[test]
    fn test_push() {
        let mut list = LinkedList::new();
        list.push(42);
        assert_eq!(list.peek(), Option::Some(&42));
    }

    #[test]
    fn test_push_twice() {
        let mut list = LinkedList::new();
        list.push(42);
        list.push(100);
        assert_eq!(list.head.unwrap().next.unwrap().value, 42);
    }

    #[test]
    fn test_pop_empty() {
        let mut list: LinkedList<u32> = LinkedList::new();
        assert_eq!(list.pop(), Option::None);
    }

    #[test]
    fn test_push_n_pop() {
        let mut list = LinkedList::new();
        list.push(42);
        assert_eq!(list.pop(), Option::Some(42));
    }

    #[test]
    fn test_double_push_n_pop() {
        let mut list = LinkedList::new();
        list.push(42);
        list.push(100);
        assert_eq!(list.peek(), Option::Some(&100));
        assert_eq!(list.pop(), Option::Some(100));
        assert_eq!(list.pop(), Option::Some(42));
        assert_eq!(list.pop(), Option::None);
    }

}
