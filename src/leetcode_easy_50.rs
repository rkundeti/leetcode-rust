use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use std::cmp::max;

#[derive(Debug)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    pub fn new(val: i32) -> Rc<RefCell<TreeNode>> {
        Rc::new(RefCell::new(TreeNode {
            val,
            left: None,
            right: None,
        }))
    }
}
// src/two_sum.rs
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    use std::collections::HashMap;
    let mut map = HashMap::new();
    for (i, num) in nums.iter().enumerate() {
        if let Some(&j) = map.get(&(target - num)) {
            return vec![j as i32, i as i32];
        }
        map.insert(num, i);
    }
    vec![]
}

//src/m_rotate_string.rs
// Approach 1: Brute Force
pub fn rotate_string(s: String, goal: String) -> bool {

            if s.len() != goal.len(){
                return false;
            }
            let mut s_rotated = s.to_string();
            for _ in 0..s.len() {
                let first_char = s_rotated.remove(0);
                s_rotated.push(first_char);
                if s_rotated == goal {
                    return true;
                }
            }
             false

    }
// Approach 2: Concatenation Check

pub fn rotate_string_concat(s: String, goal: String)->bool {
        if s.len() != goal.len(){
                    return false;
                }
        let double_string = s.clone() + &s;
        double_string.find(&goal).is_some()

}


pub fn check_perfect_number(num: i32) -> bool {
    if num <= 0 {
        return false;
    }

    let mut sum = 0;
    let num_u = num as u32;

    for i in 1..=((num_u as f64).sqrt() as u32) {
        if num_u % i == 0 {
            sum += i;
            if i != num_u / i {
                sum += num_u / i;
            }
        }
    }

    sum - num_u == num_u

} 

pub fn fizz_buzz(n: i32) -> Vec<String> {
    let word_dict = vec![(3, "Fizz"), (5, "Buzz")];  // Ordered list of pairs

    let mut answer = Vec::new();

    for i in 1..=n {
        let mut ans_str = String::new();
        for &(key,val) in &word_dict {
            if i % key == 0 {
                ans_str += val;
            }
        }
        if ans_str.is_empty() {
            ans_str = i.to_string();
        }
        
        answer.push(ans_str);
    }
answer
}

pub fn third_max(nums: Vec<i32>) -> i32 {
    let mut first_max = i64::MIN;
    let mut second_max = i64::MIN;
    let mut third_max = i64::MIN;

    for &num in &nums {
        let num = num as i64;

        if num == first_max || num == second_max || num == third_max {
            continue;
        }

        if num > first_max {
            third_max = second_max;
            second_max = first_max;
            first_max = num;
        }
        else if num > second_max {
            third_max = second_max;
            second_max = num;
        } else if num > third_max {
            third_max = num;
        }
    }

    if third_max == i64::MIN {
        first_max as i32
    } else {
        third_max as i32
    }
}


pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut result = Vec::new();
    let mut stack = Vec::new();

    if let Some(node) = root {
        stack.push(node);
    }

    while let Some(node_rc) = stack.pop() {
        let node = node_rc.borrow();
        result.push(node.val);

        if let Some(ref right) = node.right {
            stack.push(right.clone());
        }

        if let Some(ref left) = node.left {
            stack.push(left.clone());
        }
    }
    result
}

pub fn length_of_longest_substring(s:String)-> i32 {
    let mut ans = 0;
    let mut hm = HashMap::new();
    let mut i = 0;
    for (j,c) in s.chars().enumerate() {
        if let Some(&next_index) = hm.get(&c) {
            i = max(i,next_index);
        }
        ans = max(ans,j-i + 1);
        hm.insert(c,j+1);
    }
    ans as i32
}

