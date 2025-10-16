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
fn test_exponentiation() {
    assert_eq!(evaluate("2 ^ 3".to_string()).unwrap(), 8.0);
}

#[test]
fn test_modulo() {
    assert_eq!(evaluate("10 % 3".to_string()).unwrap(), 1.0);
}

#[test]
fn test_logarithm() {
    assert_eq!(evaluate("100 log 10".to_string()).unwrap(), 2.0);
}

#[test]
fn test_factorial() {
    assert_eq!(evaluate("5 !".to_string()).unwrap(), 120.0);
}

#[test]
fn test_square_root() {
    assert_eq!(evaluate("sqrt 16".to_string()).unwrap(), 4.0);
}

#[test]
fn test_sine() {
    assert_eq!(evaluate("sin 90".to_string()).unwrap(), 0.8939966636005579);
}

#[test]
fn test_cosine() {
    assert_eq!(evaluate("cos 0".to_string()).unwrap(), 1.0);
}

#[test]
fn test_tangent() {
    assert_eq!(evaluate("tan 45".to_string()).unwrap(), 1.6197751905438615);
}

#[test]
fn test_natural_log() {
    assert_eq!(evaluate("ln 2.718281828459045".to_string()).unwrap(), 1.0);
}

#[test]
fn test_complex_expression() {
    assert_eq!(
        evaluate("3 + 5 * 2 - 4 / 2 ^ 2".to_string()).unwrap(),
        12.0
    );
    assert_eq!(evaluate("sqrt 16 + 3 ! - 2 ^ 3".to_string()).unwrap(), 2.0);
    assert_eq!(
        evaluate("10 log 10 + sin 90 * 2".to_string()).unwrap(),
        2.7879933272011157
    );
}

#[test]
fn test_last_result_feature() {
    assert_eq!(evaluate("10 - 5".to_string()).unwrap(), 5.0);
    assert_eq!(evaluate("- 2".to_string()).unwrap(), 3.0);
    assert_eq!(evaluate("sqrt".to_string()).unwrap(), 1.7320508075688772);
    assert_eq!(evaluate("2 + 3".to_string()).unwrap(), 5.0);
}

#[test]
fn test_last_result_only_at_beginning() {
    assert_eq!(evaluate("10 - 5".to_string()).unwrap(), 5.0);
    assert!(evaluate("2 *".to_string()).is_err());
    assert!(evaluate("2 +".to_string()).is_err());
}

#[test]
fn test_nested_factorial() {
    assert_eq!(evaluate("(2!)!".to_string()).unwrap(), 2.0);
    assert_eq!(evaluate("(3!)!".to_string()).unwrap(), 720.0);
    assert_eq!(evaluate("3!!".to_string()).unwrap(), 720.0);
}
