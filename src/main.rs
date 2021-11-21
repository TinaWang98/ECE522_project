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
    // 添加元素并打印完整树
    let mut avl_tree = None;
    for i in vec![5, 8, 6, 9, 0, 2, 13, 16] {
        avl_tree.insert_node(i);
    }
    avl_tree.print_tree_diagram();
    // 打印基础信息
    println!("Number of leaves: {}", avl_tree.number_of_leaves());
    println!("Height of tree: {}", avl_tree.height_of_tree());
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
    // 删除节点 -> 结果应为[8, 9, 2, 13]
    avl_tree.delete_node(5);
    avl_tree.delete_node(6);
    let s = avl_tree.delete_node(0);
    println!("The deleted Node(0) contains {:?}", s);
    avl_tree.delete_node(16);
    avl_tree.delete_node(17);
    // 节点是否存在？
    for i in vec![8,9,2,13,5,7,20,100] {
        println!("Does {} exist? {}", i, avl_tree.exist_or_not(i));
    }
    // 删除完之后中序遍历并打印
    println!("Get Inorder List: {:?}", avl_tree.get_inorder_list());
    avl_tree.print_tree_diagram();
    // 重新加入几个新元素
    for i in vec![5, 7, 20, 100] {
        avl_tree.insert_node(i);
    }
    avl_tree.print_tree_diagram();
    println!("Get Inorder List: {:?}", avl_tree.get_inorder_list());

    let mut avl_tree2 = None;
    avl_tree2.insert_node(2);
    avl_tree2.delete_node(2);
    avl_tree2.print_tree_diagram();
    // =========== RB Tree Sample Test ==========
}
