
pub fn find_duplicate(nums: Vec<i32>) -> Option<i32> {
    let n = nums.len();
    for j in 0..n {
        for i in (j + 1)..n {
            if nums[i] == nums[j] {
                return Some(nums[i]);
            }
        }
    }
    None
}



fn main() {   
    let v = vec![1, 3, 4, 2, 2];
    match find_duplicate(v) {
        Some(val) => println!("Duplicate found: {}", val),
        None => println!("No duplicate found"),
    }
}