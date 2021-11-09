use criterion::{black_box, criterion_group, criterion_main, Criterion};
use ECE522_project::AVL::AvlTree;
use ECE522_project::BST;

pub fn test_avl_tree(tree_size: i32) {
    let mut avl_t2 = None;
    for i in 0..tree_size {
        avl_t2.insert_node(i);
    }
    let end = tree_size/10;
    for j in 0..end {
        avl_t2.exist_or_not(j);
    }
}

// Binary Search Tree as a basement
pub fn test_BST(tree_size: i32) {
    let mut BS_tree= BST::BSTree::new_empty();
    for i in 0..tree_size {
        BS_tree.insert(i);
    }
    let end = tree_size/10;
    for j in 0..end {
        BS_tree.search(j);
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("test_BST", |b| {
        b.iter(|| test_BST(black_box(100000)))  //black_box(100)
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);