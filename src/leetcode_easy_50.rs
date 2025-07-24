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
