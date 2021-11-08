use crate::AVL::{AvlTree, AvlTreeNode};

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

    for i in vec![10, 4, 7, 1, 3, 8, 9].into_iter() {
        avl_tree.insert_node(i);
    }

    //       7
    //      / \
    //     3   9
    //   / \  / \
    //  1  4 8  10
    // avl_tree.delete_node(9);
    //       7
    //      / \
    //     3   10
    //   / \  /
    //  1  4 8
    println!("{:#?}", avl_tree);

    if AvlTreeNode::validate_tree(&avl_tree) {
        println!("树是AVL");
    } else { println!("不是AVL树！") }

    if avl_tree.is_tree_empty() {
        println!("树是空的");
    } else { println!("不是空的树") }

    println!("叶子节点的数量：{}", avl_tree.number_of_leaves());
    println!("树的高度是:{}", avl_tree.height_of_tree());
    avl_tree.in_order_traverse();
}
