/*
	double linked list reverse
	This problem requires you to reverse a doubly linked list
*/


use std::fmt::{self, Display, Formatter};
use std::ptr::NonNull;
use std::vec::*;

#[derive(Debug)]
struct Node<T> {
    val: T,
    next: Option<NonNull<Node<T>>>,
    prev: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
    fn new(t: T) -> Node<T> {
        Node {
            val: t,
            prev: None,
            next: None,
        }
    }
}
#[derive(Debug)]
struct LinkedList<T> {
    length: u32,
    start: Option<NonNull<Node<T>>>,
    end: Option<NonNull<Node<T>>>,
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            length: 0,
            start: None,
            end: None,
        }
    }

    pub fn add(&mut self, obj: T) {
        let mut node = Box::new(Node::new(obj));
        node.next = None;
        node.prev = self.end;
        let node_ptr = Some(unsafe { NonNull::new_unchecked(Box::into_raw(node)) });
        match self.end {
            None => self.start = node_ptr,
            Some(end_ptr) => unsafe { (*end_ptr.as_ptr()).next = node_ptr },
        }
        self.end = node_ptr;
        self.length += 1;
    }

    pub fn get(&mut self, index: i32) -> Option<&T> {
        self.get_ith_node(self.start, index)
    }

    fn get_ith_node(&mut self, node: Option<NonNull<Node<T>>>, index: i32) -> Option<&T> {
        match node {
            None => None,
            Some(next_ptr) => match index {
                0 => Some(unsafe { &(*next_ptr.as_ptr()).val }),
                _ => self.get_ith_node(unsafe { (*next_ptr.as_ptr()).next }, index - 1),
            },
        }
    }
    pub fn get_raw(&mut self, index: i32) -> Option<NonNull<Node<T>>> {
        self.get_ith_node_raw(self.start, index)
    }

    pub fn push_back(&mut self, node: Option<NonNull<Node<T>>>) {
        match self.end {
            None => self.start = node,
            Some(end_ptr) => unsafe { (*end_ptr.as_ptr()).next = node },
        }
        self.end = node;
        self.length += 1;
    }
    pub fn push_front(&mut self, node: Option<NonNull<Node<T>>>) {
        if let Some(node) = node {
            unsafe { (*node.as_ptr()).next = self.start };
        }
        self.start = node;
        self.length += 1;
    }
    
    fn get_ith_node_raw(&mut self, node: Option<NonNull<Node<T>>>, index: i32) -> Option<NonNull<Node<T>>> {
        match node {
            None => None,
            Some(next_ptr) => match index {
                0 => Some(next_ptr),
                _ => self.get_ith_node_raw(unsafe { (*next_ptr.as_ptr()).next }, index - 1),
            },
        }
    }
    
    fn insert(&mut self, node: Option<NonNull<Node<T>>>, index: i32) {
        if let Some(node) = node {
            match index {
                0 => {
                    self.push_front(Some(node));
                },
                l if self.length as i32 == l =>{
                    self.push_back(Some(node));
                },
                _ => {
                    if let Some(pos) = self.get_raw(index - 1){
                        unsafe { (*node.as_ptr()).next = (*pos.as_ptr()).next };
                        unsafe { (*pos.as_ptr()).next = Some(node) };
                    }
                    self.length += 1;
                }
            }
        }
    }

    fn insert_sorted(&mut self, node: Option<NonNull<Node<T>>>) where T:Ord {
        //使用双指针，边查找边插入，假定self有序
        if let Some(node) = node {
            let mut prev = None;
            let mut current = self.start;
            while let Some(current_ptr) = current {
                if unsafe { (*current_ptr.as_ptr()).val   <  (*node.as_ptr()).val  } {
                    prev = current;
                    current = unsafe { (*current_ptr.as_ptr()).next };
                } else {
                    break;
                }
            }
            //如果前一个结点非空，则在该处插入
            if let Some(prev_ptr) = prev {
                unsafe { (*node.as_ptr()).next = current };
                unsafe { (*prev_ptr.as_ptr()).next = Some(node) };
                //修改尾部
                if self.end == prev {
                    self.end = Some(node);
                }
            } else {
                //如果前一个结点为空，则在头部插入
                unsafe { (*node.as_ptr()).next = self.start };
                self.start = Some(node);
            }
            self.length += 1;
        }
    }

    fn take_first(&mut self) -> Option<NonNull<Node<T>>> {
        let first = self.start;
        if let Some(first) = first {
            self.start = unsafe { (*first.as_ptr()).next };
            self.length -= 1;
        }
        first
    }

	pub fn reverse(&mut self){
		// 原地翻转链表
        match self.length {
            2.. =>{
                let mut first = self.start;
                let mut current = self.start;
                let mut prev = None;
                while let Some(current_ptr) = current {
                    let next = unsafe { (*current_ptr.as_ptr()).next };
                    unsafe { (*current_ptr.as_ptr()).next = prev };
                    prev = Some(current_ptr);
                    current = next;
                }
                self.start = prev;
                self.end = first;
            },
            _ => {}
        }
	}
}

impl<T> Display for LinkedList<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.start {
            Some(node) => write!(f, "{}", unsafe { node.as_ref() }),
            None => Ok(()),
        }
    }
}

impl<T> Display for Node<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.next {
            Some(node) => write!(f, "{}, {}", self.val, unsafe { node.as_ref() }),
            None => write!(f, "{}", self.val),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::LinkedList;

    #[test]
    fn create_numeric_list() {
        let mut list = LinkedList::<i32>::new();
        list.add(1);
        list.add(2);
        list.add(3);
        println!("Linked List is {}", list);
        assert_eq!(3, list.length);
    }

    #[test]
    fn create_string_list() {
        let mut list_str = LinkedList::<String>::new();
        list_str.add("A".to_string());
        list_str.add("B".to_string());
        list_str.add("C".to_string());
        println!("Linked List is {}", list_str);
        assert_eq!(3, list_str.length);
    }

    #[test]
    fn test_reverse_linked_list_1() {
		let mut list = LinkedList::<i32>::new();
		let original_vec = vec![2,3,5,11,9,7];
		let reverse_vec = vec![7,9,11,5,3,2];
		for i in 0..original_vec.len(){
			list.add(original_vec[i]);
		}
		println!("Linked List is {}", list);
		list.reverse();
		println!("Reversed Linked List is {}", list);
		for i in 0..original_vec.len(){
			assert_eq!(reverse_vec[i],*list.get(i as i32).unwrap());
		}
	}

	#[test]
	fn test_reverse_linked_list_2() {
		let mut list = LinkedList::<i32>::new();
		let original_vec = vec![34,56,78,25,90,10,19,34,21,45];
		let reverse_vec = vec![45,21,34,19,10,90,25,78,56,34];
		for i in 0..original_vec.len(){
			list.add(original_vec[i]);
		}
		println!("Linked List is {}", list);
		list.reverse();
		println!("Reversed Linked List is {}", list);
		for i in 0..original_vec.len(){
			assert_eq!(reverse_vec[i],*list.get(i as i32).unwrap());
		}
	}
}