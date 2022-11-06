use std::cmp::Ordering;
use std::collections::{LinkedList, VecDeque};
use std::fmt::{Debug, Display};

#[derive(Debug)]
pub enum BTree<T: Ord> {
    Leaf {
        v: T,
        l: Box<BTree<T>>,
        r: Box<BTree<T>>,
    },
    Empty,
}

impl<T: Display + Ord> BTree<T> {
    pub fn new() -> BTree<T> {
        BTree::Empty
    }

    pub fn insert(&mut self, nv: T) {
        match self {
            &mut BTree::Leaf { ref v, ref mut l, ref mut r } => {
                match nv.cmp(v) {
                    Ordering::Less => r.insert(nv),
                    Ordering::Greater => l.insert(nv),
                    // Ordering::Equal => r.insert(nv),
                    Ordering::Equal => return,
                }
            },
            &mut BTree::Empty => {
                *self = BTree::Leaf { v: nv, l: Box::new(BTree::Empty), r: Box::new(BTree::Empty) }
            },
        };
    }

    pub fn is_empty(&self) -> bool {
        match self {
            &BTree::Leaf { .. } => false,
            &BTree::Empty => true,
        }
    }

    pub fn find(&self, fv: T) -> bool {
        match self {
            &BTree::Leaf { ref v, ref l, ref r } => {
                match fv.cmp(v) {
                    Ordering::Less => r.find(fv),
                    Ordering::Greater => l.find(fv),
                    _  => true
                }
            },
            &BTree::Empty => false,
        }
    }

    pub fn get_size(&self) -> u32 {
        match self {
            &BTree::Empty => return 0,
            &BTree::Leaf {
                v: _,
                l: ref left,
                r: ref right,
            } => {
                return left.get_size() + right.get_size() + 1
            }
        };
    }

    pub fn print_preorder(&self) {
        match self {
            &BTree::Empty => return,
            &BTree::Leaf {
                v: ref data,
                l: ref left,
                r: ref right,
            } => {
                print!("{:}, ", data);
                left.print_preorder();
                right.print_preorder();
            }
        };
    }

    pub fn list_preorder(&self) -> Option<LinkedList<&T>> {
        let mut list: LinkedList<&T> = LinkedList::new();
        match self {
            &BTree::Empty => return None,
            &BTree::Leaf {
                v: ref data,
                l: ref left,
                r: ref right,
            } => {
                list.push_back(data);
                match left.list_preorder() {
                    Some(mut ll) => list.append(&mut ll),
                    None => (),
                };
                match right.list_preorder() {
                    Some(mut ll) => list.append( &mut ll),
                    None => (),
                };
                return Some(list);
            }};
    }

    pub fn print_postorder(&self) {
        match self {
            &BTree::Empty => return,
            &BTree::Leaf {
                v: ref data,
                l: ref left,
                r: ref right,
            } => {
                left.print_postorder();
                right.print_postorder();
                print!("{}, ", data);
            }
        };
    }

    pub fn list_postorder(&self) -> Option<LinkedList<&T>> {
        let mut list: LinkedList<&T> = LinkedList::new();
        match self {
            &BTree::Empty => return None,
            &BTree::Leaf {
                v: ref data,
                l: ref left,
                r: ref right,
            } => {
                match left.list_postorder() {
                    Some(mut data) => list.append(&mut data),
                    None => (),
                };
                match right.list_postorder() {
                    Some(mut data) => list.append(&mut data),
                    None => (),
                };
                list.push_back(data);
                return Some(list);
            }
        };
    }

    pub fn print_inorder(&self) {
        match self {
            &BTree::Empty => return,
            &BTree::Leaf {
                v: ref data,
                l: ref left,
                r: ref right,
            } => {
                left.print_inorder();
                print!("{}, ", data);
                right.print_inorder();
            }
        };
    }
    pub fn get_left(&self) -> Option<&Box<BTree<T>>> {
        match self {
            &BTree::Empty => None,
            &BTree::Leaf {
                v: _,
                l: ref left,
                r: _,
            } => Some(left),
        }
    }

    pub fn get_right(&self) -> Option<&Box<BTree<T>>> {
        match self {
            &BTree::Empty => None,
            &BTree::Leaf {
                v: _,
                l: _,
                ref r,
            } => Some(r),
        }
    }
    pub fn get_data(&self) -> Option<&T> {
        match self {
            &BTree::Empty => None,
            &BTree::Leaf {
                ref v,
                l: _,
                r: _,
            } => Some(v),
        }
    }

    pub fn print_levelorder(&self) {
        match self {
            &BTree::Empty => return,
            &BTree::Leaf {
                v: _,
                l: _,
                r: _,
            } => {
                let mut q = VecDeque::new();
                q.push_front(self);
                while !q.is_empty() {
                    let node = q.pop_front().unwrap();
                    if let Some(data) = node.get_data() {
                        print!("{}, ", data);
                    }
                    if let Some(l) = node.get_left() {
                        q.push_back(l);
                    }
                    if let Some(r) = node.get_right() {
                        q.push_back(r);
                    }
                }
                println!();
            }
        }
    }
}