use leetcode_rust::m_two_sum::two_sum;

#[test]
fn test_two_sum_basic() {
    assert_eq!(two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
}

#[test]
fn test_two_sum_other() {
    assert_eq!(two_sum(vec![3, 2, 4], 6), vec![1, 2]);
}
