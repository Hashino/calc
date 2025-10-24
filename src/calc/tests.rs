use crate::calc::calculator::evaluate;

#[test]
fn test_addition() {
    assert_eq!(evaluate("2 + 3".to_string()).unwrap(), 5.0);
}

#[test]
fn test_subtraction() {
    assert_eq!(evaluate("5 - 3".to_string()).unwrap(), 2.0);
}

#[test]
fn test_multiplication() {
    assert_eq!(evaluate("4 * 2".to_string()).unwrap(), 8.0);
}

#[test]
fn test_division() {
    assert_eq!(evaluate("8 / 2".to_string()).unwrap(), 4.0);
}

#[test]
fn test_precedence() {
    assert_eq!(evaluate("2 + 3 * 4".to_string()).unwrap(), 14.0);
}
