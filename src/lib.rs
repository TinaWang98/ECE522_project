use crate::AVL::{AvlTree, AvlTreeNode};

// 将src目录下面的文件(binary crate)都添加在这里
// 并且将mod设为公有，这样就可以在../benches文件夹中调用
pub mod AVL;
// use ECE522_project::AVL::AvlTree; <- my_benchmark.rs
pub mod BST;
pub mod RBTree;

pub fn run_avl_tree_example() {
    // 添加元素并打印完整树
    let mut avl_tree: AvlTreeNode<_> = AvlTree::generate_empty_tree();
    for i in vec![5, 1, 0, 6, 2, 4, 9, 3, 7] {
        avl_tree.insert_node(i);
    }
    avl_tree.print_tree_diagram();
    // 打印基础信息
    println!("Number of leaves: {}", avl_tree.number_of_leaves());
    println!("Height of tree: {}", avl_tree.height_of_tree());
    println!("Tree? {}", AvlTree::validate_tree(&avl_tree));
    // 前中后序遍历
    println!("In Order Traverse: {:?}", avl_tree.in_order_traverse());
    println!("Pre Order Traverse: {:?}", avl_tree.pre_order_traverse());
    println!("Post Order Traverse: {:?}",avl_tree.post_order_traverse());
    // 是否为空？
    if avl_tree.is_tree_empty() { println!("Tree is Empty") } else { println!("Tree is not empty!") }
    // 删除节点 -> 结果应为[1, 2, 4, 9, 3]
    avl_tree.delete_node(5);
    avl_tree.delete_node(6);
    let s = avl_tree.delete_node(0);
    println!("The deleted Node(0) contains {:?}", s);
    avl_tree.delete_node(7);
    avl_tree.delete_node(10);
    // 节点是否存在？
    for i in vec![1, 2, 11, 9, 3, 6, 5, 20, 100] {
        println!("Does {} exist? {}", i, avl_tree.exist_or_not(i));
    }
    // 删除完之后中序遍历并打印
    avl_tree.print_tree_diagram();
    // 重新加入几个新元素 -> 结果应为[1, 2, 4, 9, 3, 6, 5, 20, 100]
    for i in vec![6, 5, 20, 100] {
        avl_tree.insert_node(i);
    }
    avl_tree.print_tree_diagram();
    avl_tree.update_node(111, 7);
    println!("In Order Traverse: {:?}", avl_tree.in_order_traverse());
}

pub fn run_rb_tree_example() {
    todo!();
}