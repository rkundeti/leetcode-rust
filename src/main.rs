
fn fizz_buzz(n: i32) -> Vec<String> {
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

fn main() {
   let result = fizz_buzz(15);
    println!("{:?}", result);
    }
