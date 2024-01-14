#![allow(dead_code)]
use alloc::boxed::Box;
use alloc::rc::Rc;
use core::cell::RefCell;
use core::fmt;
use core::fmt::Display;

pub type Link<T> = Option<Rc<RefCell<Box<Node<T>>>>>;
#[derive(Debug, Clone)]
pub struct Node<T: Clone> {
    pub data: T,
    pub prev: Link<T>,
    pub next: Link<T>,
}

impl<T: Clone> Node<T> {
    pub fn new(data: T) -> Self {
        Self {
            data,
            prev: None,
            next: None,
        }
    }
}
#[derive(Debug, Clone)]
pub struct List<T: Clone> {
    head: Link<T>,
    last: Link<T>,
    len: usize,
}

impl<T: Clone> List<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            last: None,
            len: 0,
        }
    }

    pub fn push_back(&mut self, data: T) {
        let new = Rc::new(RefCell::new(Box::new(Node::new(data))));
        if self.head.is_some() {
            if let Some(ref last) = self.last.clone() {
                new.borrow_mut().prev.replace(last.clone());
                last.borrow_mut().next.replace(new.clone());
                self.last = Some(new.clone());
                self.len += 1;
            }
        } else {
            self.head = Some(new.clone());
            self.last = Some(new);
        }
    }
    pub fn push_front(&mut self, data: T) {
        let new = Rc::new(RefCell::new(Box::new(Node::new(data))));
        if let Some(ref head) = self.head {
            head.borrow_mut().prev = Some(new.clone());
            new.borrow_mut().next = Some(head.clone());
            self.head.replace(new);
            self.len += 1;
        } else {
            self.head.replace(new.clone());
            self.last.replace(new);
        }
    }
    pub fn pop_back(&mut self) -> Option<Box<Node<T>>> {
        match self.last.is_some() {
            true => self.last.take().map(|last| {
                if let Some(ref prev) = last.borrow().prev {
                    prev.borrow_mut().next = None;
                    self.last = Some(prev.clone());
                    self.len -= 1;
                } else {
                    self.last = None;
                    self.head = None;
                }
                Rc::try_unwrap(last).ok().expect("None").into_inner()
            }),
            false => None,
        }
    }

    pub fn pop_front(&mut self) -> Option<Box<Node<T>>> {
        match self.head.is_some() {
            true => self.head.take().map(|head| {
                if let Some(ref next) = head.borrow().next {
                    next.borrow_mut().prev = None;
                    self.head = Some(next.clone());
                    self.len -= 1;
                } else {
                    self.last = None;
                    self.head = None;
                }
                Rc::try_unwrap(head).ok().expect("None").into_inner()
            }),
            false => None,
        }
    }
    pub fn remove(&mut self, f: &dyn Fn(T) -> bool) -> Option<T> {
        let list = self.clone();
        let mut data = None;
        let mut node = match self.head.clone() {
            None => return None,
            Some(n) => n,
        };
        for _ in 0..list.len {
            let t = node.borrow().data.clone();
            if f(t.clone()) {
                if let Some(p) = &node.borrow().prev {
                    p.borrow_mut().next = node.borrow().next.clone();
                } else {
                    self.head = node.borrow().next.clone()
                }
                if let Some(n) = &node.borrow().next {
                    n.borrow_mut().prev = node.borrow().prev.clone();
                } else {
                    self.last = node.borrow().prev.clone()
                }
                data = Some(t);
            }
            let next = node.borrow().next.clone();
            if let Some(next) = next {
                node = next.clone();
            } else {
                return None;
            };
        }
        data
    }
}

impl<T: Clone> Iterator for List<T> {
    type Item = Rc<RefCell<Box<Node<T>>>>;
    fn next(&mut self) -> Option<Self::Item> {
        let current = self.head.clone();
        match current {
            None => return None,
            Some(ref c) => self.head = c.borrow().next.clone(),
        }
        if current.is_none() {
            loop {
                match self.head.clone() {
                    None => return None,
                    Some(head) => self.head = head.borrow().prev.clone(),
                }
            }
        }
        current.clone()
    }
}

impl<T: Display + Clone> Display for List<T> {
    fn fmt(&self, w: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(w, "[")?;
        let mut node = self.head.clone();
        while let Some(n) = node {
            write!(w, "{}", n.borrow().data)?;
            node = n.borrow().next.clone();
            if node.is_some() {
                write!(w, ", ")?;
            }
        }
        write!(w, "]")
    }
}
