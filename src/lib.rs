// 将src目录下面的文件(binary crate)都添加在这里
// 并且将mod设为公有，这样就可以在../benches文件夹中调用
pub mod AVL; // use ECE522_project::AVL::AvlTree; <- my_benchmark.rs
