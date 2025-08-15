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

#[test]
    fn test_mixed_values() {
        let mut nums = vec![0, 1, 0, 3, 12];
        move_zeroes(&mut nums);
        assert_eq!(nums, vec![1, 3, 12, 0, 0]);
    }

    #[test]
    fn test_all_zeros() {
        let mut nums = vec![0, 0, 0];
        move_zeroes(&mut nums);
        assert_eq!(nums, vec![0, 0, 0]);
    }

#[test]
fn all_odds() {
    let v = vec![1,2,34,3,4,5,7,23,12];
    assert!(three_consecutive_odds(v));
}

#[test]
    fn test_empty_vector() {
        let mut v: Vec<i32> = vec![];
        sort_ascending_insert(&mut v);
        assert_eq!(v, vec![]);
    }

    #[test]
    fn test_single_element() {
        let mut v = vec![42];
        sort_ascending_insert(&mut v);
        assert_eq!(v, vec![42]);
    }

    #[test]
    fn test_already_sorted() {
        let mut v = vec![1, 2, 3, 4, 5];
        sort_ascending_insert(&mut v);
        assert_eq!(v, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_reverse_sorted() {
        let mut v = vec![5, 4, 3, 2, 1];
        sort_ascending_insert(&mut v);
        assert_eq!(v, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_unsorted() {
        let mut v = vec![3, 1, 4, 1, 5, 9, 2];
        sort_ascending_insert(&mut v);
        assert_eq!(v, vec![1, 1, 2, 3, 4, 5, 9]);
    }

    #[test]
    fn test_with_duplicates() {
        let mut v = vec![2, 3, 2, 1, 3, 1];
        sort_ascending_insert(&mut v);
        assert_eq!(v, vec![1, 1, 2, 2, 3, 3]);
    }

    #[test]
    fn test_negative_numbers() {
        let mut v = vec![0, -5, 3, -1, 2];
        sort_ascending_insert(&mut v);
        assert_eq!(v, vec![-5, -1, 0, 2, 3]);
    }
 #[test]
    fn test_empty_vector_d() {
        let mut v: Vec<i32> = vec![];
        sort_descending_insert(&mut v);
        assert_eq!(v, vec![]);
    }

    #[test]
    fn test_single_element_d() {
        let mut v = vec![42];
        sort_descending_insert(&mut v);
        assert_eq!(v, vec![42]);
    }

    #[test]
    fn test_already_descending_d() {
        let mut v = vec![9, 7, 5, 3, 1];
        sort_descending_insert(&mut v);
        assert_eq!(v, vec![9, 7, 5, 3, 1]);
    }

    #[test]
    fn test_ascending_input_d() {
        let mut v = vec![1, 2, 3, 4, 5];
        sort_descending_insert(&mut v);
        assert_eq!(v, vec![5, 4, 3, 2, 1]);
    }

    #[test]
    fn test_unsorted_d() {
        let mut v = vec![3, 1, 4, 1, 5, 9, 2];
        sort_descending_insert(&mut v);
        assert_eq!(v, vec![9, 5, 4, 3, 2, 1, 1]);
    }

    #[test]
    fn test_with_duplicates_d() {
        let mut v = vec![2, 3, 2, 1, 3, 1];
        sort_descending_insert(&mut v);
        assert_eq!(v, vec![3, 3, 2, 2, 1, 1]);
    }

    #[test]
    fn test_negative_numbers_d() {
        let mut v = vec![0, -5, 3, -1, 2];
        sort_descending_insert(&mut v);
        assert_eq!(v, vec![3, 2, 0, -1, -5]);
    }

    #[test]
    fn finds_first_element() {
        let arr = [5, 10, 15, 20];
        assert_eq!(linear_search(5, &arr), Some(0));
    }

    #[test]
    fn finds_middle_element() {
        let arr = [5, 10, 15, 20];
        assert_eq!(linear_search(15, &arr), Some(2));
    }

    #[test]
    fn finds_last_element() {
        let arr = [5, 10, 15, 20];
        assert_eq!(linear_search(20, &arr), Some(3));
    }

    #[test]
    fn returns_none_if_not_found() {
        let arr = [5, 10, 15, 20];
        assert_eq!(linear_search(99, &arr), None);
    }

    #[test]
    fn works_with_empty_array() {
        let arr: [i32; 0] = [];
        assert_eq!(linear_search(1, &arr), None);
    }

    #[test]
    fn finds_first_match_if_duplicates() {
        let arr = [1, 2, 3, 2, 1];
        assert_eq!(linear_search(2, &arr), Some(1));
    }








