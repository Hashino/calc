use super::Calculator;

fn eval(expression: &str) -> f64 {
    Calculator::evaluate(expression.to_string()).unwrap()
}

fn eval_err(expression: &str) -> bool {
    Calculator::evaluate(expression.to_string()).is_err()
}

#[test]
fn test_addition() {
    assert_eq!(eval("2 + 3"), 5.0);
    assert_eq!(eval("1 + 2 + 3"), 6.0);
}

#[test]
fn test_subtraction() {
    assert_eq!(eval("5 - 3"), 2.0);
    assert_eq!(eval("10 - 2 - 3"), 5.0);
}

#[test]
fn test_multiplication() {
    assert_eq!(eval("4 * 2"), 8.0);
    assert_eq!(eval("2 * 3 * 4"), 24.0);
}

#[test]
fn test_division() {
    assert_eq!(eval("8 / 2"), 4.0);
    assert_eq!(eval("20 / 5 / 2"), 2.0);
}

#[test]
fn test_precedence() {
    assert_eq!(eval("2 + 3 * 4"), 14.0);
    assert_eq!(eval("10 - 2 / 2"), 9.0);
    assert_eq!(eval("2 + 3 * 4 - 5 / 5"), 13.0);
}

#[test]
fn test_parentheses() {
    assert_eq!(eval("(2 + 3) * 4"), 20.0);
}

#[test]
fn test_negative() {
    assert_eq!(eval("10 * -1"), -10.0);
}

#[test]
fn test_exponentiation() {
    assert_eq!(eval("2 ^ 3"), 8.0);
}

#[test]
fn test_modulo() {
    assert_eq!(eval("10 % 3"), 1.0);
}

#[test]
fn test_logarithm() {
    assert_eq!(eval("100 log 10"), 2.0);
}

#[test]
fn test_factorial() {
    assert_eq!(eval("5 !"), 120.0);
}

#[test]
fn test_square_root() {
    assert_eq!(eval("sqrt 16"), 4.0);
}

#[test]
fn test_sine() {
    assert_eq!(eval("sin 90"), 0.8939966636005579);
}

#[test]
fn test_cosine() {
    assert_eq!(eval("cos 0"), 1.0);
}

#[test]
fn test_tangent() {
    assert_eq!(eval("tan 45"), 1.6197751905438615);
}

#[test]
fn test_natural_log() {
    assert_eq!(eval("ln 2.718281828459045"), 1.0);
}

#[test]
fn test_complex_expression() {
    assert_eq!(eval("3 + 5 * 2 - 4 / 2 ^ 2"), 12.0);
    assert_eq!(eval("sqrt 16 + 3 ! - 2 ^ 3"), 2.0);
    assert_eq!(eval("10 log 10 + sin 90 * 2"), 2.7879933272011157);
}

#[test]
fn test_last_result_feature() {
    assert_eq!(eval("10 - 5"), 5.0);
    assert_eq!(eval("- 2"), 3.0);
    assert_eq!(eval("sqrt"), 1.7320508075688772);
    assert_eq!(eval("2 + 3"), 5.0);
}

#[test]
fn test_last_result_only_at_beginning() {
    assert_eq!(eval("10 - 5"), 5.0);
    assert!(eval_err("2 *"));
    assert!(eval_err("2 +"));
}

#[test]
fn test_nested_factorial() {
    assert_eq!(eval("(2!)!"), 2.0);
    assert_eq!(eval("(3!)!"), 720.0);
    assert_eq!(eval("3!!"), 720.0);
}

#[test]
fn test_division_by_zero() {
    assert!(eval_err("5 / 0"));
    assert!(eval_err("0 / 0"));
}

#[test]
fn test_factorial_edge_cases() {
    assert!(eval_err("2.5!"));
    assert_eq!(eval("0!"), 1.0);
}

#[test]
fn test_sqrt_negative() {
    assert!(eval_err("sqrt -1"));
    assert!(eval_err("sqrt -4"));
}

#[test]
fn test_ln_edge_cases() {
    assert!(eval_err("ln 0"));
    assert!(eval_err("ln -1"));
}

#[test]
fn test_log_edge_cases() {
    assert!(eval_err("10 log 1")); // base 1
    assert!(eval_err("-10 log 10")); // negative argument
    assert!(eval_err("10 log -10")); // negative base
}

#[test]
fn test_parsing_errors() {
    assert!(eval_err("")); // empty string
    assert!(eval_err("abc")); // invalid characters
    assert!(eval_err("2 +")); // incomplete expression
    assert!(eval_err("(2 + 3")); // unmatched parentheses
    assert!(eval_err("2 + 3)")); // extra closing paren
}

#[test]
fn test_modulo_edge_cases() {
    assert_eq!(eval("7 % 3"), 1.0);
    assert_eq!(eval("7.5 % 2"), 1.5);
}

#[test]
fn test_large_numbers() {
    assert_eq!(eval("20!"), 2432902008176640000.0);
    assert_eq!(eval("2 ^ 100"), 1.2676506002282294e30);
    assert!(eval("10 ^ 1000").is_infinite());
}

#[test]
fn test_floor() {
    assert_eq!(eval("floor 3.7"), 3.0);
    assert_eq!(eval("floor (0 - 2.3)"), -3.0);
    assert_eq!(eval("floor 5.0"), 5.0);
    assert_eq!(eval("floor (0 - 5.0)"), -5.0);
}

#[test]
fn test_ceil() {
    assert_eq!(eval("ceil 3.2"), 4.0);
    assert_eq!(eval("ceil (0 - 2.7)"), -2.0);
    assert_eq!(eval("ceil 5.0"), 5.0);
    assert_eq!(eval("ceil (0 - 5.0)"), -5.0);
}

#[test]
fn test_abs() {
    assert_eq!(eval("abs 5"), 5.0);
    assert_eq!(eval("abs (0 - 5)"), 5.0);
    assert_eq!(eval("abs 0"), 0.0);
    assert_eq!(eval("abs (0 - 3.7)"), 3.7);
}

#[test]
fn test_round() {
    assert_eq!(eval("round 3.4"), 3.0);
    assert_eq!(eval("round 3.6"), 4.0);
    assert_eq!(eval("round (0 - 2.4)"), -2.0);
    assert_eq!(eval("round (0 - 2.6)"), -3.0);
    assert_eq!(eval("round 5.0"), 5.0);
}

#[test]
fn test_new_functions_complex() {
    assert_eq!(eval("abs (floor (0 - 3.7))"), 4.0);
    assert_eq!(eval("ceil (sqrt 15)"), 4.0);
    assert_eq!(eval("round 2.5 + 1"), 4.0);
}

#[test]
fn test_pi_constant() {
    let result = eval("pi");
    assert!((result - std::f64::consts::PI).abs() < 1e-10);
}

#[test]
fn test_e_constant() {
    let result = eval("e");
    assert!((result - std::f64::consts::E).abs() < 1e-10);
}

#[test]
fn test_constants_in_expressions() {
    let result = eval("2 * pi");
    assert!((result - 2.0 * std::f64::consts::PI).abs() < 1e-10);

    let result = eval("e ^ 2");
    assert!((result - std::f64::consts::E.powi(2)).abs() < 1e-10);

    let result = eval("sin pi");
    assert!(result.abs() < 1e-10); // sin(π) ≈ 0
}
