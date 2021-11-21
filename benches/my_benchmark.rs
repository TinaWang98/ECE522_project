use criterion::{black_box, criterion_group, criterion_main, Criterion};
use ECE522_project::AVL::{AvlTree,AvlTreeNode};
use ECE522_project::RBTree::RBTree;
use ECE522_project::BST::BSTree;

pub fn test_rb_tree(n: i32) {
    let mut rb_tree = RBTree::new();
    for i in 0..n {
        rb_tree.insert_node(i as u32);
    }
    let end = n/10;
    for j in 0..end {
        rb_tree.search_node(j as u32);
    }
}

pub fn test_avl_tree(tree_size: i32) {
    let mut avl_tree:AvlTreeNode<_> = AvlTree::generate_empty_tree();
    for i in 0..tree_size {
        avl_tree.insert_node(i);
    }
    let end = tree_size/10;
    for j in 0..end {
        avl_tree.exist_or_not(j);
    }
}

// Binary Search Tree as a basement
pub fn test_BST(tree_size: i32) {
    let mut bs_tree= BSTree::new_empty();
    for i in 0..tree_size {
        bs_tree.insert(i);
    }
    let end = tree_size/10;
    for j in 0..end {
        bs_tree.search(j);
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    // c.bench_function("test_BST", |b| {
    //     b.iter(|| test_BST(black_box(100000)))
    // });

    c.bench_function("test_AVL", |b| {
        b.iter(|| test_avl_tree(black_box(100000)))
    });

    // c.bench_function("test_RBT", |b| {
    //     b.iter(|| test_rb_tree(black_box(100)))
    // });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
