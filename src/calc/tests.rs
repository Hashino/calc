use super::Calculator;

#[test]
fn test_addition() {
    assert_eq!(Calculator::evaluate("2 + 3".to_string()).unwrap(), 5.0);
}

#[test]
fn test_subtraction() {
    assert_eq!(Calculator::evaluate("5 - 3".to_string()).unwrap(), 2.0);
}

#[test]
fn test_multiplication() {
    assert_eq!(Calculator::evaluate("4 * 2".to_string()).unwrap(), 8.0);
}

#[test]
fn test_division() {
    assert_eq!(Calculator::evaluate("8 / 2".to_string()).unwrap(), 4.0);
}

#[test]
fn test_precedence() {
    assert_eq!(Calculator::evaluate("2 + 3 * 4".to_string()).unwrap(), 14.0);
}

#[test]
fn test_parentheses() {
    assert_eq!(
        Calculator::evaluate("(2 + 3) * 4".to_string()).unwrap(),
        20.0
    );
}

// #[test]
// fn test_negative() {
//     assert_eq!(Calculator::evaluate("10 * -1".to_string()).unwrap(), -10.0);
// }
//
// #[test]
// fn test_exponentiation() {
//     assert_eq!(Calculator::evaluate("2 ^ 3".to_string()).unwrap(), 8.0);
// }
//
// #[test]
// fn test_modulo() {
//     assert_eq!(Calculator::evaluate("10 % 3".to_string()).unwrap(), 1.0);
// }
//
// #[test]
// fn test_logarithm() {
//     assert_eq!(Calculator::evaluate("100 log 10".to_string()).unwrap(), 2.0);
// }
//
// #[test]
// fn test_factorial() {
//     assert_eq!(Calculator::evaluate("5 !".to_string()).unwrap(), 120.0);
// }
//
// #[test]
// fn test_square_root() {
//     assert_eq!(Calculator::evaluate("sqrt 16".to_string()).unwrap(), 4.0);
// }
//
// #[test]
// fn test_sine() {
//     assert_eq!(
//         Calculator::evaluate("sin 90".to_string()).unwrap(),
//         0.8939966636005579
//     );
// }
//
// #[test]
// fn test_cosine() {
//     assert_eq!(Calculator::evaluate("cos 0".to_string()).unwrap(), 1.0);
// }
//
// #[test]
// fn test_tangent() {
//     assert_eq!(
//         Calculator::evaluate("tan 45".to_string()).unwrap(),
//         1.6197751905438615
//     );
// }
//
// #[test]
// fn test_natural_log() {
//     assert_eq!(
//         Calculator::evaluate("ln 2.718281828459045".to_string()).unwrap(),
//         1.0
//     );
// }
//
// #[test]
// fn test_complex_expression() {
//     assert_eq!(
//         Calculator::evaluate("3 + 5 * 2 - 4 / 2 ^ 2".to_string()).unwrap(),
//         12.0
//     );
//     assert_eq!(
//         Calculator::evaluate("sqrt 16 + 3 ! - 2 ^ 3".to_string()).unwrap(),
//         2.0
//     );
//     assert_eq!(
//         Calculator::evaluate("10 log 10 + sin 90 * 2".to_string()).unwrap(),
//         2.7879933272011157
//     );
// }
//
// #[test]
// fn test_last_result_feature() {
//     assert_eq!(Calculator::evaluate("10 - 5".to_string()).unwrap(), 5.0);
//     assert_eq!(Calculator::evaluate("- 2".to_string()).unwrap(), 3.0);
//     assert_eq!(
//         Calculator::evaluate("sqrt".to_string()).unwrap(),
//         1.7320508075688772
//     );
//     assert_eq!(Calculator::evaluate("2 + 3".to_string()).unwrap(), 5.0);
// }
//
// #[test]
// fn test_last_result_only_at_beginning() {
//     assert_eq!(Calculator::evaluate("10 - 5".to_string()).unwrap(), 5.0);
//     assert!(Calculator::evaluate("2 *".to_string()).is_err());
//     assert!(Calculator::evaluate("2 +".to_string()).is_err());
// }
//
// #[test]
// fn test_nested_factorial() {
//     assert_eq!(Calculator::evaluate("(2!)!".to_string()).unwrap(), 2.0);
//     assert_eq!(Calculator::evaluate("(3!)!".to_string()).unwrap(), 720.0);
//     assert_eq!(Calculator::evaluate("3!!".to_string()).unwrap(), 720.0);
// }
//
// #[test]
// fn test_division_by_zero() {
//     assert!(Calculator::evaluate("5 / 0".to_string()).is_err());
//     assert!(Calculator::evaluate("0 / 0".to_string()).is_err());
// }
//
// #[test]
// fn test_factorial_edge_cases() {
//     assert!(Calculator::evaluate("2.5!".to_string()).is_err());
//     assert!(Calculator::evaluate("0!".to_string()).unwrap() == 1.0);
// }
//
// #[test]
// fn test_sqrt_negative() {
//     assert!(Calculator::evaluate("sqrt -1".to_string()).is_err());
//     assert!(Calculator::evaluate("sqrt -4".to_string()).is_err());
// }
//
// #[test]
// fn test_ln_edge_cases() {
//     assert!(Calculator::evaluate("ln 0".to_string()).is_err());
//     assert!(Calculator::evaluate("ln -1".to_string()).is_err());
// }
//
// #[test]
// fn test_log_edge_cases() {
//     assert!(Calculator::evaluate("10 log 1".to_string()).is_err()); // base 1
//     assert!(Calculator::evaluate("-10 log 10".to_string()).is_err()); // negative argument
//     assert!(Calculator::evaluate("10 log -10".to_string()).is_err()); // negative base
// }
//
// #[test]
// fn test_parsing_errors() {
//     assert!(Calculator::evaluate("".to_string()).is_err()); // empty string
//     assert!(Calculator::evaluate("abc".to_string()).is_err()); // invalid characters
//     assert!(Calculator::evaluate("2 +".to_string()).is_err()); // incomplete expression
//     assert!(Calculator::evaluate("(2 + 3".to_string()).is_err()); // unmatched parentheses
//     assert!(Calculator::evaluate("2 + 3)".to_string()).is_err()); // extra closing paren
// }
//
// #[test]
// fn test_modulo_edge_cases() {
//     assert_eq!(Calculator::evaluate("7 % 3".to_string()).unwrap(), 1.0);
//     assert_eq!(Calculator::evaluate("7.5 % 2".to_string()).unwrap(), 1.5);
// }
//
// #[test]
// fn test_large_numbers() {
//     // Large factorial - 20! is 2432902008176640000, within u64
//     assert_eq!(
//         Calculator::evaluate("20!".to_string()).unwrap(),
//         2432902008176640000.0
//     );
//     // Large exponentiation
//     assert_eq!(
//         Calculator::evaluate("2 ^ 100".to_string()).unwrap(),
//         1.2676506002282294e30
//     );
//     // Very large exponent might cause inf
//     assert!(
//         Calculator::evaluate("10 ^ 1000".to_string())
//             .unwrap()
//             .is_infinite()
//     );
// }
//
// #[test]
// fn test_floor() {
//     assert_eq!(Calculator::evaluate("floor 3.7".to_string()).unwrap(), 3.0);
//     assert_eq!(
//         Calculator::evaluate("floor (0 - 2.3)".to_string()).unwrap(),
//         -3.0
//     );
//     assert_eq!(Calculator::evaluate("floor 5.0".to_string()).unwrap(), 5.0);
//     assert_eq!(
//         Calculator::evaluate("floor (0 - 5.0)".to_string()).unwrap(),
//         -5.0
//     );
// }
//
// #[test]
// fn test_ceil() {
//     assert_eq!(Calculator::evaluate("ceil 3.2".to_string()).unwrap(), 4.0);
//     assert_eq!(
//         Calculator::evaluate("ceil (0 - 2.7)".to_string()).unwrap(),
//         -2.0
//     );
//     assert_eq!(Calculator::evaluate("ceil 5.0".to_string()).unwrap(), 5.0);
//     assert_eq!(
//         Calculator::evaluate("ceil (0 - 5.0)".to_string()).unwrap(),
//         -5.0
//     );
// }
//
// #[test]
// fn test_abs() {
//     assert_eq!(Calculator::evaluate("abs 5".to_string()).unwrap(), 5.0);
//     assert_eq!(
//         Calculator::evaluate("abs (0 - 5)".to_string()).unwrap(),
//         5.0
//     );
//     assert_eq!(Calculator::evaluate("abs 0".to_string()).unwrap(), 0.0);
//     assert_eq!(
//         Calculator::evaluate("abs (0 - 3.7)".to_string()).unwrap(),
//         3.7
//     );
// }
//
// #[test]
// fn test_round() {
//     assert_eq!(Calculator::evaluate("round 3.4".to_string()).unwrap(), 3.0);
//     assert_eq!(Calculator::evaluate("round 3.6".to_string()).unwrap(), 4.0);
//     assert_eq!(
//         Calculator::evaluate("round (0 - 2.4)".to_string()).unwrap(),
//         -2.0
//     );
//     assert_eq!(
//         Calculator::evaluate("round (0 - 2.6)".to_string()).unwrap(),
//         -3.0
//     );
//     assert_eq!(Calculator::evaluate("round 5.0".to_string()).unwrap(), 5.0);
// }
//
// #[test]
// fn test_new_functions_complex() {
//     assert_eq!(
//         Calculator::evaluate("abs (floor (0 - 3.7))".to_string()).unwrap(),
//         4.0
//     );
//     assert_eq!(
//         Calculator::evaluate("ceil (sqrt 15)".to_string()).unwrap(),
//         4.0
//     );
//     assert_eq!(
//         Calculator::evaluate("round 2.5 + 1".to_string()).unwrap(),
//         4.0
//     );
// }
//
// #[test]
// fn test_pi_constant() {
//     let result = Calculator::evaluate("pi".to_string()).unwrap();
//     assert!((result - std::f64::consts::PI).abs() < 1e-10);
// }
//
// #[test]
// fn test_e_constant() {
//     let result = Calculator::evaluate("e".to_string()).unwrap();
//     assert!((result - std::f64::consts::E).abs() < 1e-10);
// }
//
// #[test]
// fn test_constants_in_expressions() {
//     let result = Calculator::evaluate("2 * pi".to_string()).unwrap();
//     assert!((result - 2.0 * std::f64::consts::PI).abs() < 1e-10);
//
//     let result = Calculator::evaluate("e ^ 2".to_string()).unwrap();
//     assert!((result - std::f64::consts::E.powi(2)).abs() < 1e-10);
//
//     let result = Calculator::evaluate("sin pi".to_string()).unwrap();
//     assert!(result.abs() < 1e-10); // sin(π) ≈ 0
// }
