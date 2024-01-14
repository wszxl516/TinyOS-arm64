use alloc::boxed::Box;
use core::ptr;

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    item: T,
    next: Link<T>,
}

unsafe impl<T> Sync for TaskQueue<T> {}

unsafe impl<T> Send for TaskQueue<T> {}

pub struct TaskQueue<T> {
    head: Link<T>,
    tail: *mut Node<T>,
    pub(crate) len: usize
}

impl<T> TaskQueue<T> {
    pub const fn new() -> Self {
        Self {
            head: None,
            tail: ptr::null_mut(),
            len: 0,
        }
    }

    pub fn push_front(&mut self, item: T) {
        let mut item = Box::new(Node {
            item,
            next: self.head.take(),
        });

        if self.tail.is_null() {
            self.tail = &mut *item;
        }
        self.len +=1;
        self.head = Some(item);
    }

    #[allow(dead_code)]
    pub fn push_back(&mut self, item: T) {
        let mut item = Box::new(Node { item, next: None });

        let new_tail: *mut _ = &mut *item;

        if self.tail.is_null() && self.head.is_none() {
            self.head = Some(item);
        } else {
            unsafe {
                (*self.tail).next = Some(item);
            }
        }
        self.len +=1;
        self.tail = new_tail;
    }

    pub fn head(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.item)
    }

    pub fn next(&mut self) -> Option<&mut T> {
        if let Some(mut head) = self.head.take() {
            self.head = head.next.take();
            if self.head.is_none() {
                self.head = Some(head);
            } else {
                let tail = self.tail;
                self.tail = &mut *head;
                unsafe {
                    (*tail).next = Some(head);
                }
            }
        }
        self.head()
    }
}
