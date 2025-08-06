use leetcode_rust::leetcode_easy_50::*;
use std::rc::Rc;
use std::cell::RefCell;



pub fn build_tree() -> Option<Rc<RefCell<TreeNode>>> {
    let root = TreeNode::new(1);
    let node2 = TreeNode::new(2);
    let node3 = TreeNode::new(3);
    let node4 = TreeNode::new(4);
    let node5 = TreeNode::new(5);
    let node6 = TreeNode::new(6);

    root.borrow_mut().left = Some(node2.clone());
    root.borrow_mut().right = Some(node3.clone());
    node2.borrow_mut().left = Some(node4.clone());
    node2.borrow_mut().right = Some(node5.clone());
    node3.borrow_mut().right = Some(node6.clone());

    Some(root)
}

#[test]
fn test_preorder_traversal() {
    let tree = build_tree();  

//   let  output_rec: Vec<i32> = Vec::new();
    let  result  = preorder_traversal(tree.clone());
    assert_eq!(result, vec![1, 2, 4, 5, 3, 6]);
}

#[test]
fn test_rotate_string() {
    assert_eq!(rotate_string("abcde".to_string(), "cdeab".to_string()), true);
}

#[test]
fn test_rotate_string_other() {
    assert_eq!(rotate_string("abcde".to_string(), "abced".to_string()), false);
}
#[test]
fn test_rotate_string_concat() {
       assert_eq!(rotate_string("abcde".to_string(), "cdeab".to_string()), true);
}     
#[test]
fn test_rotate_string_concat_other() {
       assert_eq!(rotate_string("abcde".to_string(), "abced".to_string()), false);
}
#[test]
fn test_check_perfect_number() {
       assert_eq!(check_perfect_number(28), true);
}

#[test]
fn test_two_sum_basic() {
    assert_eq!(two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
}

#[test]
fn test_two_sum_other() {
    assert_eq!(two_sum(vec![3, 2, 4], 6), vec![1, 2]);
}

#[test]
    fn test_fizz_buzz_15() {
        let expected = vec![
            "1", "2", "Fizz", "4", "Buzz", "Fizz", "7", "8", "Fizz", "Buzz",
            "11", "Fizz", "13", "14", "FizzBuzz",
        ].into_iter().map(String::from).collect::<Vec<String>>();

        let result = fizz_buzz(15);
        assert_eq!(result, expected);
    }

#[test]
    fn test_third_max() {
        assert_eq!(third_max(vec![3, 2, 1]), 1);
        assert_eq!(third_max(vec![1, 2]), 2);
        assert_eq!(third_max(vec![2, 2, 3, 1]), 1);
        assert_eq!(third_max(vec![1, 2, 2, 5, 3, 5]), 2);
    }

#[test]
fn test_length_of_longest_substring() {
    assert_eq!(length_of_longest_substring("pwwkew".to_string()),3);
    assert_eq!(length_of_longest_substring("abcabcbb".to_string()),3);
    assert_eq!(length_of_longest_substring("abcdefbdgcbc".to_string()),6);
    assert_eq!(length_of_longest_substring("bbbbbbb".to_string()),1);
    
}






