use leetcode_rust::leetcode_easy_50::*;


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





