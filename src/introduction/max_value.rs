use std::i32::MIN;

pub fn max_value(nums: Vec<i32>) -> i32 {
    let mut max = MIN;
    for x in nums {
        if x > max {
            max = x;
        }
    }
    max
}

#[test]
fn test_max_value() {
    assert_eq!(max_value(vec![4, 7, 2, 8, 10, 9]), 10);
    assert_eq!(max_value(vec![10, 5, 40, 45]), 45);
    assert_eq!(max_value(vec![-5, -2, -1, -11]), -1);
    assert_eq!(max_value(vec![42]), 42);
    assert_eq!(max_value(vec![1000, 8]), 1000);
    assert_eq!(max_value(vec![1000, 8, 9000]), 9000);
    assert_eq!(max_value(vec![2, 5, 1, 1, 4]), 5);
}
