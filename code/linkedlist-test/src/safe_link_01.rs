// 关于如何写一个 链表：unsafe type

/**
 * [] in stack
 * () in heap
 * 
 * [ptr] -|-> (new node | ptr) -> (old node | ptr) -> ......
*/

// 使用Option后，可以用 take() 优化 mem::replace(dest, src)
// use std::mem;

// in stack
pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

// in heap
struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List {
            head: None,
        }
    }

    // 会改变这个结构体内部成员的值
    pub fn push(&mut self, elem: T) {
        let new_node = Box::new(Node {
            elem: elem,
            next: self.head.take(),
        });
        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        // match self.head.take() {
        //     None => None,
        //     Some(node) => {
        //         self.head = node.next;
        //         Some(node.elem)
        //     }
        // }
        // 优化
        self.head.take().map(|node|{
            self.head = node.next;
            node.elem
        })
    }

    // 返回表头元素的引用
    pub fn peek(&self) -> Option<&T> {
        // self.head.map(|node|{
        //     &node.elem
        // })
        // node本身是map里面闭包的，不能传出指针（空指针）
        self.head.as_ref().map(|node|{
            &node.elem
        })
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node|{
            &mut node.elem
        })
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        while let Some(mut node) = cur_link {
            cur_link = node.next.take();
        }
    }
}

pub struct IntoIter<T>(List<T>); // 元组结构体
// while list but not link?
// 因为 List 实体么？
// 迭代器本身就是一个 struct

impl<T> List<T> {
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self) // 用() 而不是 {}
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T; // 把trait留出来的位置都补上
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop() // 拿走了所有权
    }
}

// 主要逻辑就是加了一个 指针 然后 next 移动到结尾
// 难点：思考为什么要生命周期
pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>, // 要保证不拿走所有权
    // 只要一个结构体的成员指针，那就是要指定生命周期？
    // 生命周期主要是为了避免 悬指针
}

impl<T> List<T> {
    pub fn iter(& self) -> Iter<T> {
        Iter {
            next: self.head.as_deref()
        } // 这个就不用as_ref()么？
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.elem
        })
    }
}

// 实现的迭代器本身就是指针
impl<T> List<T> {
    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut {
            next: self.head.as_deref_mut()
        }
    }
}
pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    /* take() means takes the value, and leave a none */
    /* leave a none for self.next */ 
    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_deref_mut();
            &mut node.elem
        })
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        let mut list = List::new();

        // Check empty list behaves right
        assert_eq!(list.pop(), None);

        // Populate list
        list.push(1);
        list.push(2);
        list.push(3);

        // Check normal removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push(4);
        list.push(5);

        // Check normal removal
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn peek() {
        let mut list = List::new();
        assert_eq!(list.peek(), None);
        assert_eq!(list.peek_mut(), None);
        list.push(1); list.push(2); list.push(3);

        assert_eq!(list.peek(), Some(&3));
        assert_eq!(list.peek_mut(), Some(&mut 3));

        list.peek_mut().map(|value| {
            *value = 42
        });

        assert_eq!(list.peek(), Some(&42));
        assert_eq!(list.pop(), Some(42));
    }

    #[test]
    fn into_iter() {
        let mut list = List::new();
        list.push(1); list.push(2); list.push(3);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter() {
        let mut list = List::new();
        list.push(1); list.push(2); list.push(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
    }

    #[test]
    fn iter_mut() {
        let mut list = List::new();
        list.push(1); list.push(2); list.push(3);

        let mut iter = list.iter_mut();
        assert_eq!(iter.next(), Some(&mut 3));
        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), Some(&mut 1));

        assert_eq!(list.pop(), Some(3));
    }
}