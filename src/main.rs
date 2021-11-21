use crate::AVL::{AvlTree};

mod RBTree;
mod AVL;

fn main() {
    // let args: Vec<String> = std::env::args().collect();
    // let length = args.len();
    // let keyword = &args[1]; // cargo[0] xxx[1] xxx[2] ...

    // =========== AVL Tree Sample Test ==========
    run_avl_tree_sample();
    // =========== RB Tree Sample Test ==========
}

// handle_input(): 允许用户在console进行一次输入，并将输入转换成一个i32类型的数字返回
fn handle_input() -> i32 {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Cannot read!");
    let res: i32 = input.trim().parse().expect("Should be a number!");
    res
}

fn run_avl_tree_sample() {
    // 添加元素并打印完整树
    let mut avl_tree = None;
    for i in vec![5, 1, 0, 6, 2, 4, 9, 3, 7] {
        avl_tree.insert_node(i);
    }
    avl_tree.print_tree_diagram();
    // 打印基础信息
    println!("Number of leaves: {}", avl_tree.number_of_leaves());
    println!("Height of tree: {}", avl_tree.height_of_tree());
    println!("Tree? {}", AvlTree::validate_tree(&avl_tree));
    // 前中后序遍历
    println!("In Order Traverse:");
    avl_tree.in_order_traverse();
    println!("\nPre Order Traverse:");
    avl_tree.pre_order_traverse();
    println!("\nPost Order Traverse:");
    avl_tree.post_order_traverse();
    println!("\n");
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
    println!("Get Inorder List: {:?}", avl_tree.get_inorder_list());
    avl_tree.print_tree_diagram();
    // 重新加入几个新元素
    for i in vec![6, 5, 20, 100] {
        avl_tree.insert_node(i);
    }
    avl_tree.print_tree_diagram();
    println!("Get Inorder List: {:?}", avl_tree.get_inorder_list());
}