use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::rc::Rc;

type AVLTreeNode<T> = Option<Rc<RefCell<TreeNode<T>>>>;
#[derive(Clone, Debug)]
// each node has 4 features: data, height, left and right child
pub struct TreeNode<Data: PartialOrd> {
    data: u32,
    height: i32,
    left_child: AVLTreeNode<T>,
    right_child: AVLTreeNode<T>,
}

pub trait AvlTree<T: PartialOrd> {
    fn new(val: T) -> Self;
    fn height(&self) -> i32;
    fn insert(&mut self, val: T);
    fn delete(&mut self, val: T) -> Self;
}

impl<T: PartialOrd> AvlTree<T> for AvlTreeNode<T> {
    // generate a pure new node
    fn new(data: T) -> AVLTreeNode<T> {
        Some(Rc::new(RefCell::new(
            TreeNode {
                data,
                height: 1,
                left_child: None,
                right_child: None,
            }
        )))
    }

    fn height(&self) -> i32 {
        match self {
            None => 0,
            // TODO
        }
    }

    fn insert(&mut self, val: T) {
        todo!()
    }

    fn delete(&mut self, val: T) -> Self {
        todo!()
    }
}

impl<T: ToString + PartialOrd> ToString for TreeNode<T> {
    fn to_string(&self) -> String {
        return format!("Node {}(h: {} l: {}, r: {})", self.data.to_string(), self.height, to_string::<T>(&self.left), to_string::<T>(&self.right));
    }
}
