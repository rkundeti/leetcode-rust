use std::collections::HashMap;
use std::cmp::max;
// use std::rc::Rc;
// use std::cell::RefCell;
// // Definition for a binary tree node
// #[derive(Debug)]
// pub struct TreeNode{
//     pub val:i32,
//     pub left: Option<Rc<RefCell<TreeNode>>>,
//     pub right: Option<Rc<RefCell<TreeNode>>>
//     }

// impl TreeNode {
//     pub fn new(val:i32) -> Rc<RefCell<TreeNode>> {
//         Rc::new(RefCell::new(TreeNode {
//           val,
//           left:None,
//           right:None,
//         }))  
//         }
//     }

// fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
//     let mut result = Vec::new();
//     let mut stack = Vec::new();

//     if let Some(node) = root {
//         stack.push(node);
//     }

//     while let Some(node_rc) = stack.pop() {
//         let node = node_rc.borrow();
//         result.push(node.val);

//         if let Some(ref right) = node.right {
//             stack.push(right.clone());
//         }

//         if let Some(ref left) = node.left {
//             stack.push(left.clone());
//         }
//     }
//     result
// }

// fn build_tree() ->Option<Rc<RefCell<TreeNode>>> {
//     let root = TreeNode::new(1);
//     let node2 = TreeNode::new(2);
//     let node3 = TreeNode::new(3);
//     let node4 = TreeNode::new(4);
//     let node5 = TreeNode::new(5);
//     let node6 = TreeNode::new(6);

//    // Set left and right of root
//     root.borrow_mut().left = Some(node2.clone());
//     root.borrow_mut().right = Some(node3.clone());

//     // Set left and right of node2
//     node2.borrow_mut().left = Some(node4.clone());
//     node2.borrow_mut().right = Some(node5.clone());

//     // Set right of node3
//     node3.borrow_mut().right = Some(node6.clone());

//     Some(root)
// }

pub fn move_zeroes(nums: &mut Vec<i32>) {
    let mut reader = 0;
    let mut writer  = 0;
    for reader in 1..nums.len() {
        if nums[reader] != 0 {
            if reader != writer {
                nums.swap(writer, reader);
                writer+=1;
            }
        }
    }
}
fn main() {    
    let mut nums = vec![0, 1, 0, 3, 12];
    move_zeroes(&mut nums);
    println!("{:?}", nums); // [1, 3, 12, 0, 0]
    }
