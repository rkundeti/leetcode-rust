
fn linear_search(val: i32, arr: &[i32]) -> Option<usize> {
    for (index,&item) in arr.iter().enumerate() {
        if item == val {
            return Some(index);
    
    }    
   }
   None
}


fn main() {   
   //linear search
    let nums = vec![4, 7, 1, 6, 3];
    let target = 9;

    match linear_search(target, &nums) {
        Some(idx) => println!("Found {} at index {}", target, idx),
        None => println!("{} not found", target),
    }
}