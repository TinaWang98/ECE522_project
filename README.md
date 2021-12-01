# AVL Tree Design Document

> This AVL tree is part of a tree data structure project by Zhaoyi Wang (Mike), Zihao Wang (Bluce) and Shuo Wang (Tina). The goal of this project is to write AVL tree and Red Black tree using `Rust` language. 
------

## Part 1: Brief Overview

Except the following basic features marked by ✅, we add some additional features to this tree marked by 🌟.

- ✅ Insert / Delete / Count Leaves / Calculate Height / In-order Traversal / Check Empty / Print Tree
- 🌟 Pre-order Traversal / Post-order Traversal / Check Element Existence / Update Node / Validate Tree / Total Number of Elements

------

## Part 2: Current Shortcomings

------

❌ Up to this stage, if users want to test in the command line, then their input can only be of numeric type (e.g., `1,2,3,4,5`). This version does not support users using character or string types in the command line interface for now (e.g., `'a','b','c'`). *However, users can import the file and use the interface we defined while writing their code and this will not have any restrictions.*

------

## Part 3: User Manual

------

This AVL Tree implementation based on `Box<>` and `Option<>`.

A tree type defined like the following:

```rust
pub type AvlTreeNode<T> = Option<Box<TreeNode<T>>>;
```

#### Quick Start

Before we start, copy the `AVL.rs` file to your root path (`~/project_name/src`), and add the following line to `main.rs`.

```rust
mod AVL;
use crate::AVL::{AvlTree, AvlTreeNode};
```

First, we need to define an empty tree, and make sure it is mutable.

```rust
let mut avl_tree: AvlTreeNode<_> = AvlTree::generate_empty_tree();
```

Second, we can add some value to this tree. Here I will use for loop.

```rust
for i in vec![5, 3, 1, 4, 2] {
        avl_tree.insert_node(i);
    }
```

Then, print the tree and validate it.

```rust
avl_tree.print_tree_diagram();
println!("Balanced Tree? {}", avl_tree.validate_tree());
```

Next, we can calculate the leaves and count the height of the tree.

```rust
println!("Number of leaves: {}", avl_tree.number_of_leaves());
println!("Height of tree: {}", avl_tree.height_of_tree());
```

Also, if you need, you can do three types of traversal to your tree.

```rust
println!("In Order Traverse: {:?}", avl_tree.in_order_traverse());
println!("Pre Order Traverse: {:?}", avl_tree.pre_order_traverse());
println!("Post Order Traverse: {:?}", avl_tree.post_order_traverse());
```

Don't forget to check whether it is empty if you are not sure about whether the above operation was successful.

```rust
if avl_tree.is_tree_empty() { 
  		println!("Tree is Empty") 
	} else { 
  		println!("Tree is not empty!") 
}
```

After that, we can delete some node.

```rust
avl_tree.delete_node(1);
```

You can get the feedback at the same time, if you want.

```rust
let res = avl_tree.delete_node(2);
println!("The deleted Node(2) contains {:?}", res);
```

To check the result of the `insert_node()` and `delete_node()` operation, you can do the following to check the existence of a specific node.

```rust
for i in vec![4, 6, 5] {
  			// Check the existence of Node(4), Node(5) and Node (6)
        println!("Does {} exist? {}", i, avl_tree.exist_or_not(i));
    }
```

By the way, you can update a node just like the way you want to update a info in your database.

```rust
avl_tree.update_node(1, 2);  // 1 is the OLD one, 2 is the NEW one
```

Finally, let us do in-order traversal again and print the final tree.

```rust
println!("In Order Traverse: {:?}", avl_tree.in_order_traverse());
avl_tree.print_tree_diagram();
```

And see how many elements we have now.

```rust
println!("This AVL tree has a total of {} elements.", avl_tree.total_number_elements());
```

#### Public Interface

```rust
fn insert_node(&mut self, val: T);
// insert a node
fn delete_node(&mut self, val: T) -> Self;
// delete a node
fn validate_tree(&self) -> bool;
// balanced or not?
fn is_tree_empty(&self) -> bool;
// empty or not?
fn height_of_tree(&self) -> i32;
// get the height of this tree
fn number_of_leaves(&self) -> i32;
// how many leaves 
fn in_order_traverse(&mut self) -> Vec<T>;
// In-order traverse
fn pre_order_traverse(&mut self) -> Vec<T>;
// Pre-order traverse
fn post_order_traverse(&mut self) -> Vec<T>;
// Post-order traverse
fn print_tree_diagram(&mut self);
// Nicely print the tree
fn exist_or_not(&self, val: T) -> bool;
// Check whether a value exists
fn generate_empty_tree() -> Self;
// generate a new empty tree
fn update_node(&mut self, old: T, new: T);
// update a node
fn total_number_elements(&mut self) -> i32;
// count total number of elements
```

------

## Part 4: Performance Evaluation 

------

The evaluation criteria are as follows:

```
for tree_size in (10000, 40000, 70000, 100000, 130000):
	Create a empty tree;
	Values with tree_size are inserted into the tree;
	A search is conducted for the (tree_size/10) lowest values.
```

This evaluation was done on the following computer configurations, and *the results may vary from computer to computer*.

> AMD Ryzen 5 3600 6-Core CPU / 16G DDR4 3200MHz Memory

We use `criterion` create to perform the benchmark. For more information, please click [here](https://crates.io/crates/criterion).

|  Size  | AVL Tree  | Binary Search Tree |
| :----: | :-------: | :----------------: |
| 10000  | 1.1143 ms |     98.981 ms      |
| 40000  | 4.6040 ms |  *Stack Overflow*  |
| 70000  | 8.2933 ms |  *Stack Overflow*  |
| 100000 | 11.893 ms |  *Stack Overflow*  |
| 130000 | 15.480 ms |  *Stack Overflow*  |

------


