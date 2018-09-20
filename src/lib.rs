type Link<T> = Option<Box<Node<T>>>;

#[derive(PartialEq, Debug)]
struct Node<T> {
    value: T,
    next: Link<T>,
}

#[derive(PartialEq, Debug)]
struct LinkedList<T> {
    head: Link<T>,
}

impl<T> LinkedList<T> {
    fn new() -> Self {
        LinkedList { head: None }
    }

    fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.value)
    }

    fn push(&mut self, item: T) {
        self.head = Some(Box::new(Node {
            value: item,
            next: self.head.take(),
        }));
    }

    fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            let node = *node;
            self.head = node.next;
            node.value
        })
    }

    fn iter(&self) -> Iter<T> {
        Iter {
            node: self.head.as_ref().map(|b| &**b),
        }
    }
}

struct Iter<'a, T: 'a> {
    node: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.node.map(|node| {
            self.node = node.next.as_ref().map(|next| &**next);
            &node.value
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
        assert_eq!(list.peek(), None);
    }

    #[test]
    fn test_push() {
        let mut list = LinkedList::new();
        list.push(42);
        assert_eq!(list.peek(), Some(&42));
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
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn test_push_n_pop() {
        let mut list = LinkedList::new();
        list.push(42);
        assert_eq!(list.pop(), Some(42));
    }

    #[test]
    fn test_double_push_n_pop() {
        let mut list = LinkedList::new();
        list.push(42);
        list.push(100);
        assert_eq!(list.peek(), Some(&100));
        assert_eq!(list.pop(), Some(100));
        assert_eq!(list.pop(), Some(42));
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn test_iter() {
        let list: LinkedList<u32> = LinkedList::new();
        let mut iter = list.iter();
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_iter_with_something() {
        let mut list = LinkedList::new();
        list.push(42);
        list.push(100);
        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&100));
        assert_eq!(iter.next(), Some(&42));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_iter_filter() {
        let mut list = LinkedList::new();
        list.push(29);
        list.push(30);
        list.push(42);
        list.push(100);
        list.push(1);
        let mut iter = list.iter().filter(|&i| *i <= 30);
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&30));
        assert_eq!(iter.next(), Some(&29));
        assert_eq!(iter.next(), None);
    }

}
