pub type Node<T> = Option<Box<Bstree<T>>>;

#[derive(Debug)]
pub struct Bstree<T> {
    val: T,
    left: Node<T>,
    right: Node<T>,
}

pub trait BinarySearchTree<T: Copy + PartialOrd> {
    fn gen_empty_tree() -> Self;
    fn new_node(&mut self, val: T) -> Self;
    fn insert_node(&mut self, val: T);
    fn search_node(&self, val: T) -> bool;
}

impl<T> BinarySearchTree<T> for Node<T> where T: Copy + PartialOrd {
    fn gen_empty_tree() -> Self {
        Self::None
    }

    fn new_node(&mut self, val: T) -> Self {
        Some(Box::from(Bstree {
            val,
            left: None,
            right: None,
        }))
    }

    fn insert_node(&mut self, val: T) {
        match self {
            None => *self = self.new_node(val),
            Some(node) => {
                if val < node.val {
                    node.left.insert_node(val);
                } else if val > node.val {
                    node.right.insert_node(val);
                } else {
                    println!("Node exists!");
                }
            }
        }
    }

    fn search_node(&self, val: T) -> bool {
        match self {
            None => false,
            Some(node) => {
                if val < node.val {
                    self.as_ref().unwrap().left.search_node(val)
                } else if val > node.val {
                    self.as_ref().unwrap().right.search_node(val)
                } else if val == node.val {
                    true
                } else {
                    false
                }
            }
        }
    }
}