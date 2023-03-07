/**
 * 共享：把box换成Rc 同时不可变
 * 
 * [list1] -> A ----+
 *                  |
 * [list2]     -- > B -> C -> D
 *                  |
 * [list3] -> X ----+
 */
use std::rc::Rc;

pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Rc<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    // 在 head 处添加一个
    pub fn prepend(&self, elem: T) -> List<T> {
        List {
            head: Some(Rc::new(Node {
                elem: elem,
                next: self.head.clone(),
            }))
        }
    } // 这里的Rc::clone() 只增加计数

    // 把链表的首个元素移除，返回剩下的链表
    // 其实是建了一个新的 [list] -> 指向 第二个元素
    pub fn tail(&self) -> List<T> {
        List {
            head: self.head.as_ref().and_then(|node|{
                node.next.clone()
            })
        }
    }

    // 返回第一个元素的索引
    pub fn head(&self) -> Option<&T> {
        self.head.as_ref().map(|node|{
            &node.elem
        })
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        let list = List::new();
        assert_eq!(list.head(), None);

        let list = list.prepend(1).prepend(2).prepend(3);
        assert_eq!(list.head(), Some(&3));

        let list = list.tail();
        assert_eq!(list.head(), Some(&2));

        let list = list.tail();
        assert_eq!(list.head(), Some(&1));

        let list = list.tail();
        assert_eq!(list.head(), None);

        // Make sure empty tail works
        let list = list.tail();
        assert_eq!(list.head(), None);

    }
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<T> List<T> {
    pub fn iter(&self) -> Iter<'_, T> {
        Iter { next: self.head.as_deref() }
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

#[test]
fn iter() {
    let list = List::new().prepend(1).prepend(2).prepend(3);

    let mut iter = list.iter();
    assert_eq!(iter.next(), Some(&3));
    assert_eq!(iter.next(), Some(&2));
    assert_eq!(iter.next(), Some(&1));
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut head = self.head.take();
        while let Some(mut node) = head {
            if let Ok(mut node) = Rc::try_unwrap(node) {
                head = node.next.take(); // 此时索引为1
            } else {
                break;
            }
        }
        // 只需要把所有的索引都解开就好了
        // 不一定要回收资源
    }
}

// 线程安全：Arc<T: Send + Sync>

/* 问题：怎么实现 intoIter && iterMut */
/* 这个不是 mut 嘛？ */
/* 还是说一定要 RefCell？ */


