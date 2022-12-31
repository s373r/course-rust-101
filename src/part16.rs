// Rust-101, Part 16: Unsafe Rust, Drop
// ====================================

use std::marker::PhantomData;
use std::mem;
use std::ptr;

// A node of the list consists of the data, and two node pointers for the predecessor and successor.
struct Node<T> {
    next: NodePtr<T>,
    prev: NodePtr<T>,
    data: T,
}
// A node pointer is a *mutable raw pointer* to a node.
type NodePtr<T> = *mut Node<T>;

// The linked list itself stores pointers to the first and the last node. In addition, we tell Rust
// that this type will own data of type `T`.
pub struct LinkedList<T> {
    first: NodePtr<T>,
    last: NodePtr<T>,
    _marker: PhantomData<T>,
}

unsafe fn raw_into_box<T>(r: *mut T) -> Box<T> {
    mem::transmute(r)
}
fn box_into_raw<T>(b: Box<T>) -> *mut T {
    unsafe { mem::transmute(b) }
}

impl<T: Copy> LinkedList<T> {
    // A new linked list just contains null pointers. `PhantomData` is how we construct any
    // `PhantomData<T>`.
    pub fn new() -> Self {
        LinkedList {
            first: ptr::null_mut(),
            last: ptr::null_mut(),
            _marker: PhantomData,
        }
    }

    pub fn from_vec(vec: Vec<T>) -> Self {
        let mut list = LinkedList::new();

        for item in vec {
            list.push_back(item);
        }

        list
    }

    pub fn get_values(&mut self) -> Vec<T> {
        self.iter_mut().map(|x| *x).collect()
    }

    // This function adds a new node to the end of the list.
    pub fn push_back(&mut self, t: T) {
        // Create the new node, and make it a raw pointer.
        let new = Box::new(Node {
            data: t,
            next: ptr::null_mut(),
            prev: self.last,
        });
        let new = box_into_raw(new);
        // Update other pointers to this node.
        if self.last.is_null() {
            debug_assert!(self.first.is_null());
            // The list is currently empty, so we have to update the head pointer.
            self.first = new;
        } else {
            debug_assert!(!self.first.is_null());
            // We have to update the `next` pointer of the tail node.
            unsafe {
                (*self.last).next = new;
            }
        }
        // Make this the last node.
        self.last = new;
    }

    pub fn pop_back(&mut self) -> Option<T> {
        if self.last.is_null() {
            debug_assert!(self.first.is_null());

            return None;
        }

        debug_assert!(!self.first.is_null());

        let last = unsafe { raw_into_box(self.last) };

        if !last.prev.is_null() {
            unsafe {
                (*last.prev).next = ptr::null_mut();
            }

            self.last = last.prev;
        } else {
            self.first = ptr::null_mut();
            self.last = ptr::null_mut();
        }

        let value = last.data;

        drop(last);

        Some(value)
    }

    pub fn push_front(&mut self, value: T) {
        let new_node = if self.first.is_null() {
            debug_assert!(self.last.is_null());

            Box::new(Node {
                data: value,
                next: ptr::null_mut(),
                prev: ptr::null_mut(),
            })
        } else {
            debug_assert!(!self.last.is_null());

            Box::new(Node {
                data: value,
                next: self.first,
                prev: ptr::null_mut(),
            })
        };

        self.first = box_into_raw(new_node)
    }

    pub fn pop_front(&mut self) -> Option<T> {
        if self.first.is_null() {
            debug_assert!(self.last.is_null());

            return None;
        }

        debug_assert!(!self.last.is_null());

        let first = unsafe { raw_into_box(self.first) };

        if !first.next.is_null() {
            unsafe {
                (*first.next).prev = ptr::null_mut();
            }

            self.first = first.next;
        } else {
            self.first = ptr::null_mut();
            self.last = ptr::null_mut();
        }

        let value = first.data;

        drop(first);

        Some(value)
    }

    // **Exercise 16.1**: Add some more operations to `LinkedList`: `pop_back`, `push_front` and
    // `pop_front`. Add testcases for `push_back` and all of your functions. The `pop` functions
    // should take `&mut self` and return `Option<T>`.
    // NOTE(DP): Done

    // Next, we are going to provide an iterator.
    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut {
            next: self.first,
            _marker: PhantomData,
        }
    }
}

#[cfg(test)]
mod tests {
    use part16::LinkedList;

    #[test]
    fn test_linked_list_push_back() {
        {
            let mut list = LinkedList::from_vec(vec![1, 2, 3]);

            list.push_back(4);

            assert_eq!(list.get_values(), vec![1, 2, 3, 4])
        }
        {
            let mut list = LinkedList::from_vec(vec![]);

            list.push_back(5);

            assert_eq!(list.get_values(), vec![5])
        }
    }

    #[test]
    fn test_linked_list_pop_back() {
        let mut list = LinkedList::from_vec(vec![1, 2, 3]);

        assert_eq!(list.pop_back(), Some(3));
        assert_eq!(list.pop_back(), Some(2));
        assert_eq!(list.pop_back(), Some(1));
        assert_eq!(list.pop_back(), None);
    }

    #[test]
    fn test_linked_list_push_front() {
        {
            let mut list = LinkedList::from_vec(vec![1, 2, 3]);

            list.push_front(0);

            assert_eq!(list.get_values(), vec![0, 1, 2, 3]);
        }
        {
            let mut list = LinkedList::from_vec(vec![]);

            list.push_front(5);

            assert_eq!(list.get_values(), vec![5]);
        }
    }

    #[test]
    fn test_linked_list_pop_front() {
        let mut list = LinkedList::from_vec(vec![1, 2, 3]);

        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), Some(2));
        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_back(), None);
    }
}

pub struct IterMut<'a, T>
where
    T: 'a,
{
    next: NodePtr<T>,
    _marker: PhantomData<&'a mut LinkedList<T>>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        // The actual iteration is straight-forward: Once we reached a null pointer, we are done.
        if self.next.is_null() {
            None
        } else {
            // Otherwise, we can convert the next pointer to a reference, get a reference to the data
            // and update the iterator.
            let next = unsafe { &mut *self.next };
            let ret = &mut next.data;

            self.next = next.next;

            Some(ret)
        }
    }
}

// **Exercise 16.2**: Add a method `iter` and a type `Iter` providing iteration for shared
// references. Add testcases for both kinds of iterators.

// ## `Drop`

impl<T> Drop for LinkedList<T> {
    // The destructor itself is a method which takes `self` in mutably borrowed form. It cannot own
    // `self`, because then the destructor of `self` would be called at the end of the function,
    // resulting in endless recursion.
    fn drop(&mut self) {
        let mut cur_ptr = self.first;
        while !cur_ptr.is_null() {
            // In the destructor, we just iterate over the entire list, successively obtaining
            // ownership (`Box`) of every node. When the box is dropped, it will call the destructor
            // on `data` if necessary, and subsequently free the node on the heap.
            let cur = unsafe { raw_into_box(cur_ptr) };
            cur_ptr = cur.next;
            drop(cur);
        }
    }
}

// ## The End
