use leetcode_rust::m_rotate_string::*;


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




