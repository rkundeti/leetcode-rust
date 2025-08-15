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

pub fn move_zeroes(nums: &mut Vec<i32>) {
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

pub fn three_consecutive_odds(arr: Vec<i32>) -> bool {
        // let has_three_odds = arr.iter()
        //     .map(|x| x % 2 != 0)
        //     .collect::<Vec<_>>()
        //     .windows(3)
        //     .any(|w| w.iter().all(|&b| b));

        // has_three_odds
        arr.windows(3).any(|w| w.iter().all(|n| n % 2 != 0))
}

pub fn my_atoi(s: String) -> i32 {
        let mut index = 0;
        let n = s.len();
        let chars: Vec<char> = s.chars().collect();
        let mut result: i32 = 0;
        let mut sign = 1;

        // Skip leading whitespace
        while index < n && chars[index] == ' ' {
            index += 1;
        }

        // Check for optional sign
        if index < n {
            if chars[index] == '-' {
                sign = -1;
                index += 1;
            } else if chars[index] == '+' {
                index += 1;
            }
        }

        // If next character is NOT a digit, return 0 (e.g., "+-12", "--1", "+abc")
        if index >= n || !chars[index].is_ascii_digit() {
            return 0;
        }

        // Parse digits
        while index < n && chars[index].is_ascii_digit() {
            let digit = chars[index].to_digit(10).unwrap() as i32;

            if result > (i32::MAX - digit) / 10 {
                return if sign == 1 { i32::MAX } else { i32::MIN };
            }

            result = result * 10 + digit;
            index += 1;
        }

        sign * result
    }


    // reverse string recursive 

pub fn reverse_string(s: &mut Vec<char>) {
fn helper(start: usize,end: usize,s: &mut Vec<char>){
    if start > end {
        return;
    }
    s.swap(start,end);
    helper(start + 1, end -1, s);

}
if !s.is_empty() {
    helper( 0, s.len() - 1,s);
}
}

pub fn sort_ascending_insert(v:&mut Vec<i32>) {
    // Initialization
    let n = v.len();
   // maintenance
    for j in 1..n{
       
        let key = v[j];
         let mut i = j;
        
      
        while i > 0 && v[i -1] > key {
            v[i] = v[i -1];
            i-=1;

        }
    // termination
        v[i] = key;
      
    }
    
}


pub fn sort_descending_insert(v:&mut Vec<i32>) {
    // Initialization
    let n = v.len();
   // maintenance
    for j in 1..n{
       
        let key = v[j];
         let mut i = j;
        
      
        while i > 0 && v[i -1] < key {
            v[i] = v[i -1];
            i-=1;

        }
    // termination
        v[i] = key;
      
    }
    
}

 #[test]
    fn test_empty_vector() {
        let mut v: Vec<i32> = vec![];
        sort_descending_insert(&mut v);
        assert_eq!(v, vec![]);
    }

    #[test]
    fn test_single_element() {
        let mut v = vec![42];
        sort_descending_insert(&mut v);
        assert_eq!(v, vec![42]);
    }

    #[test]
    fn test_already_descending() {
        let mut v = vec![9, 7, 5, 3, 1];
        sort_descending_insert(&mut v);
        assert_eq!(v, vec![9, 7, 5, 3, 1]);
    }

    #[test]
    fn test_ascending_input() {
        let mut v = vec![1, 2, 3, 4, 5];
        sort_descending_insert(&mut v);
        assert_eq!(v, vec![5, 4, 3, 2, 1]);
    }

    #[test]
    fn test_unsorted() {
        let mut v = vec![3, 1, 4, 1, 5, 9, 2];
        sort_descending_insert(&mut v);
        assert_eq!(v, vec![9, 5, 4, 3, 2, 1, 1]);
    }

    #[test]
    fn test_with_duplicates() {
        let mut v = vec![2, 3, 2, 1, 3, 1];
        sort_descending_insert(&mut v);
        assert_eq!(v, vec![3, 3, 2, 2, 1, 1]);
    }

    #[test]
    fn test_negative_numbers() {
        let mut v = vec![0, -5, 3, -1, 2];
        sort_descending_insert(&mut v);
        assert_eq!(v, vec![3, 2, 0, -1, -5]);
    }


