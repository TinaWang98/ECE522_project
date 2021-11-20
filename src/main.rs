use crate::AVL::{AvlTree};
mod RBTree;
mod AVL;

fn main() {
    /*
    *                            o8888888o
    *                             _ooOoo_
    *                            88" . "88
    *                            (| -_- |)
    *                            O\  =  /O
    *                         ____/`---'\____
    *                       .'  \\|     |//  `.
    *                      /  \\|||  :  |||//  \
    *                     /  _||||| -:- |||||-  \
    *                     |   | \\\  -  /// |   |
    *                     | \_|  ''\---/''  |   |
    *                     \  .-\__  `-`  ___/-. /
    *                   ___`. .'  /--.--\  `. . __
    *                ."" '<  `.___\_<|>_/___.'  >'"".
    *               | | :  `- \`.;`\ _ /`;.`/ - ` : | |
    *               \  \ `-.   \_ __\ /__ _/   .-` /  /
    *          ======`-.____`-.___\_____/___.-`____.-'======
    *                             `=---='
    *          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    *                     佛祖保佑        永无BUG
    *            佛曰:
    *                   写字楼里写字间，写字间里程序员；
    *                   程序人员写程序，又拿程序换酒钱。
    *                   酒醒只在网上坐，酒醉还来网下眠；
    *                   酒醉酒醒日复日，网上网下年复年。
    *                   但愿老死电脑间，不愿鞠躬老板前；
    *                   奔驰宝马贵者趣，公交自行程序员。
    *                   别人笑我忒疯癫，我笑自己命太贱；
    *                   不见满街漂亮妹，哪个归得程序员？
    */

    // =========== AVL Tree Sample Test ==========
    let mut avl_tree = None;
    for i in vec![1, 6, 5, 2, 7, 4] {
        avl_tree.insert_node(i);
    }
    avl_tree.tree_diagram_print();
    println!("Number of leaves: {}", avl_tree.number_of_leaves());

    println!("Height of tree: {}", avl_tree.height_of_tree());

    println!("In Order Traverse:");
    avl_tree.in_order_traverse();

    println!("\nPre Order Traverse:");
    avl_tree.pre_order_traverse();

    println!("\nPost Order Traverse:");
    avl_tree.post_order_traverse();
    println!("\n");

    if avl_tree.is_tree_empty() { println!("Tree is Empty") } else { println!("Tree is not empty!") }


    avl_tree.delete_node(6);
    avl_tree.delete_node(2);
    let s = avl_tree.delete_node(7);
    println!("The deleted Node(7) contains {:?}", s);
    avl_tree.delete_node(4);
    avl_tree.delete_node(100);

    println!("Does Node(11) exist? {}", avl_tree.exist_or_not(11));
    println!("Does Node(111) exist? {}", avl_tree.exist_or_not(111));

    avl_tree.tree_diagram_print();

    // =========== RB Tree Sample Test ==========

}
