use criterion::{black_box, criterion_group, criterion_main, Criterion};
use ECE522_project::AVL::AvlTree;

pub fn test_avl_tree(tree_size: u32) {
    let mut avl_t2 = None;
    for i in 0..tree_size {
        avl_t2.insert_node(i);
    }
    let end = tree_size/10;
    for j in 0..end {
        avl_t2.exist_or_not(j);
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("test_avl_tree", |b| {
        b.iter(|| test_avl_tree(1000))  //black_box(100)
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);